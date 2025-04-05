#[cfg(test)]
use mockall::{automock, predicate::*};

use tonic::async_trait;
use uuid::Uuid;

use super::model::Unit;
use crate::domain::repository::RepositoryResult;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait UnitRepository: Send + Sync {
    async fn list_units(&self, user_uuid: Uuid) -> RepositoryResult<Vec<Unit>>;
    async fn insert_unit(&self, unit: &Unit) -> RepositoryResult<()>;
}

#[async_trait]
pub trait UnitTxRespository: Send + Sync {
    async fn list_units(&mut self, user_uuid: Uuid) -> RepositoryResult<Vec<Unit>>;
    async fn insert_unit(&mut self, unit: &Unit) -> RepositoryResult<()>;
}
