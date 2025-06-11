use serde::Serialize;

use crate::models::gateway::opcodes::Opcode;


#[derive(Serialize)]
pub struct WebsocketMessage<S: Serialize> {
    #[serde(rename = "op")]
    operation: Opcode,
    #[serde(rename = "d")]
    data: Option<S>,
}

impl<S: Serialize> WebsocketMessage<S> {
    pub fn new(operation: Opcode, data: Option<S>) -> Self {
        Self {
            operation,
            data
        }
    }
}
