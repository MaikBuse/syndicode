use sqlx::PgPool;
use std::sync::Arc;

use crate::{
    application::{admin::create_user::CreateUserUseCase, bootstrap::Bootstrap, uow::UnitOfWork},
    infrastructure::postgres::migration::PostgresMigrator,
};

pub async fn run<U: UnitOfWork>(
    pool: Arc<PgPool>,
    create_user_uc: Arc<CreateUserUseCase<U>>,
) -> anyhow::Result<()> {
    let migrator = Arc::new(PostgresMigrator::new(pool.clone()));

    let bootstrapper = Bootstrap::new(migrator, create_user_uc.clone());

    bootstrapper
        .run()
        .await
        .map_err(|err| anyhow::format_err!(err))?;

    Ok(())
}
