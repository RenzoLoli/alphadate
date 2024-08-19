use std::sync::Arc;

use crate::domain::{User, UserLogin, UserRegister};

use super::{PasswordService, TokenService, UserService};

type Token = String;

pub struct AuthService {
    user_service: Arc<UserService>,
}

impl AuthService {
    pub fn new(user_service: Arc<UserService>) -> Self {
        Self { user_service }
    }

    pub async fn login(&self, user_login: UserLogin) -> Result<Token, String> {
        let finded_user = self
            .user_service
            .find_by_email(&user_login.email)
            .await
            .ok_or("Cannot find user".to_owned())?;

        if !PasswordService::validate(&user_login.password, &finded_user.password().to_string()) {
            return Err("Incorrect Password".to_owned());
        }

        let token = TokenService::create_token(finded_user.id().to_string().as_str())?;

        Ok(token)
    }

    pub async fn register(&self, user_register: &UserRegister) -> Result<User, String> {
        let e_password = PasswordService::encrypt(&user_register.password);

        user_register.to_owned().password = e_password;
        let n_user = User::from(user_register);

        self.user_service.create_user(&n_user).await
    }
}
