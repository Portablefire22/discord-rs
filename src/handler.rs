use log::error;
use reqwest::{header, Client, Error};
use std::{
    borrow::Borrow,
    io::{self, Write},
};

use crate::models::{
    channels::{self, Channel},
    guilds::{self, Guild},
    messages, post_message, users,
};

#[derive(Debug, Clone, Default)]
pub struct MessageHandler {
    authorization_token: String,
    client: reqwest::Client,
}

impl MessageHandler {
    pub fn new(authorization_token: String) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert("Accept", header::HeaderValue::from_static("*/*"));
        headers.insert(
            "Accept-Encoding",
            header::HeaderValue::from_static("gzip, deflate, br"),
        );
        headers.insert(
            "Accept-Language",
            header::HeaderValue::from_static("en-GB,en;q=0.5"),
        );
        headers.insert(
            "Content-Type",
            header::HeaderValue::from_static("application/json"),
        );
        headers.insert("Connection", header::HeaderValue::from_static("keep-alive"));
        headers.insert(
            "Authorization",
            header::HeaderValue::from_str(&authorization_token)
                .expect("Could not create header value from authorization_token"),
        );
        let client = Client::builder()
            .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:124.0) Gecko/20100101 Firefox/124.0")
            .default_headers(headers)
            .build()
            .unwrap();
        Self {
            authorization_token,
            client,
        }
    }
    pub async fn get_current_user_guilds(&self) -> Result<Vec<guilds::Guild>, Error> {
        let url = format!("https://discord.com/api/v9/users/@me/guilds");
        let res = self.client.get(url).build();
        match res {
            Ok(request) => {
                self.client
                    .execute(request)
                    .await
                    .unwrap()
                    .json::<Vec<guilds::Guild>>()
                    .await
            }
            Err(request) => {
                error!("Error sending request!");
                error!("{}", request);
                Err(request)
            }
        }
    }

    pub async fn get_guild_channels(
        &self,
        guild_id: String,
    ) -> Result<Vec<channels::Channel>, Error> {
        let url = format!("https://discord.com/api/v9/guilds/{}/channels", guild_id);
        let res = self.client.get(url).build();
        match res {
            Ok(request) => {
                println!(
                    "Get Guild Channels: {}",
                    self.client
                        .execute(request.try_clone().unwrap())
                        .await
                        .unwrap()
                        //.json::<Vec<guilds::Guild>>()
                        .text()
                        .await
                        .unwrap()
                );
                self.client
                    .execute(request)
                    .await
                    .unwrap()
                    .json::<Vec<channels::Channel>>()
                    .await
            }
            Err(request) => {
                error!("Error sending request!");
                error!("{}", request);
                Err(request)
            }
        }
    }

    pub async fn display_current_user_guilds(&self) {
        let guilds: Vec<Guild> = vec![];
        match self.get_current_user_guilds().await {
            Ok(guilds) => guilds,
            Err(e) => {
                panic!("{}", e);
            }
        };
        for guild in guilds {
            println!("{} | {}", &guild.name, &guild.id);
            let channels: Vec<Channel> = vec![];
            match self.get_guild_channels(guild.id).await {
                Ok(channels) => channels,
                Err(e) => {
                    panic!("{}", e);
                }
            };
            for chan in channels {
                println!(
                    "   {} | {} | {}",
                    &chan.channel_type,
                    &chan.name.unwrap(),
                    &chan.id
                )
            }
        }
    }

    pub async fn send_message(
        &self,
        msg: &post_message::Message,
        channel_id: String,
    ) -> Result<(), reqwest::Error> {
        let body = serde_json::to_string(msg);
        let url = format!(
            "https://discord.com/api/v9/channels/{}/messages",
            channel_id
        );
        match body {
            Ok(body) => {
                let res = self.client.post(url).body(body).build();
                match res {
                    Ok(request) => {
                        println!("{:?}", self.client.execute(request).await.unwrap().status());
                        Ok(())
                    }
                    Err(request) => {
                        error!("Error sending request!");
                        error!("{}", request);
                        Err(request)
                    }
                }
            }
            Err(e) => {
                error!("Error parsing given message to json!");
                error!("{:?}", &msg);
                panic!("{}", e);
            }
        }
    }
    pub async fn dbg_message(&self) {
        print!("Message to send to channel: ");
        std::io::stdout().flush().unwrap();
        let mut content = String::new();
        io::stdin().read_line(&mut content).unwrap();
        content = content.trim().to_string();
        print!("Channel ID: ");
        std::io::stdout().flush().unwrap();
        let mut id = String::new();
        io::stdin().read_line(&mut id).unwrap();
        id = id.trim().to_string();
        let msg = post_message::Message {
            content: Some(content),
            ..Default::default()
        };
        let _ = self.send_message(&msg, id).await;
    }

    pub async fn dbg_get_messages(&self) {
        print!("Channel ID: ");
        std::io::stdout().flush().unwrap();
        let mut id = String::new();
        io::stdin().read_line(&mut id).unwrap();
        id = id.trim().to_string();
        let msgs = self.get_messages(id).await;
        let msgs_iter = msgs.iter().rev();
        for msg in msgs_iter {
            println!(
                "{} | {} | {} | {}",
                msg.author
                    .as_ref()
                    .expect(&format!("Message ID '{}' has no author!", &msg.id))
                    .username,
                msg.id,
                msg.timestamp,
                msg.content
                    .as_ref()
                    .expect(&format!("Message ID '{}' has no content!", &msg.id))
            );
            match &msg.attachments {
                Some(message_attachments) => {
                    for attach in message_attachments {
                        println!("  {} | {} | {}", &attach.id, &attach.filename, &attach.url)
                    }
                }
                None => (),
            }
        }
    }

    pub async fn get_messages(&self, channel_id: String) -> Vec<messages::Message> {
        let url = format!(
            "https://discord.com/api/v9/channels/{}/messages?limit=50",
            channel_id
        );
        let res = self.client.get(url).build();
        match res {
            Ok(request) => self
                .client
                .execute(request)
                .await
                .unwrap()
                .json::<Vec<messages::Message>>()
                .await
                .unwrap(),
            Err(request) => {
                error!("Error sending request!");
                error!("{}", request);
                panic!("Could not read messages!")
            }
        }
    }

    pub async fn display_current_user_information(&self) {
        let url = format!("https://discord.com/api/v9/users/@me");
        let res = self.client.get(url).build();
        match res {
            Ok(request) => {
                let tmp = self.client.execute(request).await.unwrap();
                println!(
                    "Logged in as: {:?}",
                    tmp.json::<users::User>().await.unwrap()
                );
            }
            Err(request) => {
                error!("Error sending request!");
                error!("{}", request);
            }
        }
    }

    pub async fn debug_get_user_info(&self) {
        print!("User ID: ");
        std::io::stdout().flush().unwrap();
        let mut id = String::new();
        io::stdin().read_line(&mut id).unwrap();
        id = id.trim().to_string();
        self.display_user_information(id).await;
    }

    pub async fn debug_get_user_profile(&self) {
        print!("User ID: ");
        std::io::stdout().flush().unwrap();
        let mut id = String::new();
        io::stdin().read_line(&mut id).unwrap();
        id = id.trim().to_string();
        self.display_user_profile(id).await;
    }

    pub async fn display_user_information(&self, user_id: String) {
        let url = format!("https://discord.com/api/v9/users/{}", user_id);
        let res = self.client.get(url).build();
        error!("{:?}", &res);
        match res {
            Ok(request) => {
                let tmp = self.client.execute(request).await.unwrap();
                println!("{:?}", tmp.json::<users::User>().await.unwrap(),);
            }
            Err(request) => {
                error!("Error sending request!");
                error!("{}", request);
            }
        }
    }

    pub async fn display_user_profile(&self, user_id: String) {
        let url = format!("https://discord.com/api/v9/users/{}/profile?with_mutual_guilds=true&with_mutual_friends=true&with_mutual_friends_count=false&guild_id=1225605857413173309", user_id);
        let res = self.client.get(url).build();
        match res {
            Ok(request) => {
                let tmp = self.client.execute(request).await.unwrap();
                println!("{:?}", tmp.json::<users::Profile>().await.unwrap(),);
            }
            Err(request) => {
                error!("Error sending request!");
                error!("{}", request);
            }
        }
    }
}

