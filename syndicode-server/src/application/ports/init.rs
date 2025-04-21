use crate::domain::repository::RepositoryResult;

#[tonic::async_trait]
pub trait InitializationRepository: Send + Sync {
    async fn is_database_initialized(&self) -> RepositoryResult<bool>;
    async fn set_advisory_lock(&self) -> RepositoryResult<()>;
}

#[tonic::async_trait]
pub trait InitializationTxRepository: Send + Sync {
    async fn is_database_initialized(&mut self) -> RepositoryResult<bool>;
    async fn set_database_initialization_flag(&mut self) -> RepositoryResult<()>;
}
