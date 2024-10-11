use std::sync::Arc;

use crate::{
    domain::{AlphabetToggleCompleteDateCommand, EUserDate},
    repository::{AlphabetRepository, BaseTransactions, UserDateRepository},
};

use super::ServiceHandlerTrait;

#[derive(Default)]
pub struct UserDateCommandService {
    alphabet_repository: Arc<AlphabetRepository>,
    user_date_repository: Arc<UserDateRepository>,
}

impl UserDateCommandService {
    pub fn new(
        alphabet_repository: Arc<AlphabetRepository>,
        user_date_repository: Arc<UserDateRepository>,
    ) -> Self {
        Self {
            alphabet_repository,
            user_date_repository,
        }
    }
}

impl ServiceHandlerTrait<AlphabetToggleCompleteDateCommand, EUserDate> for UserDateCommandService {
    async fn _handle(
        &self,
        command: AlphabetToggleCompleteDateCommand,
    ) -> Result<EUserDate, String> {
        if self
            .alphabet_repository
            .find_by_id(&command.id)
            .await
            .is_none()
        {
            return Err("Alphabet not found".to_owned());
        }

        let mut user_date = match self
            .user_date_repository
            .find_by_id(&command.user_date_id)
            .await
        {
            Some(user_date) => user_date,
            None => return Err("User date not found".to_owned()),
        };

        user_date.toggle_complete();

        match self.user_date_repository.update(user_date).await {
            Some(user_date) => Ok(user_date),
            None => Err("User date cannot be updated".to_owned()),
        }
    }
}
