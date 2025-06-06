pub mod handler;
pub mod login;
pub mod types;
pub mod gateway;

#[cfg(test)]
mod tests {
    use std::env;

    use dotenvy::dotenv;

    use super::*;

    #[tokio::test]
    async fn unauthorised_guild_error() {
        dotenv().ok();

        let req_handle = handler::MessageHandler::new("fds".to_string());
        let res = req_handle.get_current_user_guilds().await;
        match res {
            Ok(..) => panic!("Unauthorised Guild Error: {:?}", res.unwrap()),
            Err(..) => (),
        }
    }

    #[tokio::test]
    async fn authorised_guild() {
        dotenv().ok();
        let req_handle = handler::MessageHandler::new(env::var("TEST_TOKEN").unwrap());
        let res = req_handle.get_current_user_guilds().await;
        match res {
            Ok(..) => (),
            Err(e) => panic!("Authorised Guild: {}", e),
        }
    }

    #[tokio::test]
    async fn unauthorised_channel_error() {
        dotenv().ok();
        let req_handle = handler::MessageHandler::new("fds".to_string());
        let res = req_handle.get_guild_channels(env::var("TEST_GUILD_ID").unwrap()).await;
        match res {
            Ok(..) => panic!("Unauthorised Channel: {:?}", res.unwrap()),
            Err(..) => (),
        }
    }

    #[tokio::test]
    async fn authorised_channel() {
        dotenv().ok();
        let req_handle = handler::MessageHandler::new(env::var("TEST_TOKEN").unwrap());
        let res = req_handle.get_guild_channels(env::var("TEST_GUILD_ID").unwrap()).await;
        match res {
            Ok(..) => (),
            Err(e) => panic!("Authorised Channel: {:?}", e),
        }
    }
}
