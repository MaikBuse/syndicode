use tonic::async_trait;
use uuid::Uuid;

use super::model::Corporation;
use crate::domain::repository::RepositoryResult;

#[async_trait]
pub trait CorporationRepository: Send + Sync {
    async fn get_corporation_by_user(&self, user_uuid: Uuid) -> RepositoryResult<Corporation>;

    async fn create_corporation(&self, corporation: Corporation) -> RepositoryResult<Corporation>;

    async fn update_corporation(&self, corporation: Corporation) -> RepositoryResult<Corporation>;
}

#[async_trait]
pub trait CorporationTxRepository: Send + Sync {
    async fn get_corporation_by_user(&mut self, user_uuid: Uuid) -> RepositoryResult<Corporation>;

    async fn create_corporation(
        &mut self,
        corporation: Corporation,
    ) -> RepositoryResult<Corporation>;

    async fn update_corporation(
        &mut self,
        corporation: Corporation,
    ) -> RepositoryResult<Corporation>;
}
