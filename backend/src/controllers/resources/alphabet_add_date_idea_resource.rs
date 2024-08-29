use serde::{Deserialize, Serialize};

use crate::domain::AlphabetAddDateIdeaCommand;

#[derive(Serialize, Deserialize)]
pub struct AlphabetAddDateIdeaResource {
    pub date_idea_id: String,
}

impl From<(String, AlphabetAddDateIdeaResource)> for AlphabetAddDateIdeaCommand {
    fn from(value: (String, AlphabetAddDateIdeaResource)) -> Self {
        let (alphabet_id, resource) = value;
        Self {
            alphabet_id,
            date_idea_id: resource.date_idea_id,
            letter: char::default(),
        }
    }
}
