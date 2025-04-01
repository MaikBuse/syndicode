use crate::infrastructure::postgres::DatabaseError;

#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
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
    Database(#[from] DatabaseError),

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type ApplicationResult<T> = std::result::Result<T, ApplicationError>;
