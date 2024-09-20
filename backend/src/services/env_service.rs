use dotenv::dotenv;
use std::env::var as _get_env;

pub struct EnvService;

impl EnvService {
    pub fn load() {
        dotenv().ok();
    }

    pub fn get_env(var: &str) -> Result<String, String> {
        _get_env(var)
            .inspect(|val| log::info!("Getting env ${var}: {val}"))
            .inspect_err(|err| log::error!("{}", err.to_string()))
            .map_err(|_| format!("Cannot get ${var}"))
    }
}
