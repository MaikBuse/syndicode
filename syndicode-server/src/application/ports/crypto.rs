#[cfg(test)]
use mockall::{automock, predicate::*};

use jsonwebtoken::TokenData;

use crate::{
    domain::user::model::{password::UserPassword, User},
    infrastructure::crypto::claims::Claims,
};

#[cfg_attr(test, automock)]
pub trait JwtHandler: Send + Sync {
    fn decode_jwt(&self, token: &str) -> anyhow::Result<TokenData<Claims>>;
    fn encode_jwt(&self, user: &User) -> anyhow::Result<String>;
}

#[cfg_attr(test, automock)]
pub trait PasswordHandler: Send + Sync {
    fn hash_user_password(&self, password: UserPassword) -> anyhow::Result<String>;

    fn verfiy_password(
        &self,
        user_password_hash: &str,
        provided_password: String,
    ) -> anyhow::Result<()>;
}
