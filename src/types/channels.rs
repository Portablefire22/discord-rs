use serde::{Deserialize, Serialize};
use std::fmt;

use super::{threads, users};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Channel {
    pub id: String,
    #[serde(rename = "type")]
    pub channel_type: ChannelType,
    pub guild_id: Option<String>,
    pub position: Option<usize>,
    pub permission_overwrites: Option<Vec<Overwrite>>,
    pub name: Option<String>,
    pub topic: Option<String>,
    pub nsfw: Option<bool>,
    pub last_message_id: Option<String>,
    pub bitrate: Option<usize>,
    pub user_limit: Option<usize>,
    pub rate_limit_per_user: Option<usize>,
    pub recipients: Option<Vec<users::User>>,
    pub icon: Option<String>,
    pub owner_id: Option<String>,
    pub application_id: Option<String>,
    pub managed: Option<bool>,
    pub parent_id: Option<String>,
    pub last_pin_timestamp: Option<String>,
    pub rtc_region: Option<String>,
    pub video_quality_mode: Option<VideoQuality>,
    pub message_count: Option<usize>,
    pub member_count: Option<usize>,
    pub thread_metadata: Option<threads::ThreadMetadata>,
    pub member: Option<threads::ThreadMember>,
    pub default_auto_archive_duration: Option<usize>,
    pub permissions: Option<String>,
    pub flags: Option<usize>,
    pub total_message_sent: Option<usize>,
    pub available_tags: Option<Vec<ForumTag>>,
    pub applied_tags: Option<String>,
    pub default_reaction_emoji: Option<DefaultReaction>,
    pub default_thread_rate_limit_per_user: Option<usize>,
    pub default_sort_order: Option<usize>,
    pub default_forum_layout: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct DefaultReaction {
    emoji_id: Option<String>,
    emoji_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ForumTag {
    pub id: String,
    pub name: String,
    pub moderated: bool,
    pub emoji_id: Option<String>,
    pub emoji_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct VoiceRegion {
    pub id: String,
    pub name: String,
    pub optimal: bool,
    pub deprecated: bool, // Avoid switching to this !
    pub custom: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum VideoQuality {
    Auto = 1,
    Full = 2,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Mention {
    pub id: String,
    pub guild_id: String,
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub channel_type: ChannelType,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Overwrite {
    pub id: String,
    pub overwrite_type: usize,
    pub allow: String,
    pub deny: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
#[repr(u8)]
pub enum ChannelType {
    GuildText = 0,
    DM = 1,
    GuildVoice = 2,
    GroupDM = 3,
    GuildCategory = 4,
    GuildAnnouncement = 5,
    AnnouncementThread = 10,
    PublicThread = 11,
    PrivateThread = 12,
    GuildStageVoice = 13,
    GuildDirectory = 14,
    GuildForum = 15,
    GuildMedia = 16,
}

impl fmt::Display for ChannelType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ChannelType::GuildText => write!(fmt, "Text"),
            ChannelType::DM => write!(fmt, "Direct Message"),
            ChannelType::GuildVoice => write!(fmt, "Guild Voice"),
            ChannelType::GroupDM => write!(fmt, "Group DM"),
            ChannelType::GuildCategory => write!(fmt, "Category"),
            ChannelType::GuildAnnouncement => write!(fmt, "Announcement"),
            ChannelType::AnnouncementThread => write!(fmt, "Announcement Thread"),
            ChannelType::PublicThread => write!(fmt, "Public Thread"),
            ChannelType::PrivateThread => write!(fmt, "Private Thread"),
            ChannelType::GuildStageVoice => write!(fmt, "Stage Voice"),
            ChannelType::GuildDirectory => write!(fmt, "Directory"),
            ChannelType::GuildForum => write!(fmt, "Forum"),
            ChannelType::GuildMedia => write!(fmt, "Media"),
            _ => {
                write!(fmt, "Invalid channel type!")
            }
        }
    }
}
