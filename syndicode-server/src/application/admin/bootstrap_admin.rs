use crate::{
    application::{
        error::{ApplicationError, ApplicationResult},
        ports::{crypto::PasswordHandler, uow::UnitOfWork},
    },
    domain::{
        corporation::model::{name::CorporationName, Corporation},
        repository::RepositoryError,
        user::model::{password::UserPassword, role::UserRole, User},
    },
};
use std::sync::Arc;
use uuid::Uuid;

pub struct BootstrapAdminUseCase<UOW, P>
where
    UOW: UnitOfWork,
    P: PasswordHandler,
{
    pw: Arc<P>,
    uow: Arc<UOW>,
}

impl<UOW, P> BootstrapAdminUseCase<UOW, P>
where
    UOW: UnitOfWork,
    P: PasswordHandler,
{
    pub fn new(pw: Arc<P>, uow: Arc<UOW>) -> Self {
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

        let user = User {
            uuid: Uuid::now_v7(),
            name: user_name,
            password_hash: password_hash.to_string(),
            role: UserRole::Admin,
        };

        let user_created = match self
            .uow
            .execute(|ctx| {
                Box::pin(async move {
                    let user_to_create = user.clone();

                    ctx.create_user(&user_to_create).await?;

                    let corporation_name = CorporationName::new(corporation_name)
                        .map_err(|err| anyhow::format_err!(err))?;

                    let corporation = Corporation::new(user_to_create.uuid, corporation_name);

                    ctx.insert_corporation(&corporation).await?;

                    Ok(user_to_create)
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
