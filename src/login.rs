use log::{debug, error};
use reqwest::{header, Client};
use rpassword::prompt_password;
use std::io::{self, prelude::*};

use crate::types::login::*;

#[derive(Debug)]
pub struct LoginHandler {
    client: reqwest::Client,
}

impl LoginHandler {
    pub fn new() -> Self {
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

        let client = Client::builder()
            .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:124.0) Gecko/20100101 Firefox/124.0")
            .default_headers(headers)
            .build()
            .unwrap();
        Self { client }
    }

    pub async fn submit_login(
        &self,
        username: String,
        password: String,
        undelete: bool,
    ) -> LoginResponse {
        let url = format!("https://discord.com/api/v9/auth/login");
        let login_response;
        let d = Details::new(username, password, undelete);
        debug!("Details: {:?}", &d);
        let body = serde_json::to_string(&d);
        debug!("{:?}", &body);
        match body {
            Ok(b) => {
                debug!("B: {:?}", &b);
                let res = self.client.post(url).body(b).build();
                debug!("RES: {:?}", res);
                match res {
                    Ok(request) => {
                        debug!("{:?}", request);
                        login_response = self
                            .client
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

    pub async fn mfa_input(&self, code: String, ticket: String, mfa_type: &MFA) -> LoginResponse {
        let url = format!(
            "https://discord.com/api/v9/auth/mfa/{}",
            mfa_type.to_string()
        );
        let mfa_body = MultiFactorBody::new(code, ticket);
        let body = serde_json::to_string(&mfa_body);
        match body {
            Ok(b) => {
                let res = self.client.post(url).body(b).build();
                match res {
                    Ok(request) => {
                        debug!("{:?}", request);
                        self
                        .client
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

    pub async fn cli_login(&self) -> LoginResponse {
        print!("Discord Email/Username: ");
        std::io::stdout().flush().unwrap();
        let mut username = String::new();
        io::stdin().read_line(&mut username).unwrap();

        let password = prompt_password("Discord Password: ").unwrap();
        let resp = self.submit_login(username, password, false).await;
        match resp.token {
            Some(_) => resp,
            None => self.cli_mfa(&resp).await,
        }
    }

    pub async fn cli_mfa(&self, response: &LoginResponse) -> LoginResponse {
        if response.mfa.is_some_and(|x| x) {
            let mut enabled_mfa = Vec::new();
            if response.totp.is_some_and(|x| x) {
                enabled_mfa.push(MFA::totp)
            }
            if response.sms.is_some_and(|x| x) {
                enabled_mfa.push(MFA::sms)
            }
            if response.backup.is_some_and(|x| x) {
                enabled_mfa.push(MFA::backup)
            }
            if response.webauthn.is_some_and(|x| x) {
                enabled_mfa.push(MFA::webauthn)
            }
            let selected_mfa: usize = loop {
                println!("\nAvailable MFA: ");
                for (i, mfa) in enabled_mfa.iter().enumerate() {
                    println!("  [{i}] | {mfa}");
                }

                print!("Select MFA: ");
                std::io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();

                match input.parse::<usize>() {
                    Ok(index) => break index,
                    _ => println!("Error: Inputted value was not a number!"),
                }
            };
            let selected = &enabled_mfa[selected_mfa];
            let code = prompt_password(format!("{} Code: ", &selected.to_string().to_uppercase()))
                .unwrap();
            return self
                .mfa_input(
                    code,
                    response.ticket.as_ref().unwrap().to_string(),
                    selected,
                )
                .await;
        }
        panic!("The user has somehow been prompted for MFA without MFA being enabled on their account!")
    }
}
