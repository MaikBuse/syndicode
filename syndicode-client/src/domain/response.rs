use bon::Builder;
use time::OffsetDateTime;

#[derive(Builder, Debug, Clone)]
pub struct DomainResponse {
    pub code: String,
    pub message: String,
    pub timestamp: OffsetDateTime,
    pub response_type: ResponseType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResponseType {
    Success,
    Error,
    Info,
    GameTickeNotification,
}

impl From<bool> for ResponseType {
    fn from(value: bool) -> Self {
        match value {
            true => Self::Success,
            false => Self::Error,
        }
    }
}
