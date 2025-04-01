use crate::{
    application::error::{ApplicationError, ApplicationResult},
    infrastructure::{crypto::CryptoService, postgres::PostgresDatabase},
};
use std::sync::Arc;

pub struct LoginUseCase {
    db: Arc<PostgresDatabase>,
    crypto: Arc<CryptoService>,
}

impl LoginUseCase {
    pub fn new(db: Arc<PostgresDatabase>, crypto: Arc<CryptoService>) -> Self {
        Self { db, crypto }
    }

    pub async fn execute(&self, user_name: String, password: String) -> ApplicationResult<String> {
        let Ok(user) = PostgresDatabase::get_user_by_name(&self.db.pool, user_name).await else {
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
