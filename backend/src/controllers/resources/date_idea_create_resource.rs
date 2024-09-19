use serde::{Deserialize, Serialize};

use crate::domain::DateIdeaCreateCommand;

#[derive(Serialize, Deserialize)]
pub struct DateIdeaCreateResource {
    pub idea: String,
    pub description: String,
}

impl From<DateIdeaCreateResource> for DateIdeaCreateCommand {
    fn from(value: DateIdeaCreateResource) -> Self {
        Self {
            idea: value.idea,
            description: value.description,
        }
    }
}
