use crate::{
    application::{
        crypto::PasswordHandler,
        error::{ApplicationError, ApplicationResult},
        uow::UnitOfWork,
    },
    domain::{
        corporation::model::Corporation,
        repository::RepositoryError,
        user::model::{password::UserPassword, role::UserRole, User},
    },
};
use std::sync::Arc;
use uuid::Uuid;

pub struct BootstrapAdminUseCase<U: UnitOfWork, P>
where
    P: PasswordHandler,
{
    pw: Arc<P>,
    uow: Arc<U>,
}

impl<U: UnitOfWork, P: PasswordHandler> BootstrapAdminUseCase<U, P> {
    pub fn new(pw: Arc<P>, uow: Arc<U>) -> Self {
        Self { pw, uow }
    }

    pub async fn execute(
        &self,
        user_name: String,
        password: String,
        corporation_name: String,
    ) -> ApplicationResult<User> {
        if user_name.is_empty() {
            return Err(ApplicationError::UsernameInvalid);
        }

        let user_password = UserPassword::new(password)?;

        let password_hash = self.pw.hash_user_password(user_password)?;

        let user_to_create = User {
            uuid: Uuid::now_v7(),
            name: user_name,
            password_hash: password_hash.to_string(),
            role: UserRole::Admin,
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
