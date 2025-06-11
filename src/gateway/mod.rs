#[cfg(test)]
mod tests;


use std::{sync::Arc, time::Duration};

use futures_util::{lock::Mutex, SinkExt, StreamExt};
use serde::Deserialize;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::{self, Message}, MaybeTlsStream, WebSocketStream};

use crate::models::gateway::{events::{identify::Identify, EventData}, opcodes::Opcode, websocket_message::WebsocketMessage};

#[derive(Debug, Clone)]
pub struct Shard {
    ws_stream: Arc<Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
    heartbeat_interval: usize,
    last_heartbeat_value: Option<usize>,
}

impl Shard {
    pub async fn new(client: &reqwest::Client, token: String) -> Result<Arc<Mutex<Self>>, ShardError> {
        let response = match client.get("https://discord.com/api/v9/gateway").build() {
            Ok(req) => client.execute(req).await.unwrap().json::<GatewayUrlResponse>().await.unwrap(),
            Err(err) => panic!("Could not get gateway url: {:?}", err),
        };
        

        let (mut ws_stream, _) = connect_async(&response.url).await.expect("Failed to connect to websocket");
       
        let hello_msg = match ws_stream.next().await {
            Some(st) => match st {
                Ok(msg) => msg,
                Err(err) => return Err(ShardError::TungsteniteError(err)),
            },
            None => return Err(ShardError::NoMessage),
        };

        let hello: WebsocketMessage = match serde_json::from_str(&hello_msg.to_string()) {
            Ok(json) => json,
            Err(err) => return Err(ShardError::JsonError(err)),
        };

        let heartbeat_interval = match hello.data {
            Some(data) => match data {
               EventData::HelloData(data) => data.heartbeat_interval,
               _ => return Err(ShardError::NoMessage)
            },
            None => return Err(ShardError::NoMessage)
        };

        let shard = Arc::new(Mutex::new(Self {
            ws_stream: Arc::new(Mutex::new(ws_stream)),
            heartbeat_interval,
            last_heartbeat_value: None,
        }));
        
        let cloned_shard = shard.clone();
        tokio::spawn(async move {
            let mut mutex = cloned_shard.lock().await;
            mutex.hearbeat().await;
            let interval = mutex.heartbeat_interval;
            drop(mutex);
            tokio::time::sleep(Duration::from_millis(interval as u64)).await;
        });

        if let Err(err) = shard.lock().await.identify(token).await {
            return Err(err); 
        };

        Ok(shard)
    }

    async fn identify(&self, token: String) -> Result<(), ShardError> {
       
        let ident = Identify::new(token, None, None, None, None, None, 5);

        let msg = WebsocketMessage::new(Opcode::Identify, Some(EventData::IdentifyData(ident)));
       
        let json_out = match serde_json::to_string(&msg) {
            Err(err) => return Err(ShardError::JsonError(err)),
            Ok(string) => string,
        };
        
        println!("{}", &json_out);

        let mut stream = self.ws_stream.lock().await;

        match stream.send(Message::Text(json_out.into())).await {
            Err(err) => return Err(ShardError::TungsteniteError(err)),
            Ok(_) => {},
        }
        
        println!("{:?}", stream.next().await);

        Ok(())
    }

    async fn hearbeat(&mut self) {
        let msg = WebsocketMessage::new(Opcode::Heartbeat, Some(EventData::Heartbeat(self.last_heartbeat_value)));
        let mut stream = self.ws_stream.lock().await;
        let json = serde_json::to_string(&msg).unwrap();
        stream.send(Message::Text(json.into())).await.unwrap();

        let b = stream.next().await;
        drop(stream);

        let msg = b.unwrap().unwrap();

        let hello: WebsocketMessage = serde_json::from_str(&msg.to_string())
            .expect("Could not convert message to json");

        self.last_heartbeat_value = match hello.data {
            Some(data) => match data {
               EventData::Heartbeat(val) => val,
               _ => self.last_heartbeat_value,
            },
            None => self.last_heartbeat_value,
        };
    }
}

#[derive(Deserialize)]
struct GatewayUrlResponse {
    url: String,
}

#[derive(Debug)]
pub enum ShardError {
    JsonError(serde_json::Error),
    TungsteniteError(tungstenite::Error),
    NoMessage,
}
