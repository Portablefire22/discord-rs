use serde::{Deserialize, Serialize};

use super::{roles, users};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Emoji {
    id: Option<String>,
    name: Option<String>,
    roles: Option<Vec<roles::Role>>,
    user: Option<users::User>,
    require_colons: Option<bool>,
    managed: Option<bool>,
    animated: Option<bool>,
    available: Option<bool>,
}
