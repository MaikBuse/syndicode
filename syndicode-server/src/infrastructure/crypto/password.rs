use crate::application::crypto::PasswordHandler;

use super::CryptoService;

use argon2::password_hash::rand_core::OsRng;
use argon2::{password_hash::SaltString, PasswordHash};
use argon2::{PasswordHasher, PasswordVerifier};

impl PasswordHandler for CryptoService {
    fn hash_password(&self, password: String) -> anyhow::Result<String> {
        let salt = SaltString::generate(&mut OsRng);

        match self.argon.hash_password(password.as_bytes(), &salt) {
            Ok(password_hash) => Ok(password_hash.to_string()),
            Err(err) => Err(anyhow::anyhow!(
                "Failed to hash password: {}",
                err.to_string()
            )),
        }
    }

    fn verfiy_password(
        &self,
        user_password_hash: &str,
        provided_password: String,
    ) -> anyhow::Result<()> {
        let parsed_hash = match PasswordHash::new(&user_password_hash) {
            Ok(password_hash) => password_hash,
            Err(err) => {
                return Err(anyhow::anyhow!(
                    "Failed to parse password hash: {}",
                    err.to_string()
                ))
            }
        };

        self.argon
            .verify_password(provided_password.as_bytes(), &parsed_hash)
            .map_err(|_| anyhow::anyhow!("The provided password failed to verify"))?;

        Ok(())
    }
}
