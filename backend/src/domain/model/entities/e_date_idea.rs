use serde::{Deserialize, Serialize};

use crate::domain::{model::value_objects::IdObject, DateIdeaCreateCommand, DateIdeaUpdateCommand};

use super::Entity;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct EDateIdea {
    pub id: IdObject,
    pub idea: String,
    pub description: String,
}

impl Entity for EDateIdea {
    fn get_table_name() -> &'static str {
        "date_ideas"
    }

    fn get_id(&self) -> &IdObject {
        &self.id
    }

    fn get_mut_id(&mut self) -> &mut IdObject {
        &mut self.id
    }
}

impl From<DateIdeaCreateCommand> for EDateIdea {
    fn from(value: DateIdeaCreateCommand) -> Self {
        Self {
            id: IdObject::default(),
            idea: value.idea,
            description: value.description,
        }
    }
}

impl EDateIdea {
    pub fn update(&mut self, command: DateIdeaUpdateCommand) {
        if let Some(idea) = command.idea {
            self.idea = idea;
        }

        if let Some(description) = command.description {
            self.description = description;
        }
    }
}
