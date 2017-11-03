# android_rust_websocket
ws-rs bindings and wrappers for Android

在Android中调用ws-rs实现native WebSocket链接。
目前仅实现了客户端链接的功能。

首先安装Rust。
编译:
1. 下载NDK Android，配置工具链
    sh E:\android\android-ndk-r10e\build\tools\make-standalone-toolchain.sh
    (具体的去Google..)

2.配置 cargo/config
    [target.arm-linux-androideabi]
    ar = "E:/ndk-standalone-16-arm/bin/arm-linux-androideabi-ar.exe"
    linker = "E:/ndk-standalone-16-arm/bin/arm-linux-androideabi-gcc.exe"
3.Rust添加编译目标
    >rustup target add arm-linux-androideabi
4.进入的rust项目文件夹, 编译:
    cargo build --target arm-linux-androideabi

5.将target\arm-linux-androideabi\release\wslib.so 复制到Android项目libs里就可以使用了

6.注意rust的jni接口中，Java_com_senmiao_app_websocket_WebSocket_connect 对应java中的包名方法名要一致。
    
    
 
   
   
