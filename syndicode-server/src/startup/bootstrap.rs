use sqlx::PgPool;
use std::sync::Arc;

use crate::{
    application::{
        admin::bootstrap_admin::BootstrapAdminUseCase,
        bootstrap::Bootstrap,
        ports::{crypto::PasswordHandler, uow::UnitOfWork},
    },
    infrastructure::postgres::migration::PostgresMigrator,
};

pub async fn run<U, P>(
    pool: Arc<PgPool>,
    bootstrap_admin_uc: Arc<BootstrapAdminUseCase<U, P>>,
) -> anyhow::Result<()>
where
    U: UnitOfWork,
    P: PasswordHandler,
{
    let migrator = Arc::new(PostgresMigrator::new(pool.clone()));

    let bootstrapper = Bootstrap::new(migrator, bootstrap_admin_uc.clone());

    bootstrapper
        .run()
        .await
        .map_err(|err| anyhow::format_err!(err))?;

    Ok(())
}
