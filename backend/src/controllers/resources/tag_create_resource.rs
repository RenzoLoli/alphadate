use serde::{Deserialize, Serialize};

use crate::domain::TagCreateCommand;

#[derive(Serialize, Deserialize)]
pub struct TagCreateResource {
    pub name: String,
}

impl From<TagCreateResource> for TagCreateCommand {
    fn from(value: TagCreateResource) -> Self {
        Self { name: value.name }
    }
}
