package com.senmiao.app.websocket;

import android.os.AsyncTask;
import android.util.Log;

/**
 * Created by days888 on 2017/10/31.
 */

public class WebSocket {
    static final String TAG = WebSocket.class.getSimpleName();
    static {
        System.loadLibrary("ws");
    }

    public WebSocket(String url, WebSocketHandler handler){
        this.url = url;
        this.webSocketHandler = handler;
    }

    private String url;

    private long timeout = 5000;
    /**
     * Client指针
     */
    private long client = 0;
    private boolean isConnecting = false;

    private WebSocketHandler webSocketHandler = new WebSocketHandler() {
        @Override
        public void onOpen() {}
        @Override
        public void onMessage(Message message) {}
        @Override
        public void onClose(int closeCode, String reason) {}
        @Override
        public void onTimeout() {}
        @Override
        public void onError(String error) {}
    };

    public boolean isConnected(){
        return client != 0;
    }

    public void setTimeout(long timeout) {
        this.timeout = timeout;
    }

    public void connect(){
        if(webSocketHandler == null){
            Log.e(TAG, "webSocketHandler = null");
            return;
        }
        if(isConnecting){
            Log.w(TAG, "正在连接!");
            return;
        }
        if(isConnected()){
            Log.w(TAG, "已经连接!");
            return;
        }
        new ConnectionTask().execute();
    }

    private native void connect(String url, WebSocketHandler handler, long timeout);
    private native boolean sendText(String message);
    private native boolean sendBinary(byte[] message);
    private native void disconnect();

    public void close(){
        if(isConnecting){
            Log.e(TAG, "正在连接!");
            return;
        }
        if(!isConnected()){
            Log.w(TAG, "未连接!");
            return;
        }
        disconnect();
    }

    public boolean sendMessage(String message){
        if(isConnecting){
            Log.w(TAG, "正在连接中...");
            return false;
        }
        if(!isConnected()){
            Log.w(TAG, "未连接!");
            return false;
        }
        return sendText(message);
    }

    public boolean sendMessage(byte[] message){
        if(isConnecting){
            Log.w(TAG, "正在连接中...");
            return false;
        }
        if(!isConnected()){
            Log.w(TAG, "未连接!");
            return false;
        }
        return sendBinary(message);
    }

    public void setWebSocketHandler(WebSocketHandler webSocketHandler) {
        this.webSocketHandler = webSocketHandler;
    }

    class ConnectionTask extends AsyncTask<Void, Void, Void>{

        @Override
        protected Void doInBackground(Void... objects) {
            connect(url, webSocketHandler, timeout);
            Log.i(TAG, "connect结束.");
            //链接结束重置状态
            isConnecting = false;
            client = 0;
            return null;
        }
    }
}
