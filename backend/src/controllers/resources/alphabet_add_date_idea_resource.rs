use serde::{Deserialize, Serialize};

use crate::domain::AlphabetAddDateIdeaCommand;

#[derive(Serialize, Deserialize)]
pub struct AlphabetAddDateIdeaResource {
    pub date_idea_id: String,
}

impl From<(String, String)> for AlphabetAddDateIdeaCommand {
    fn from(value: (String, String)) -> Self {
        let (alphabet_id, date_idea_id) = value;
        Self {
            alphabet_id,
            date_idea_id,
            letter: String::new(),
        }
    }
}
