use bon::builder;
use sqlx::PgPool;
use std::sync::Arc;

use crate::{
    application::{
        admin::bootstrap_admin::BootstrapAdminUseCase,
        bootstrap::Bootstrap,
        economy::bootstrap_economy::BootstrapEconomyUseCase,
        ports::{crypto::PasswordHandler, init::InitializationRepository, uow::UnitOfWork},
    },
    infrastructure::postgres::migration::PostgresMigrator,
};

#[builder]
pub async fn run<UOW, INI, P>(
    pool: Arc<PgPool>,
    bootstrap_admin_uc: Arc<BootstrapAdminUseCase<UOW, P>>,
    bootstrap_economy_uc: Arc<BootstrapEconomyUseCase<UOW, INI>>,
) -> anyhow::Result<()>
where
    UOW: UnitOfWork,
    INI: InitializationRepository,
    P: PasswordHandler,
{
    let migrator = Arc::new(PostgresMigrator::new(pool.clone()));

    let bootstrapper = Bootstrap::builder()
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
