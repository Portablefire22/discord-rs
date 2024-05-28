pub mod handler;
pub mod login;
pub mod types;

#[cfg(test)]
mod tests {
    use super::*;

    const TOKEN: &str = ""; // For the love of God, do not commit this
    const GUILD_ID: &str = "";
    const CHANNEL_ID: &str = "";
    const USER_ID: &str = "546903520662650892"; // Safe to commit, anyone can get this ID

    #[tokio::test]
    async fn unauthorised_guild_error() {
        let req_handle = handler::MessageHandler::new("fds".to_string());
        let res = req_handle.get_current_user_guilds().await;
        match res {
            Ok(..) => panic!("Unauthorised Guild Error: {:?}", res.unwrap()),
            Err(..) => (),
        }
    }

    #[tokio::test]
    async fn authorised_guild() {
        let req_handle = handler::MessageHandler::new(TOKEN.to_string());
        let res = req_handle.get_current_user_guilds().await;
        match res {
            Ok(..) => (),
            Err(e) => panic!("Authorised Guild: {}", e),
        }
    }

    #[tokio::test]
    async fn unauthorised_channel_error() {
        let req_handle = handler::MessageHandler::new("fds".to_string());
        let res = req_handle.get_guild_channels(GUILD_ID.to_string()).await;
        match res {
            Ok(..) => panic!("Unauthorised Channel: {:?}", res.unwrap()),
            Err(..) => (),
        }
    }

    #[tokio::test]
    async fn authorised_channel() {
        let req_handle = handler::MessageHandler::new(TOKEN.to_string());
        let res = req_handle.get_guild_channels(GUILD_ID.to_string()).await;
        match res {
            Ok(..) => (),
            Err(e) => panic!("Authorised Channel: {:?}", e),
        }
    }
}
