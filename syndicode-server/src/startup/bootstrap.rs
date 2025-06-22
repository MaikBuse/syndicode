use bon::builder;
use std::sync::Arc;

use crate::{
    application::{
        admin::bootstrap::BootstrapAdminUseCase,
        bootstrap::BootstrapOrchestrator,
        economy::bootstrap::BootstrapEconomyUseCase,
        ports::{crypto::PasswordHandler, init::InitializationRepository, uow::UnitOfWork},
    },
    config::ServerConfig,
    infrastructure::postgres::migration::PostgresMigrator,
};

#[builder]
pub async fn run<UOW, INI, P>(
    config: Arc<ServerConfig>,
    migrator: Arc<PostgresMigrator>,
    bootstrap_admin_uc: Arc<BootstrapAdminUseCase<UOW, P>>,
    bootstrap_economy_uc: Arc<BootstrapEconomyUseCase<UOW, INI>>,
) -> anyhow::Result<()>
where
    UOW: UnitOfWork,
    INI: InitializationRepository,
    P: PasswordHandler,
{
    let bootstrapper = BootstrapOrchestrator::builder()
        .config(config)
        .migrator(migrator)
        .bootstrap_admin_uc(bootstrap_admin_uc)
        .bootstrap_economy_uc(bootstrap_economy_uc)
        .build();

    bootstrapper
        .run()
        .await
        .map_err(|err| anyhow::format_err!(err))?;

    Ok(())
}
