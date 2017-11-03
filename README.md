# android_rust_websocket
ws-rs bindings and wrappers for Android

在Android中调用ws-rs实现native WebSocket链接。<br/>
目前仅实现了客户端链接的功能。<br/>

首先安装Rust。<br/>
编译:<br/>
1. 下载NDK Android，配置工具链<br/>
    sh E:\android\android-ndk-r10e\build\tools\make-standalone-toolchain.sh<br/>
    (具体的去Google..)<br/>

2.配置 cargo/config<br/>
    [target.arm-linux-androideabi] <br/>
    ar = "E:/ndk-standalone-16-arm/bin/arm-linux-androideabi-ar.exe"<br/>
    linker = "E:/ndk-standalone-16-arm/bin/arm-linux-androideabi-gcc.exe"<br/>
3.Rust添加编译目标<br/>
    >rustup target add arm-linux-androideabi<br/>
4.进入的rust项目文件夹, 编译:<br/>
    cargo build --target arm-linux-androideabi<br/>

5.将target\arm-linux-androideabi\release\wslib.so 复制到Android项目libs里就可以使用了<br/>

6.注意rust的jni接口中，Java_com_senmiao_app_websocket_WebSocket_connect 对应java中的包名方法名要一致。<br/>
    
    
 
   
   
