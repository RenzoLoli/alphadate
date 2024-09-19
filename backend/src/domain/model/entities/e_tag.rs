use serde::{Deserialize, Serialize};

use crate::domain::{model::value_objects::IdObject, TagCreateCommand, TagUpdateCommand};

use super::Entity;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ETag {
    pub id: IdObject,
    pub name: String,
}

impl Entity for ETag {
    fn get_table_name() -> &'static str {
        "tags"
    }

    fn get_id(&self) -> &IdObject {
        &self.id
    }

    fn get_mut_id(&mut self) -> &mut IdObject {
        &mut self.id
    }
}

impl From<TagCreateCommand> for ETag {
    fn from(value: TagCreateCommand) -> Self {
        Self {
            id: IdObject::default(),
            name: value.name,
        }
    }
}

impl ETag {
    pub fn update(&mut self, command: TagUpdateCommand) {
        if let Some(name) = command.name {
            self.name = name;
        }
    }
}
