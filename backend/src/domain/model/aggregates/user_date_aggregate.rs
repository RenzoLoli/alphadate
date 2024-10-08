use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserDateAggregate {
    pub id: String,
    pub letter: char,
    pub completed: bool,
    pub date_idea_id: String,
}
