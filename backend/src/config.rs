use serde::{Deserialize, Serialize};

use crate::database::ConfigConnection;

use super::services::EnvService;

#[derive(Serialize, Deserialize, Clone)]
pub struct ServerOptions {
    pub port: u16,
    pub host: String,
    pub secret: String,
    pub password_encryption_key: String,
    pub expiration_token_time: usize,
    pub origins: Vec<String>,
    pub config_connection: ConfigConnection,
}

impl ServerOptions {
    pub fn load() -> ServerOptions {
        log::info!("Loading server options");

        let port = EnvService::get_env("BACKEND_PORT")
            .unwrap_or(String::from("3000"))
            .parse::<u16>()
            .unwrap();
        log::debug!("port: {}", port);

        let host = EnvService::get_env("BACKEND_HOST").unwrap_or(String::from("0.0.0.0"));
        log::debug!("host: {}", host);

        let secret = EnvService::get_env("BACKEND_SECRET_KEY").unwrap_or(String::from("secret"));
        log::debug!("secret: {}", secret);

        let password_encryption_key =
            EnvService::get_env("BACKEND_PASSWORD_ENCRYPTION_KEY").unwrap_or(String::from(""));
        log::debug!("password_encryption_key: {}", password_encryption_key);

        let expiration_token_time = EnvService::get_env("BACKEND_EXPIRATION_TOKEN_TIME_IN_MS")
            .unwrap_or(String::from("86400000"))
            .parse::<usize>()
            .unwrap();
        log::debug!("expiration_token_time: {}", expiration_token_time);

        let origins: Vec<String> = EnvService::get_env("BACKEND_CORS_ORIGINS")
            .map(|origins| origins.split(',').map(|s| s.to_string()).collect())
            .unwrap_or_default();
        log::debug!("origins: {:?}", origins);

        // config database variables
        let username = EnvService::get_env("BACKEND_DB_USER").unwrap_or(String::from("root"));
        let password = EnvService::get_env("BACKEND_DB_PASS").unwrap_or(String::from("root"));
        let address = EnvService::get_env("BACKEND_DB_HOST").unwrap_or(String::from("127.0.0.1"));
        let namespace =
            EnvService::get_env("BACKEND_DB_NAMESPACE").unwrap_or(String::from("alphadate"));
        let database =
            EnvService::get_env("BACKEND_DB_DATABASE").unwrap_or(String::from("resources"));

        let config_connection = ConfigConnection {
            username,
            password,
            address,
            namespace,
            database,
        };

        log::debug!("config_connection: {:#?}", config_connection);

        ServerOptions {
            port,
            host,
            password_encryption_key,
            expiration_token_time,
            secret,
            origins,
            config_connection,
        }
    }
}
