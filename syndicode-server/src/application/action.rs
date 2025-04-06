use super::ports::queue::ActionQueuer;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub enum QueuedAction {
    SpawnUnit { req_user_uuid: Uuid },
}

pub struct ActionHandler<Q>
where
    Q: ActionQueuer,
{
    action_queue: Arc<Q>,
}

impl<Q> ActionHandler<Q>
where
    Q: ActionQueuer,
{
    pub fn new(action_queue: Arc<Q>) -> Self {
        Self { action_queue }
    }

    pub async fn submit_action(&self, action: QueuedAction) -> anyhow::Result<()> {
        // ... 1. Perform initial validation ...
        // if validation_fails { return Err(...) }

        match self.action_queue.enqueue_action(action).await {
            Ok(entry_id) => {
                // Log success, maybe include entry_id
                tracing::info!("Successfully enqueued action with ID: {}", entry_id);
                Ok(()) // Signal success to Interface layer to send Ack
            }
            Err(err) => {
                // Log the error
                tracing::error!("Failed to enqueue action: {:?}", err);

                Err(anyhow::format_err!(err))
            }
        }
    }
}
