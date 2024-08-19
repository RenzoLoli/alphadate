use std::sync::Arc;

use crate::{
    domain::{User, UserUpdate},
    repository::UserRepository,
};

pub struct UserService {
    user_repository: Arc<UserRepository>,
}

impl UserService {
    pub fn new(user_repository: Arc<UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn get_all(&self) -> Vec<User> {
        self.user_repository.get_all().await
    }

    pub async fn find_by_id(&self, id: &str) -> Option<User> {
        self.user_repository.find_by_id(id).await
    }

    pub async fn find_by_email(&self, email: &str) -> Option<User> {
        self.user_repository.find_by_email(email).await
    }

    pub async fn create_user(&self, user: &User) -> Result<User, String> {
        if self.find_by_email(user.email()).await.is_some() {
            return Err("User already exist".to_owned());
        }

        self.user_repository
            .create(user)
            .await
            .ok_or("Cannot Create User".to_owned())
    }

    pub async fn update_user(&self, id: &str, options: &UserUpdate) -> Result<User, String> {
        let finded_user = self.user_repository.find_by_id(id).await;

        let Some(mut user) = finded_user else {
            return Err("User not exist".to_owned());
        };

        user.update(options);

        self.user_repository
            .update(id, &user)
            .await
            .ok_or("Cannot Update User".to_owned())
    }

    pub async fn delete_user(&self, id: &str) -> Result<User, String> {
        let finded_user = self.user_repository.find_by_id(id).await;

        if finded_user.is_none() {
            return Err("User not exist".to_owned());
        };

        self.user_repository
            .delete(id)
            .await
            .ok_or("Cannot Update User".to_owned())
    }
}
