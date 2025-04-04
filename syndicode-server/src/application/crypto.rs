#[cfg(test)]
use mockall::{automock, predicate::*};

use jsonwebtoken::TokenData;
use uuid::Uuid;

use crate::{domain::user::model::role::UserRole, infrastructure::crypto::claims::Claims};

#[cfg_attr(test, automock)]
pub trait JwtHandler: Send + Sync {
    fn decode_jwt(&self, token: &str) -> anyhow::Result<TokenData<Claims>>;
    fn encode_jwt(&self, user_uuid: Uuid, user_role: UserRole) -> anyhow::Result<String>;
}

#[cfg_attr(test, automock)]
pub trait PasswordHandler: Send + Sync {
    fn hash_password(&self, password: String) -> anyhow::Result<String>;

    fn verfiy_password(
        &self,
        user_password_hash: &str,
        provided_password: String,
    ) -> anyhow::Result<()>;
}
