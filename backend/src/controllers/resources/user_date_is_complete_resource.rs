use serde::{Deserialize, Serialize};

use crate::domain::EUserDate;

#[derive(Serialize, Deserialize)]
pub struct UserDateIsCompleteResource {
    completed: bool,
}

impl From<EUserDate> for UserDateIsCompleteResource {
    fn from(user_date: EUserDate) -> Self {
        Self {
            completed: user_date.completed,
        }
    }
}
