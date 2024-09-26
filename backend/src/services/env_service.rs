use dotenv::dotenv;
use std::env::var as _get_env;

pub struct EnvService;

impl EnvService {
    pub fn load() {
        log::info!("Loading dotenv variables");
        dotenv().ok();
    }

    pub fn get_env(var: &str) -> Result<String, String> {
        _get_env(var)
            .map_err(|e| e.to_string())
            .inspect(|res| log::debug!("env var {} = {}", var, res))
            .inspect_err(|e| log::debug!("failed to get env var <{}>: {}", var, e))
    }
}
