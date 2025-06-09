use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub struct ConnectionProperties {
    pub os: String,
    pub browser: String,
    pub device: String,
}
