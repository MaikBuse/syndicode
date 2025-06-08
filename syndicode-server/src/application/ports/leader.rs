#[cfg(test)]
use mockall::{automock, predicate::*};

#[derive(thiserror::Error, Clone, Debug)]
pub enum LeaderElectionError {
    #[error("Redis command failed: {0}")]
    RedisCommandError(String),

    #[error("Failed to acquire lock for key '{key}': {details}")]
    LockAcquireFailed { key: String, details: String },

    #[error("Failed to release lock for key '{key}': {details}")]
    LockReleaseFailed { key: String, details: String },

    #[error("Failed to refresh lock for key '{key}': {details}")]
    LockRefreshFailed { key: String, details: String },

    #[error("Cannot release/refresh lock for key '{key}' as it is not held by this instance ('{instance_id}') or has expired.")]
    NotHoldingLock { key: String, instance_id: String },
}

pub type LeaderElectionResult<T> = Result<T, LeaderElectionError>;

#[cfg_attr(test, automock)]
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
