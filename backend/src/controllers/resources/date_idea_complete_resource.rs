use serde::{Deserialize, Serialize};

use crate::domain::RDateIdeaTag;

use super::TagResource;

#[derive(Serialize, Deserialize)]
pub struct DateIdeaCompleteResource {
    pub id: String,
    pub idea: String,
    pub description: String,
    pub tags: Vec<TagResource>,
}

impl From<RDateIdeaTag> for DateIdeaCompleteResource {
    fn from(tag: RDateIdeaTag) -> Self {
        Self {
            id: tag.id.to_string(),
            idea: tag.idea,
            description: tag.description,
            tags: tag
                .tags
                .into_iter()
                .map(|tag| TagResource {
                    id: tag.id.to_string(),
                    name: tag.name,
                })
                .collect::<Vec<TagResource>>(),
        }
    }
}
