use crate::application::action::QueuedActionPayload;

#[derive(thiserror::Error, Debug)]
pub enum PullError {
    #[error("Failed to establish a connection: {0}")]
    ConnectionError(String),

    #[error("Failed to serialize action with msgpack:: {0}")]
    SerializationError(String),

    #[error("Failed to enqueue action: {0}")]
    EnqueueFailed(String),

    #[error("An unexpected queue error occurred: {0}")]
    Unexpected(#[from] anyhow::Error),
}

pub type PullResult<T> = Result<T, PullError>;

/// Trait defining the port for an action queue.
#[tonic::async_trait]
pub trait ActionPullable: Send + Sync {
    /// Pulls up to `count` new actions for the consumer.
    /// Returns a vector of tuples containing (message_id, QueuedAction).
    async fn pull_actions(&self, count: usize) -> PullResult<Vec<(String, QueuedActionPayload)>>;

    /// Pulls *all* available new actions for the consumer in batches.
    /// Returns a vector of tuples containing (message_id, QueuedAction).
    async fn pull_all_available_actions(&self) -> PullResult<Vec<(String, QueuedActionPayload)>>;

    /// Acknowledges processed messages using XACK.
    async fn acknowledge_actions(&self, ids: Vec<String>) -> PullResult<()>;
}
