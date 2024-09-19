use serde::{Deserialize, Serialize};

use crate::domain::DateIdeaUpdateCommand;

#[derive(Serialize, Deserialize)]
pub struct DateIdeaUpdateResource {
    pub idea: Option<String>,
    pub description: Option<String>,
}

impl From<(String, DateIdeaUpdateResource)> for DateIdeaUpdateCommand {
    fn from(value: (String, DateIdeaUpdateResource)) -> Self {
        let (id, value) = value;
        Self {
            id,
            idea: value.idea,
            description: value.description,
        }
    }
}
