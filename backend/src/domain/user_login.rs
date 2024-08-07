use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}

impl UserLogin {
    pub fn new(email: String, password: String) -> Self {
        Self { email, password }
    }
}
