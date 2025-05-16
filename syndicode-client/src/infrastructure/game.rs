use super::grpc::GrpcHandler;
use crate::domain::game::GameRepository;
use syndicode_proto::{
    syndicode_economy_v1::QueryBusinessListingsRequest,
    syndicode_interface_v1::{player_action::Action, GameUpdate, PlayerAction, SortDirection},
};
use tokio::sync::mpsc::{self};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{metadata::MetadataValue, Request, Streaming};
use uuid::Uuid;

impl GameRepository for GrpcHandler {
    async fn play_stream(&mut self, token: String) -> anyhow::Result<Streaming<GameUpdate>> {
        let (client_action_tx, client_action_rx) = mpsc::channel::<PlayerAction>(10);
        self.maybe_client_action_tx = Some(client_action_tx);

        let request_stream = ReceiverStream::new(client_action_rx);

        // Create a tonic::Request from the stream
        let mut grpc_request = Request::new(request_stream);

        self.add_ip_metadata(grpc_request.metadata_mut())?;

        // Add the token as metadata
        let auth_header_value = format!("Bearer {}", token);
        match MetadataValue::try_from(&auth_header_value) {
            Ok(metadata_value) => {
                grpc_request
                    .metadata_mut()
                    .insert("authorization", metadata_value);
            }
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "Failed to create metadata for token: {}",
                    e
                ));
            }
        }

        // Call the `play_stream` method with the modified gRPC request
        match self.game_client.play_stream(grpc_request).await {
            Ok(response) => {
                let server_updates_stream = response.into_inner();
                Ok(server_updates_stream)
            }
            Err(status) => Err(anyhow::anyhow!("Error calling play_stream: {}", status)),
        }
    }

    async fn query_business_listings(&self) -> anyhow::Result<()> {
        let Some(client_action_tx) = self.maybe_client_action_tx.clone() else {
            return Err(anyhow::anyhow!(
                "Failed to retrieve client action sender from grpc handler"
            ));
        };

        let player_action = PlayerAction {
            request_uuid: Uuid::now_v7().to_string(),
            action: Some(Action::QueryBusinessListings(
                QueryBusinessListingsRequest {
                    min_asking_price: None,
                    max_asking_price: None,
                    seller_corporation_uuid: None,
                    market_uuid: None,
                    min_operational_expenses: None,
                    max_operational_expenses: None,
                    sort_by: "name".to_string(),
                    sort_direction: SortDirection::Ascending.into(),
                    limit: Some(10),
                    offset: Some(0),
                },
            )),
        };

        Ok(client_action_tx.send(player_action).await?)
    }
}
