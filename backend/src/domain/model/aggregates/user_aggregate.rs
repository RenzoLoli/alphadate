use serde::{Deserialize, Serialize};

use crate::domain::EUser;

#[derive(Serialize, Deserialize)]
pub struct UserAggregate {
    pub id: String,
    pub email: String,
    pub username: String,
    pub anniversary: String,
    pub photo: String,
    pub couplename: String,
}

impl From<EUser> for UserAggregate {
    fn from(value: EUser) -> Self {
        Self {
            id: value.id.to_string(),
            email: value.email,
            username: value.username,
            anniversary: value.anniversary,
            photo: value.photo,
            couplename: value.couplename,
        }
    }
}
