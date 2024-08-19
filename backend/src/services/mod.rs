mod auth_service;
mod date_idea_service;
mod env_service;
mod password_service;
mod tag_service;
mod token_service;
mod user_service;

use std::sync::Arc;

use actix_web::web;
pub use auth_service::AuthService;
pub use date_idea_service::DateIdeaService;
pub use env_service::EnvService;
pub use password_service::PasswordService;
pub use tag_service::TagService;
pub use token_service::TokenService;
pub use user_service::UserService;

pub struct Services {
    pub user_service: Arc<UserService>,
    pub auth_service: Arc<AuthService>,
    pub date_idea_service: Arc<DateIdeaService>,
}

pub type ContextServices = web::Data<Services>;