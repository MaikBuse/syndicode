use std::sync::Arc;

use super::middleware::USER_UUID_KEY;
use crate::application::{
    error::ApplicationError,
    ports::limiter::{LimitationError, LimiterCategory, RateLimitEnforcer},
};
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
        | ApplicationError::CorporationNameTooShort(_)
        | ApplicationError::CorporationNameTooLong(_)
        | ApplicationError::UsernameInvalid
        | ApplicationError::UniqueConstraint => Status::invalid_argument(err.to_string()),
        ApplicationError::WrongUserCredentials | ApplicationError::MissingAuthentication => {
            Status::unauthenticated(err.to_string())
        }
        ApplicationError::Unauthorized => Status::permission_denied(err.to_string()),
        ApplicationError::Limitation(_)
        | ApplicationError::Database(_)
        | ApplicationError::Queue(_)
        | ApplicationError::Other(_) => Status::internal(err.to_string()),
    }
}

pub(super) fn limitation_error_into_status(err: LimitationError) -> Status {
    match err {
        LimitationError::RateExhausted => Status::resource_exhausted("Rate limit exceeded"),
        LimitationError::Internal(msg) => {
            tracing::error!("Rate limiter internal error: {}", msg);
            Status::internal("Rate limiter error")
        }
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

pub(super) fn ip_address_from_metadata(
    metadata: &MetadataMap,
    ip_address_header: &str,
) -> Result<String, Status> {
    metadata
        .get(ip_address_header)
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_owned())
        .ok_or_else(|| {
            tracing::warn!("Failed to retrieve client IP address from metadata");
            Status::invalid_argument("Missing required client identification (IP)")
        })
}

pub(super) async fn check_rate_limit<R>(
    limit: Arc<R>,
    metadata: &MetadataMap,
    ip_address_header: &str,
    category: LimiterCategory,
) -> Result<(), Status>
where
    R: RateLimitEnforcer,
{
    let ip_address = ip_address_from_metadata(metadata, ip_address_header)?;

    if let Err(err) = limit.check(category, &ip_address).await {
        return Err(limitation_error_into_status(err));
    }

    Ok(())
}
