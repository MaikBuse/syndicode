use std::fmt::Display;

use syndicode_proto::syndicode_interface_v1::{
    game_update::Update, ActionFailedResponse, GameUpdate,
};
use tonic::Status;

use crate::application::error::ApplicationError;

#[allow(dead_code)]
#[derive(Debug)]
pub(super) enum PresentationError {
    /// The operation was cancelled.
    Cancelled,

    /// Unknown error.
    Unknown,

    /// Client specified an invalid argument.
    InvalidArgument(String),

    /// Deadline expired before operation could complete.
    DeadlineExceeded,

    /// Some requested entity was not found.
    NotFound,

    /// Some entity that we attempted to create already exists.
    AlreadyExists,

    /// The caller does not have permission to execute the specified operation.
    PermissionDenied,

    /// Some resource has been exhausted.
    ResourceExhausted,

    /// The system is not in a state required for the operation's execution.
    FailedPrecondition,

    /// The operation was aborted.
    Aborted,

    /// Operation was attempted past the valid range.
    OutOfRange,

    /// Operation is not implemented or not supported.
    Unimplemented,

    /// Internal error.
    Internal,

    /// The service is currently unavailable.
    Unavailable,

    /// Unrecoverable data loss or corruption.
    DataLoss,

    /// The request does not have valid authentication credentials
    Unauthenticated,
}

impl From<ApplicationError> for PresentationError {
    fn from(err: ApplicationError) -> Self {
        match err {
            ApplicationError::CorporationForUserNotFound | ApplicationError::UserNotPending => {
                Self::FailedPrecondition
            }
            ApplicationError::VerificationCodeExpired => Self::DeadlineExceeded,
            ApplicationError::PasswordTooShort(_)
            | ApplicationError::PasswordTooLong(_)
            | ApplicationError::EmailInvalid(_)
            | ApplicationError::UserNameTooLong(_)
            | ApplicationError::UserNameTooShort(_)
            | ApplicationError::CorporationNameAlreadyTaken
            | ApplicationError::CorporationNameTooShort(_)
            | ApplicationError::CorporationNameTooLong(_)
            | ApplicationError::VerificationCodeFalse
            | ApplicationError::UniqueConstraint => Self::InvalidArgument(err.to_string()),
            ApplicationError::WrongUserCredentials | ApplicationError::UserInactive => {
                Self::Unauthenticated
            }
            ApplicationError::Unauthorized => Self::PermissionDenied,
            ApplicationError::Limitation(_)
            | ApplicationError::Database(_)
            | ApplicationError::Queue(_)
            | ApplicationError::Pull(_)
            | ApplicationError::VerificationSendable(_)
            | ApplicationError::Sqlx(_)
            | ApplicationError::Other(_) => Self::Internal,
        }
    }
}

impl Display for PresentationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cancelled => write!(f, "The operation was cancelled"),
            Self::Unknown => write!(f, "Unknown error"),
            Self::InvalidArgument(msg) => {
                write!(f, "Client specified an invalid argument: {}", msg)
            }
            Self::DeadlineExceeded => write!(f, "Deadline expired before operation could complete"),
            Self::NotFound => write!(f, "Some requested entity was not found"),
            Self::AlreadyExists => {
                write!(f, "Some entity that we attempted to create already exists")
            }
            Self::PermissionDenied => {
                write!(
                    f,
                    "The caller does not have permission to execute the specified operation"
                )
            }
            Self::ResourceExhausted => write!(f, "Some resource has been exhausted"),
            Self::FailedPrecondition => {
                write!(
                    f,
                    "The system is not in a state required for the operation's execution"
                )
            }
            Self::Aborted => write!(f, "The operation was aborted"),
            Self::OutOfRange => write!(f, "Operation was attempted past the valid range"),
            Self::Unimplemented => write!(f, "Operation is not implemented or not supported"),
            Self::Internal => write!(f, "Internal error"),
            Self::Unavailable => write!(f, "The service is currently unavailable"),
            Self::DataLoss => write!(f, "Unrecoverable data loss or corruption"),
            Self::Unauthenticated => write!(
                f,
                "The request does not have valid authentication credentials"
            ),
        }
    }
}

impl From<PresentationError> for Status {
    fn from(value: PresentationError) -> Self {
        match value {
            PresentationError::Cancelled => Self::cancelled(String::new()),
            PresentationError::Unknown => Self::unknown(String::new()),
            PresentationError::InvalidArgument(msg) => Self::invalid_argument(msg),
            PresentationError::DeadlineExceeded => Self::deadline_exceeded(String::new()),
            PresentationError::NotFound => Self::not_found(String::new()),
            PresentationError::AlreadyExists => Self::already_exists(String::new()),
            PresentationError::PermissionDenied => Self::permission_denied(String::new()),
            PresentationError::ResourceExhausted => Self::resource_exhausted(String::new()),
            PresentationError::FailedPrecondition => Self::failed_precondition(String::new()),
            PresentationError::Aborted => Self::aborted(String::new()),
            PresentationError::OutOfRange => Self::out_of_range(String::new()),
            PresentationError::Unimplemented => Self::unimplemented(String::new()),
            PresentationError::Internal => Self::internal(String::new()),
            PresentationError::Unavailable => Self::unavailable(String::new()),
            PresentationError::DataLoss => Self::data_loss(String::new()),
            PresentationError::Unauthenticated => Self::unauthenticated(String::new()),
        }
    }
}

impl PresentationError {
    pub(super) fn into_game_update(self, game_tick: i64, request_uuid: String) -> GameUpdate {
        GameUpdate {
            game_tick,
            update: Some(Update::ActionFailedResponse(ActionFailedResponse {
                request_uuid,
                reason: self.to_string(),
            })),
        }
    }
}
