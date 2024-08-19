use serde::{Deserialize, Serialize};
use surrealdb::sql::Id;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdObject {
    tb: String,
    id: Id,
}

impl Default for IdObject {
    fn default() -> Self {
        Self {
            tb: Default::default(),
            id: Id::String("".to_owned()),
        }
    }
}

impl IdObject {
    pub fn new(table: &str) -> Self {
        Self {
            id: Id::String(uuid::Uuid::new_v4().to_string()),
            tb: table.to_string(),
        }
    }

    pub fn tb(&self) -> &str {
        &self.tb
    }

    pub fn id(&self) -> String {
        self.id.to_string().clone()
    }

    pub fn set_tb(&mut self, tb: String) {
        self.tb = tb;
    }

    pub fn set_id(&mut self, id: String) {
        self.id = Id::String(id);
    }
}
