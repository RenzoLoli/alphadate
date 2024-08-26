use serde::{Deserialize, Serialize};

use crate::domain::SignInCommand;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignInResource {
    pub email: String,
    pub password: String,
}

impl From<SignInResource> for SignInCommand {
    fn from(signin_resource: SignInResource) -> Self {
        SignInCommand {
            email: signin_resource.email,
            password: signin_resource.password,
        }
    }
}
