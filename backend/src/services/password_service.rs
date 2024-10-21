use core::panic;

use base64::prelude::*;
use chacha20poly1305::aead::generic_array::typenum::Unsigned;
use chacha20poly1305::{
    aead::{generic_array::GenericArray, Aead, OsRng},
    AeadCore, ChaCha20Poly1305, KeyInit,
};

#[derive(Default)]
pub struct PasswordService {
    secret: Vec<u8>,
}

impl PasswordService {
    pub fn new(secret: &str) -> Self {
        let secret = BASE64_STANDARD.decode(secret).expect("Invalid secret");
        if secret.is_empty() {
            panic!("Password encryption key is required");
        }
        Self { secret }
    }
}

impl PasswordService {
    pub fn validate(&self, password: &str, saved: &str) -> bool {
        if password.is_empty() || saved.is_empty() {
            return false;
        }

        let decrypted = match self.decrypt(saved) {
            Ok(decrypted) => decrypted,
            Err(err) => {
                log::debug!("decrypt error: {}", err);
                return false;
            }
        };

        decrypted == password
    }

    pub fn encrypt(&self, password: &str) -> Result<String, String> {
        let generic_array = GenericArray::from_slice(&self.secret);
        let cipher = ChaCha20Poly1305::new(generic_array);
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
        let mut obsf = cipher
            .encrypt(&nonce, password.as_bytes())
            .map_err(|err| err.to_string())?;
        obsf.splice(..0, nonce.iter().copied());
        Ok(BASE64_STANDARD.encode(obsf.clone()))
    }

    fn decrypt(&self, password: &str) -> Result<String, String> {
        type NonceSize = <ChaCha20Poly1305 as AeadCore>::NonceSize;
        let generic_array = GenericArray::from_slice(&self.secret);
        let cipher = ChaCha20Poly1305::new(generic_array);
        let password = BASE64_STANDARD
            .decode(password)
            .map_err(|err| err.to_string())?;
        let (nonce, ciphertext) = password.split_at(NonceSize::to_usize());
        let nonce = GenericArray::from_slice(nonce);
        let text = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|err| err.to_string())?;

        String::from_utf8(text).map_err(|err| err.to_string())
    }
}
