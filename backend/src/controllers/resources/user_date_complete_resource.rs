use serde::{Deserialize, Serialize};

use crate::domain::RUserDate;

use super::DateIdeaCompleteResource;

#[derive(Serialize, Deserialize)]
pub struct UserDateCompleteResource {
    pub id: String,
    pub letter: String,
    pub completed: bool,
    pub date_idea: DateIdeaCompleteResource,
}

impl From<RUserDate> for UserDateCompleteResource {
    fn from(user_date: RUserDate) -> Self {
        Self {
            id: user_date.id.to_string(),
            letter: user_date.letter,
            completed: user_date.completed,
            date_idea: DateIdeaCompleteResource::from(user_date.date_idea),
        }
    }
}
