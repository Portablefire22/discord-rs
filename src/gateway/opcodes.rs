use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub enum Opcode {
    Dispatch,
    Heartbeat,
    Identify,
    PresenceUpdate,
    VoiceStateUpdate,
    Resume,
    Reconnect,
    RequestGuildMembers,
    InvalidSession,
    Hello,
    HeartbeatACK,
    SubscribeUser, // Undocumented, but only shows when clicking on user DM
    RequestSoundboardSounds,
    SubscribeGuild, // Undocumented, seemingly subscribe to a guild?
    UnknownOperation
}

impl From<usize> for Opcode {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Dispatch,
            1 => Self::Heartbeat,
            2 => Self::Identify,
            3 => Self::PresenceUpdate,
            4 => Self::VoiceStateUpdate,
            6 => Self::Resume,
            7 => Self::Reconnect,
            8 => Self::RequestGuildMembers,
            9 => Self::InvalidSession,
            10 => Self::Hello,
            11 => Self::HeartbeatACK,
            13 => Self::SubscribeUser,
            31 => Self::RequestSoundboardSounds,
            37 => Self::SubscribeGuild,
            _ => Self::UnknownOperation,
        }
    }
}
