use std::fmt::Display;

use syndicode_proto::syndicode_interface_v1::{
    game_update::Update, ActionFailedResponse, GameUpdate,
};
use tonic::{Code, Status};
use tonic_types::{ErrorDetails, StatusExt};

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
    AlreadyExists { field: String, description: String },

    /// The caller does not have permission to execute the specified operation.
    PermissionDenied,

    /// Some resource has been exhausted.
    ResourceExhausted(String),

    /// The system is not in a state required for the operation's execution.
    FailedPrecondition(String),

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
            ApplicationError::CorporationForUserNotFound => Self::Unknown,
            ApplicationError::VerificationCodeExpired => Self::DeadlineExceeded,
            ApplicationError::PasswordTooShort(_)
            | ApplicationError::PasswordTooLong(_)
            | ApplicationError::EmailInvalid(_)
            | ApplicationError::UserNameTooLong(_)
            | ApplicationError::UserNameTooShort(_)
            | ApplicationError::CorporationNameTooShort(_)
            | ApplicationError::CorporationNameTooLong(_)
            | ApplicationError::VerificationCodeFalse => Self::InvalidArgument(err.to_string()),
            ApplicationError::UserNameAlreadyTaken => Self::AlreadyExists {
                field: "user_name".to_string(),
                description: err.to_string(),
            },
            ApplicationError::CorporationNameAlreadyTaken => Self::AlreadyExists {
                field: "corporation_name".to_string(),
                description: err.to_string(),
            },
            ApplicationError::EmailInUse => Self::AlreadyExists {
                field: "email".to_string(),
                description: err.to_string(),
            },
            ApplicationError::UserInactive => {
                Self::FailedPrecondition("The user is inactive".to_string())
            }
            ApplicationError::UserNotPending => {
                Self::FailedPrecondition("The user should be in a pending state".to_string())
            }
            ApplicationError::WrongUserCredentials => {
                Self::InvalidArgument("The provided credentials are invalid".to_string())
            }
            ApplicationError::Unauthorized => Self::PermissionDenied,
            ApplicationError::Limitation(err) => Self::ResourceExhausted(err.to_string()),
            ApplicationError::Queue(_)
            | ApplicationError::Download(_)
            | ApplicationError::Restore(_)
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
                write!(f, "Client specified an invalid argument: {msg}")
            }
            Self::DeadlineExceeded => write!(f, "Deadline expired before operation could complete"),
            Self::NotFound => write!(f, "Some requested entity was not found"),
            Self::AlreadyExists { description, .. } => {
                write!(f, "{description}")
            }
            Self::PermissionDenied => {
                write!(
                    f,
                    "The caller does not have permission to execute the specified operation"
                )
            }
            Self::ResourceExhausted(msg) => write!(f, "Some resource has been exhausted: {msg}"),
            Self::FailedPrecondition(msg) => {
                write!(
                    f,
                    "The system is not in a state required for the operation's execution: {msg}"
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
            PresentationError::AlreadyExists { field, description } => {
                let mut err_details = ErrorDetails::new();

                err_details.add_bad_request_violation(field, description.clone());

                Self::with_error_details(Code::AlreadyExists, description, err_details)
            }
            PresentationError::PermissionDenied => Self::permission_denied(String::new()),
            PresentationError::ResourceExhausted(msg) => Self::resource_exhausted(msg),
            PresentationError::FailedPrecondition(msg) => Self::failed_precondition(msg),
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
