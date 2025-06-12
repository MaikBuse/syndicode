use bon::Builder;
use syndicode_proto::syndicode_interface_v1::GameUpdate;
use tonic::Streaming;

#[derive(Builder)]
pub struct QueryBusinessListingsDomainRequest {
    pub min_asking_price: Option<i64>,
    pub max_asking_price: Option<i64>,
    pub seller_corporation_uuid: Option<String>,
    pub market_uuid: Option<String>,
    pub min_operational_expenses: Option<i64>,
    pub max_operational_expenses: Option<i64>,
    pub sort_by: i32,
    pub sort_direction: i32,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[tonic::async_trait]
pub trait GameRepository {
    async fn get_corporation(&mut self) -> anyhow::Result<()>;

    async fn play_stream(&mut self, token: String) -> anyhow::Result<Streaming<GameUpdate>>;

    async fn query_business_listings(
        &self,
        req: QueryBusinessListingsDomainRequest,
    ) -> anyhow::Result<()>;

    async fn acquire_listed_business(&self, business_listing_uuid: String) -> anyhow::Result<()>;
}
