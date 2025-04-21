use bon::Builder;

use super::{
    admin::bootstrap_admin::BootstrapAdminUseCase,
    economy::bootstrap_economy::BootstrapEconomyUseCase,
    error::{ApplicationError, ApplicationResult},
    ports::{
        crypto::PasswordHandler, init::InitializationRepository, migration::MigrationRunner,
        uow::UnitOfWork,
    },
};
use crate::utils::read_env_var;
use std::sync::Arc;

const ADMIN_CORPORATION_NAME: &str = "Shinkai Heavyworks";
const ADMIN_USERNAME: &str = "admin";

#[derive(Builder)]
pub struct Bootstrap<UOW, INI, P, M>
where
    UOW: UnitOfWork,
    INI: InitializationRepository,
    P: PasswordHandler,
    M: MigrationRunner,
{
    migrator: Arc<M>,
    bootstrap_admin_uc: Arc<BootstrapAdminUseCase<UOW, P>>,
    bootstrap_economy_uc: Arc<BootstrapEconomyUseCase<UOW, INI>>,
}

impl<UOW, INI, P, M> Bootstrap<UOW, INI, P, M>
where
    UOW: UnitOfWork,
    INI: InitializationRepository,
    P: PasswordHandler,
    M: MigrationRunner,
{
    pub async fn run(&self) -> ApplicationResult<()> {
        tracing::info!("Bootstrapping server...");

        let admin_password = read_env_var("ADMIN_PASSWORD")?;
        let admin_email = read_env_var("ADMIN_EMAIL")?;

        self.migrator.run_migration().await?;

        if let Err(err) = self
            .bootstrap_admin_uc
            .execute()
            .user_name(ADMIN_USERNAME.to_string())
            .password(admin_password)
            .corporation_name(ADMIN_CORPORATION_NAME.to_string())
            .user_email(admin_email)
            .call()
            .await
        {
            match err {
                ApplicationError::UniqueConstraint => {}
                _ => return Err(err),
            };
        };

        self.bootstrap_economy_uc.execute().await?;

        Ok(())
    }
}
