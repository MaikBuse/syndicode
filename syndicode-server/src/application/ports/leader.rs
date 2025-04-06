#[derive(thiserror::Error, Debug)]
pub enum LeaderElectionError {
    #[error("Failed to configure Redis client: {0}")]
    ConfigurationError(String),

    #[error("Redis command failed: {0}")]
    RedisCommandError(#[from] redis::RedisError),

    #[error("Failed to acquire lock for key '{key}': {details}")]
    LockAcquireFailed { key: String, details: String },

    #[error("Failed to release lock for key '{key}': {details}")]
    LockReleaseFailed { key: String, details: String },

    #[error("Failed to refresh lock for key '{key}': {details}")]
    LockRefreshFailed { key: String, details: String },

    #[error("Cannot release/refresh lock for key '{key}' as it is not held by this instance ('{instance_id}') or has expired.")]
    NotHoldingLock { key: String, instance_id: String },

    #[error("An unexpected error occurred: {0}")]
    Unexpected(#[from] anyhow::Error),
}

pub type LeaderElectionResult<T> = Result<T, LeaderElectionError>;

#[tonic::async_trait]
pub trait LeaderElector: Send + Sync + 'static {
    /// Attempts to acquire the leader lock.
    /// This operation should be atomic and respect the configured TTL.
    async fn try_acquire(&self) -> LeaderElectionResult<bool>;

    /// Releases the leader lock, but *only* if it is still held by this instance.
    /// This prevents accidentally releasing a lock acquired by another instance
    /// after this instance's lock expired or it crashed.
    async fn release(&self) -> LeaderElectionResult<()>;

    /// Refreshes the TTL of the leader lock, but *only* if it is still held by this instance.
    /// This should be called periodically by the active leader to prevent the lock
    /// from expiring while it's still working.
    async fn refresh(&self) -> LeaderElectionResult<()>;
}
