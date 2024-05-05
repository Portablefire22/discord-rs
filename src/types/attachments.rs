use serde::{Deserialize, Serialize};

// https://discord.com/developers/docs/resources/channel#attachment-object
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Attachment {
    pub id: String,
    pub filename: String,
    pub description: Option<String>,
    pub content_type: Option<String>,
    pub size: isize,
    pub url: String,           // Source of the file
    pub proxy_url: String,     // Proxy url of file
    pub height: Option<isize>, // Image
    pub width: Option<isize>,  // Image
    pub ephemeral: Option<bool>,
    pub duration_secs: Option<String>, // Duration of the audio file (voice messages)
    pub waveform: Option<String>,      // base64 encoded bytearray of waveform (voice messages)
    pub flags: Option<isize>,          // Bit field of attachment flags
}
