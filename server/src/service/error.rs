use crate::domain::repository::{
    control::ControlDatabaseError, economy::EconomyDatabaseError, warfare::WarfareDatabaseError,
};

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("The game is already running")]
    SessionAlreadyRunning,

    #[error("The game is not running")]
    SessionNotRunning,

    #[error("The game is already initialized")]
    SessionAlreadyInitialized,

    #[error("Failed to turn slice of bytes into Uuid")]
    UuidFromSlice,

    #[error("The provided credentials are wrong")]
    WrongUserCredentials,

    #[error(transparent)]
    ControlDatabase(#[from] ControlDatabaseError),

    #[error(transparent)]
    EconomyDatabase(#[from] EconomyDatabaseError),

    #[error(transparent)]
    WarfareDatabase(#[from] WarfareDatabaseError),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type ServiceResult<T> = std::result::Result<T, ServiceError>;
