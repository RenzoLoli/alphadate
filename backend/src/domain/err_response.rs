use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    message: String,
}

impl ErrorResponse {
    pub fn new(msg: String) -> Self {
        Self { message: msg }
    }
}
