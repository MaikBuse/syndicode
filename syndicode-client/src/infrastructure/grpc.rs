use syndicode_proto::syndicode_interface_v1::{
    admin_service_client::AdminServiceClient, auth_service_client::AuthServiceClient,
    game_service_client::GameServiceClient, PlayerAction,
};
use time::OffsetDateTime;
use tokio::sync::mpsc;
use tonic::{
    metadata::{Ascii, MetadataKey, MetadataMap, MetadataValue},
    transport::Channel,
    Response, Status,
};

use crate::domain::response::{DomainResponse, ResponseType};

pub struct GrpcHandler {
    pub auth_client: AuthServiceClient<Channel>,
    pub admin_client: AdminServiceClient<Channel>,
    pub game_client: GameServiceClient<Channel>,
    pub maybe_client_action_tx: Option<mpsc::Sender<PlayerAction>>,
}

impl GrpcHandler {
    pub async fn new(address: String) -> anyhow::Result<Self> {
        let auth_client = AuthServiceClient::connect(address.clone()).await?;
        let admin_client = AdminServiceClient::connect(address.clone()).await?;
        let game_client = GameServiceClient::connect(address).await?;

        Ok(GrpcHandler {
            auth_client,
            admin_client,
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

    pub(super) fn add_token_metadata(
        &self,
        metadata: &mut MetadataMap,
        token: String,
    ) -> anyhow::Result<()> {
        let key: MetadataKey<Ascii> = "authorization".parse()?;
        let value: MetadataValue<Ascii> = format!("Bearer {}", token).parse()?;

        metadata.insert(key, value);

        Ok(())
    }

    pub(super) fn response_from_result<T>(
        &self,
        result: Result<Response<T>, Status>,
    ) -> anyhow::Result<DomainResponse>
    where
        T: std::fmt::Debug,
    {
        match result {
            Ok(response) => Ok(DomainResponse::builder()
                .response_type(ResponseType::Success)
                .code("OK".to_string())
                .message(format!("{:#?}", response))
                .timestamp(OffsetDateTime::now_utc())
                .build()),
            Err(status) => Ok(DomainResponse::builder()
                .response_type(ResponseType::Error)
                .code(status.code().description().to_string())
                .message(format!("{:#?}", status.message()))
                .timestamp(OffsetDateTime::now_utc())
                .build()),
        }
    }
}
