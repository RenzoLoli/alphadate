use crate::domain::{RenewTokenCommand, ValidateTokenCommand};

use super::{ServiceHandlerTrait, TokenService};

type Token = String;

#[derive(Default)]
pub struct TokenCommandService;

impl TokenCommandService {
    pub fn new() -> Self {
        Self {}
    }
}

impl ServiceHandlerTrait<RenewTokenCommand, Token> for TokenCommandService {
    async fn handle(&self, query: RenewTokenCommand) -> Result<Token, String> {
        match TokenService::renew_token(query.token) {
            Ok(token) => Ok(token),
            Err(_) => Err("Invalid Token".to_owned()),
        }
    }
}

impl ServiceHandlerTrait<ValidateTokenCommand, Token> for TokenCommandService {
    async fn handle(&self, query: ValidateTokenCommand) -> Result<Token, String> {
        let n_token = query.token.clone();
        match TokenService::validate_token(query.token) {
            Ok(_) => Ok(n_token),
            Err(_) => Err("Invalid Token".to_owned()),
        }
    }
}
