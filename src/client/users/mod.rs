use log::error;
use crate::models::{channels::Channel, guilds::Guild, users::User};

use super::Client;

#[cfg(test)]
mod tests;

impl Client {

    pub async fn get_current_user_guilds(&self) -> Result<Vec<Guild>, reqwest::Error> {
        let url: &'static str = "https://discord.com/api/v9/users/@me/guilds";
        let res = self.http.get(url).build();
        match res {
            Ok(req) => self.http.execute(req)
                .await
                .unwrap()
                .json::<Vec<Guild>>()
                .await,
            Err(req) => {
                error!("Could not get channel: {:?}", req);
                Err(req)
            }
        }
    }

    pub async fn get_current_user(&self) -> Result<User, reqwest::Error> {
        self.get_user("@me").await
    }

    pub async fn get_user<S: AsRef<str> + std::fmt::Display>(&self, user_id: S) -> Result<User, reqwest::Error> {
        let url: String = format!("https://discord.com/api/v9/users/{}", user_id);
        let res = self.http.get(url).build();
        match res {
            Ok(req) => self.http.execute(req)
                .await
                .unwrap()
                .json::<User>()
                .await,
            Err(req) => {
                error!("Could not get channel: {:?}", req);
                Err(req)
            }
        }
    }
}
