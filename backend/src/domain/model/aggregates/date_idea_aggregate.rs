use serde::{Deserialize, Serialize};

use crate::domain::RDateIdeaTag;

use super::TagAggregate;

#[derive(Serialize, Deserialize, Debug)]
pub struct DateIdeaAggregate {
    pub id: String,
    pub idea: String,
    pub description: String,
    pub tags: Vec<TagAggregate>,
}

impl From<RDateIdeaTag> for DateIdeaAggregate {
    fn from(value: RDateIdeaTag) -> Self {
        Self {
            id: value.id.to_string(),
            idea: value.idea,
            description: value.description,
            tags: value
                .tags
                .into_iter()
                .map(|tag| TagAggregate {
                    id: tag.id.to_string(),
                    name: tag.name,
                })
                .collect(),
        }
    }
}
