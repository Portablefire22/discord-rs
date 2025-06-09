use serde::Deserialize;

use crate::models::login::{captcha::Captcha, mfa::{MFAResponse, UserSettings}};


#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum LoginResponse {
    Success {user_id: String, token: String, user_settings: UserSettings},
    Captcha(Captcha),
    MFA(MFAResponse),
}
