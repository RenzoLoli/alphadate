use serde::{Deserialize, Serialize};

use crate::domain::{RAlphabet, RUserDate};

use super::{DateIdeaAggregate, TagAggregate, UserDateAggregate};

#[derive(Serialize, Deserialize, Debug)]
pub struct AlphabetAggregate {
    pub id: String,
    pub title: String,
    pub user_dates: Vec<UserDateAggregate>,
}

impl From<RAlphabet> for AlphabetAggregate {
    fn from(alphabet: RAlphabet) -> Self {
        Self {
            id: alphabet.id.to_string(),
            title: alphabet.title,
            user_dates: alphabet
                .user_dates
                .into_iter()
                .map(|user_date: RUserDate| UserDateAggregate {
                    id: user_date.id.to_string(),
                    letter: user_date.letter,
                    completed: user_date.completed,
                    date_idea: DateIdeaAggregate {
                        id: user_date.date_idea.id.to_string(),
                        idea: user_date.date_idea.idea,
                        description: user_date.date_idea.description,
                        tags: user_date
                            .date_idea
                            .tags
                            .into_iter()
                            .map(|tag| TagAggregate {
                                id: tag.id.to_string(),
                                name: tag.name,
                            })
                            .collect::<Vec<TagAggregate>>(),
                    },
                })
                .collect::<Vec<UserDateAggregate>>(),
        }
    }
}
