use serde::{Deserialize, Serialize};

use super::guilds;

// https://discord.com/developers/docs/resources/user#user-object
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub global_name: Option<String>,
    pub avatar: Option<String>, // avatar hash
    pub bot: Option<bool>,
    pub system: Option<bool>,
    pub mfa_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<isize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    pub verified: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_type: Option<usize>, // Type of nitro user has
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_flags: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_decoration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    pub nsfw_allowed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_users: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authenticator_types: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct UserProfile {
    pub bio: Option<String>,
    pub accent_color: Option<usize>,
    pub pronouns: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Profile {
    pub user: User,
    pub connected_accounts: Option<Vec<ConnectedAccount>>,
    pub premium_since: Option<String>,
    pub premium_type: Option<String>,
    pub premium_guild_since: Option<String>,
    pub profile_theme_experiment_bucket: Option<usize>,
    pub user_profile: UserProfile,
    pub badges: Option<Vec<Badge>>,
    pub guild_badges: Option<Vec<GuildBadge>>,
    pub mutual_friends: Option<Vec<User>>,
    pub mutual_guilds: Option<Vec<MutGuild>>,
    pub guild_member: Option<guilds::Member>,
    pub guild_member_profile: Option<guilds::MemberProfile>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct MutGuild {
    pub id: String,
    pub nick: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct GuildBadge {} // TODO Figure out what to put here

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Badge {} // TODO Figure out what to put here

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ConnectedAccount {
    #[serde(rename = "type")]
    pub account_type: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub verified: Option<bool>,
}
