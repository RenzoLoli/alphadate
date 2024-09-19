use serde::{Deserialize, Serialize};

use crate::domain::EDateIdea;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DateIdeaResource {
    pub id: String,
    pub idea: String,
    pub description: String,
}

impl From<EDateIdea> for DateIdeaResource {
    fn from(signin_resource: EDateIdea) -> Self {
        Self {
            id: signin_resource.id.to_string(),
            idea: signin_resource.idea,
            description: signin_resource.description,
        }
    }
}
