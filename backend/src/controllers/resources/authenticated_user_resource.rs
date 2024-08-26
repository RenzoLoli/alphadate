use serde::{Deserialize, Serialize};

use crate::domain::EUser;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthenticatedUserResource {
    pub id: String,
    pub username: String,
    pub password: String,
    pub email: String,
    pub couplename: String,
    pub anniversary: String,
    pub photo: String,
}

impl From<EUser> for AuthenticatedUserResource {
    fn from(signin_resource: EUser) -> Self {
        Self {
            id: signin_resource.id.to_string(),
            username: signin_resource.username,
            password: signin_resource.password,
            email: signin_resource.email,
            couplename: signin_resource.couplename,
            anniversary: signin_resource.anniversary,
            photo: signin_resource.photo,
        }
    }
}
