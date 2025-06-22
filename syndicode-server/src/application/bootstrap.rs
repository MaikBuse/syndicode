use bon::Builder;

use super::{
    admin::bootstrap::BootstrapAdminUseCase,
    economy::bootstrap::BootstrapEconomyUseCase,
    error::{ApplicationError, ApplicationResult},
    ports::{
        crypto::PasswordHandler, init::InitializationRepository, migration::MigrationRunner,
        uow::UnitOfWork,
    },
};
use crate::config::ServerConfig;
use std::sync::Arc;

#[derive(Builder)]
pub struct BootstrapOrchestrator<UOW, INI, P, M>
where
    UOW: UnitOfWork,
    INI: InitializationRepository,
    P: PasswordHandler,
    M: MigrationRunner,
{
    config: Arc<ServerConfig>,
    migrator: Arc<M>,
    bootstrap_admin_uc: Arc<BootstrapAdminUseCase<UOW, P>>,
    bootstrap_economy_uc: Arc<BootstrapEconomyUseCase<UOW, INI>>,
}

impl<UOW, INI, P, M> BootstrapOrchestrator<UOW, INI, P, M>
where
    UOW: UnitOfWork,
    INI: InitializationRepository,
    P: PasswordHandler,
    M: MigrationRunner,
{
    pub async fn run(&self) -> ApplicationResult<()> {
        tracing::info!("Bootstrapping server...");

        self.migrator.run_migration().await?;

        if let Err(err) = self
            .bootstrap_admin_uc
            .execute()
            .user_name(self.config.auth.admin_username.clone())
            .password(self.config.auth.admin_password.clone())
            .corporation_name(self.config.auth.admin_corporation_name.clone())
            .user_email(self.config.auth.admin_email.clone())
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
