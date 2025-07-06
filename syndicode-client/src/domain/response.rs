use bon::Builder;
use time::OffsetDateTime;

#[derive(Builder, Debug, Clone)]
pub struct DomainResponse {
    pub code: String,
    pub message: String,
    pub timestamp: OffsetDateTime,
    pub response_type: ResponseType,
}

impl<T> From<anyhow::Result<T>> for DomainResponse
where
    T: std::fmt::Debug,
{
    fn from(value: anyhow::Result<T>) -> Self {
        let code = match value.is_ok() {
            true => "OK".to_string(),
            false => "ERR".to_string(),
        };
        DomainResponse::builder()
            .response_type(value.is_ok().into())
            .code(code)
            .message(format!("{value:#?}"))
            .timestamp(OffsetDateTime::now_utc())
            .build()
    }
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
