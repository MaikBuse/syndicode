use tonic::async_trait;
use uuid::Uuid;

use super::model::Corporation;
use crate::domain::repository::RepositoryResult;

#[async_trait]
pub trait CorporationRepository: Send + Sync {
    async fn insert_corporation(&self, corporation: &Corporation) -> RepositoryResult<()>;

    async fn get_corporation_by_user(&self, user_uuid: Uuid) -> RepositoryResult<Corporation>;

    async fn get_corporation_by_uuid(
        &self,
        corporation_uuid: Uuid,
    ) -> RepositoryResult<Corporation>;

    async fn list_corporations(&self) -> RepositoryResult<Vec<Corporation>>;
}

#[async_trait]
pub trait CorporationTxRepository: Send + Sync {
    async fn insert_corporation(&mut self, corporation: &Corporation) -> RepositoryResult<()>;

    async fn get_corporation_by_user(&mut self, user_uuid: Uuid) -> RepositoryResult<Corporation>;

    async fn get_corporation_by_uuid(&mut self, uuid: Uuid) -> RepositoryResult<Corporation>;
}
