use serde::{Deserialize, Serialize};

use crate::domain::ETag;

#[derive(Serialize, Deserialize)]
pub struct DateIdeaAggregate {
    pub id: String,
    pub idea: String,
    pub description: String,
    pub tags: Vec<ETag>,
}
