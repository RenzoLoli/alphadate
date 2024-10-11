use serde::{Deserialize, Serialize};

use crate::domain::{model::value_objects::IdObject, AlphabetCreateCommand, AlphabetUpdateCommand};

use super::{EUser, Entity};

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

impl From<AlphabetCreateCommand> for EAlphabet {
    fn from(value: AlphabetCreateCommand) -> Self {
        Self {
            id: IdObject::default(),
            title: value.title,
            user_id: IdObject::new(EUser::get_table_name(), &value.user_id),
        }
    }
}

impl EAlphabet {
    pub fn update(&mut self, command: AlphabetUpdateCommand) {
        if let Some(title) = command.title {
            self.title = title;
        }
    }
}
