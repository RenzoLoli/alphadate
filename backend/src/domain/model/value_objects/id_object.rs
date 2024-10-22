use serde::{Deserialize, Serialize};
use surrealdb::sql::Id;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "$surrealdb::private::sql::Thing")]
pub struct IdObject {
    tb: String,
    id: Id,
}

impl IdObject {
    pub fn new(table: &str, id: &str) -> Self {
        Self {
            id: Id::String(id.to_string()),
            tb: table.to_string(),
        }
    }
}

impl Default for IdObject {
    fn default() -> Self {
        Self {
            id: Id::String(String::from("")),
            tb: String::new(),
        }
    }
}

impl std::fmt::Display for IdObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id.to_raw())
    }
}

impl PartialEq for IdObject {
    fn eq(&self, other: &Self) -> bool {
        self.tb == other.tb && self.id.to_string() == other.id.to_string()
    }
}

impl Eq for IdObject {}

impl IdObject {
    pub fn generate(&mut self, table: &str) {
        log::debug!("Generating id for <{}>", table);

        self.id = Id::rand();
        self.tb = table.to_string();

        log::debug!("id -> {:?}", self.id);
    }
}
