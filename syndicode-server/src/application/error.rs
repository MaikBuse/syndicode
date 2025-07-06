use super::ports::{
    downloader::DownloadError, limiter::LimitationError, puller::PullError, queuer::QueueError,
    restorer::RestoreError, verification::VerificationSendableError,
};
use crate::domain::repository::RepositoryError;

pub type ApplicationResult<T> = std::result::Result<T, ApplicationError>;

#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error("The provided corporation name has already been taken")]
    CorporationNameAlreadyTaken,

    #[error("The provided user name has already been taken")]
    UserNameAlreadyTaken,

    #[error("The provided email is already in use")]
    EmailInUse,

    #[error("Failed to retrieve the corporation of the provided user")]
    CorporationForUserNotFound,

    #[error("The provided corporation name can't be longer than {0} characters")]
    CorporationNameTooLong(usize),

    #[error("The provided corporation name needs to have at least {0} characters")]
    CorporationNameTooShort(usize),

    #[error("Please activate the user by providing the activation code")]
    UserInactive,

    #[error("The provided user is not in the pending state")]
    UserNotPending,

    #[error("The provided username can't be longer than {0} characters")]
    UserNameTooLong(usize),

    #[error("The provided username needs to have at least {0} characters")]
    UserNameTooShort(usize),

    #[error("The provided email '{0}' is invalid")]
    EmailInvalid(String),

    #[error("The provided password can't be longer than {0} characters")]
    PasswordTooLong(usize),

    #[error("The provided password needs to have at least {0} characters")]
    PasswordTooShort(usize),

    #[error("The requesting user is not authorized to perform this action")]
    Unauthorized,

    #[error("The provided credentials are wrong")]
    WrongUserCredentials,

    #[error("The verification code has expired")]
    VerificationCodeExpired,

    #[error("The provided verification code is false")]
    VerificationCodeFalse,

    #[error(transparent)]
    Queue(#[from] QueueError),

    #[error(transparent)]
    VerificationSendable(#[from] VerificationSendableError),

    #[error(transparent)]
    Pull(#[from] PullError),

    #[error(transparent)]
    Limitation(#[from] LimitationError),

    #[error(transparent)]
    Restore(#[from] RestoreError),

    #[error(transparent)]
    Download(#[from] DownloadError),

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl From<RepositoryError> for ApplicationError {
    fn from(value: RepositoryError) -> Self {
        match value {
            RepositoryError::UserNameAlreadyTaken => ApplicationError::UserNameAlreadyTaken,
            RepositoryError::EmailInUse => ApplicationError::EmailInUse,
            _ => ApplicationError::from(value),
        }
    }
}
