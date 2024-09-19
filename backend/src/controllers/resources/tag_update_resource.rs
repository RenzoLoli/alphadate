use serde::{Deserialize, Serialize};

use crate::domain::TagUpdateCommand;

#[derive(Serialize, Deserialize)]
pub struct TagUpdateResource {
    pub name: Option<String>,
}

impl From<(String, TagUpdateResource)> for TagUpdateCommand {
    fn from(value: (String, TagUpdateResource)) -> Self {
        let (id, value) = value;
        Self {
            id,
            name: value.name,
        }
    }
}
