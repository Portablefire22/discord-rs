use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize)]
pub struct Hello {
    pub heartbeat_interval: usize,
    #[serde(rename = "_trace")]
    trace: Option<Value>,
}
