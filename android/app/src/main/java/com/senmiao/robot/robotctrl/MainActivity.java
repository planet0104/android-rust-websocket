package com.senmiao.robot.robotctrl;

import android.app.Activity;
import android.os.Bundle;
import android.support.annotation.Nullable;
import android.view.View;
import android.widget.Button;

import com.senmiao.app.websocket.Message;
import com.senmiao.app.websocket.WebSocket;
import com.senmiao.app.websocket.WebSocketHandler;

public class MainActivity extends Activity implements  WebSocketHandler{
    WebSocket webSocket;
    Button btn_connect;
    Button btn_send;
    Button btn_stop;

    @Override
    protected void onCreate(@Nullable Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        btn_connect = findViewById(R.id.btn_connect);
        btn_send = findViewById(R.id.btn_send);
        btn_stop = findViewById(R.id.btn_stop);
        
        webSocket = new WebSocket("ws://127.0.0.1:8081", this);

        btn_send.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                webSocket.sendMessage(new byte[]{0x1, 0x2});
                webSocket.sendMessage("哈喽 android!");
            }
        });

        btn_stop.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                webSocket.close();
            }
        });

        btn_connect.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                webSocket.connect();
            }
        });
    }

    void log(String s){
        System.out.println(s);
    }

    @Override
    protected void onDestroy() {
        webSocket.close();
        super.onDestroy();
    }

    @Override
    public void onOpen() {
        log("onOpen...");
    }

    @Override
    public void onMessage(Message message) {
        if(message.isBinary()){
            log("onMessage 二进制:"+message.getData());
        }else{
            log("onMessage 文本:"+message.getText());
        }
    }

    @Override
    public void onClose(int closeCode, String reason) {
        log("onClose code="+closeCode+" reason="+reason);
    }

    @Override
    public void onError(String error) {
        log("onError... "+error);
    }

    @Override
    public void onTimeout() {
        log("onTimeout");
    }
}
