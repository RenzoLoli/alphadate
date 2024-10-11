use std::sync::Arc;

use crate::{
    domain::{GetAllDateIdeasQuery, GetDateIdeaByIdQuery, RDateIdeaTag},
    repository::DateIdeaTagRefRepository,
};

use super::ServiceHandlerTrait;

#[derive(Default)]
pub struct DateIdeaQueryService {
    date_idea_tag_ref_repository: Arc<DateIdeaTagRefRepository>,
}

impl DateIdeaQueryService {
    pub fn new(date_idea_tag_ref_repository: Arc<DateIdeaTagRefRepository>) -> Self {
        Self {
            date_idea_tag_ref_repository,
        }
    }
}

impl ServiceHandlerTrait<GetAllDateIdeasQuery, Vec<RDateIdeaTag>> for DateIdeaQueryService {
    async fn _handle(&self, _query: GetAllDateIdeasQuery) -> Result<Vec<RDateIdeaTag>, String> {
        Ok(self.date_idea_tag_ref_repository.get_refs().await)
    }
}

impl ServiceHandlerTrait<GetDateIdeaByIdQuery, RDateIdeaTag> for DateIdeaQueryService {
    async fn _handle(&self, query: GetDateIdeaByIdQuery) -> Result<RDateIdeaTag, String> {
        match self.date_idea_tag_ref_repository.find_ref(&query.id).await {
            Some(date_idea_ref) => Ok(date_idea_ref),
            None => Err("Date Idea not found".to_owned()),
        }
    }
}
