use crate::application::action::QueuedAction;

#[derive(thiserror::Error, Debug)]
pub enum QueueError {
    #[error("Failed to establish a connection: {0}")]
    ConnectionError(String),

    #[error("Failed to serialize action with msgpack:: {0}")]
    SerializationError(String),

    #[error("Failed to enqueue action: {0}")]
    EnqueueFailed(String),

    #[error("An unexpected queue error occurred: {0}")]
    Unexpected(#[from] anyhow::Error),
}

pub type QueueResult<T> = Result<T, QueueError>;

/// Trait defining the port for an action queue.
#[tonic::async_trait]
pub trait ActionQueuer: Send + Sync {
    async fn enqueue_action(&self, action: QueuedAction) -> QueueResult<String>;

    /// Pulls up to `count` new actions for the consumer.
    /// Returns a vector of tuples containing (message_id, QueuedAction).
    async fn pull_actions(&self, count: usize) -> QueueResult<Vec<(String, QueuedAction)>>;

    /// Pulls *all* available new actions for the consumer in batches.
    /// Returns a vector of tuples containing (message_id, QueuedAction).
    async fn pull_all_available_actions(&self) -> QueueResult<Vec<(String, QueuedAction)>>;

    /// Acknowledges processed messages using XACK.
    async fn acknowledge_actions(
        &self,
        ids: &[&str], // Changed to slice of owned Strings
    ) -> QueueResult<()>;
}
