use crate::{
    application::{
        crypto::PasswordHandler,
        error::{ApplicationError, ApplicationResult},
        uow::UnitOfWork,
    },
    domain::{
        corporation::model::Corporation,
        repository::RepositoryError,
        user::{
            model::{role::UserRole, User},
            repository::UserRepository,
        },
    },
};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use std::sync::Arc;
use uuid::Uuid;

pub struct CreateUserUseCase<U: UnitOfWork> {
    pw: Arc<dyn PasswordHandler>,
    uow: Arc<U>,
    user_repo: Arc<dyn UserRepository>,
}

impl<U: UnitOfWork> CreateUserUseCase<U> {
    pub fn new(
        pw: Arc<dyn PasswordHandler>,
        uow: Arc<U>,
        user_repo: Arc<dyn UserRepository>,
    ) -> Self {
        Self { pw, uow, user_repo }
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

            let req_user = self.user_repo.get_user(req_user_uuid).await?;

            if req_user.role != UserRole::Admin {
                return Err(ApplicationError::Unauthorized);
            }
        }

        let salt = SaltString::generate(&mut OsRng);

        let password_hash = self.pw.hash_password(password, &salt)?;

        let user_to_create = User {
            uuid: Uuid::now_v7(),
            name: user_name,
            password_hash: password_hash.to_string(),
            role: user_role,
        };

        let user_created = match self
            .uow
            .execute(|ctx| {
                Box::pin(async move {
                    let user_created = ctx.create_user(user_to_create).await?;

                    let corporation = Corporation::new(user_created.uuid, corporation_name);

                    ctx.create_corporation(corporation).await?;

                    Ok(user_created)
                })
            })
            .await
        {
            Ok(user_created) => user_created,
            Err(err) => match err {
                RepositoryError::UniqueConstraint => {
                    return Err(ApplicationError::UniqueConstraint)
                }
                _ => return Err(err.into()),
            },
        };

        Ok(user_created)
    }
}
