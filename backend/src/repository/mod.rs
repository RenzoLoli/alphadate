mod alphabet_repository;
mod base_repository;
mod date_idea_repository;
mod date_idea_tag_repository;
mod tag_repository;
mod user_date_repository;
mod user_repository;

use std::sync::Arc;

pub use base_repository::BaseRepository;

pub use alphabet_repository::AlphabetRepository;
pub use date_idea_repository::DateIdeaRepository;
pub use date_idea_tag_repository::DateIdeaTagRepository;
pub use tag_repository::TagRepository;
pub use user_date_repository::UserDateRepository;
pub use user_repository::UserRepository;

use crate::database::Connection;

pub struct Repositories {
    pub user_repository: Arc<UserRepository>,
    pub tag_repository: Arc<TagRepository>,
    pub date_idea_repository: Arc<DateIdeaRepository>,
    pub date_idea_tag_repository: Arc<DateIdeaTagRepository>,
    pub user_date_repository: Arc<UserDateRepository>,
    pub alphabet_repository: Arc<AlphabetRepository>,
}

impl Repositories {
    pub fn new(connection: Arc<Connection>) -> Self {
        Self {
            user_repository: Arc::new(UserRepository::new(connection.clone())),
            tag_repository: Arc::new(TagRepository::new(connection.clone())),
            date_idea_repository: Arc::new(DateIdeaRepository::new(connection.clone())),
            date_idea_tag_repository: Arc::new(DateIdeaTagRepository::new(connection.clone())),
            user_date_repository: Arc::new(UserDateRepository::new(connection.clone())),
            alphabet_repository: Arc::new(AlphabetRepository::new(connection.clone())),
        }
    }
}
