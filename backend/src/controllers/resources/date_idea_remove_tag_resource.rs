use serde::{Deserialize, Serialize};

use crate::domain::DateIdeaRemoveTagCommand;

#[derive(Serialize, Deserialize)]
pub struct DateIdeaRemoveTagResource {
    pub tag_id: String,
}

impl From<(String, DateIdeaRemoveTagResource)> for DateIdeaRemoveTagCommand {
    fn from(value: (String, DateIdeaRemoveTagResource)) -> Self {
        let (id, value) = value;
        Self {
            date_idea_id: id,
            tag_id: value.tag_id,
        }
    }
}
