use serde::{Deserialize, Serialize};

use super::{guilds, users};

// https://discord.com/developers/docs/resources/application#application-object
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Application {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub description: String,
    pub rpc_origins: Option<Vec<String>>,
    pub bot_public: bool,
    pub bot_require_code_grant: bool,
    pub bot: Option<users::User>,
    pub terms_of_service_url: Option<String>,
    pub privacy_policy_url: Option<String>,
    pub owner: Option<users::User>,
    pub summary: Option<String>,
    pub verify_key: String,
    pub team: Option<Team>,
    pub guild_id: Option<String>,
    pub guild: Option<guilds::Guild>,
    pub primary_sku_id: Option<String>,
    pub slug: Option<String>,
    pub cover_image: Option<String>,
    pub flags: Option<usize>,
    pub approximate_guild_count: Option<usize>,
    pub redirect_uris: Option<Vec<String>>,
    pub interactions_endpoint_url: Option<String>,
    pub role_connections_verification_url: Option<String>,
    pub tags: Option<Vec<String>>,
    pub install_params: InstallParams,
    pub integration_types_config: Option<IntegrationTypes>,
    pub custom_install_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum IntegrationTypes {
    GuildInstall = 0,
    UserInstall = 1,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct InstallParams {
    scopes: Vec<String>,
    permissions: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Team {
    pub icon: Option<String>,
    pub id: String,
    pub members: Vec<TeamMember>,
    pub name: String,
    pub owner_user_id: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct TeamMember {
    pub membership_state: MembershipState,
    pub team_id: String,
    pub user: users::User,
    pub role: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum MembershipState {
    Invited = 1,
    Accepted = 2,
}
