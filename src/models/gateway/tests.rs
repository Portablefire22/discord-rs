use std::env;

use dotenvy::dotenv;

use crate::client::Client;

fn token() -> String {
    dotenv().ok();
    env::var("TEST_TOKEN").unwrap()
}

#[tokio::test]
async fn gateway_connection() {
    dotenv().ok(); 
    let client = Client::new(token(), None).await;
    ()
}
