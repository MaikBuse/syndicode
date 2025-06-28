use crate::{
    application::{
        error::{ApplicationError, ApplicationResult},
        ports::{
            crypto::PasswordHandler,
            init::{FlagKey, InitializationRepository},
            uow::UnitOfWork,
        },
    },
    domain::{
        economy::corporation::model::{name::CorporationName, Corporation},
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
pub struct BootstrapAdminUseCase<UOW, P, INI>
where
    UOW: UnitOfWork,
    P: PasswordHandler,
    INI: InitializationRepository,
{
    pw: Arc<P>,
    uow: Arc<UOW>,
    init_repo: Arc<INI>,
}

#[bon]
impl<UOW, P, INI> BootstrapAdminUseCase<UOW, P, INI>
where
    UOW: UnitOfWork,
    P: PasswordHandler,
    INI: InitializationRepository,
{
    #[builder]
    pub async fn execute(
        &self,
        user_name: String,
        password: String,
        user_email: String,
        corporation_name: String,
    ) -> ApplicationResult<()> {
        if self.init_repo.is_flag_set(FlagKey::AdminDomainInit).await? {
            tracing::info!("Admin Domain initialization flag is already set. Skipping.");

            return Ok(());
        }

        tracing::info!(
            "Admin Domain initialization flag not set or missing. Attempting initialization lock..."
        );

        self.init_repo.set_advisory_lock().await?;

        tracing::info!("Acquired initialization advisory lock.");

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

        self.uow
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

                    ctx.create_corporation(&corporation)
                        .await
                        .map_err(ApplicationError::from)?;

                    ctx.set_flag(FlagKey::AdminDomainInit).await?;

                    Ok(user_to_create)
                })
            })
            .await?;

        tracing::info!("Admin Domain initialization complete and flag set.");

        self.init_repo.set_advisory_lock().await?;

        tracing::info!("Released initialization advisory lock.");

        Ok(())
    }
}
