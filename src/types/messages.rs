use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{
    applications, attachments, channels, embeds, guilds, reactions, roles, stickers, users,
};

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq, Clone)]
pub struct Message {
    pub id: String,
    pub channel_id: String,
    pub author: Option<users::User>,
    pub content: Option<String>, // None if lacking permissions
    pub timestamp: String,
    pub edited_timestamp: Option<String>, // None if never edited
    pub tts: bool,
    pub mention_everyone: bool,
    pub mentions: Option<Vec<users::User>>,
    pub mention_roles: Option<Vec<roles::Role>>,
    pub mention_channels: Option<Vec<channels::Mention>>,
    pub attachments: Option<Vec<attachments::Attachment>>,
    pub embeds: Vec<embeds::Embed>,
    pub reactions: Option<Vec<reactions::Reaction>>,
    pub nonce: Option<String>,
    pub pinned: bool,
    pub webhook_id: Option<String>,
    #[serde(rename = "type")]
    pub message_type: usize,
    pub activity: Option<MessageActivity>,
    pub application: Option<applications::Application>,
    pub application_id: Option<String>,
    pub message_reference: Option<MessageReference>,
    pub flags: Option<usize>,
    pub referenced_message: Option<Box<Message>>,
    pub interaction_metadata: Option<InteractionMetadata>,
    pub interaction: Option<Interaction>,
    pub thread: Option<channels::Channel>,
    pub components: Option<Vec<MessageComponent>>,
    pub sticker_items: Option<Vec<stickers::StickerItem>>,
    pub stickers: Option<Vec<stickers::Sticker>>,
    pub position: Option<usize>,
    pub role_subscription_data: Option<roles::SubscriptionData>,
    pub resolved: Option<ResolvedData>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ResolvedData {
    /*pub users: HashMap<String, users::User>,
    pub members: HashMap<String, guilds::Member>,
    pub roles: HashMap<String, roles::Role>,
    pub channels: HashMap<String, channels::Channel>,
    pub messages: HashMap<String, Message>,
    pub attachments: HashMap<String, attachments::Attachment>,*/
    pub users: String,
    pub members: String,
    pub roles: String,
    pub channels: String,
    pub messages: String,
    pub attachments: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum MessageComponent {
    ActionRow = 1,
    Button = 2,
    StringSelect = 3,
    TextInput = 4,
    UserSelect = 5,
    RoleSelect = 6,
    MentionableSelect = 7,
    ChannelSelect = 8,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Interaction {
    pub id: String,
    #[serde(rename = "type")]
    pub interaction_type: InteractionType,
    pub name: String,
    pub user: users::User,
    pub member: Option<guilds::Member>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct InteractionMetadata {
    pub id: String,
    #[serde(rename = "type")]
    pub interaction_type: InteractionType,
    pub user_id: String,
    pub authorizing_integration_owners: InteractionType,
    pub original_response_message_id: Option<String>,
    pub interacted_message_id: Option<String>,
    pub triggering_interaction_metadata: Option<Box<Self>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum InteractionType {
    Ping = 1,
    ApplicationCommand = 2,
    MessageComponent = 3,
    ApplicationCommandAutocomplete = 4,
    ModalSubmit = 5,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct MessageActivity {
    #[serde(rename = "type")]
    activity_type: usize,
    party_id: Option<String>,
}

//https://discord.com/developers/docs/resources/channel#allowed-mentions-object
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum AllowedMentionTypes {
    Role,
    User,
    Everyone,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum AllowedMentionsStructures {
    Parse,
    Roles,
    Users,
    RepliedUser,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct MessageReference {
    pub message_id: String,
    pub channel_id: Option<String>, // Optional when replying but always present when receiving
    pub guild_id: String,
    pub fail_if_not_exits: bool, // Should error if the referenced message does not exist, sends a
                                 // non-reply when non-existent if set to false
}
