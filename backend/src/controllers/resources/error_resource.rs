use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResource {
    pub message: String,
}

impl ErrorResource {
    pub fn new(message: &str) -> Self {
        ErrorResource {
            message: message.to_string(),
        }
    }
}
