use serde::{Deserialize, Serialize};

use super::User;

#[derive(Clone, Serialize, Deserialize)]
pub struct UserRegister {
    pub username: String,
    pub password: String,
    pub email: String,
    pub couplename: String,
    pub anniversary: String,
    pub photo: String,
}

impl From<&UserRegister> for User {
    fn from(register: &UserRegister) -> Self {
        User::new(
            &register.username,
            &register.password,
            &register.email,
            &register.couplename,
            &register.anniversary,
            &register.photo,
        )
    }
}
