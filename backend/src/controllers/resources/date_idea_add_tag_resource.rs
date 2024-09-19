use serde::{Deserialize, Serialize};

use crate::domain::DateIdeaAddTagCommand;

#[derive(Serialize, Deserialize)]
pub struct DateIdeaAddTagResource {
    pub tag_id: String,
}

impl From<(String, DateIdeaAddTagResource)> for DateIdeaAddTagCommand {
    fn from(value: (String, DateIdeaAddTagResource)) -> Self {
        let (id, value) = value;
        Self {
            date_idea_id: id,
            tag_id: value.tag_id,
        }
    }
}
