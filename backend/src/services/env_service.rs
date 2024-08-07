use dotenv::dotenv;
use std::env::var as _get_env;

pub struct EnvService;

impl EnvService {
    pub fn load() {
        dotenv().ok();
    }

    pub fn get_env(var: &str) -> Result<String, String> {
        _get_env(var).map_err(|_| format!("Cannot get ${var}"))
    }
}
