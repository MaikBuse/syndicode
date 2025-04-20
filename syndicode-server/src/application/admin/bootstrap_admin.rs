use crate::{
    application::{
        error::{ApplicationError, ApplicationResult},
        ports::{crypto::PasswordHandler, uow::UnitOfWork},
    },
    domain::{
        corporation::model::{name::CorporationName, Corporation},
        repository::RepositoryError,
        user::model::{
            email::UserEmail, name::UserName, password::UserPassword, role::UserRole,
            status::UserStatus, User,
        },
    },
};
use bon::{bon, Builder};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Builder)]
pub struct BootstrapAdminUseCase<UOW, P>
where
    UOW: UnitOfWork,
    P: PasswordHandler,
{
    pw: Arc<P>,
    uow: Arc<UOW>,
}

#[bon]
impl<UOW, P> BootstrapAdminUseCase<UOW, P>
where
    UOW: UnitOfWork,
    P: PasswordHandler,
{
    #[builder]
    pub async fn execute(
        &self,
        user_name: String,
        password: String,
        user_email: String,
        corporation_name: String,
    ) -> ApplicationResult<User> {
        let user_name = UserName::new(user_name)?;

        let user_password = UserPassword::new(password)?;

        let password_hash = self.pw.hash_user_password(user_password)?;

        let user = User {
            uuid: Uuid::now_v7(),
            name: user_name,
            password_hash: password_hash.to_string(),
            email: UserEmail::new(user_email)?,
            role: UserRole::Admin,
            status: UserStatus::Active,
        };

        let user_created = self
            .uow
            .execute(|ctx| {
                Box::pin(async move {
                    let user_to_create = user.clone();

                    if let Err(err) = ctx.create_user(&user_to_create).await {
                        match err {
                            RepositoryError::UniqueConstraint => {
                                return Err(ApplicationError::UniqueConstraint)
                            }
                            _ => return Err(ApplicationError::from(err)),
                        }
                    }

                    let corporation_name = CorporationName::new(corporation_name)?;

                    let corporation = Corporation::new(user_to_create.uuid, corporation_name);

                    ctx.insert_corporation(&corporation)
                        .await
                        .map_err(ApplicationError::from)?;

                    Ok(user_to_create)
                })
            })
            .await?;

        Ok(user_created)
    }
}
