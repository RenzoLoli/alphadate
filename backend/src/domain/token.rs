use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Token {
    pub token: String,
}

impl Token {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}
