#[cfg(test)]
use mockall::{automock, predicate::*};

use tokio::io::AsyncRead;
use tonic::async_trait;

#[derive(thiserror::Error, Debug)]
pub enum RestoreError {
    #[error("Restore command failed: {stdout}, {stderr}")]
    CommandFailed { stdout: String, stderr: String },

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Could not find the '{0}' command. Is it installed and in your PATH?")]
    CommandNotFound(String),
}

pub type RestoreResult<T> = Result<T, RestoreError>;

/// Readable stream of data to perform a database restore.
#[cfg_attr(test, automock)]
#[async_trait]
pub trait DatabaseRestorer: Send + Sync {
    async fn restore(&self, data_stream: Box<dyn AsyncRead + Unpin + Send>) -> RestoreResult<()>;
}
