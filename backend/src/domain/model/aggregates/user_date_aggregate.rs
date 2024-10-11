use serde::{Deserialize, Serialize};

use super::DateIdeaAggregate;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserDateAggregate {
    pub id: String,
    pub letter: String,
    pub completed: bool,
    pub date_idea: DateIdeaAggregate,
}
