use crate::application::action::QueuedAction;

#[derive(thiserror::Error, Debug)]
pub enum QueueError {
    #[error("Failed to establish a connection: {0}")]
    ConnectionError(String),

    #[error("Failed to enqueue action: {0}")]
    EnqueueFailed(String),

    #[error("An unexpected queue error occurred: {0}")]
    Unexpected(#[from] anyhow::Error),
}

pub type QueueResult<T> = Result<T, QueueError>;

/// Trait defining the port for an action queue.
#[tonic::async_trait]
pub trait ActionQueuer: Send + Sync + 'static {
    /// Enqueues a serialized action payload onto the appropriate stream/queue.
    async fn enqueue_action(&self, action: QueuedAction) -> QueueResult<String>;

    async fn pull_actions(&self, count: usize) -> QueueResult<Vec<QueuedAction>>;

    async fn acknowledge_actions(
        &self,
        ids: &[&str], // Slice of message IDs to acknowledge
    ) -> QueueResult<()>;
}
