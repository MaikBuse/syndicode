use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum ResultError {
    #[error("Failed to establish a connection: {0}")]
    ConnectionError(String),

    #[error("Failed to serialize action with msgpack:: {0}")]
    SerializationError(String),

    #[error("Failed to enqueue action: {0}")]
    EnqueueFailed(String),

    #[error("An unexpected queue error occurred: {0}")]
    Unexpected(#[from] anyhow::Error),
}

pub type ResultResult<T> = Result<T, ResultError>;

#[tonic::async_trait]
pub trait ResultStoreWriter: Send + Sync {
    async fn store_result(&self, request_uuid: Uuid, payload: &[u8]) -> ResultResult<()>;
}

#[tonic::async_trait]
pub trait ResultStoreReader: Send + Sync {
    /// Option if TTL expired / not found
    async fn retrieve_result(&self, request_uuid: Uuid) -> ResultResult<Option<Vec<u8>>>;

    /// Optional cleanup
    async fn delete_result(&self, request_uuid: Uuid) -> ResultResult<()>;
}

#[tonic::async_trait]
pub trait ResultNotifier: Send + Sync {
    /// Define specific error type
    async fn notify_result_ready(&self, user_uuid: Uuid, request_uuid: Uuid) -> ResultResult<()>;
}
