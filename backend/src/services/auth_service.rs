use crate::domain::{User, UserLogin, UserRegister};

use super::{PasswordService, TokenService, UserService};

type Token = String;

pub struct AuthService;

impl AuthService {
    pub fn login(user_login: UserLogin) -> Result<Token, String> {
        // validate username and password
        let finded_user = match UserService::find_by_email(&user_login.email) {
            Some(user) => user,
            None => return Err("Cannot find user".to_owned()),
        };

        if !PasswordService::validate(&user_login.password, &finded_user.password().to_string()) {
            return Err("Incorrect Password".to_owned());
        }

        let token = TokenService::create_token(finded_user.id().to_string().as_str())?;

        Ok(token)
    }

    pub fn register(user_register: UserRegister) -> Result<User, String> {
        let e_password = PasswordService::encrypt(&user_register.password);
        let n_user = User::new(
            user_register.username,
            e_password,
            user_register.email,
            user_register.couplename,
            user_register.anniversary,
            user_register.photo,
        );

        UserService::create_user(&n_user)
    }
}
