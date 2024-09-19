use serde::{Deserialize, Serialize};

use super::TagAggregate;

#[derive(Serialize, Deserialize)]
pub struct DateIdeaAggregate {
    pub id: String,
    pub idea: String,
    pub description: String,
    pub tags: Vec<TagAggregate>,
}
