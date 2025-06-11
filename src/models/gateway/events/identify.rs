use serde::{Deserialize, Serialize};

use crate::models::gateway::update_presence::UpdatePresence;

const LIB_NAME: &'static str = "Kitten-rs";

#[derive(Serialize, Deserialize)]
#[serde_with::skip_serializing_none]
pub struct Identify {
    token: String,
    properties: ConnectionProperties,
    #[serde(skip_serializing_if = "Option::is_none")]
    compress: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    large_threshold: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shard: Option<Vec<usize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    presence: Option<UpdatePresence>,
    intents: usize,
}

impl Identify {
    pub fn new(token: String, 
        properties: Option<ConnectionProperties>, 
        compress: Option<bool>, 
        large_threshold: Option<usize>, 
        shard: Option<Vec<usize>>, 
        presence: Option<UpdatePresence>, 
        intents: usize) -> Self {
        
        let properties = match properties {
            Some(prop) => prop,
            None => {
                ConnectionProperties {
                    os: std::env::consts::OS.to_string(),
                    browser: LIB_NAME.to_string(),
                    device: String::new(),
                }
            }
        };

        Self {
            token,
            properties,
            compress,
            large_threshold, 
            shard,
            presence,
            intents
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct ConnectionProperties {
    pub os: String,
    pub browser: String,
    pub device: String,
}
