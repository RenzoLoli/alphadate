use serde::{Deserialize, Serialize};

use crate::domain::ETag;

#[derive(Serialize, Deserialize)]
pub struct TagResource {
    pub id: String,
    pub name: String,
}

impl From<ETag> for TagResource {
    fn from(tag: ETag) -> Self {
        Self {
            id: tag.id.to_string(),
            name: tag.name,
        }
    }
}
