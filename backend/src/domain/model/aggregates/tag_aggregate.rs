use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TagAggregate {
    pub id: String,
    pub name: String,
}
