use serde::{Deserialize, Serialize};

use crate::models::messages::MessageActivity;


#[derive(Serialize, Deserialize)]
pub struct UpdatePresence {
    since: Option<usize>,
    activities: Vec<String>, // TODO: Add activity object https://discord.com/developers/docs/events/gateway-events#activity-object
    status: String,
    afk: bool,
}

