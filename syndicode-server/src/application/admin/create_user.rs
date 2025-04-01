use crate::{
    application::error::{ApplicationError, ApplicationResult},
    domain::{
        corporation::Corporation,
        user::{role::UserRole, User},
    },
    infrastructure::{crypto::CryptoService, postgres::PostgresDatabase},
};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use std::sync::Arc;
use uuid::Uuid;

const DEFAULT_BALANCE: i64 = 1000000;

pub struct CreateUserUseCase {
    db: Arc<PostgresDatabase>,
    crypto: Arc<CryptoService>,
}

impl CreateUserUseCase {
    pub fn new(db: Arc<PostgresDatabase>, crypto: Arc<CryptoService>) -> Self {
        Self { db, crypto }
    }

    pub async fn execute(
        &self,
        maybe_req_user_uuid: Option<Uuid>,
        user_name: String,
        password: String,
        user_role: UserRole,
        corporation_name: String,
    ) -> ApplicationResult<User> {
        if user_name.is_empty() {
            return Err(ApplicationError::UsernameInvalid);
        }

        if password.len() < 8 {
            return Err(ApplicationError::PasswordTooShort);
        }

        if user_role == UserRole::Admin {
            let Some(req_user_uuid) = maybe_req_user_uuid else {
                return Err(ApplicationError::MissingAuthentication);
            };

            let req_user = PostgresDatabase::get_user(&self.db.pool, req_user_uuid).await?;

            if req_user.role != UserRole::Admin {
                return Err(ApplicationError::Unauthorized);
            }
        }

        let salt = SaltString::generate(&mut OsRng);

        let password_hash = self.crypto.hash_password(password, &salt)?;

        // Start database transaction
        let mut tx = self.db.pool.begin().await?;

        let user = User {
            uuid: Uuid::now_v7(),
            name: user_name,
            password_hash: password_hash.to_string(),
            role: user_role,
        };

        let user = PostgresDatabase::create_user(&mut *tx, user).await?;

        let corporation = Corporation {
            uuid: Uuid::now_v7(),
            user_uuid: user.uuid,
            name: corporation_name,
            balance: DEFAULT_BALANCE,
        };

        PostgresDatabase::create_corporation(&mut *tx, corporation).await?;

        tx.commit().await?;

        Ok(user)
    }
}
