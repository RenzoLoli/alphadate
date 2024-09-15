use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TagAggregate {
    pub id: String,
    pub name: String,
}
