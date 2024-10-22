use serde::{Deserialize, Serialize};

use crate::domain::RAlphabet;

use super::UserDateCompleteResource;

#[derive(Serialize, Deserialize)]
pub struct AlphabetCompleteResource {
    pub id: String,
    pub title: String,
    pub user_dates: Vec<UserDateCompleteResource>,
}

impl From<RAlphabet> for AlphabetCompleteResource {
    fn from(alphabet: RAlphabet) -> Self {
        Self {
            id: alphabet.id.to_string().clone(),
            title: alphabet.title.clone(),
            user_dates: alphabet
                .user_dates
                .into_iter()
                .map(UserDateCompleteResource::from)
                .collect::<Vec<UserDateCompleteResource>>(),
        }
    }
}
