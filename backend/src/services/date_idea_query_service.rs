use rand::prelude::*;
use std::sync::Arc;

use crate::{
    domain::{GetAllDateIdeasQuery, GetDateIdeaByIdQuery, GetRandomDateIdeaQuery, RDateIdeaTag},
    repository::{DateIdeaTagRefRepository, UserDateRepository},
};

use super::ServiceHandlerTrait;

#[derive(Default)]
pub struct DateIdeaQueryService {
    date_idea_tag_ref_repository: Arc<DateIdeaTagRefRepository>,
    user_date_repository: Arc<UserDateRepository>,
}

impl DateIdeaQueryService {
    pub fn new(
        date_idea_tag_ref_repository: Arc<DateIdeaTagRefRepository>,
        user_date_repository: Arc<UserDateRepository>,
    ) -> Self {
        Self {
            date_idea_tag_ref_repository,
            user_date_repository,
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

impl ServiceHandlerTrait<GetRandomDateIdeaQuery, RDateIdeaTag> for DateIdeaQueryService {
    async fn _handle(&self, query: GetRandomDateIdeaQuery) -> Result<RDateIdeaTag, String> {
        let refs = if query.exclude_active {
            let alphabet_id = query.alphabet_id;

            let user_dates = self
                .user_date_repository
                .find_by_alphabet_id(&alphabet_id)
                .await;

            let active_letters = user_dates
                .iter()
                .map(|user_date| user_date.letter.clone())
                .collect::<Vec<String>>();

            self.date_idea_tag_ref_repository
                .find_ref_not_by_letters(active_letters)
                .await
        } else {
            self.date_idea_tag_ref_repository.get_refs().await
        };

        if refs.is_empty() {
            return Err("No dates found".to_owned());
        }

        let random = rand::thread_rng().gen_range(0..refs.len());

        Ok(refs[random].clone())
    }
}
