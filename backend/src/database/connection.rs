use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

pub struct ConfigConnection<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub address: &'a str,
    pub namespace: &'a str,
    pub database: &'a str,
}

#[derive(Clone)]
pub struct Connection {
    db: Surreal<Client>,
}

impl Default for Connection {
    fn default() -> Self {
        Connection::new()
    }
}

impl Connection {
    pub fn new() -> Self {
        Self {
            db: Surreal::init(),
        }
    }

    pub async fn connect(&self, config_connection: &ConfigConnection<'_>) -> Result<Self, String> {
        self.db
            .connect::<Ws>(config_connection.address)
            .await
            .map_err(|err| err.to_string())?;

        let credentials = Root {
            username: config_connection.username,
            password: config_connection.password,
        };

        self.db
            .signin(credentials)
            .await
            .map_err(|err| err.to_string())?;

        self.db
            .use_ns(config_connection.namespace)
            .use_db(config_connection.database)
            .await
            .map_err(|err| err.to_string())?;

        Ok(self.to_owned())
    }

    pub fn db(&self) -> &Surreal<Client> {
        &self.db
    }
}
