use serde::{Deserialize, Serialize};

use super::UserDateAggregate;

#[derive(Serialize, Deserialize, Debug)]
pub struct AlphabetAggregate {
    pub id: String,
    pub title: String,
    pub user_dates: Vec<UserDateAggregate>,
}
