use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use super::EnvService;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Serialize, Deserialize)]
struct UserId {
    id: String,
}

pub struct TokenService;

impl TokenService {
    pub fn create_token(id: String) -> Result<String, String> {
        let user_id = UserId { id };

        let secret = EnvService::get_env("SECRET_KEY").map_err(|_| "Server Error".to_owned())?;
        let payload = serde_json::to_string(&user_id).map_err(|_| "Server Error".to_owned())?;

        let claims = Claims {
            sub: payload,
            //TODO: improve maximum expiration time
            exp: (chrono::offset::Local::now() + chrono::Duration::days(1)).timestamp() as usize,
        };
        let token = encode::<Claims>(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_str().as_ref()),
        )
        .map_err(|err| err.to_string())?;

        Ok(token)
    }

    pub fn validate_token(token: String) -> Result<(), String> {
        let secret = EnvService::get_env("SECRET_KEY").map_err(|_| "Server Error".to_owned())?;
        decode::<Claims>(
            &token,
            &DecodingKey::from_secret(secret.as_str().as_ref()),
            &Validation::default(),
        )
        .map_err(|err| err.to_string())?;

        Ok(())
    }
}
