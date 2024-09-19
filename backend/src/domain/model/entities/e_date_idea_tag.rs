use serde::{Deserialize, Serialize};

use crate::domain::{model::value_objects::IdObject, DateIdeaAddTagCommand};

use super::Entity;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct EDateIdeaTag {
    pub id: IdObject,
    pub date_idea_id: IdObject,
    pub tag_id: IdObject,
}

impl Entity for EDateIdeaTag {
    fn get_table_name() -> &'static str {
        "date_idea_tags"
    }

    fn get_id(&self) -> &IdObject {
        &self.id
    }

    fn get_mut_id(&mut self) -> &mut IdObject {
        &mut self.id
    }
}

impl From<DateIdeaAddTagCommand> for EDateIdeaTag {
    fn from(value: DateIdeaAddTagCommand) -> Self {
        let table_name = EDateIdeaTag::get_table_name();

        let date_idea_id = IdObject::new(table_name, &value.date_idea_id);
        let tag_id = IdObject::new(table_name, &value.tag_id);

        Self {
            id: Default::default(),
            date_idea_id,
            tag_id,
        }
    }
}
