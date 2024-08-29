mod alphabet_command_service;
mod alphabet_query_service;
mod auth_command_service;
mod date_idea_command_service;
mod date_idea_query_service;
mod env_service;
mod handler;
mod password_service;
mod tag_command_service;
mod tag_query_service;
mod token_command_service;
mod token_service;
mod user_command_service;
mod user_query_service;

use std::sync::Arc;

use actix_web::web;
pub use alphabet_command_service::AlphabetCommandService;
pub use alphabet_query_service::AlphabetQueryService;
pub use auth_command_service::AuthCommandService;
pub use date_idea_command_service::DateIdeaCommandService;
pub use date_idea_query_service::DateIdeaQueryService;
pub use env_service::EnvService;
pub use handler::ServiceHandlerTrait;
pub use password_service::PasswordService;
pub use tag_query_service::TagQueryService;
pub use token_command_service::TokenCommandService;
pub use token_service::TokenService;
pub use user_command_service::UserCommandService;
pub use user_query_service::UserQueryService;

use crate::repository::Repositories;

use self::tag_command_service::TagCommandService;

#[derive(Clone)]
pub struct Services {
    pub user_query_service: Arc<UserQueryService>,
    pub user_command_service: Arc<UserCommandService>,

    pub auth_command_service: Arc<AuthCommandService>,

    pub alphabet_command_service: Arc<AlphabetCommandService>,
    pub alphabet_query_service: Arc<AlphabetQueryService>,

    pub date_idea_command_service: Arc<DateIdeaCommandService>,
    pub date_idea_query_service: Arc<DateIdeaQueryService>,

    pub tag_query_service: Arc<TagQueryService>,
    pub tag_command_service: Arc<TagCommandService>,

    pub token_command_service: Arc<TokenCommandService>,
}

impl Services {
    pub fn new() -> Self {
        Self {
            user_query_service: Arc::default(),
            user_command_service: Arc::default(),
            auth_command_service: Arc::default(),
            date_idea_command_service: Arc::default(),
            date_idea_query_service: Arc::default(),
            alphabet_query_service: Arc::default(),
            alphabet_command_service: Arc::default(),
            tag_query_service: Arc::default(),
            tag_command_service: Arc::default(),
            token_command_service: Arc::default(),
        }
    }

    pub fn load(mut self, repositories: Repositories) -> Self {
        self.user_query_service =
            Arc::new(UserQueryService::new(repositories.user_repository.clone()));

        self.user_command_service = Arc::new(UserCommandService::new(
            repositories.user_repository.clone(),
        ));
        self.auth_command_service = Arc::new(AuthCommandService::new(
            repositories.user_repository.clone(),
        ));

        self.alphabet_query_service = Arc::new(AlphabetQueryService::new(
            repositories.alphabet_repository.clone(),
            repositories.user_date_repository.clone(),
            repositories.user_repository.clone(),
        ));
        self.alphabet_command_service = Arc::new(AlphabetCommandService::new(
            repositories.alphabet_repository.clone(),
            repositories.user_date_repository.clone(),
            repositories.user_repository.clone(),
            repositories.date_idea_repository.clone(),
        ));

        self.date_idea_command_service = Arc::new(DateIdeaCommandService::new(
            repositories.date_idea_repository.clone(),
            repositories.date_idea_tag_repository.clone(),
            repositories.tag_repository.clone(),
        ));
        self.date_idea_query_service = Arc::new(DateIdeaQueryService::new(
            repositories.date_idea_repository.clone(),
            repositories.date_idea_tag_repository.clone(),
            repositories.tag_repository.clone(),
        ));
        self.tag_query_service =
            Arc::new(TagQueryService::new(repositories.tag_repository.clone()));

        self.tag_command_service = Arc::new(TagCommandService::new(
            repositories.tag_repository.clone(),
            repositories.date_idea_tag_repository.clone(),
        ));

        self.token_command_service = Arc::new(TokenCommandService::new());

        self
    }
}

pub type ContextServices = web::Data<Services>;
