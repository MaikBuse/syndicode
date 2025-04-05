use super::ports::limiter::LimitationError;
use crate::domain::repository::RepositoryError;

pub type ApplicationResult<T> = std::result::Result<T, ApplicationError>;

#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error("The database returned with a violation of a unique/primary key constraint")]
    UniqueConstraint,

    #[error("The provided password can't be longer than {0} characters")]
    PasswordTooLong(usize),

    #[error("The provided password needs to have at least {0} characters")]
    PasswordTooShort(usize),

    #[error("The provided username '' is invalid")]
    UsernameInvalid,

    #[error("The requesting user is not authorized to perform this action")]
    Unauthorized,

    #[error("User authentication is missing")]
    MissingAuthentication,

    #[error("The provided credentials are wrong")]
    WrongUserCredentials,

    #[error(transparent)]
    Limitation(#[from] LimitationError),

    #[error(transparent)]
    Database(#[from] RepositoryError),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
