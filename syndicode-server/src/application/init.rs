use bon::Builder;

use super::{
    admin::bootstrap::BootstrapAdminUseCase,
    economy::bootstrap::BootstrapEconomyUseCase,
    error::ApplicationResult,
    ports::{
        crypto::PasswordHandler, downloader::BackupDownloader, init::InitializationRepository,
        migration::MigrationRunner, restorer::DatabaseRestorer, uow::UnitOfWork,
    },
};
use crate::{application::ports::init::FlagKey, config::ServerConfig};
use std::sync::Arc;

#[derive(Builder)]
pub struct InitializationOrchestrator<UOW, INI, RES, DOW, P, M>
where
    UOW: UnitOfWork,
    INI: InitializationRepository,
    RES: DatabaseRestorer,
    DOW: BackupDownloader,
    P: PasswordHandler,
    M: MigrationRunner,
{
    config: Arc<ServerConfig>,
    migrator: Arc<M>,
    init_repo: Arc<INI>,
    restorer: Arc<RES>,
    downloader: Arc<DOW>,
    bootstrap_admin_uc: Arc<BootstrapAdminUseCase<UOW, P, INI>>,
    bootstrap_economy_uc: Arc<BootstrapEconomyUseCase<UOW, INI>>,
    restore_url: Option<String>,
}

impl<UOW, INI, RES, DOW, P, M> InitializationOrchestrator<UOW, INI, RES, DOW, P, M>
where
    UOW: UnitOfWork,
    INI: InitializationRepository,
    RES: DatabaseRestorer,
    DOW: BackupDownloader,
    P: PasswordHandler,
    M: MigrationRunner,
{
    pub async fn run(&self) -> ApplicationResult<()> {
        self.migrator.run_migration().await?;

        let is_initialized = self.init_repo.is_flag_set(FlagKey::Database).await?;
        if is_initialized {
            tracing::info!("Database has already been initialized...");

            return Ok(());
        }

        match self.restore_url.clone() {
            Some(url) => {
                tracing::info!("Downloading database dump...");
                let stream = self.downloader.download(url).await?;

                tracing::info!("Restoring database...");
                self.restorer.restore(stream).await?;
            }
            None => {
                tracing::info!("Bootstrapping database...");
                self.bootstrap_admin_uc
                    .execute()
                    .user_name(self.config.auth.admin_username.clone())
                    .password(self.config.auth.admin_password.clone())
                    .corporation_name(self.config.auth.admin_corporation_name.clone())
                    .user_email(self.config.auth.admin_email.clone())
                    .call()
                    .await?;

                self.bootstrap_economy_uc.execute().await?;
            }
        };

        self.init_repo.set_flag(FlagKey::Database).await?;

        Ok(())
    }
}
