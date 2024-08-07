use crate::domain::{Token, User, UserAuth, UserLogin, UserRegister};

use super::{PasswordService, TokenService};

pub struct AuthService;

static mut USERS: Vec<User> = vec![];

impl AuthService {
    pub fn login(user_login: UserLogin) -> Result<Token, String> {
        // validate username and password
        let finded_user = unsafe {
            USERS
                .clone()
                .into_iter()
                .find(|user: &'_ User| user.email == user_login.email)
        };

        if finded_user.is_none() {
            return Err("Cannot find user".to_owned());
        }

        let finded_user = finded_user.unwrap();

        if !PasswordService::validate(&user_login.password, &finded_user.password) {
            return Err("Incorrect Password".to_owned());
        }

        let user_auth = UserAuth::new(user_login.email, user_login.password);
        let token = TokenService::create_token(user_auth)?;

        Ok(Token::new(token))
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

        unsafe { USERS.push(n_user.clone()) };

        Ok(n_user)
    }

    pub fn all() -> Vec<User> {
        unsafe { USERS.clone() }
    }
}
