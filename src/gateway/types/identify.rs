use serde::{Deserialize, Serialize};

use crate::gateway::types::{connection_properties::ConnectionProperties, update_presence::UpdatePresence};


#[derive(Serialize, Deserialize)]
pub struct Identify {
    token: String,
    properties: ConnectionProperties,
    compress: Option<bool>,
    large_threshold: Option<usize>,
    shard: Option<Vec<usize>>,
    presence: Option<UpdatePresence>,
    intents: usize,
}
