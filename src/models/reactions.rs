use serde::{Deserialize, Serialize};

use super::emojis;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Reaction {
    count: usize,
    count_details: CountDetails,
    me: bool,
    me_burst: bool,
    emoji: emojis::Emoji,
    burst_colors: Vec<usize>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct CountDetails {
    burst: usize,
    normal: usize,
}
