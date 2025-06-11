use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum Status {
    Online,
    DoNotDisturb,
    Idle,
    Invisible,
    Offline,
    UnknownStatus
}

impl From<&'static str> for Status {
    fn from(value: &'static str) -> Self {
        match value {
            "online" => Self::Online,
            "dnd" => Self::DoNotDisturb,
            "idle" => Self::Idle,
            "invisible" => Self::Invisible,
            "offline" => Self::Offline,
            _ => Self::UnknownStatus,
        }
    }
}
