use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Details {
    login: String,
    password: String,
    undelete: bool,
}

impl Details {
    pub fn new(login: String, password: String, undelete: bool) -> Self {
        Self {
            login,
            password,
            undelete,
        }
    }
}
