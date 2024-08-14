use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DateIdeaUpdate {
    pub idea: Option<String>,
    pub description: Option<String>,
}

impl DateIdeaUpdate {
    pub fn new(idea: Option<String>, description: Option<String>) -> Self {
        Self { idea, description }
    }
}
