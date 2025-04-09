use std::sync::Arc;

use uuid::Uuid;

use crate::application::{
    action::QueuedAction, error::ApplicationResult, ports::queue::ActionQueuer,
};

pub struct SpawnUnitUseCase<Q>
where
    Q: ActionQueuer,
{
    action_queuer: Arc<Q>,
}

impl<Q> SpawnUnitUseCase<Q>
where
    Q: ActionQueuer,
{
    pub fn new(action_queuer: Arc<Q>) -> Self {
        Self { action_queuer }
    }

    pub async fn execute(&self, req_user_uuid: Uuid) -> ApplicationResult<()> {
        let action = QueuedAction::SpawnUnit { req_user_uuid };

        match self.action_queuer.enqueue_action(action).await {
            Ok(entry_id) => {
                tracing::info!(
                    "Successfully enqueued SpawnUnit action with ID: {}",
                    entry_id
                );
                Ok(())
            }
            Err(err) => {
                tracing::error!("Failed to enqueue SpawnUnit action: {:?}", err);

                Err(err.into())
            }
        }
    }
}
