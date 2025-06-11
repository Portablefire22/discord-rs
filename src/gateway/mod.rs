#[cfg(test)]
mod tests;


use std::sync::Arc;

use futures_util::{lock::Mutex, SinkExt, StreamExt};
use serde::Deserialize;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::{self, Message}, MaybeTlsStream, WebSocketStream};

use crate::models::gateway::{events::identify::{Identify}, opcodes::Opcode, websocket_message::WebsocketMessage};

#[derive(Debug, Clone)]
pub struct Shard {
    ws_stream: Arc<Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
}

impl Shard {
    pub async fn new(client: &reqwest::Client, token: String) -> Result<Self, ShardError> {
        let response = match client.get("https://discord.com/api/v9/gateway").build() {
            Ok(req) => client.execute(req).await.unwrap().json::<GatewayUrlResponse>().await.unwrap(),
            Err(err) => panic!("Could not get gateway url: {:?}", err),
        };
        

        let (ws_stream, _) = connect_async(&response.url).await.expect("Failed to connect to websocket");
        
        let shard = Self {ws_stream: Arc::new(Mutex::new(ws_stream))};
        if let Err(err) = shard.identify(token).await {
            return Err(err); 
        };

        Ok(shard)
    }

    async fn identify(&self, token: String) -> Result<(), ShardError> {
       
        let ident = Identify::new(token, None, None, None, None, None, 5);

        let msg = WebsocketMessage::new(Opcode::Identify, Some(ident));
       
        let json_out = match serde_json::to_string(&msg) {
            Err(err) => return Err(ShardError::JsonError(err)),
            Ok(string) => string,
        };
        
        let mut stream = self.ws_stream.lock().await;

        match stream.send(Message::Text(json_out.into())).await {
            Err(err) => return Err(ShardError::TungsteniteError(err)),
            Ok(_) => {},
        }
        
        println!("{:?}", stream.next().await);

        Ok(())
    }

    fn hearbeat(&self) {

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
}
