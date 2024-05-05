use serde::{Deserialize, Serialize};

use super::guilds;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ThreadMetadata {
    pub archived: bool,
    pub auto_archive_duration: usize,
    pub archive_timestamp: String,
    pub locked: bool,
    pub invitable: Option<bool>,
    pub create_timestamp: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ThreadMember {
    pub id: Option<String>,
    pub user_id: Option<String>,
    pub join_timestamp: String,
    pub flags: usize,
    pub member: Option<guilds::Member>,
}
