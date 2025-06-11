use crate::application::action::QueuedAction;

#[derive(thiserror::Error, Debug)]
pub enum PullError {
    #[error("Failed to establish a connection: {0}")]
    ConnectionError(String),

    #[error("An unexpected queue error occurred: {0}")]
    Unexpected(#[from] anyhow::Error),
}

pub type PullResult<T> = Result<T, PullError>;

/// Trait defining the port for an action queue.
#[tonic::async_trait]
pub trait ActionPullable: Send + Sync {
    /// Pulls *all* available new actions for the consumer in batches.
    /// Returns a vector of tuples containing (message_id, QueuedAction).
    async fn pull_all_available_actions(&self) -> PullResult<Vec<QueuedAction>>;

    /// Acknowledges processed messages using XACK.
    async fn acknowledge_actions(&self, ids: Vec<String>) -> PullResult<()>;
}
