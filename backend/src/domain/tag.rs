use serde::{Deserialize, Serialize};

use super::TagUpdate;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Tag {
    id: String,
    name: String,
}

impl Tag {
    pub fn new(name: String) -> Self {
        Self {
            id: "".to_owned(),
            name,
        }
    }

    // GETTERS
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    // SETTERS
    pub fn set_id(&mut self, id: &str) {
        self.id = String::from(id);
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }

    // DOMAIN RULES

    pub fn update(&mut self, options: &TagUpdate) {
        if let Some(name) = options.name.clone() {
            self.name = name.clone();
        }
    }
}
