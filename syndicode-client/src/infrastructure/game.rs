use super::grpc::GrpcHandler;
use crate::domain::game::GameRepository;
use syndicode_proto::{
    syndicode_economy_v1::QueryBusinessListingsRequest,
    syndicode_interface_v1::{player_action::Action, GameUpdate, PlayerAction},
};
use tokio::sync::mpsc::{self};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Streaming};
use uuid::Uuid;

impl GameRepository for GrpcHandler {
    async fn play_stream(&mut self, token: String) -> anyhow::Result<Streaming<GameUpdate>> {
        let (client_action_tx, client_action_rx) = mpsc::channel::<PlayerAction>(10);
        self.maybe_client_action_tx = Some(client_action_tx);

        let request_stream = ReceiverStream::new(client_action_rx);

        // Create a tonic::Request from the stream
        let mut grpc_request = Request::new(request_stream);

        self.add_ip_metadata(grpc_request.metadata_mut())?;
        self.add_token_metadata(grpc_request.metadata_mut(), token)?;

        // Call the `play_stream` method with the modified gRPC request
        match self.game_client.play_stream(grpc_request).await {
            Ok(response) => {
                let server_updates_stream = response.into_inner();
                Ok(server_updates_stream)
            }
            Err(status) => Err(anyhow::anyhow!("Error calling play_stream: {}", status)),
        }
    }

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
    ) -> anyhow::Result<()> {
        let Some(client_action_tx) = self.maybe_client_action_tx.clone() else {
            return Err(anyhow::anyhow!(
                "Failed to retrieve client action sender from grpc handler"
            ));
        };

        let player_action = PlayerAction {
            request_uuid: Uuid::now_v7().to_string(),
            action: Some(Action::QueryBusinessListings(
                QueryBusinessListingsRequest {
                    min_asking_price,
                    max_asking_price,
                    seller_corporation_uuid,
                    market_uuid,
                    min_operational_expenses,
                    max_operational_expenses,
                    sort_by,
                    sort_direction,
                    limit,
                    offset,
                },
            )),
        };

        Ok(client_action_tx.send(player_action).await?)
    }
}
