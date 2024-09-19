use serde::{Deserialize, Serialize};

use crate::domain::SignUpCommand;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignUpResource {
    pub username: String,
    pub password: String,
    pub email: String,
    pub couplename: String,
    pub anniversary: String,
    pub photo: String,
}

impl From<SignUpResource> for SignUpCommand {
    fn from(signup_resource: SignUpResource) -> Self {
        SignUpCommand {
            username: signup_resource.username,
            password: signup_resource.password,
            email: signup_resource.email,
            couplename: signup_resource.couplename,
            anniversary: signup_resource.anniversary,
            photo: signup_resource.photo,
        }
    }
}
