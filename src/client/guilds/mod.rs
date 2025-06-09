use log::error;
use crate::models::channels::Channel;

use super::Client;

#[cfg(test)]
mod tests;

impl Client {

    pub async fn get_guild_channels(&self, guild_id: String) -> Result<Vec<Channel>, reqwest::Error> {
        let url: String = format!("https://discord.com/api/v9/guilds/{}/channels", guild_id);
        let res = self.http.get(url).build();
        match res {
            Ok(req) => self.http.execute(req)
                .await
                .unwrap()
                .json::<Vec<Channel>>()
                .await,
            Err(req) => {
                error!("Could not get channel: {:?}", req);
                Err(req)
            }
        }
    }
}

