use serde::{Deserialize, Serialize};

use crate::domain::AlphabetRemoveDateIdeaCommand;

#[derive(Serialize, Deserialize)]
pub struct AlphabetRemoveDateIdeaResource {
    pub date_idea_id: String,
}

impl From<(String, AlphabetRemoveDateIdeaResource)> for AlphabetRemoveDateIdeaCommand {
    fn from(value: (String, AlphabetRemoveDateIdeaResource)) -> Self {
        let (alphabet_id, resource) = value;
        Self {
            alphabet_id,
            date_idea_id: resource.date_idea_id,
        }
    }
}
