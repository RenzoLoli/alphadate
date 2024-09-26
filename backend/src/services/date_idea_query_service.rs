use std::{collections::HashSet, sync::Arc};

use crate::{
    domain::{DateIdeaAggregate, GetAllDateIdeasQuery, GetDateIdeaByIdQuery, TagAggregate},
    repository::{BaseRepository, DateIdeaRepository, DateIdeaTagRepository, TagRepository},
};

use super::ServiceHandlerTrait;

#[derive(Default)]
pub struct DateIdeaQueryService {
    date_idea_repository: Arc<DateIdeaRepository>,
    date_idea_tag_repository: Arc<DateIdeaTagRepository>,
    tag_repository: Arc<TagRepository>,
}

impl DateIdeaQueryService {
    pub fn new(
        date_idea_repository: Arc<DateIdeaRepository>,
        date_idea_tag_repository: Arc<DateIdeaTagRepository>,
        tag_repository: Arc<TagRepository>,
    ) -> Self {
        Self {
            date_idea_repository,
            date_idea_tag_repository,
            tag_repository,
        }
    }
}

impl ServiceHandlerTrait<GetAllDateIdeasQuery, Vec<DateIdeaAggregate>> for DateIdeaQueryService {
    async fn _handle(
        &self,
        _query: GetAllDateIdeasQuery,
    ) -> Result<Vec<DateIdeaAggregate>, String> {
        let date_ideas = self.date_idea_repository.get_all().await;

        if date_ideas.is_empty() {
            return Ok(vec![]);
        }

        let date_idea_tags = self.date_idea_tag_repository.get_all().await;
        let tags = self.tag_repository.get_all().await;

        let idea_tag_tag_ids_set: HashSet<(String, String)> = date_idea_tags
            .iter()
            .map(|idea_tag| {
                (
                    idea_tag.date_idea_id.to_string(),
                    idea_tag.tag_id.to_string(),
                )
            })
            .collect();

        let aggregates = date_ideas
            .iter()
            .map(|date_idea| {
                let idea_tags = tags
                    .iter()
                    .filter(|tag| {
                        idea_tag_tag_ids_set
                            .contains(&(date_idea.id.to_string(), tag.id.to_string()))
                    })
                    .map(|tag| TagAggregate {
                        id: tag.id.to_string(),
                        name: tag.name.clone(),
                    })
                    .collect();

                DateIdeaAggregate {
                    id: date_idea.id.to_string(),
                    idea: date_idea.idea.clone(),
                    description: date_idea.description.clone(),
                    tags: idea_tags,
                }
            })
            .collect();

        Ok(aggregates)
    }
}

impl ServiceHandlerTrait<GetDateIdeaByIdQuery, DateIdeaAggregate> for DateIdeaQueryService {
    async fn _handle(&self, query: GetDateIdeaByIdQuery) -> Result<DateIdeaAggregate, String> {
        let date_idea = match self.date_idea_repository.find_by_id(&query.id).await {
            Some(date_idea) => date_idea,
            None => return Err("Date Idea not found".to_owned()),
        };

        let idea_tag_ids = self
            .date_idea_tag_repository
            .find_by_date_idea_id(&query.id)
            .await
            .into_iter()
            .map(|date_idea_tag| date_idea_tag.tag_id.to_string())
            .collect();

        let tags = self
            .tag_repository
            .find_by_ids(idea_tag_ids)
            .await
            .iter()
            .map(|tag| TagAggregate {
                id: tag.id.to_string(),
                name: tag.name.clone(),
            })
            .collect();

        Ok(DateIdeaAggregate {
            id: query.id.to_string(),
            idea: date_idea.idea,
            description: date_idea.description,
            tags,
        })
    }
}
