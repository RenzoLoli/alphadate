use serde::{Deserialize, Serialize};

use crate::domain::EUser;

#[derive(Serialize, Deserialize)]
pub struct UserResource {
    pub id: String,
    pub username: String,
    pub password: String,
    pub email: String,
    pub couplename: String,
    pub anniversary: String,
    pub photo: String,
}

impl From<EUser> for UserResource {
    fn from(user: EUser) -> Self {
        Self {
            id: user.id.to_string(),
            username: user.username,
            password: user.password,
            email: user.email,
            couplename: user.couplename,
            anniversary: user.anniversary,
            photo: user.photo,
        }
    }
}
