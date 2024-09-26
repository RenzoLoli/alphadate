use serde::{Deserialize, Serialize};

use crate::database::ConfigConnection;

use super::services::EnvService;

#[derive(Serialize, Deserialize, Clone)]
pub struct ServerOptions {
    pub port: u16,
    pub host: String,
    pub secret: String,
    pub log_level: String,
    pub password_encryption_key: String,
    pub origins: Vec<String>,
    pub config_connection: ConfigConnection,
}

impl ServerOptions {
    pub fn load() -> ServerOptions {
        log::info!("Loading server options");

        let port = EnvService::get_env("PORT")
            .unwrap_or(String::from("3000"))
            .parse::<u16>()
            .unwrap();
        log::debug!("port: {}", port);

        let host = EnvService::get_env("HOST").unwrap_or(String::from("0.0.0.0"));
        log::debug!("host: {}", host);

        let secret = EnvService::get_env("SECRET_KEY").unwrap_or(String::from("secret"));
        log::debug!("secret: {}", secret);

        let password_encryption_key =
            EnvService::get_env("PASSWORD_ENCRYPTION_KEY").unwrap_or(String::from(""));
        log::debug!("password_encryption_key: {}", password_encryption_key);

        let origins: Vec<String> = EnvService::get_env("CORS_ORIGINS")
            .map(|origins| origins.split(',').map(|s| s.to_string()).collect())
            .unwrap_or_default();
        log::debug!("origins: {:?}", origins);

        let log_level = EnvService::get_env("LOG_LEVEL").unwrap_or(String::from("info"));
        log::debug!("log_level: {}", log_level);

        // config database variables
        let username = EnvService::get_env("DB_USER").unwrap_or(String::from("root"));
        let password = EnvService::get_env("DB_PASS").unwrap_or(String::from("root"));
        let address = EnvService::get_env("DB_HOST").unwrap_or(String::from("127.0.0.1"));
        let namespace = EnvService::get_env("DB_NAMESPACE").unwrap_or(String::from("alphadate"));
        let database = EnvService::get_env("DB_DATABASE").unwrap_or(String::from("resources"));

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
            secret,
            origins,
            config_connection,
            log_level,
        }
    }
}
