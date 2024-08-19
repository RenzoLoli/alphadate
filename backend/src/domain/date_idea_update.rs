use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DateIdeaUpdate {
    pub idea: Option<String>,
    pub description: Option<String>,
}
