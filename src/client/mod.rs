use std::{env, sync::{Arc, LazyLock}};

use futures_util::lock::Mutex;
use log::{debug, error};
use reqwest::header;

use crate::models::{ login::{details::Details, login_response::LoginResponse, mfa::{MultiFactorBody, MFA}}};
use crate::gateway::Shard;
pub mod channels;
pub mod users;
pub mod guilds;

const DEFAULT_USER_AGENT: LazyLock<String> = LazyLock::new(|| {
    format!("Mozilla/5.0 (X11; {} {}) Gecko/20100101 Kitten-rs/{}",
            std::env::consts::OS,
            std::env::consts::ARCH,
            env::var("CARGO_PKG_VERSION").unwrap_or(String::from("0.0.0")))
});

#[derive(Clone, Debug)]
pub struct Client {
    http: reqwest::Client,
    pub token: String,
    shard: Arc<Mutex<Shard>>,
}

impl Client {
    #[must_use = "Client will not initialise without awaiting"]
    pub async fn new(token: String, user_agent: Option<String>) -> Self {
        
        let http = new_http_client(Some(token.clone()), user_agent);
        let shard = Shard::new(&http, token.clone()).await.expect("Could not create websocket connection");

        Self {
            token,
            http,
            shard
        }
    }

    pub async fn new_from_login(username: String, password: String, user_agent: Option<String>) -> Result<Self, LoginResponse> {
        let http = new_http_client(None, user_agent.clone());
        let response = submit_login(http, username, password, false).await;
        match response {
            LoginResponse::Success { user_id, token, user_settings } => Ok(Self::new(token, user_agent).await),
            _ => Err(response),
        }
    }
}

pub async fn submit_login (client: reqwest::Client, username: String, password: String, undelete: bool) -> LoginResponse {
    const URL: &'static str = "https://discord.com/api/v9/auth/login";
    let details = Details::new(username, password, undelete);
    let body = serde_json::to_string(&details);
    let login_response: LoginResponse;
    match body {
            Ok(b) => {
                let res = client.post(URL).body(b).build();
                match res {
                    Ok(request) => {
                        login_response = client
                            .execute(request)
                            .await
                            .unwrap()
                            .json::<LoginResponse>()
                            .await
                            .unwrap();
                        debug!("Info: {:?}", login_response);
                    }
                    Err(request) => {
                        error!("Error sending request!");
                        error!("{}", request);
                        panic!("Could not get current user's Guilds!");
                    }
                }
            }
            _ => {
                error!("Failed sending user details to discord!");
                panic!("Error sending username and password to discord!")
            }
        }
        login_response
}


fn new_http_client(token: Option<String>, user_agent: Option<String>) -> reqwest::Client {
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

        if let Some(t) = token {
            headers.insert(
                "Authorization",
                header::HeaderValue::from_str(&t)
                .expect("Could not create header value from authorization_token"),
            );
        }
        reqwest::Client::builder()
            .user_agent(user_agent.unwrap_or(DEFAULT_USER_AGENT.clone()))
            .default_headers(headers)
            .build()
            .unwrap()
}

pub async fn mfa_input(client: reqwest::Client, code: String, ticket: String, mfa_type: &MFA) -> LoginResponse {
    let url = format!(
        "https://discord.com/api/v9/auth/mfa/{}",
        mfa_type.to_string()
    );
    let mfa_body = MultiFactorBody::new(code, ticket);
    let body = serde_json::to_string(&mfa_body);
    match body {
        Ok(b) => {
            let res = client.post(url).body(b).build();
            match res {
                Ok(request) => {
                    debug!("{:?}", request);
                    client
                    .execute(request)
                    .await
                    .unwrap()
                    .json::<LoginResponse>()
                    .await
                    .unwrap()
                },
                Err(request) => {
                    error!("Error sending request!");
                    error!("{}", request);
                    panic!("Could not send MFA to Discord!");
                }
            }
        }
        Err(_) => {
            error!("Error creating body for MFA submission!");
            panic!("Error creating body for MFA")
        }
    }
}

