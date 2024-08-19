use serde::{Deserialize, Serialize};

use super::{IdObject, User};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDto {
    pub id: Option<IdObject>,
    pub username: String,
    pub password: String,
    pub email: String,
    pub couplename: String,
    pub anniversary: String,
    pub photo: String,
}

impl From<&User> for UserDto {
    fn from(user: &User) -> Self {
        UserDto {
            id: None,
            username: user.username().to_string(),
            password: user.password().to_string(),
            email: user.email().to_string(),
            couplename: user.couplename().to_string(),
            anniversary: user.anniversary().to_string(),
            photo: user.photo().to_string(),
        }
    }
}

impl From<&UserDto> for User {
    fn from(dto: &UserDto) -> Self {
        let mut user = User::new(
            &dto.username,
            &dto.password,
            &dto.email,
            &dto.couplename,
            &dto.anniversary,
            &dto.photo,
        );

        user.set_id(&dto.id.clone().unwrap_or_default().id());

        user
    }
}
