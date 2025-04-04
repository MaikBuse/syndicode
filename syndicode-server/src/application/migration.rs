use crate::domain::repository::RepositoryResult;

#[tonic::async_trait]
pub trait MigrationRunner {
    async fn run_migration(&self) -> RepositoryResult<()>;
}
