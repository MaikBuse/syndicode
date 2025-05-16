use syndicode_proto::syndicode_interface_v1::{
    auth_service_client::AuthServiceClient, game_service_client::GameServiceClient, PlayerAction,
};
use tokio::sync::mpsc;
use tonic::{
    metadata::{Ascii, MetadataKey, MetadataMap, MetadataValue},
    transport::Channel,
};

pub struct GrpcHandler {
    pub auth_client: AuthServiceClient<Channel>,
    pub game_client: GameServiceClient<Channel>,
    pub maybe_client_action_tx: Option<mpsc::Sender<PlayerAction>>,
}

impl GrpcHandler {
    pub async fn new(address: String) -> anyhow::Result<Self> {
        let auth_client = AuthServiceClient::connect(address.clone()).await?;
        let game_client = GameServiceClient::connect(address).await?;

        Ok(GrpcHandler {
            auth_client,
            game_client,
            maybe_client_action_tx: None,
        })
    }

    pub(super) fn add_ip_metadata(&self, metadata: &mut MetadataMap) -> anyhow::Result<()> {
        let key: MetadataKey<Ascii> = "CF-Connecting-IP".parse()?;
        let value: MetadataValue<Ascii> = "127.0.0.1".parse()?;
        metadata.insert(key, value);

        Ok(())
    }
}
