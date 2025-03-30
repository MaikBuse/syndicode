use crate::domain::repository::{
    control::ControlDatabaseError, economy::EconomyDatabaseError, warfare::WarfareDatabaseError,
};

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("The provided password needs to have at least 8 characters")]
    PasswordTooShort,

    #[error("The provided username '' is invalid")]
    UsernameInvalid,

    #[error("The requesting user is not authorized to perform this action")]
    Unauthorized,

    #[error("User authentication is missing")]
    MissingAuthentication,

    #[error("The provided credentials are wrong")]
    WrongUserCredentials,

    #[error(transparent)]
    ControlDatabase(#[from] ControlDatabaseError),

    #[error(transparent)]
    EconomyDatabase(#[from] EconomyDatabaseError),

    #[error(transparent)]
    WarfareDatabase(#[from] WarfareDatabaseError),

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type ServiceResult<T> = std::result::Result<T, ServiceError>;
