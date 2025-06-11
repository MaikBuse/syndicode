use super::middleware::USER_UUID_KEY;
use crate::application::ports::limiter::{LimitationError, LimiterCategory, RateLimitEnforcer};
use anyhow::Result;
use std::{str::FromStr, sync::Arc};
use tonic::{metadata::MetadataMap, Code, Status};
use uuid::Uuid;

pub(crate) fn parse_uuid(uuid_str: &str) -> Result<Uuid, Box<Status>> {
    match Uuid::parse_str(uuid_str) {
        Ok(uuid) => Ok(uuid),
        Err(err) => Err(Box::new(Status::new(
            Code::InvalidArgument,
            format!("Failed to parse uuid from string: {}", err),
        ))),
    }
}

pub fn parse_maybe_uuid(
    maybe_uuid: Option<String>,
    context: &str,
) -> Result<Option<Uuid>, Box<Status>> {
    if let Some(uuid_string) = maybe_uuid {
        let Ok(uuid) = Uuid::from_str(uuid_string.as_str()) else {
            return Err(Box::new(Status::invalid_argument(format!(
                "Failed to parse {} as uuid",
                context
            ))));
        };

        return Ok(Some(uuid));
    }

    Ok(None)
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

pub(super) fn uuid_from_metadata(metadata: &MetadataMap) -> Result<Uuid, Box<Status>> {
    let Some(uuid_metadata) = metadata.get(USER_UUID_KEY) else {
        return Err(Box::new(Status::unauthenticated(
            "Failed to retrieve user id",
        )));
    };

    let Ok(uuid_str) = uuid_metadata.to_str() else {
        return Err(Box::new(Status::internal(
            "Failed to parse uuid metadata as string",
        )));
    };

    parse_uuid(uuid_str)
}

pub(super) fn ip_address_from_metadata(
    metadata: &MetadataMap,
    ip_address_header: &str,
) -> Result<String, Box<Status>> {
    metadata
        .get(ip_address_header)
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_owned())
        .ok_or_else(|| {
            tracing::warn!("Failed to retrieve client IP address from metadata");
            Box::new(Status::invalid_argument(
                "Missing required client identification (IP)",
            ))
        })
}

pub(super) async fn check_rate_limit<R>(
    limit: Arc<R>,
    metadata: &MetadataMap,
    ip_address_header: &str,
    category: LimiterCategory,
) -> Result<(), Box<Status>>
where
    R: RateLimitEnforcer,
{
    let ip_address = ip_address_from_metadata(metadata, ip_address_header)?;

    if let Err(err) = limit.check(category, &ip_address).await {
        return Err(Box::new(limitation_error_into_status(err)));
    }

    Ok(())
}
