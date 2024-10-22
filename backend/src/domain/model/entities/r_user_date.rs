use serde::{Deserialize, Serialize};

use crate::domain::IdObject;

use super::{Entity, RDateIdeaTag};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RUserDate {
    pub id: IdObject,
    pub letter: String,
    pub completed: bool,
    pub date_idea: RDateIdeaTag,
}

impl Entity for RUserDate {
    fn get_table_name() -> &'static str {
        "user_dates"
    }

    fn get_id(&self) -> &IdObject {
        &self.id
    }

    fn get_mut_id(&mut self) -> &mut IdObject {
        &mut self.id
    }
}
