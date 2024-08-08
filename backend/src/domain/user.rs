use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
    pub email: String,
    pub couplename: String,
    pub anniversary: String,
    pub photo: String,
}

impl User {
    pub fn new(
        username: String,
        password: String,
        email: String,
        couplename: String,
        anniversary: String,
        photo: String,
    ) -> Self {
        Self {
            id: "".to_owned(),
            username,
            password,
            email,
            couplename,
            anniversary,
            photo,
        }
    }
}
