use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenResource {
    pub token: String,
}

impl TokenResource {
    pub fn new(token: &str) -> Self {
        TokenResource {
            token: token.to_string(),
        }
    }
}
