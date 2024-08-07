use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OkResponse {
    message: String,
}

impl OkResponse {
    pub fn new(msg: String) -> Self {
        Self { message: msg }
    }
}
