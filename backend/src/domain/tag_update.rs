use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TagUpdate {
    pub name: Option<String>,
}

impl TagUpdate {
    pub fn new(name: Option<String>) -> Self {
        Self { name }
    }
}
