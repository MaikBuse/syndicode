#[cfg(test)]
use mockall::{automock, predicate::*};

use super::{outcome::OutcomeError, puller::PullError};
use crate::{application::error::ApplicationError, domain::repository::RepositoryError};

#[derive(thiserror::Error, Debug)]
pub enum ProcessorError {
    #[error("Database not initialized")]
    NotInitialized,

    #[error(transparent)]
    Outcome(#[from] OutcomeError),

    #[error(transparent)]
    Pull(#[from] PullError),

    #[error(transparent)]
    Application(#[from] ApplicationError),

    #[error(transparent)]
    Repository(#[from] RepositoryError),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type ProcessorResult<T> = std::result::Result<T, ProcessorError>;

#[cfg_attr(test, automock)]
#[tonic::async_trait]
pub trait GameTickProcessable: Send + Sync {
    async fn process_next_tick(&self) -> ProcessorResult<i64>;
}
