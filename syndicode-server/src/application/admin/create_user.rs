use crate::{
    application::{
        error::{ApplicationError, ApplicationResult},
        ports::{crypto::PasswordHandler, uow::UnitOfWork},
    },
    domain::{
        economy::corporation::model::{name::CorporationName, Corporation},
        repository::RepositoryError,
        user::{
            model::{
                email::UserEmail, name::UserName, password::UserPassword, role::UserRole,
                status::UserStatus, User,
            },
            repository::UserRepository,
        },
    },
};
use bon::{bon, Builder};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Builder)]
pub struct CreateUserUseCase<P, UOW, USR>
where
    P: PasswordHandler,
    UOW: UnitOfWork,
    USR: UserRepository,
{
    pw: Arc<P>,
    uow: Arc<UOW>,
    user_repo: Arc<USR>,
}

#[bon]
impl<P, UOW, USR> CreateUserUseCase<P, UOW, USR>
where
    P: PasswordHandler,
    UOW: UnitOfWork,
    USR: UserRepository,
{
    #[builder]
    pub async fn execute(
        &self,
        req_user_uuid: Uuid,
        user_name: String,
        password: String,
        user_email: String,
        user_role: UserRole,
        corporation_name: String,
    ) -> ApplicationResult<User> {
        let user_name = UserName::new(user_name)?;
        let user_password = UserPassword::new(password)?;
        let user_email = UserEmail::new(user_email)?;

        // check authorization
        let req_user = self.user_repo.get_user(req_user_uuid).await?;
        if req_user.role != UserRole::Admin {
            return Err(ApplicationError::Unauthorized);
        }

        let password_hash = self.pw.hash_user_password(user_password)?;

        let user = User {
            uuid: Uuid::now_v7(),
            name: user_name,
            password_hash: password_hash.to_string(),
            email: user_email,
            role: user_role,
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
