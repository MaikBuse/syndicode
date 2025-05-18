use syndicode_proto::syndicode_interface_v1::GameUpdate;
use tonic::Streaming;

pub trait GameRepository {
    async fn play_stream(&mut self, token: String) -> anyhow::Result<Streaming<GameUpdate>>;

    async fn query_business_listings(
        &self,
        min_asking_price: Option<i64>,
        max_asking_price: Option<i64>,
        seller_corporation_uuid: Option<String>,
        market_uuid: Option<String>,
        min_operational_expenses: Option<i64>,
        max_operational_expenses: Option<i64>,
        sort_by: String,
        sort_direction: i32,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> anyhow::Result<()>;
}
