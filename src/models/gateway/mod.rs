pub mod opcodes;
pub mod event;
pub mod types;

#[cfg(test)]
mod tests;

use futures_util::{future, pin_mut, StreamExt};
use serde::Deserialize;
use tokio_tungstenite::connect_async;

#[derive(Debug, Clone)]
pub struct Shard {

}

impl Shard {
    pub async fn new(client: &reqwest::Client) -> Self {
        let response = match client.get("https://discord.com/api/v9/gateway").build() {
            Ok(req) => client.execute(req).await.unwrap().json::<GatewayUrlResponse>().await.unwrap(),
            Err(err) => panic!("Could not get gateway url: {:?}", err),
        };
        

        let (ws_stream, _) = connect_async(&response.url).await.expect("Failed to connect to websocket");
        

        Self {}
    }

    fn hearbeat(&self) {

    }
}

#[derive(Deserialize)]
struct GatewayUrlResponse {
    url: String,
}
