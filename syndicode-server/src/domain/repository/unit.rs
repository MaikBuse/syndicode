use super::RepositoryResult;
use crate::domain::unit::Unit;
use tonic::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UnitRepository: Send + Sync {
    async fn list_units(&self, user_uuid: Uuid) -> RepositoryResult<Vec<Unit>>;
    async fn create_unit(&self, unit: Unit) -> RepositoryResult<Unit>;
}

#[async_trait]
pub trait UnitTxRespository: Send + Sync {
    async fn list_units(&mut self, user_uuid: Uuid) -> RepositoryResult<Vec<Unit>>;
    async fn create_unit(&mut self, unit: Unit) -> RepositoryResult<Unit>;
}
