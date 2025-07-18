#[cfg(test)]
use mockall::{automock, predicate::*};

use crate::application::action::QueuedActionPayload;

#[derive(thiserror::Error, Debug)]
pub enum QueueError {
    #[error("Failed to serialize action with msgpack:: {0}")]
    SerializationError(String),

    #[error("Failed to enqueue action: {0}")]
    EnqueueFailed(String),

    #[error("An unexpected queue error occurred: {0}")]
    Unexpected(#[from] anyhow::Error),
}

pub type QueueResult<T> = Result<T, QueueError>;

/// Trait defining the port for an action queue.
#[cfg_attr(test, automock)]
#[tonic::async_trait]
pub trait ActionQueueable: Send + Sync {
    async fn enqueue_action(&self, action: QueuedActionPayload) -> QueueResult<String>;
}
