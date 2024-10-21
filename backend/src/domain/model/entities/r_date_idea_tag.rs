use serde::{Deserialize, Serialize};

use crate::domain::model::value_objects::IdObject;

use super::{ETag, Entity};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct RDateIdeaTag {
    pub id: IdObject,
    pub idea: String,
    pub description: String,
    pub tags: Vec<ETag>,
}

impl Entity for RDateIdeaTag {
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
