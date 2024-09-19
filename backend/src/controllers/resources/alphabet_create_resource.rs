use serde::{Deserialize, Serialize};

use crate::domain::AlphabetCreateCommand;

#[derive(Serialize, Deserialize)]
pub struct AlphabetCreateResource {
    pub title: String,
    pub user_id: String,
}

impl From<AlphabetCreateResource> for AlphabetCreateCommand {
    fn from(value: AlphabetCreateResource) -> Self {
        Self {
            title: value.title,
            user_id: value.user_id,
        }
    }
}
