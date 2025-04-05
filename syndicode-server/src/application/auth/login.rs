use crate::{
    application::{
        error::{ApplicationError, ApplicationResult},
        ports::crypto::{JwtHandler, PasswordHandler},
    },
    domain::user::repository::UserRepository,
};
use std::sync::Arc;

pub struct LoginUseCase {
    pw: Arc<dyn PasswordHandler>,
    jwt: Arc<dyn JwtHandler>,
    user_repo: Arc<dyn UserRepository>,
}

impl LoginUseCase {
    pub fn new(
        pw: Arc<dyn PasswordHandler>,
        jwt: Arc<dyn JwtHandler>,
        user_repo: Arc<dyn UserRepository>,
    ) -> Self {
        Self { pw, jwt, user_repo }
    }

    pub async fn execute(&self, user_name: String, password: String) -> ApplicationResult<String> {
        let Ok(user) = self.user_repo.get_user_by_name(user_name).await else {
            return Err(ApplicationError::WrongUserCredentials);
        };

        if self
            .pw
            .verfiy_password(&user.password_hash, password)
            .is_err()
        {
            return Err(ApplicationError::WrongUserCredentials);
        }

        Ok(self.jwt.encode_jwt(user.uuid, user.role)?)
    }
}
