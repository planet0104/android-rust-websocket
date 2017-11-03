extern crate ws;
use ws::{connect, Handler, Sender, Handshake, Result, Message, CloseCode,Error};
use ws::util::Token;
#[macro_use] extern crate log;
extern crate android_logger;
use log::LogLevel;

//定义Android JNI接口
#[cfg(target_os="android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;
    use super::*;
    use self::jni::{JNIEnv};
    use self::jni::objects::{JObject, JValue, JString};
    use self::jni::sys::{jstring, jint, jboolean, jlong, jobject, jbyteArray};

    struct Client<'a>{
        sender: Sender,
        env:&'a JNIEnv<'a>,
        this:&'a JObject<'a>,
        callback: JObject<'a>,
    }

    impl <'a> Handler for Client<'a> {
        fn on_open(&mut self, _: Handshake) -> Result<()> {
            info!("on_open触发");
            //将Client指针保存到WebSocket对象中
            let p:*const Client = self;
            let client_pointer = p as i64;
            if let Err(err) = set_client_ptr(self.env, self.this, client_pointer){
                error!("设置client_ptr: {:?}", err);
                on_error(self.env, &self.callback, format!("set_field client error:{:?}", err));
                
                //关闭连接
                self.sender.shutdown().unwrap_or_else(|err|{
                    error!("sender.shutdown: {:?}", err);
                });
                //返回
                return Ok(());
            }

            //非连接中
            set_connecting(self.env, self.this, false);
            //通知 java handler 已连接
            self.env.call_method(self.callback, "onOpen", "()V", &[]).unwrap_or_else(|err|{
                error!("onOpen调用出错 {:?}", err);
                JValue::from(())
            });
            Ok(())
        }

        fn on_close(&mut self, code: CloseCode, reason: &str){
            info!("on_close触发{:?}", reason);

            //因为手动调用sender.close()将会触发两次on_close，这里判断一下
            //如果服务器已经已关闭，不再通知onClose
            match get_client(self.env, self.this){
                Err(err) => {
                    error!("on_close 服务器未连接.: {:?}", err);
                }
                Ok(_) => {
                    set_client_ptr(self.env, self.this, 0).unwrap_or_else(|err|{
                        error!("设置client_ptr: {:?}", err);
                    });
                    //服务器关闭时、调用disconnect时触发
                    //通知 java handler连接关闭
                    let reason_str = self.env.new_string(reason).expect("Couldn't create java string!").into_inner();
                    let close_code:u16 = code.into();
                    self.env.call_method(self.callback, "onClose", "(ILjava/lang/String;)V", &[JValue::from(close_code as jint), JValue::from(JObject::from(reason_str))]).unwrap_or_else(|err|{
                        error!("onClose调用出错 {:?}", err);
                        JValue::from(())
                    });
                }
            }
        }

        fn on_timeout(&mut self, _event: Token) -> Result<()> {
            info!("on_timeout 触发");
            //链接超时触发
            //通知 java handler 超时
            self.env.call_method(self.callback, "onTimeout", "()V", &[]).unwrap_or_else(|err|{
                error!("onTimeout调用出错 {:?}", err);
                JValue::from(())
            });
            //关闭链接
            self.sender.shutdown().unwrap_or_else(|err|{
                error!("sender.shutdown: {:?}", err);
            });
            set_connecting(self.env, self.this, false);
            Ok(())
        }

        fn on_error(&mut self, err: Error) {
            info!("on_error 触发{:?}", err);
            //链接超时、服务器关闭时触发
            on_error(self.env, &self.callback, format!("链接失败: {:?}", err));
        }

        fn on_message(&mut self, msg: Message) -> Result<()> {
            info!("on_message 触发 {:?}", msg);
            //创建com.senmiao.app.websocket.Message对象
            match self.env.new_object("com/senmiao/app/websocket/Message", "()V", &[]){
                Ok(jobj) => {
                    let data:JValue =
                        match msg{
                            Message::Text(string) => {
                                let jstr = self.env.new_string(string).expect("Message读取错误").into_inner();
                                JValue::from(JObject::from(jstr))
                            }
                            Message::Binary(data_u8) => {
                                if let Ok(jarray)=self.env.byte_array_from_slice(data_u8.as_slice()){
                                    JValue::from(JObject::from(jarray))
                                }else{
                                    JValue::from(())
                                }
                            }
                        };
                    //设置字段
                    self.env.set_field(jobj, "data", "Ljava/lang/Object;", data).unwrap_or_else(|err|{
                        error!("设置isConnecting出错 {:?}", err);
                    });
                    //通知handler 有消息
                    self.env.call_method(self.callback, "onMessage", "(Lcom/senmiao/app/websocket/Message;)V", &[JValue::from(jobj)]).unwrap_or_else(|err|{
                        error!("onMessage调用出错 {:?}", err);
                        JValue::from(())
                    });
                }
                Err(err) => {
                    error!("创建Message对象失败: {:?}", err);
                    on_error(self.env, &self.callback, format!("创建Message对象失败: {:?} message:{:?}", err, msg));
                }
            }
            Ok(())
        }
    }

    //设置连接中状态
    fn set_connecting(env:&JNIEnv, this:&JObject, b:bool)->bool{
        if let Err(err) = env.set_field(*this, "isConnecting", "Z", JValue::from(b)){
            error!("设置isConnecting出错 {:?}", err);
            false
        }else{
            true
        }
    }

    //返回错误信息
    fn on_error(env:&JNIEnv, callback:&JObject, err_str:String){
        let err_str = env.new_string(err_str).expect("Couldn't create java string!").into_inner();
        env.call_method(*callback, "onError", "(Ljava/lang/String;)V", &[JValue::from(JObject::from(err_str))]).unwrap_or_else(|err|{
            error!("onError调用出错 {:?}", err);
            JValue::from(())
        });
    }

    //设置client裸指针
    fn set_client_ptr(env:&JNIEnv, this:&JObject, ptr:i64)->std::result::Result<(), android::jni::errors::Error>{
        env.set_field(*this, "client", "J", JValue::from(ptr as jlong))
    }

    //查看是否已连接
    fn get_client<'a>(env:&JNIEnv, this:&JObject)->std::result::Result<*const Client<'a>, String>{
        let result = env.get_field(*this, "client", "J");
        match result {
            Err(why) => {
                Err(format!("get_client {:?}", why))
            }
            Ok(value) => {
                match value.j(){
                    Ok(i_ptr) => {
                        if i_ptr == 0 {
                            Err(String::from("get_client 连接已断开."))
                        }else{
                            let ptr:*const Client = i_ptr as *const Client;
                            Ok(ptr)
                        }
                    }
                    Err(err) => {
                        Err(format!("get_client {:?}", err))
                    }
                }
            }
        }
    }

    //链接WebSocket服务器
    // 参数:
    // jurl:jstring => 链接地址
    // callback:jobject => 回调函数
    // timeout:jlong => 超时时间ms
    #[no_mangle]
    pub unsafe extern "C" fn Java_com_senmiao_app_websocket_WebSocket_connect(env: JNIEnv, this:jobject, jurl:jstring, callback:jobject, _timeout:jlong){
        android_logger::init_once(LogLevel::Info);

        let this = JObject::from(this);
        let callback = JObject::from(callback);
        set_connecting(&env, &this, true);

        //获取URL
        match env.get_string(JString::from(jurl)){
            Ok(javastr) => {
                let url:String = javastr.into();
                if let Err(err) = connect(url.clone(), |out| {
                    //设置超时时间
                    // out.timeout(timeout as u64, Token(1)).unwrap_or_else(|err|{
                    //     error!("timeout设置失败: {:?}", err);
                    // });
                    
                    Client{
                        sender: out,
                        callback: callback,
                        env:&env,
                        this:&this
                    }
                }){
                    error!("链接失败: {:?}", err);
                    set_connecting(&env, &this, false);
                    //通知 java handler连接出错
                    on_error(&env, &callback, format!("连接失败: {:?}", err));
                }
                //连接断开
                //这里不用设置client指针了，因为connect方法执行完毕会自动重置
                // set_client_ptr(&env, &this, 0).unwrap_or_else(|err|{
                //     error!("set_client_ptr {:?}", err);    
                // });
            }
            Err(err) => {
                error!("URL错误 {:?}", err);
                on_error(&env, &callback, format!("URL转换失败: {:?}", err));
                //这里不用设置connecting状态了，因为connect方法执行完毕会自动重置
                //set_connecting(&env, &this, false);
            }
        }
    }

    //断开连接
    #[no_mangle]
    pub unsafe extern "C" fn Java_com_senmiao_app_websocket_WebSocket_disconnect(env: JNIEnv, this:jobject){
        let this = JObject::from(this);
        match get_client(&env, &this){
            Err(err) => {
                error!("disconnect: {:?}", err);
            }
            Ok(ptr) => {
                (*ptr).sender.close_with_reason(CloseCode::Normal, String::from("用户断开")).unwrap_or_else(|err|{
                    error!("disconnect: {:?}", err);
                });
            }
        }
    }

    //发送文本消息
    #[no_mangle]
    pub unsafe extern "C" fn Java_com_senmiao_app_websocket_WebSocket_sendText(env: JNIEnv, this:jobject, string:jstring)->jboolean{
        let this = JObject::from(this);
        match env.get_string(JString::from(string)){
            Ok(string) => {
                match get_client(&env, &this){
                    Err(err) => {
                        error!("sendText: {:?}", err);
                        false as jboolean
                    }
                    Ok(ptr) => {
                        (*ptr).sender.send(Message::text(string)).unwrap_or_else(|err|{
                            error!("sendText: {:?}", err);
                        });
                        true as jboolean
                    }
                }
            }
            Err(err) => {
                error!("sendText: {:?}", err);
                false as jboolean
            }
        }
    }

    //发送二进制消息
    #[no_mangle]
    pub unsafe extern "C" fn Java_com_senmiao_app_websocket_WebSocket_sendBinary(env: JNIEnv, this:jobject, bytes:jbyteArray)->jboolean{
        let this = JObject::from(this);
        match env.convert_byte_array(bytes) {
            Ok(bytes) => {
                match get_client(&env, &this){
                        Err(err) => {
                            error!("sendBinary: {:?}", err);
                            false as jboolean
                        }
                        Ok(ptr) => {
                            (*ptr).sender.send(Message::binary(bytes)).unwrap_or_else(|err|{
                                error!("sendBinary: {:?}", err);
                            });
                            true as jboolean
                        }
                }
            }
            Err(err) => {
                error!("sendBinary: {:?}", err);
                false as jboolean
            }   
        }
    }
}