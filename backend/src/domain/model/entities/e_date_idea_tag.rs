use serde::{Deserialize, Serialize};

use crate::domain::model::value_objects::IdObject;

use super::Entity;

#[derive(Default,Debug, Clone, Serialize, Deserialize)]
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