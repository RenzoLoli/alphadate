use serde::{Deserialize, Serialize};

use crate::domain::{model::value_objects::IdObject, AlphabetAddDateIdeaCommand};

use super::Entity;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct EUserDate {
    pub id: IdObject,
    pub letter: char,
    pub completed: bool,
    pub alphabet_id: IdObject,
    pub date_idea_id: IdObject,
}

impl Entity for EUserDate {
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

impl From<AlphabetAddDateIdeaCommand> for EUserDate {
    fn from(value: AlphabetAddDateIdeaCommand) -> Self {
        let table_name = EUserDate::get_table_name();

        let alphabet_id = IdObject::new(table_name, &value.alphabet_id);
        let date_idea_id = IdObject::new(table_name, &value.date_idea_id);

        Self {
            id: Default::default(),
            alphabet_id,
            date_idea_id,
            letter: value.letter,
            completed: false,
        }
    }
}
