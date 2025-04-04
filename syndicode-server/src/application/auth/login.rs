use crate::{
    application::error::{ApplicationError, ApplicationResult},
    domain::user::repository::UserRepository,
    infrastructure::crypto::CryptoService,
};
use std::sync::Arc;

pub struct LoginUseCase {
    crypto: Arc<CryptoService>,
    user_repo: Arc<dyn UserRepository>,
}

impl LoginUseCase {
    pub fn new(crypto: Arc<CryptoService>, user_repo: Arc<dyn UserRepository>) -> Self {
        Self { crypto, user_repo }
    }

    pub async fn execute(&self, user_name: String, password: String) -> ApplicationResult<String> {
        let Ok(user) = self.user_repo.get_user_by_name(user_name).await else {
            return Err(ApplicationError::WrongUserCredentials);
        };

        if self
            .crypto
            .verfiy_password(&user.password_hash, password)
            .is_err()
        {
            return Err(ApplicationError::WrongUserCredentials);
        }

        Ok(self.crypto.encode_jwt(user.uuid, user.role)?)
    }
}
