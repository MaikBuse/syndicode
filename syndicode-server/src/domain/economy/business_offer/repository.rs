use super::model::BusinessOffer;
use crate::domain::repository::RepositoryResult;
use tonic::async_trait;

#[async_trait]
pub trait BusinessOfferRepository: Send + Sync {
    async fn list_business_offers_in_tick(
        &self,
        game_tick: i64,
    ) -> RepositoryResult<Vec<BusinessOffer>>;
}

#[async_trait]
pub trait BusinessOfferTxRepository: Send + Sync {
    async fn insert_business_offers_in_tick(
        &mut self,
        game_tick: i64,
        business_offers: &[BusinessOffer],
    ) -> RepositoryResult<()>;

    async fn delete_business_offers_before_tick(&mut self, game_tick: i64)
        -> RepositoryResult<u64>;
}
