use serde::{Deserialize, Serialize};

use crate::domain::EAlphabet;

#[derive(Serialize, Deserialize)]
pub struct AlphabetResource {
    pub id: String,
    pub title: String,
}

impl From<EAlphabet> for AlphabetResource {
    fn from(tag: EAlphabet) -> Self {
        Self {
            id: tag.id.to_string(),
            title: tag.title,
        }
    }
}
