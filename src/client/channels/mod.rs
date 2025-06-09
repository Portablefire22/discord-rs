use log::error;
use crate::models::{channels::Channel, messages::{self, Message}};
use super::Client;

#[cfg(test)]
mod tests;

impl Client {

    pub async fn get_channel(&self, channel_id: String) -> Result<Channel, reqwest::Error> {
        let url: String = format!("https://discord.com/api/v9/channels/{}", channel_id);
        let res = self.http.get(url).build();
        match res {
            Ok(req) => self.http.execute(req)
                .await
                .unwrap()
                .json::<Channel>()
                .await,
            Err(req) => {
                error!("Could not get channel: {:?}", req);
                Err(req)
            }
        }
    }

    pub async fn get_channel_messages(&self, channel_id: String, limit: Option<usize>) -> Result<Vec<messages::Message>, reqwest::Error> {
        let url: String = format!("https://discord.com/api/v9/channels/{}/messages?limit={}", 
            channel_id, 
            limit.unwrap_or(50));
        let res = self.http.get(url).build();
        match res {
            Ok(request) => self.http.execute(request)
                .await
                .unwrap()
                .json::<Vec<messages::Message>>()
                .await,
            Err(request) => {
                error!("Could not get messages: {:?}", request);
                Err(request)
            }
        }
    }

    pub async fn send_message<S: AsRef<str> + std::fmt::Display>(&self, channel_id: S, msg: &Message) -> Result<Message, SendMessageError> {
        let url = format!("https://discord.com/api/v9/channels/{}/messages", channel_id);
        let body = serde_json::to_string(msg);
        let response = match body {
            Ok(body) => {
                let res = self.http.post(url).body(body).build();
                match res {
                    Ok(req) => {
                        self.http.execute(req).await
                    }
                    Err(req) => {
                        return Err(SendMessageError::RequestError(req))
                    }
                }
            }
            Err(err) => {
                return Err(SendMessageError::JsonError(err))
            }
        };
        
        match response {
            Ok(res) => {
                match res.error_for_status() {
                    Ok(res) => {
                        match res.json::<Message>().await {
                            Ok(msg) => Ok(msg),
                            Err(err) => Err(SendMessageError::RequestError(err))
                        }
                    },
                    Err(err) => Err(SendMessageError::RequestError(err))
                } 
            }
            Err(err) => Err(SendMessageError::RequestError(err)),
        }
    } 

}

#[derive(Debug)]
pub enum SendMessageError {
    RequestError(reqwest::Error),
    JsonError(serde_json::Error),
}
