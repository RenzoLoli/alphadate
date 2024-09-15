use serde::{Deserialize, Serialize};

use crate::domain::model::value_objects::IdObject;
use crate::domain::{SignUpCommand, UserUpdateCommand};

use super::Entity;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct EUser {
    pub id: IdObject,
    pub username: String,
    pub password: String,
    pub email: String,
    pub couplename: String,
    pub anniversary: String,
    pub photo: String,
}

impl Entity for EUser {
    fn get_table_name() -> &'static str {
        "users"
    }

    fn get_id(&self) -> &IdObject {
        &self.id
    }

    fn get_mut_id(&mut self) -> &mut IdObject {
        &mut self.id
    }
}

impl From<SignUpCommand> for EUser {
    fn from(value: SignUpCommand) -> Self {
        Self {
            id: IdObject::default(),
            username: value.username,
            password: value.password,
            email: value.email,
            couplename: value.couplename,
            anniversary: value.anniversary,
            photo: value.photo,
        }
    }
}

impl EUser {
    pub fn update(&mut self, command: UserUpdateCommand) {
        // TODO: check if parameters are equal to the current values (unnecessary update)
        if let Some(username) = command.username {
            self.username = username;
        }
        if let Some(couplename) = command.couplename {
            self.couplename = couplename;
        }
        if let Some(anniversary) = command.anniversary {
            self.anniversary = anniversary;
        }
        if let Some(photo) = command.photo {
            self.photo = photo;
        }
    }
}
