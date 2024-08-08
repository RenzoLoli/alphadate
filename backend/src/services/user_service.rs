use crate::domain::User;
use uuid::Uuid;

pub struct UserService;

static mut USERS: Vec<User> = vec![];

impl UserService {
    pub fn get_all() -> Vec<User> {
        unsafe { USERS.clone() }
    }

    pub fn find_by_id(id: &String) -> Option<User> {
        unsafe {
            USERS
                .clone()
                .into_iter()
                .find(|user: &'_ User| &user.id == id)
        }
    }

    pub fn find_by_email(email: &String) -> Option<User> {
        unsafe {
            USERS
                .clone()
                .into_iter()
                .find(|user: &'_ User| &user.email == email)
        }
    }

    pub fn create_user(user: &User) -> Result<User, String> {
        let mut n_user = user.clone();

        if UserService::find_by_email(&user.email).is_some() {
            return Err("User already exist".to_owned());
        }

        n_user.id = Uuid::new_v4().to_string();

        unsafe { USERS.push(n_user.clone()) }
        Ok(n_user)
    }
}
