use serde::{Deserialize, Serialize};

use super::{emojis, roles, stickers, users};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Guild {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub icon_hash: Option<String>,
    pub splash: Option<String>,
    pub discovery_splash: Option<String>,
    pub owner: Option<bool>,
    pub owner_id: Option<String>,
    pub permissions: Option<String>,
    pub region: Option<String>,
    pub afk_channel_id: Option<String>,
    pub afk_timeout: Option<usize>,
    pub widget_enabled: Option<bool>,
    pub widget_channel_id: Option<String>,
    pub verification_level: Option<VerificationLevel>,
    pub default_message_notifications: Option<DefaultNotifications>,
    pub explicit_content_filter: Option<ExplicitContentFilter>,
    pub roles: Option<Vec<roles::Role>>,
    pub emojis: Option<Vec<emojis::Emoji>>,
    pub features: Option<Vec<String>>,
    pub mfa_level: Option<MFALevel>,
    pub application_id: Option<String>,
    pub system_channel_id: Option<String>,
    pub system_channel_flags: Option<usize>,
    pub rules_channel_id: Option<String>,
    pub max_presences: Option<usize>,
    pub max_members: Option<usize>,
    pub vanity_url_code: Option<String>,
    pub description: Option<String>,
    pub banner: Option<String>,
    pub premium_tier: Option<PremiumTier>,
    pub premium_subscription_count: Option<usize>,
    pub preferred_locale: Option<String>,
    pub public_updates_channel_id: Option<String>,
    pub max_video_channel_users: Option<usize>,
    pub max_stage_video_channel_users: Option<usize>,
    pub approximate_member_count: Option<usize>,
    pub welcome_screen: Option<WelcomeScreen>,
    pub nsfw_level: Option<NSFWLevel>,
    pub stickers: Option<Vec<stickers::Sticker>>,
    pub premium_progress_bar_enabled: Option<bool>,
    pub safety_alerts_channel_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct MemberProfile {
    pub guild_id: Option<String>,
    pub pronouns: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct WelcomeScreen {
    pub description: Option<String>,
    pub welcome_channels: Vec<WelcomeScreenChannel>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct WelcomeScreenChannel {
    pub channel_id: String,
    pub description: String,
    pub emoji_id: Option<String>,
    pub emoji_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum VerificationLevel {
    None = 0,
    Low = 1,
    Medium = 2,
    High = 3,
    VeryHigh = 4,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum DefaultNotifications {
    AllMessages = 0,
    OnlyMentions = 1,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum ExplicitContentFilter {
    Disabled = 0,
    MembersWithoutRoles = 1,
    AllMembers = 2,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum NSFWLevel {
    Default = 0,
    Explicit = 1,
    Safe = 2,
    AgeRestricted = 3,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum PremiumTier {
    None = 0,
    Tier1 = 1,
    Tier2 = 2,
    Tier3 = 3,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum MFALevel {
    None = 0,
    Elevated = 1,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Member {
    pub user: Option<users::User>,
    pub nick: Option<String>,
    pub avatar: Option<String>,
    pub roles: Vec<String>,
    pub joined_at: String,
    pub premium_since: Option<String>,
    pub deaf: bool,
    pub mute: bool,
    pub flags: usize,
    pub pending: Option<bool>,
    pub permissions: Option<String>,
    pub communication_disabled_until: Option<String>,
}
