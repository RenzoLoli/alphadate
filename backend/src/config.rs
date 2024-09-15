use serde::{Deserialize, Serialize};

use super::services::EnvService;

#[derive(Serialize, Deserialize, Clone)]
pub struct ServerOptions {
    pub port: u16,
    pub host: String,
    pub origins: Vec<String>,
}

impl ServerOptions {
    pub fn load() -> ServerOptions {
        ServerOptions {
            port: EnvService::get_env("PORT")
                .unwrap_or(String::from("3000"))
                .parse::<u16>()
                .unwrap(),
            host: EnvService::get_env("HOST").unwrap_or(String::from("0.0.0.0")),
            origins: EnvService::get_env("CORS_ORIGINS")
                .map(|origins| origins.split(',').map(|s| s.to_string()).collect())
                .unwrap_or(vec![]),
        }
    }
}
