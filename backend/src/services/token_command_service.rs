use std::sync::Arc;

use crate::domain::{RenewTokenCommand, ValidateTokenCommand};

use super::{ServiceHandlerTrait, TokenService};

type Token = String;

#[derive(Default)]
pub struct TokenCommandService {
    token_service: Arc<TokenService>,
}

impl TokenCommandService {
    pub fn new(token_service: Arc<TokenService>) -> Self {
        Self { token_service }
    }
}

impl ServiceHandlerTrait<RenewTokenCommand, Token> for TokenCommandService {
    async fn _handle(&self, query: RenewTokenCommand) -> Result<Token, String> {
        match self.token_service.renew_token(query.token) {
            Ok(token) => Ok(token),
            Err(_) => Err("Invalid Token".to_owned()),
        }
    }
}

impl ServiceHandlerTrait<ValidateTokenCommand, Token> for TokenCommandService {
    async fn _handle(&self, query: ValidateTokenCommand) -> Result<Token, String> {
        let n_token = query.token.clone();
        match self.token_service.validate_token(query.token) {
            Ok(_) => Ok(n_token),
            Err(_) => Err("Invalid Token".to_owned()),
        }
    }
}
