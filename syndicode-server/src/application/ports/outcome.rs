use uuid::Uuid;
#[derive(thiserror::Error, Debug)]
pub enum OutcomeError {
    #[error("Failed to establish a connection: {0}")]
    ConnectionError(String),

    #[error("Failed to serialize action with msgpack:: {0}")]
    SerializationError(String),

    #[error("Failed to enqueue action: {0}")]
    EnqueueFailed(String),

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
    /// Define specific error type
    async fn notify_outcome_ready(&self, user_uuid: Uuid, request_uuid: Uuid) -> OutcomeResult<()>;
}
