#[derive(Debug, thiserror::Error)]
pub enum LimitationError {
    #[error("Internal limitation infrastructure error: {0}")]
    Internal(String),

    #[error("Rate limit exceeded")]
    RateExhausted,
}

pub type LimitationResult<T> = std::result::Result<T, LimitationError>;

#[tonic::async_trait]
pub trait RateLimitationEnforcer: Send + Sync {
    async fn check(&self, ip_address: &str) -> LimitationResult<()>;
}
