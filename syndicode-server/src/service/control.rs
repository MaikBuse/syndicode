use super::error::{ServiceError, ServiceResult};
use crate::{
    domain::model::{
        control::{Claims, UserModel, UserRole},
        economy::CorporationModel,
    },
    infrastructure::postgres::{control, economy, PostgresDatabase},
};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    PasswordHash, PasswordVerifier,
};
use argon2::{Argon2, PasswordHasher};
use jsonwebtoken::{encode, EncodingKey, Header};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;

const DEFAULT_BALANCE: i64 = 1000000;

#[derive(Debug)]
pub struct ControlService {
    postgres_db: Arc<PostgresDatabase>,
    argon: Argon2<'static>,
    jwt_secret: String,
}

impl ControlService {
    pub fn new(postgres_db: Arc<PostgresDatabase>, jwt_secret: String) -> Self {
        Self {
            postgres_db,
            jwt_secret,
            argon: Argon2::default(),
        }
    }

    pub async fn login(&self, user_name: String, password: String) -> ServiceResult<String> {
        let Ok(user) = control::get_user_by_name(&self.postgres_db.pool, user_name).await else {
            return Err(ServiceError::WrongUserCredentials);
        };

        let parsed_hash = match PasswordHash::new(&user.password_hash) {
            Ok(password_hash) => password_hash,
            Err(err) => {
                return Err(
                    anyhow::anyhow!("Failed to parse password hash: {}", err.to_string()).into(),
                )
            }
        };

        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|_| ServiceError::WrongUserCredentials)?;

        let expiration = SystemTime::now()
            .checked_add(Duration::from_secs(86400))
            .unwrap()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        let claims = Claims {
            sub: user.uuid.to_string(),
            exp: expiration,
            role: user.role.to_string(),
        };

        let jwt = match encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        ) {
            Ok(jwt) => jwt,
            Err(err) => {
                return Err(anyhow::anyhow!("{}", err).into());
            }
        };

        Ok(jwt)
    }

    pub async fn create_user(
        &self,
        maybe_req_user_uuid: Option<Uuid>,
        user_name: String,
        password: String,
        user_role: UserRole,
        corporation_name: String,
    ) -> ServiceResult<UserModel> {
        if user_name.is_empty() {
            return Err(ServiceError::UsernameInvalid);
        }

        if password.len() < 8 {
            return Err(ServiceError::PasswordTooShort);
        }

        if user_role == UserRole::Admin {
            let Some(req_user_uuid) = maybe_req_user_uuid else {
                return Err(ServiceError::MissingAuthentication);
            };

            let req_user = control::get_user(&self.postgres_db.pool, req_user_uuid).await?;

            if req_user.role != UserRole::Admin {
                return Err(ServiceError::Unauthorized);
            }
        }

        let salt = SaltString::generate(&mut OsRng);

        let password_hash = match self.argon.hash_password(password.as_bytes(), &salt) {
            Ok(pw) => pw,
            Err(err) => {
                return Err(anyhow::anyhow!("Failed to hash password: {}", err.to_string()).into())
            }
        };

        // Start database transaction
        let mut tx = self.postgres_db.pool.begin().await?;

        let user = UserModel {
            uuid: Uuid::now_v7(),
            name: user_name,
            password_hash: password_hash.to_string(),
            role: user_role,
        };

        let user = control::create_user(&mut *tx, user).await?;

        let corporation = CorporationModel {
            uuid: Uuid::now_v7(),
            user_uuid: user.uuid,
            name: corporation_name,
            balance: DEFAULT_BALANCE,
        };

        economy::create_corporation(&mut *tx, corporation).await?;

        tx.commit().await?;

        Ok(user)
    }

    pub async fn delete_user(&self, req_user_uuid: Uuid, user_uuid: Uuid) -> ServiceResult<()> {
        if req_user_uuid != user_uuid {
            let req_user = control::get_user(&self.postgres_db.pool, req_user_uuid).await?;

            if req_user.role != UserRole::Admin {
                return Err(ServiceError::Unauthorized);
            }
        }

        // The corporation automatically gets deleted with the user
        control::delete_user(&self.postgres_db.pool, user_uuid).await?;

        Ok(())
    }

    pub async fn get_user(&self, req_user_uuid: Uuid, user_uuid: Uuid) -> ServiceResult<UserModel> {
        if req_user_uuid != user_uuid {
            let req_user = control::get_user(&self.postgres_db.pool, req_user_uuid).await?;

            if req_user.role != UserRole::Admin {
                return Err(ServiceError::Unauthorized);
            }
        }

        Ok(control::get_user(&self.postgres_db.pool, user_uuid).await?)
    }
}
