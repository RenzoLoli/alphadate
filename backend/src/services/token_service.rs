use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use super::EnvService;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

type Token = String;

pub struct TokenService;

impl TokenService {
    pub fn create_token(id: &str) -> Result<Token, String> {
        let secret = EnvService::get_env("SECRET_KEY").map_err(|_| "Server Error".to_owned())?;
        let claims = Claims {
            sub: id.to_string(),
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

    pub fn validate_token(token: Token) -> Result<(), String> {
        let secret = EnvService::get_env("SECRET_KEY").map_err(|_| "Server Error".to_owned())?;
        decode::<Claims>(
            &token,
            &DecodingKey::from_secret(secret.as_str().as_ref()),
            &Validation::default(),
        )
        .map_err(|err| err.to_string())?;

        Ok(())
    }

    pub fn renew_token(token: Token) -> Result<Token, String> {
        let token = EnvService::get_env("SECRET_KEY")
            .map_err(|_| "Server Error".to_owned())
            .and_then(|secret| {
                decode::<Claims>(
                    &token,
                    &DecodingKey::from_secret(secret.as_str().as_ref()),
                    &Validation::default(),
                )
                .and_then(|data| {
                    let claims = Claims {
                        sub: data.claims.sub,
                        exp: (chrono::offset::Local::now() + chrono::Duration::days(1)).timestamp()
                            as usize,
                    };
                    encode::<Claims>(
                        &Header::default(),
                        &claims,
                        &EncodingKey::from_secret(secret.as_str().as_ref()),
                    )
                })
                .map_err(|_| "Invalid Token".to_owned())
            })?;

        Ok(token)
    }
}
