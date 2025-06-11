use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::models::gateway::{events::EventData, opcodes::Opcode};


#[derive(Serialize, Deserialize)]
#[serde_with::skip_serializing_none]
pub struct WebsocketMessage {
    #[serde(rename = "op")]
    pub operation: Opcode,
    #[serde(rename = "d")]
    pub data: Option<EventData>,
    t: Option<usize>,
    s: Option<usize>,
}

impl WebsocketMessage {
    pub fn new(operation: Opcode, data: Option<EventData>) -> Self {
        Self {
            operation,
            data,
            t: None,
            s: None
        }
    }
}
