use std::sync::Arc;

use crate::domain::{EUser, GetAllUsersQuery, GetUserByEmailQuery, GetUserByIdQuery};
use crate::repository::{BaseTransactions, UserRepository};

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

impl ServiceHandlerTrait<GetAllUsersQuery, Vec<EUser>> for UserQueryService {
    async fn _handle(&self, _query: GetAllUsersQuery) -> Result<Vec<EUser>, String> {
        Ok(self.user_repository.get_all().await)
    }
}

impl ServiceHandlerTrait<GetUserByIdQuery, EUser> for UserQueryService {
    async fn _handle(&self, query: GetUserByIdQuery) -> Result<EUser, String> {
        let finded = self.user_repository.find_by_id(&query.id).await;

        match finded {
            Some(user) => Ok(user),
            None => Err("User not found".to_owned()),
        }
    }
}

impl ServiceHandlerTrait<GetUserByEmailQuery, EUser> for UserQueryService {
    async fn _handle(&self, query: GetUserByEmailQuery) -> Result<EUser, String> {
        let finded = self.user_repository.find_by_email(&query.email).await;

        match finded {
            Some(user) => Ok(user),
            None => Err("User not found".to_owned()),
        }
    }
}
