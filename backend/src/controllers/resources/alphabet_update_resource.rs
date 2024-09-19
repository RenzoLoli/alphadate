use serde::{Deserialize, Serialize};

use crate::domain::AlphabetUpdateCommand;

#[derive(Serialize, Deserialize)]
pub struct AlphabetUpdateResource {
    pub title: Option<String>,
}

impl From<(String, AlphabetUpdateResource)> for AlphabetUpdateCommand {
    fn from(value: (String, AlphabetUpdateResource)) -> Self {
        let (id, value) = value;
        Self {
            id,
            title: value.title,
        }
    }
}
