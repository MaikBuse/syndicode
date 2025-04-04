use argon2::{password_hash::SaltString, PasswordHash};
use jsonwebtoken::TokenData;
use uuid::Uuid;

use crate::{domain::user::model::role::UserRole, infrastructure::crypto::claims::Claims};

pub trait JwtHandler: Send + Sync {
    fn decode_jwt(&self, token: &str) -> anyhow::Result<TokenData<Claims>>;
    fn encode_jwt(&self, user_uuid: Uuid, user_role: UserRole) -> anyhow::Result<String>;
}

pub trait PasswordHandler: Send + Sync {
    fn hash_password<'a>(
        &self,
        password: String,
        salt: &'a SaltString,
    ) -> anyhow::Result<PasswordHash<'a>>;

    fn verfiy_password(
        &self,
        user_password_hash: &str,
        provided_password: String,
    ) -> anyhow::Result<()>;
}
