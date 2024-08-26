use std::sync::Arc;

use crate::{
    domain::{EUser, UserDeleteCommand, UserUpdateCommand},
    repository::{BaseRepository, UserRepository},
};

use super::ServiceHandlerTrait;

#[derive(Default)]
pub struct UserCommandService {
    user_repository: Arc<UserRepository>,
}

impl UserCommandService {
    pub fn new(user_repository: Arc<UserRepository>) -> Self {
        Self { user_repository }
    }
}

impl ServiceHandlerTrait<UserUpdateCommand, EUser> for UserCommandService {
    async fn handle(&self, command: UserUpdateCommand) -> Result<EUser, String> {
        let mut user = match self.user_repository.find_by_id(command.id.as_str()).await {
            Some(user) => user,
            None => return Err("User not found".to_owned()),
        };

        user.update(command);

        match self.user_repository.update(user).await {
            Some(user) => Ok(user),
            None => Err("User cannot be updated".to_owned()),
        }
    }
}

impl ServiceHandlerTrait<UserDeleteCommand, EUser> for UserCommandService {
    async fn handle(&self, command: UserDeleteCommand) -> Result<EUser, String> {
        let _ = match self.user_repository.find_by_id(&command.id).await {
            Some(user) => user,
            None => return Err("User not found".to_owned()),
        };

        match self.user_repository.delete(&command.id).await {
            Some(user) => Ok(user),
            None => Err("User cannot be deleted".to_owned()),
        }
    }
}
