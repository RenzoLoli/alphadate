use std::sync::Arc;

use crate::domain::{GetAllUsersQuery, GetUserByEmailQuery, GetUserByIdQuery, UserAggregate};
use crate::repository::{BaseRepository, UserRepository};

use super::ServiceHandlerTrait;

#[derive(Default)]
pub struct UserQueryService {
    user_repository: Arc<UserRepository>,
}

impl UserQueryService {
    pub fn new(user_repository: Arc<UserRepository>) -> Self {
        Self { user_repository }
    }
}

impl ServiceHandlerTrait<GetAllUsersQuery, Vec<UserAggregate>> for UserQueryService {
    async fn handle(&self, _query: GetAllUsersQuery) -> Result<Vec<UserAggregate>, String> {
        let users = self.user_repository.get_all().await;
        Ok(users.into_iter().map(UserAggregate::from).collect())
    }
}

impl ServiceHandlerTrait<GetUserByIdQuery, UserAggregate> for UserQueryService {
    async fn handle(&self, query: GetUserByIdQuery) -> Result<UserAggregate, String> {
        let finded = self.user_repository.find_by_id(&query.id).await;

        match finded {
            Some(user) => Ok(UserAggregate::from(user)),
            None => Err("User not found".to_owned()),
        }
    }
}

impl ServiceHandlerTrait<GetUserByEmailQuery, UserAggregate> for UserQueryService {
    async fn handle(&self, query: GetUserByEmailQuery) -> Result<UserAggregate, String> {
        let finded = self.user_repository.find_by_email(&query.email).await;

        match finded {
            Some(user) => Ok(UserAggregate::from(user)),
            None => Err("User not found".to_owned()),
        }
    }
}
