#[cfg(test)]
use mockall::{automock, predicate::*};

use tonic::async_trait;
use uuid::Uuid;

use super::model::Corporation;
use crate::domain::repository::RepositoryResult;

pub struct GetCorporationOutcome {
    pub game_tick: i64,
    pub corporation: Corporation,
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait CorporationRepository: Send + Sync {
    async fn insert_corporation(&self, corporation: &Corporation) -> RepositoryResult<()>;

    async fn get_corporation_by_user(
        &self,
        user_uuid: Uuid,
    ) -> RepositoryResult<GetCorporationOutcome>;

    async fn get_corporation_by_uuid(
        &self,
        corporation_uuid: Uuid,
    ) -> RepositoryResult<Corporation>;

    async fn get_corporation_by_name(
        &self,
        corporation_name: String,
    ) -> RepositoryResult<Corporation>;

    async fn list_corporations_in_tick(&self, game_tick: i64)
        -> RepositoryResult<Vec<Corporation>>;
}

#[async_trait]
pub trait CorporationTxRepository: Send + Sync {
    async fn create_corporation(&mut self, corporation: &Corporation) -> RepositoryResult<()>;

    async fn insert_corporations_in_tick(
        &mut self,
        game_tick: i64,
        corporations: Vec<Corporation>,
    ) -> RepositoryResult<()>;

    async fn get_corporation_by_user(&mut self, user_uuid: Uuid) -> RepositoryResult<Corporation>;

    async fn get_corporation_by_uuid(&mut self, uuid: Uuid) -> RepositoryResult<Corporation>;

    async fn get_corporation_by_name(
        &mut self,
        corporation_name: String,
    ) -> RepositoryResult<Corporation>;

    async fn delete_corporations_before_tick(&mut self, game_tick: i64) -> RepositoryResult<u64>;
}
