use uuid::Uuid;
#[derive(thiserror::Error, Debug)]
pub enum OutcomeError {
    #[error("Failed to enqueue outcome: {0}")]
    EnqueueFailed(String),

    #[error("Failed to dequeue outcome: {0}")]
    DequeueFailed(String),

    #[error("Failed to delete outcome: {0}")]
    DeletionFailed(String),

    #[error("Failed to publish the readiness of an outcome: {0}")]
    PublishingOutcomeFailed(String),

    #[error("Failed to publish the progression of the game tick: {0}")]
    PublishingGametickFailed(String),

    #[error("An unexpected queue error occurred: {0}")]
    Unexpected(#[from] anyhow::Error),
}

pub type OutcomeResult<T> = Result<T, OutcomeError>;

#[tonic::async_trait]
pub trait OutcomeStoreWriter: Send + Sync {
    async fn store_outcome(&self, request_uuid: Uuid, payload: &[u8]) -> OutcomeResult<()>;
}

#[tonic::async_trait]
pub trait OutcomeStoreReader: Send + Sync {
    /// Option if TTL expired / not found
    async fn retrieve_outcome(&self, request_uuid: Uuid) -> OutcomeResult<Option<Vec<u8>>>;

    /// Optional cleanup
    async fn delete_outcome(&self, request_uuid: Uuid) -> OutcomeResult<()>;
}

#[tonic::async_trait]
pub trait OutcomeNotifier: Send + Sync {
    async fn notify_outcome_ready(&self, user_uuid: Uuid, request_uuid: Uuid) -> OutcomeResult<()>;

    async fn notify_game_tick_advanced(&self, game_tick: i64) -> OutcomeResult<()>;
}
