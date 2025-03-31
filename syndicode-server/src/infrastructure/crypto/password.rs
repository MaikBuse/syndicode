use super::CryptoService;

use argon2::{password_hash::SaltString, PasswordHash};
use argon2::{PasswordHasher, PasswordVerifier};

impl CryptoService {
    pub fn hash_password<'a>(
        &self,
        password: String,
        salt: &'a SaltString,
    ) -> anyhow::Result<PasswordHash<'a>> {
        match self.argon.hash_password(password.as_bytes(), salt) {
            Ok(password_hash) => Ok(password_hash),
            Err(err) => Err(anyhow::anyhow!(
                "Failed to hash password: {}",
                err.to_string()
            )),
        }
    }

    pub fn verfiy_password(
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
