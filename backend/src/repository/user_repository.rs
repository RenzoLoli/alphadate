use crate::{
    database::Connection,
    domain::{User, UserDto},
};

pub struct UserRepository {
    connection: Connection,
}

impl UserRepository {
    pub fn new(connection: Connection) -> Self {
        Self { connection }
    }

    pub async fn get_all(&self) -> Vec<User> {
        let db = self.connection.db();

        let users: Vec<UserDto> = db.select("users").await.ok().unwrap_or(vec![]);

        users.iter().map(User::from).collect()
    }

    pub async fn create(&self, user: &User) -> Option<User> {
        let db = self.connection.db();

        let dto = UserDto::from(user);

        db.create("users")
            .content(dto)
            .await
            .inspect_err(|err| log::error!("{}", err.to_string()))
            .map(|mut res| res.pop())
            .ok()?
            .map(|dto: UserDto| User::from(&dto))
    }

    pub async fn update(&self, id: &str, user: &User) -> Option<User> {
        let db = self.connection.db();

        let dto = UserDto::from(user);

        db.update(("users", id))
            .merge(dto)
            .await
            .inspect_err(|err| log::error!("{}", err.to_string()))
            .ok()?
            .map(|dto: UserDto| User::from(&dto))
    }

    pub async fn delete(&self, id: &str) -> Option<User> {
        let db = self.connection.db();

        db.delete(("users", id))
            .await
            .inspect_err(|err| log::error!("{}", err.to_string()))
            .ok()?
            .map(|dto: UserDto| User::from(&dto))
    }

    pub async fn find_by_id(&self, id: &str) -> Option<User> {
        let db = self.connection.db();

        db.select(("users", id))
            .await
            .inspect_err(|err| log::error!("{}", err.to_string()))
            .ok()?
            .map(|dto: UserDto| User::from(&dto))
    }

    pub async fn find_by_email(&self, email: &str) -> Option<User> {
        let db = self.connection.db();

        db.query("SELECT * FROM users WHERE email = $email")
            .bind(("email", email))
            .await
            .inspect_err(|err| log::error!("{}", err))
            .ok()?
            .take(0)
            .inspect_err(|err| log::error!("{}", err))
            .map(|mut res: Vec<UserDto>| res.pop())
            .ok()?
            .map(|dto: UserDto| User::from(&dto))
    }
}
