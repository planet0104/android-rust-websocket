package com.senmiao.app.websocket;

/**
 * Created by days888 on 2017/11/1.
 */

public class Message {
    private Object data;
    public String getText(){
        if(isText())
            return (String)data;
        else
            return null;
    }
    public byte[] getData(){
        if(isBinary())
            return (byte[])data;
        else
            return null;
    }

    public boolean isText(){
        return data!=null && data instanceof String;
    }

    public boolean isBinary(){
        return data !=null && data instanceof byte[];
    }

    public boolean isEmpty(){
        return data==null;
    }
}
