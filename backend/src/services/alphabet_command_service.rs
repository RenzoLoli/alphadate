use std::sync::Arc;

use crate::{
    domain::{
        AlphabetAddDateIdeaCommand, AlphabetCreateCommand, AlphabetDeleteCommand,
        AlphabetRemoveDateIdeaCommand, AlphabetUpdateCommand, EAlphabet, EUserDate,
    },
    repository::{
        AlphabetRepository, BaseRepository, DateIdeaRepository, UserDateRepository, UserRepository,
    },
};

use super::ServiceHandlerTrait;

#[derive(Default)]
pub struct AlphabetCommandService {
    alphabet_repository: Arc<AlphabetRepository>,
    user_date_repository: Arc<UserDateRepository>,
    user_repository: Arc<UserRepository>,
    date_idea_repository: Arc<DateIdeaRepository>,
}

impl AlphabetCommandService {
    pub fn new(
        alphabet_repository: Arc<AlphabetRepository>,
        user_date_repository: Arc<UserDateRepository>,
        user_repository: Arc<UserRepository>,
        date_idea_repository: Arc<DateIdeaRepository>,
    ) -> Self {
        Self {
            alphabet_repository,
            user_date_repository,
            user_repository,
            date_idea_repository,
        }
    }
}

impl ServiceHandlerTrait<AlphabetCreateCommand, EAlphabet> for AlphabetCommandService {
    async fn _handle(&self, command: AlphabetCreateCommand) -> Result<EAlphabet, String> {
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
    async fn _handle(&self, command: AlphabetUpdateCommand) -> Result<EAlphabet, String> {
        let mut entity = match self.alphabet_repository.find_by_id(&command.id).await {
            Some(alphabet_ent) => alphabet_ent,
            None => return Err("alphabet not found".to_owned()),
        };

        if !command.need_changes() {
            return Err("No changes to update".to_owned());
        }

        entity.update(command);

        let alphabet_ent = match self.alphabet_repository.update(entity).await {
            Some(alphabet_ent) => alphabet_ent,
            None => return Err("alphabet cannot be not updated".to_owned()),
        };

        Ok(alphabet_ent)
    }
}

impl ServiceHandlerTrait<AlphabetDeleteCommand, EAlphabet> for AlphabetCommandService {
    async fn _handle(&self, command: AlphabetDeleteCommand) -> Result<EAlphabet, String> {
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

impl ServiceHandlerTrait<AlphabetAddDateIdeaCommand, EAlphabet> for AlphabetCommandService {
    async fn _handle(&self, mut command: AlphabetAddDateIdeaCommand) -> Result<EAlphabet, String> {
        let alphabet_ent = match self
            .alphabet_repository
            .find_by_id(&command.alphabet_id)
            .await
        {
            Some(alphabet_ent) => alphabet_ent,
            None => return Err("Alphabet not found".to_owned()),
        };

        let date_idea_ent = match self
            .date_idea_repository
            .find_by_id(&command.date_idea_id)
            .await
        {
            Some(date_idea_ent) => date_idea_ent,
            None => return Err("Date idea not found".to_owned()),
        };

        command.letter = match date_idea_ent.idea.chars().next() {
            Some(letter) => letter,
            None => return Err("Something is wrong with date idea".to_owned()),
        };

        if self
            .user_date_repository
            .find_by_alphabet_id_and_letter(command.letter)
            .await
            .pop()
            .is_some()
        {
            return Err("Letter is already used".to_owned());
        };

        let user_date_ent = EUserDate::from(command);

        match self.user_date_repository.create(user_date_ent).await {
            Some(_) => Ok(alphabet_ent),
            None => Err("Reference cannot be not added".to_owned()),
        }
    }
}

impl ServiceHandlerTrait<AlphabetRemoveDateIdeaCommand, EAlphabet> for AlphabetCommandService {
    async fn _handle(&self, command: AlphabetRemoveDateIdeaCommand) -> Result<EAlphabet, String> {
        let alphabet_ent = match self
            .alphabet_repository
            .find_by_id(&command.alphabet_id)
            .await
        {
            Some(alphabet_ent) => alphabet_ent,
            None => return Err("Alphabet not found".to_owned()),
        };

        let user_date_ent = match self
            .user_date_repository
            .find_by_alphabet_and_date_idea_id(&command.alphabet_id, &command.date_idea_id)
            .await
        {
            Some(user_date_ent) => user_date_ent,
            None => return Err("User date not found".to_owned()),
        };

        match self
            .user_date_repository
            .delete(&user_date_ent.id.to_string())
            .await
        {
            Some(_) => Ok(alphabet_ent),
            None => Err("Reference cannot be not deleted".to_owned()),
        }
    }
}
