use serde::{Deserialize, Serialize};

use super::UserUpdate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    id: String,
    username: String,
    password: String,
    email: String,
    couplename: String,
    anniversary: String,
    photo: String,
}

impl User {
    pub fn new(
        username: &str,
        password: &str,
        email: &str,
        couplename: &str,
        anniversary: &str,
        photo: &str,
    ) -> Self {
        Self {
            id: "".to_owned(),
            username: username.to_string(),
            password: password.to_string(),
            email: email.to_string(),
            couplename: couplename.to_string(),
            anniversary: anniversary.to_string(),
            photo: photo.to_string(),
        }
    }

    // GETTERS
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn password(&self) -> &str {
        &self.password
    }
    pub fn email(&self) -> &str {
        &self.email
    }
    pub fn couplename(&self) -> &str {
        &self.couplename
    }
    pub fn anniversary(&self) -> &str {
        &self.anniversary
    }
    pub fn photo(&self) -> &str {
        &self.photo
    }

    //SETTERS
    pub fn set_id(&mut self, id: &str) {
        self.id = id.to_string();
    }

    pub fn set_username(&mut self, username: &str) {
        self.username = String::from(username);
    }

    pub fn set_password(&mut self, password: &str) {
        self.password = String::from(password);
    }

    pub fn set_email(&mut self, email: &str) {
        self.email = String::from(email);
    }

    pub fn set_couplename(&mut self, couplename: &str) {
        self.couplename = String::from(couplename);
    }

    pub fn set_anniversary(&mut self, anniversary: &str) {
        self.anniversary = String::from(anniversary);
    }

    pub fn set_photo(&mut self, photo: &str) {
        self.photo = String::from(photo);
    }

    // DOMAIN LOGIC

    pub fn update(&mut self, options: &UserUpdate) {
        if let Some(username) = options.username.clone() {
            self.username = username;
        }

        if let Some(couplename) = options.couplename.clone() {
            self.couplename = couplename;
        }

        if let Some(anniversary) = options.anniversary.clone() {
            self.anniversary = anniversary;
        }

        if let Some(photo) = options.photo.clone() {
            self.photo = photo;
        }
    }
}
