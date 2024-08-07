use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct UserAuth {
    pub email: String,
    pub password: String,
}

impl UserAuth {
    pub fn new(email: String, password: String) -> Self {
        Self { email, password }
    }
}
