use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

type Token = String;

#[derive(Default)]
pub struct TokenService {
    secret_key: String,
}

impl TokenService {
    pub fn new(secret_key: String) -> Self {
        Self { secret_key }
    }
}

impl TokenService {
    pub fn create_token(&self, id: &str) -> Result<Token, String> {
        log::debug!("Creating token for user <{}>", id);

        let now = Utc::now();
        let days = chrono::Duration::days(1);
        let expiration = now + days;

        let claims = Claims {
            sub: id.to_string(),
            //TODO: improve maximum expiration time
            exp: expiration.timestamp_millis() as usize,
        };

        let token = encode::<Claims>(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret_key.as_bytes()),
        )
        .map_err(|err| {
            log::debug!("Invalid Token -> {}", err.to_string());
            format!("Invalid Token ( {} )", err)
        })?;

        Ok(token)
    }

    pub fn validate_token(&self, token: Token) -> Result<(), String> {
        log::debug!("Validating token <{}>", token);

        decode::<Claims>(
            &token,
            &DecodingKey::from_secret(self.secret_key.as_bytes()),
            &Validation::default(),
        )
        .map_err(|err| {
            log::debug!("Invalid Token -> {}", err.to_string());
            format!("Invalid Token ( {} )", err)
        })?;

        Ok(())
    }

    pub fn renew_token(&self, token: Token) -> Result<Token, String> {
        log::debug!("Renewing token <{}>", token);

        let token = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(self.secret_key.as_bytes()),
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
                &EncodingKey::from_secret(self.secret_key.as_bytes()),
            )
        })
        .map_err(|err| {
            log::debug!("Invalid Token -> {}", err.to_string());
            format!("Invalid Token ( {} )", err)
        })?;

        Ok(token)
    }
}
