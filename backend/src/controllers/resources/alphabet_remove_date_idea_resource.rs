use serde::{Deserialize, Serialize};

use crate::domain::AlphabetRemoveDateIdeaCommand;

#[derive(Serialize, Deserialize)]
pub struct AlphabetRemoveDateIdeaResource {
    pub date_idea_id: String,
}

impl From<(String, String)> for AlphabetRemoveDateIdeaCommand {
    fn from(value: (String, String)) -> Self {
        let (alphabet_id, date_idea_id) = value;
        Self {
            alphabet_id,
            date_idea_id,
        }
    }
}
