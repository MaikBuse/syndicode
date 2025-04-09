#[cfg(test)]
use mockall::{automock, predicate::*};

use tonic::async_trait;
use uuid::Uuid;

use super::model::Unit;
use crate::domain::repository::RepositoryResult;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait UnitRepository: Send + Sync {
    async fn list_units(&self) -> RepositoryResult<Vec<Unit>>;
    async fn list_units_by_user(&self, user_uuid: Uuid) -> RepositoryResult<Vec<Unit>>;
}

#[async_trait]
pub trait UnitTxRespository: Send + Sync {
    async fn list_units(&mut self) -> RepositoryResult<Vec<Unit>>;
    async fn list_units_by_user(&mut self, user_uuid: Uuid) -> RepositoryResult<Vec<Unit>>;
    async fn insert_units_in_tick(
        &mut self,
        game_tick: i64,
        units: Vec<Unit>,
    ) -> RepositoryResult<()>;
    async fn delete_units_before_tick(&mut self, game_tick: i64) -> RepositoryResult<u64>;
}
