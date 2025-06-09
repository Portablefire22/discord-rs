use std::env;
use dotenvy::dotenv;

use crate::models::messages::Message;

fn token() -> String {
    dotenv().ok();
    env::var("TEST_TOKEN").unwrap()
}

#[tokio::test]
async fn authorised_send_message() {
    dotenv().ok(); 
    let client = super::Client::new(token(), None).await;

    let msg = Message {
        channel_id: String::from("1381709249968607314"),
        content: Some("Test message".to_string()),
        ..Default::default()
    };

    match client.send_message(String::from("1381709249968607314"), &msg).await {
        Ok(res) => {
            //println!("{:?}", res.get(0));
            ()
        },
        Err(err) => panic!("{:?}", err),
    }
}
