use std::sync::Arc;

use crate::{
    domain::{EUser, Entity, SignInCommand, SignUpCommand},
    repository::{BaseRepository, UserRepository},
};

use super::{PasswordService, ServiceHandlerTrait, TokenService};

type Token = String;

#[derive(Default)]
pub struct AuthCommandService {
    password_service: Arc<PasswordService>,
    user_repository: Arc<UserRepository>,
    token_service: Arc<TokenService>,
}

impl AuthCommandService {
    pub fn new(
        password_service: Arc<PasswordService>,
        token_service: Arc<TokenService>,
        user_repository: Arc<UserRepository>,
    ) -> Self {
        Self {
            password_service,
            token_service,
            user_repository,
        }
    }
}

impl ServiceHandlerTrait<SignInCommand, Token> for AuthCommandService {
    async fn _handle(&self, command: SignInCommand) -> Result<Token, String> {
        let finded_user = self.user_repository.find_by_email(&command.email).await;

        let Some(user) = finded_user else {
            return Err("User not found".to_owned());
        };

        if !self
            .password_service
            .validate(&command.password, &user.password)
        {
            return Err("Incorrect Password".to_owned());
        };

        let id = user.get_id().to_string();
        let token = self.token_service.create_token(&id)?;

        Ok(token)
    }
}

impl ServiceHandlerTrait<SignUpCommand, EUser> for AuthCommandService {
    async fn _handle(&self, mut command: SignUpCommand) -> Result<EUser, String> {
        let finded_user = self.user_repository.find_by_email(&command.email).await;

        if finded_user.is_some() {
            return Err("User is already registered".to_owned());
        }

        let encrypted_password = match self.password_service.encrypt(&command.password) {
            Ok(password) => password,
            Err(error) => {
                log::debug!("Password encryption error: {}", error);
                return Err("Internal Server Error".to_owned());
            }
        };

        command.password = encrypted_password;

        let created = self.user_repository.create(EUser::from(command)).await;

        match created {
            Some(user) => Ok(user),
            None => Err("User not created".to_owned()),
        }
    }
}
