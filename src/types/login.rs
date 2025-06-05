use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Details {
    login: String,
    password: String,
    undelete: bool,
}

impl Details {
    pub fn new(login: String, password: String, undelete: bool) -> Self {
        Self {
            login,
            password,
            undelete,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResponse {
    pub user_id: Option<String>,
    pub mfa: Option<bool>,
    pub sms: Option<bool>,
    pub ticket: Option<String>,
    pub backup: Option<bool>,
    pub totp: Option<bool>,
    pub webauthn: Option<bool>,
    pub token: Option<String>,
    user_setting: Option<UserSettings>,

    message: Option<String>,
    code: Option<String>,
    errors: Option<LoginError>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginError {
    login: Option<Error>,
    password: Option<Error>,
    email: Option<Error>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    errors: Option<Vec<ErrorContents>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorContents {
    code: Option<String>,
    message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MultiFactorBody {
    code: String,
    ticket: String,
}

impl MultiFactorBody {
    pub fn new(code: String, ticket: String) -> Self {
        Self { code, ticket }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserSettings {
    locale: String,
    theme: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_camel_case_types)]
pub enum MFA {
    sms,
    backup,
    totp,
    webauthn,
}

impl fmt::Display for MFA {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MFA::sms => write!(f, "sms"),
            MFA::backup => write!(f, "backup"),
            MFA::totp => write!(f, "totp"),
            MFA::webauthn => write!(f, "webauthn"),
        }
    }
}
