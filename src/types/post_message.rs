use std::io::{self, Write};

use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::{attachments, messages};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<usize>, // Used to verify if a message was sent, random 25 char
    pub tts: bool, // True for TTS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_mentions: Option<messages::AllowedMentionTypes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_reference: Option<messages::MessageReference>, // Replying to a message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<u8>, // Don't think this is needed at all
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_ids: Option<Vec<String>>, // Array of sticker snowflakes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<String>, // Contents of the file being sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_json: Option<String>, // JSON-encoded body of non-file params https://discord.com/developers/docs/reference#uploading-files
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<attachments::Attachment>>,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            content: None,
            nonce: None,
            tts: false,
            embed: None,
            allowed_mentions: None,
            message_reference: None, // Replying to a message
            components: None,        // Don't think this is needed at all
            sticker_ids: None,       // Array of sticker snowflakes
            files: None,             // Contents of the file being sent
            payload_json: None,
            attachments: None,
        }
    }
}
