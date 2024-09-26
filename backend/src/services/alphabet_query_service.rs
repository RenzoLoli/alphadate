use std::sync::Arc;

use crate::{
    domain::{AlphabetAggregate, GetAllAlphabetsQuery, GetAlphabetByIdQuery, UserDateAggregate},
    repository::{AlphabetRepository, BaseRepository, UserDateRepository, UserRepository},
};

use super::ServiceHandlerTrait;

#[derive(Default)]
pub struct AlphabetQueryService {
    alphabet_repository: Arc<AlphabetRepository>,
    user_date_repository: Arc<UserDateRepository>,
    user_repository: Arc<UserRepository>,
}

impl AlphabetQueryService {
    pub fn new(
        alphabet_repository: Arc<AlphabetRepository>,
        user_date_repository: Arc<UserDateRepository>,
        user_repository: Arc<UserRepository>,
    ) -> Self {
        Self {
            alphabet_repository,
            user_date_repository,
            user_repository,
        }
    }
}

impl ServiceHandlerTrait<GetAllAlphabetsQuery, Vec<AlphabetAggregate>> for AlphabetQueryService {
    async fn _handle(&self, query: GetAllAlphabetsQuery) -> Result<Vec<AlphabetAggregate>, String> {
        if (self.user_repository.find_by_id(&query.user_id).await).is_none() {
            return Err("User not found".to_owned());
        };

        let alphabet_ents = self
            .alphabet_repository
            .find_by_user_id(&query.user_id)
            .await;

        if alphabet_ents.is_empty() {
            return Ok(vec![]);
        }

        let mut aggregates: Vec<AlphabetAggregate> = vec![];
        for alphabet in alphabet_ents.iter() {
            let user_dates = self
                .user_date_repository
                .find_by_alphabet_id(&alphabet.id.to_string())
                .await;

            let aggregate = AlphabetAggregate {
                id: alphabet.id.to_string(),
                title: alphabet.title.clone(),
                user_dates: user_dates
                    .into_iter()
                    .map(|user_date| UserDateAggregate {
                        id: user_date.id.to_string(),
                        letter: user_date.letter,
                        completed: user_date.completed,
                        date_idea_id: user_date.date_idea_id.to_string(),
                    })
                    .collect::<Vec<UserDateAggregate>>(),
            };

            aggregates.push(aggregate);
        }

        Ok(aggregates)
    }
}

impl ServiceHandlerTrait<GetAlphabetByIdQuery, AlphabetAggregate> for AlphabetQueryService {
    async fn _handle(&self, query: GetAlphabetByIdQuery) -> Result<AlphabetAggregate, String> {
        let alphabet_ent = match self.alphabet_repository.find_by_id(&query.id).await {
            Some(alphabet_ent) => alphabet_ent,
            None => return Err("alphabet not found".to_owned()),
        };

        let user_date_ents = self.user_date_repository.get_all().await;

        Ok(AlphabetAggregate {
            id: alphabet_ent.id.to_string(),
            title: alphabet_ent.title,
            user_dates: user_date_ents
                .into_iter()
                .map(|user_date| UserDateAggregate {
                    id: user_date.id.to_string(),
                    letter: user_date.letter,
                    completed: user_date.completed,
                    date_idea_id: user_date.date_idea_id.to_string(),
                })
                .collect::<Vec<UserDateAggregate>>(),
        })
    }
}
