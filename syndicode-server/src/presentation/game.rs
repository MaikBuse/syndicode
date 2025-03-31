mod economy;
mod warfare;

use crate::{
    engine::Job,
    service::{economy::EconomyService, warfare::WarfareService},
};
use dashmap::DashMap;
use economy::get_corporation;
use std::{collections::VecDeque, future::Future, pin::Pin, sync::Arc};
use syndicode_proto::syndicode_interface_v1::{
    game_service_server::GameService, player_action::Action, GameUpdate, PlayerAction,
};
use tokio::sync::{
    mpsc::{self, Sender},
    Mutex,
};
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use tonic::{Request, Response, Status, Streaming};
use uuid::Uuid;
use warfare::{list_units, spawn_unit};

type UserTx = mpsc::Sender<Result<GameUpdate, Status>>;

use super::common::uuid_from_metadata;
pub struct GamePresenter {
    pub jobs: Arc<Mutex<VecDeque<Job>>>,
    pub user_channels: Arc<DashMap<Uuid, UserTx>>,
    pub economy_service: Arc<EconomyService>,
    pub warfare_service: Arc<WarfareService>,
}

#[tonic::async_trait]
impl GameService for GamePresenter {
    type PlayStreamStream = Pin<Box<dyn Stream<Item = Result<GameUpdate, Status>> + Send>>;

    async fn play_stream(
        &self,
        request: Request<Streaming<PlayerAction>>,
    ) -> Result<Response<Self::PlayStreamStream>, Status> {
        let req_user_uuid = match uuid_from_metadata(request.metadata()) {
            Ok(uuid) => uuid,
            Err(status) => return Err(status),
        };

        let mut stream = request.into_inner();

        // Channel to send game updates to this player
        let (tx, rx) = mpsc::channel(16);
        let player_rx = ReceiverStream::new(rx);

        self.user_channels.insert(req_user_uuid, tx.clone());

        // Use Arc for shared ownership
        let tx_arc = Arc::new(tx);
        let jobs = Arc::clone(&self.jobs);
        let economy_service = Arc::clone(&self.economy_service);
        let warfare_service = Arc::clone(&self.warfare_service);

        // Spawn receiver of user actions
        tokio::spawn(async move {
            let tx = tx_arc.clone(); // Clone Arc to move into async block

            while let Some(Ok(action)) = stream.next().await {
                if let Some(request_enum) = action.action {
                    match request_enum {
                        Action::GetCorporation(req) => {
                            handle_request(
                                || {
                                    get_corporation(
                                        req,
                                        Arc::clone(&economy_service),
                                        req_user_uuid,
                                    )
                                },
                                &tx,
                            )
                            .await;
                        }
                        Action::SpawnUnit(_) => {
                            handle_request(|| spawn_unit(Arc::clone(&jobs), req_user_uuid), &tx)
                                .await;
                        }
                        Action::ListUnit(_) => {
                            handle_request(
                                || list_units(Arc::clone(&warfare_service), req_user_uuid),
                                &tx,
                            )
                            .await;
                        }
                    }
                }
            }
        });

        Ok(Response::new(Box::pin(player_rx) as Self::PlayStreamStream))
    }
}

async fn handle_request<F, Fut>(fut: F, tx: &Arc<Sender<Result<GameUpdate, Status>>>)
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<GameUpdate, Status>>,
{
    match fut().await {
        Ok(resp) => {
            if let Err(err) = tx.send(Ok(resp)).await {
                tracing::error!("{}", err);
            }
        }
        Err(status) => {
            if let Err(err) = tx.send(Err(status)).await {
                tracing::error!("{}", err);
            }
        }
    }
}
