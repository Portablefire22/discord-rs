use serde::{de, Deserialize, Serialize};
// https://discord.com/developers/docs/topics/permissions#role-object
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub color: isize,
    pub hoise: bool,
    pub icon: Option<String>,
    pub unicode_emoji: Option<String>,
    pub position: usize,
    pub permissions: String,
    pub managed: bool,
    pub mentionable: bool,
    pub tags: Option<RoleTag>,
    pub flags: usize,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct RoleTag {
    bot_id: Option<String>,
    integration_id: Option<String>,
    #[serde(deserialize_with = "deserialize_tag_bool")]
    premium_subsriber: Option<Option<bool>>, // Double option required because for some fucking
    // reason discord decided to use "null" as true and
    // actual null as fucking false
    #[serde(deserialize_with = "deserialize_tag_bool")]
    subscription_listing_id: Option<Option<String>>,
    #[serde(deserialize_with = "deserialize_tag_bool")]
    available_for_purchase: Option<Option<bool>>,
    #[serde(deserialize_with = "deserialize_tag_bool")]
    guild_connections: Option<Option<bool>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct SubscriptionData {
    pub role_subscription_listing_id: String,
    pub tier_name: String,
    pub total_months_subscribed: usize,
    pub is_renewal: bool,
}

fn deserialize_tag_bool<'de, T, D>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    D: de::Deserializer<'de>,
    T: Deserialize<'de>,
{
    Ok(Some(Option::deserialize(deserializer)?))
}
