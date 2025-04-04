use super::middleware::USER_UUID_KEY;
use crate::application::error::ApplicationError;
use anyhow::Result;
use tonic::{metadata::MetadataMap, Code, Status};
use uuid::Uuid;

pub(crate) fn parse_uuid(uuid_str: &str) -> Result<Uuid, Status> {
    match Uuid::parse_str(uuid_str) {
        Ok(uuid) => Ok(uuid),
        Err(err) => Err(Status::new(
            Code::InvalidArgument,
            format!("Failed to parse uuid from string: {}", err),
        )),
    }
}

pub(super) fn application_error_into_status(err: ApplicationError) -> Status {
    match err {
        ApplicationError::PasswordTooShort(_)
        | ApplicationError::PasswordTooLong(_)
        | ApplicationError::UsernameInvalid
        | ApplicationError::UniqueConstraint => Status::invalid_argument(err.to_string()),
        ApplicationError::WrongUserCredentials | ApplicationError::MissingAuthentication => {
            Status::unauthenticated(err.to_string())
        }
        ApplicationError::Unauthorized => Status::permission_denied(err.to_string()),
        ApplicationError::Limitation(_)
        | ApplicationError::Database(_)
        | ApplicationError::Other(_) => Status::internal(err.to_string()),
    }
}

pub(super) fn uuid_from_metadata(metadata: &MetadataMap) -> Result<Uuid, Status> {
    let Some(uuid_metadata) = metadata.get(USER_UUID_KEY) else {
        return Err(Status::unauthenticated("Failed to retrieve user id"));
    };

    let Ok(uuid_str) = uuid_metadata.to_str() else {
        return Err(Status::internal("Failed to parse uuid metadata as string"));
    };

    parse_uuid(uuid_str)
}
