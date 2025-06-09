use serde::{Deserialize, Serialize};

use super::users;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Sticker {
    pub id: String,
    pub pack_id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub tags: String,
    pub asset: Option<String>,
    #[serde(rename = "type")]
    pub sticker_type: StickerType,
    pub format_type: StickerFormat,
    pub available: Option<bool>,
    pub guild_id: Option<String>,
    pub user: Option<users::User>,
    pub sort_value: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct StickerItem {
    pub id: String,
    pub name: String,
    pub format_type: StickerFormat,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum StickerType {
    Standard = 1,
    Guild = 2,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum StickerFormat {
    Png = 1,
    Apng = 2,
    Lottie = 3,
    Gid = 4,
}
