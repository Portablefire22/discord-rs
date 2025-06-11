use std::{env, sync::LazyLock};

use dotenvy::dotenv;
use reqwest::header;

use crate::gateway::Shard;

const DEFAULT_USER_AGENT: LazyLock<String> = LazyLock::new(|| {
    format!("Mozilla/5.0 (X11; {} {}) Gecko/20100101 Kitten-rs/{}",
            std::env::consts::OS,
            std::env::consts::ARCH,
            env::var("CARGO_PKG_VERSION").unwrap_or(String::from("0.0.0")))
});

fn token() -> String {
    dotenv().ok();
    env::var("TEST_TOKEN").unwrap()
}

#[tokio::test]
async fn gateway_connection() {
    dotenv().ok(); 
    
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
        header::HeaderValue::from_str(&token())
        .expect("Could not create header value from authorization_token"),
    );
    let http = reqwest::Client::builder()
        .user_agent(DEFAULT_USER_AGENT.clone())
        .default_headers(headers)
        .build()
        .unwrap();



    match Shard::new(&http, token()).await {
        Ok(_) => (),
        Err(err) => panic!("Could not create client: {:?}", err),
    }
}
