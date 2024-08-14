use crate::domain::{User, UserUpdate};
use uuid::Uuid;

pub struct UserService;

static mut USERS: Vec<User> = vec![];

impl UserService {
    pub fn get_all() -> Vec<User> {
        unsafe { USERS.clone() }
    }

    pub fn find_by_id(id: &str) -> Option<User> {
        unsafe {
            USERS
                .clone()
                .into_iter()
                .find(|user: &'_ User| user.id() == id)
        }
    }

    pub fn find_by_email(email: &str) -> Option<User> {
        unsafe {
            USERS
                .clone()
                .into_iter()
                .find(|user: &'_ User| user.email() == email)
        }
    }

    pub fn create_user(user: &User) -> Result<User, String> {
        let mut n_user = user.clone();

        if UserService::find_by_email(user.email()).is_some() {
            return Err("User already exist".to_owned());
        }

        n_user.set_id(Uuid::new_v4().to_string().as_str());

        unsafe { USERS.push(n_user.clone()) }
        Ok(n_user)
    }

    pub fn update_user(id: &str, options: &UserUpdate) -> Result<User, String> {
        let finded_user = unsafe { USERS.iter_mut().find(|user: &'_ &mut User| user.id() == id) };

        let Some(user) = finded_user else {
            return Err("User not exist".to_owned());
        };

        user.update(options);

        Ok(user.clone())
    }

    pub fn delete_user(id: &str) -> Result<User, String> {
        unsafe {
            USERS
                .iter()
                .position(|user: &User| user.id() == id)
                .map(|pos| USERS.remove(pos))
                .ok_or("user not Exists".to_owned())
        }
    }
}
