use std::sync::Arc;

use crate::{
    domain::{
        AlphabetCreateCommand, AlphabetDeleteCommand, AlphabetUpdateCommand, EAlphabet, EUserDate,
    },
    repository::{AlphabetRepository, BaseRepository, UserDateRepository, UserRepository},
};

use super::ServiceHandlerTrait;

#[derive(Default)]
pub struct AlphabetCommandService {
    alphabet_repository: Arc<AlphabetRepository>,
    user_date_repository: Arc<UserDateRepository>,
    user_repository: Arc<UserRepository>,
}

impl AlphabetCommandService {
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

impl ServiceHandlerTrait<AlphabetCreateCommand, EAlphabet> for AlphabetCommandService {
    async fn handle(&self, command: AlphabetCreateCommand) -> Result<EAlphabet, String> {
        if (self.user_repository.find_by_id(&command.user_id).await).is_none() {
            return Err("User not found".to_owned());
        };

        let entity = EAlphabet::from(command);
        let alphabet_ent = match self.alphabet_repository.create(entity).await {
            Some(alphabet_ent) => alphabet_ent,
            None => return Err("alphabet not created".to_owned()),
        };

        Ok(alphabet_ent)
    }
}

impl ServiceHandlerTrait<AlphabetUpdateCommand, EAlphabet> for AlphabetCommandService {
    async fn handle(&self, command: AlphabetUpdateCommand) -> Result<EAlphabet, String> {
        let mut entity = match self.alphabet_repository.find_by_id(&command.id).await {
            Some(alphabet_ent) => alphabet_ent,
            None => return Err("alphabet not found".to_owned()),
        };

        entity.update(command);

        let alphabet_ent = match self.alphabet_repository.update(entity).await {
            Some(alphabet_ent) => alphabet_ent,
            None => return Err("alphabet cannot be not updated".to_owned()),
        };

        Ok(alphabet_ent)
    }
}

impl ServiceHandlerTrait<AlphabetDeleteCommand, EAlphabet> for AlphabetCommandService {
    async fn handle(&self, command: AlphabetDeleteCommand) -> Result<EAlphabet, String> {
        let entity = match self.alphabet_repository.find_by_id(&command.id).await {
            Some(alphabet_ent) => alphabet_ent,
            None => return Err("alphabet not found".to_owned()),
        };

        let user_date_ents = self.user_date_repository.get_all().await;

        let user_date_ent_refs = user_date_ents
            .into_iter()
            .filter(|user_date| user_date.alphabet_id == entity.id)
            .collect::<Vec<EUserDate>>();

        for user_date_ent_ref in user_date_ent_refs {
            self.user_date_repository
                .delete(&user_date_ent_ref.id.to_string())
                .await;
        }

        let alphabet_ent = match self.alphabet_repository.delete(&command.id).await {
            Some(alphabet_ent) => alphabet_ent,
            None => return Err("alphabet cannot be not deleted".to_owned()),
        };

        Ok(alphabet_ent)
    }
}
