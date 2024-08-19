use serde::{self, Deserialize, Serialize};

use super::User;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UserUpdate {
    pub username: Option<String>,
    pub couplename: Option<String>,
    pub anniversary: Option<String>,
    pub photo: Option<String>,
}
