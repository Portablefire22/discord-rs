use serde::{Deserialize, Serialize};

use crate::models::gateway::opcodes::Opcode;


#[derive(Serialize, Deserialize)]
pub struct GatewayEvent {
    #[serde(rename = "op")]
    operation: Opcode,
    #[serde(rename = "d")]
    data: Option<String>,
    #[serde(rename = "s")]
    sequence_number: Option<usize>,
    #[serde(rename = "t")]
    event_name: Option<String>,
}
