use crate::application::{
    action::{ActionDetails, QueuedActionPayload},
    error::ApplicationResult,
    ports::{game_tick::GameTickRepository, queuer::ActionQueueable},
};
use bon::{bon, Builder};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Builder)]
pub struct SpawnUnitUseCase<Q, GTR>
where
    Q: ActionQueueable,
    GTR: GameTickRepository,
{
    action_queuer: Arc<Q>,
    game_tick_repo: Arc<GTR>,
}

#[bon]
impl<Q, GTR> SpawnUnitUseCase<Q, GTR>
where
    Q: ActionQueueable,
    GTR: GameTickRepository,
{
    #[builder]
    pub async fn execute(&self, request_uuid: Uuid, req_user_uuid: Uuid) -> ApplicationResult<i64> {
        let action = QueuedActionPayload::builder()
            .request_uuid(request_uuid)
            .user_uuid(req_user_uuid)
            .details(ActionDetails::SpawnUnit)
            .build();

        match self.action_queuer.enqueue_action(action).await {
            Ok(entry_id) => {
                tracing::info!(
                    "Successfully enqueued SpawnUnit action with ID: {}",
                    entry_id
                );
            }
            Err(err) => {
                tracing::error!("Failed to enqueue SpawnUnit action: {:?}", err);

                return Err(err.into());
            }
        };

        Ok(self.game_tick_repo.get_current_game_tick().await?)
    }
}
