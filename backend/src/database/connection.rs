use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct ConfigConnection {
    pub username: String,
    pub password: String,
    pub address: String,
    pub namespace: String,
    pub database: String,
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
}

impl Connection {
    pub async fn connect(&self, config_connection: &ConfigConnection) -> Result<Self, String> {
        self.db
            .connect::<Ws>(config_connection.address.as_str())
            .await
            .map_err(|err| err.to_string())?;

        let credentials = Root {
            username: config_connection.username.as_str(),
            password: config_connection.password.as_str(),
        };

        self.db
            .signin(credentials)
            .await
            .map_err(|err| err.to_string())?;

        self.db
            .use_ns(config_connection.namespace.as_str())
            .use_db(config_connection.database.as_str())
            .await
            .map_err(|err| err.to_string())?;

        Ok(self.to_owned())
    }

    pub fn db(&self) -> &Surreal<Client> {
        &self.db
    }
}
