use super::error::{ServiceError, ServiceResult};
use crate::domain::{
    model::control::{Claims, UserModel, UserRole},
    repository::{control::ControlDatabaseRepository, economy::EconomyDatabaseRepository},
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
    control_db: Arc<dyn ControlDatabaseRepository>,
    economy_db: Arc<dyn EconomyDatabaseRepository>,
    argon: Argon2<'static>,
    jwt_secret: String,
}

impl ControlService {
    pub fn new(
        control_db: Arc<dyn ControlDatabaseRepository>,
        economy_db: Arc<dyn EconomyDatabaseRepository>,
        jwt_secret: String,
    ) -> Self {
        Self {
            control_db,
            economy_db,
            jwt_secret,
            argon: Argon2::default(),
        }
    }

    pub async fn login(&self, username: String, password: String) -> ServiceResult<String> {
        let Ok(user) = self.control_db.get_user_by_name(username).await else {
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
        username: String,
        password: String,
        user_role: UserRole,
    ) -> ServiceResult<UserModel> {
        let salt = SaltString::generate(&mut OsRng);

        let password_hash = match self.argon.hash_password(password.as_bytes(), &salt) {
            Ok(pw) => pw,
            Err(err) => {
                return Err(anyhow::anyhow!("Failed to hash password: {}", err.to_string()).into())
            }
        };

        let user = UserModel {
            uuid: Uuid::now_v7(),
            name: username,
            password_hash: password_hash.to_string(),
            role: user_role,
        };

        Ok(self.control_db.create_user(user).await?)
    }

    pub async fn delete_user(&self, user_uuid: Uuid) -> ServiceResult<()> {
        Ok(self.control_db.delete_user(user_uuid).await?)
    }

    pub async fn get_user(&self, user_uuid: Uuid) -> ServiceResult<UserModel> {
        Ok(self.control_db.get_user(user_uuid).await?)
    }
}
