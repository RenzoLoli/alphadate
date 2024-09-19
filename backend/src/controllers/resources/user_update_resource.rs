use serde::{Deserialize, Serialize};

use crate::domain::UserUpdateCommand;

#[derive(Serialize, Deserialize)]
pub struct UserUpdateResource {
    pub username: Option<String>,
    pub couplename: Option<String>,
    pub anniversary: Option<String>,
    pub photo: Option<String>,
}

impl From<(String, UserUpdateResource)> for UserUpdateCommand {
    fn from(value: (String, UserUpdateResource)) -> Self {
        let (id, value) = value;
        Self {
            id,
            username: value.username,
            couplename: value.couplename,
            anniversary: value.anniversary,
            photo: value.photo,
        }
    }
}
