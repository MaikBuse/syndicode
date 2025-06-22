use crate::{application::ports::migration::MigrationRunner, domain::repository::RepositoryResult};
use std::sync::Arc;

use super::PostgresDatabase;

pub struct PostgresMigrator {
    pg_db: Arc<PostgresDatabase>,
}

impl PostgresMigrator {
    pub fn new(pg_db: Arc<PostgresDatabase>) -> Self {
        Self { pg_db }
    }
}

#[tonic::async_trait]
impl MigrationRunner for PostgresMigrator {
    async fn run_migration(&self) -> RepositoryResult<()> {
        sqlx::migrate!().run(&self.pg_db.pool).await?;

        Ok(())
    }
}
