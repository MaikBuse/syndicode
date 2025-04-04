use crate::{application::migration::MigrationRunner, domain::repository::RepositoryResult};
use sqlx::PgPool;
use std::sync::Arc;

pub struct PostgresMigrator {
    pool: Arc<PgPool>,
}

impl PostgresMigrator {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[tonic::async_trait]
impl MigrationRunner for PostgresMigrator {
    async fn run_migration(&self) -> RepositoryResult<()> {
        sqlx::migrate!().run(&*self.pool).await?;

        Ok(())
    }
}
