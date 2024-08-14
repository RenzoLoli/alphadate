use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct UserUpdate {
    pub username: Option<String>,
    pub couplename: Option<String>,
    pub anniversary: Option<String>,
    pub photo: Option<String>,
}

impl UserUpdate {
    pub fn new(
        username: Option<String>,
        couplename: Option<String>,
        anniversary: Option<String>,
        photo: Option<String>,
    ) -> Self {
        Self {
            username,
            couplename,
            anniversary,
            photo,
        }
    }
}
