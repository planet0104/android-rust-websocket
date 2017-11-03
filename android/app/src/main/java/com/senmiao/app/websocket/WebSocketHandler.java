package com.senmiao.app.websocket;

/**
 * 回调
 * Created by JiaYe on 2017/11/1.
 */

public interface WebSocketHandler {
    public void onOpen();
    public void onMessage(Message message);
    public void onClose(int closeCode, String reason);
    public void onError(String error);
    public void onTimeout();
}
