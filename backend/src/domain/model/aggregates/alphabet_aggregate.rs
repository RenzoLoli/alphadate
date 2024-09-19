use serde::{Deserialize, Serialize};

use super::UserDateAggregate;

#[derive(Serialize, Deserialize)]
pub struct AlphabetAggregate {
    pub id: String,
    pub title: String,
    pub user_dates: Vec<UserDateAggregate>,
}
