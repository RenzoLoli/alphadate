use serde::{Deserialize, Serialize};

use crate::domain::model::value_objects::IdObject;

use super::Entity;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct EAlphabet {
    pub id: IdObject,
    pub title: String,
    pub user_id: IdObject,
}

impl Entity for EAlphabet {
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
