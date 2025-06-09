use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct Captcha {
    captcha_key: Vec<String>,
    captcha_sitekey: String,
    captcha_service: String,
    captcha_session_id: String,
    captcha_rqdata: String,
    captcha_rqtoken: String,
}
