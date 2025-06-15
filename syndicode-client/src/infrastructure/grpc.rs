use std::str::FromStr;

use syndicode_proto::syndicode_interface_v1::{
    admin_service_client::AdminServiceClient, auth_service_client::AuthServiceClient,
    game_service_client::GameServiceClient, PlayerAction,
};
use time::OffsetDateTime;
use tokio::sync::mpsc;
use tonic::{
    metadata::{Ascii, MetadataKey, MetadataMap, MetadataValue},
    transport::{Channel, Endpoint},
    Response, Status,
};

use crate::domain::response::{DomainResponse, ResponseType};

pub struct GrpcHandler {
    pub is_local_test: bool,
    pub auth_client: AuthServiceClient<Channel>,
    pub admin_client: AdminServiceClient<Channel>,
    pub game_client: GameServiceClient<Channel>,
    pub maybe_client_action_tx: Option<mpsc::Sender<PlayerAction>>,
}

impl GrpcHandler {
    /// The address string should be a full URI like "http://localhost:50051"
    /// or "https://my-prod-server.com:443"
    pub async fn new(address: String, is_local_test: bool) -> anyhow::Result<Self> {
        rustls::crypto::ring::default_provider()
            .install_default()
            .expect("Failed to install rustls crypto provider");

        let endpoint = Endpoint::from_str(&address)?;

        let channel = endpoint
            .tls_config(tonic::transport::ClientTlsConfig::new().with_native_roots())?
            .connect()
            .await
            .map_err(|err| {
                anyhow::anyhow!(
                    "Failed to establish gRPC connection to {}: {:?}",
                    address,
                    err
                )
            })?;

        let auth_client = AuthServiceClient::new(channel.clone());
        let admin_client = AdminServiceClient::new(channel.clone());
        let game_client = GameServiceClient::new(channel);

        Ok(GrpcHandler {
            is_local_test,
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
