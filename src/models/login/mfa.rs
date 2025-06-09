use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct MFAResponse {
    user_id: String,
    mfa: bool,
    smf: bool,
    ticket: String,
    backup: bool,
    totp: bool,
    webauthn: Option<bool>,
}


#[derive(Serialize, Debug)]
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
