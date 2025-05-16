use syndicode_proto::syndicode_interface_v1::GameUpdate;
use tonic::Streaming;

pub trait GameRepository {
    async fn play_stream(&mut self, token: String) -> anyhow::Result<Streaming<GameUpdate>>;

    async fn query_business_listings(&self) -> anyhow::Result<()>;
}
