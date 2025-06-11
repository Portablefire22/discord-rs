use serde::{Deserialize, Serialize};

use crate::models::gateway::events::{hello::Hello, identify::Identify};

pub mod hello;
pub mod identify;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum EventData {
    #[serde(skip_deserializing)]
    IdentifyData(Identify),
    #[serde(skip_serializing)]
    HelloData(Hello),
    Heartbeat(Option<usize>)
}
