use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct OkResource {
    pub message: String,
}

impl OkResource {
    pub fn new(message: &str) -> Self {
        OkResource {
            message: message.to_string(),
        }
    }
}
