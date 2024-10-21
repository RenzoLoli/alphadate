use serde::{Deserialize, Serialize};

use crate::domain::IdObject;

use super::{Entity, RUserDate};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RAlphabet {
    pub id: IdObject,
    pub title: String,
    pub user_dates: Vec<RUserDate>,
}

impl Entity for RAlphabet {
    fn get_table_name() -> &'static str {
        "alphabets"
    }

    fn get_id(&self) -> &IdObject {
        &self.id
    }

    fn get_mut_id(&mut self) -> &mut IdObject {
        &mut self.id
    }
}
