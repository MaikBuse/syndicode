use std::fmt::Display;

#[derive(Debug, thiserror::Error)]
pub enum LimitationError {
    #[error("Internal limitation infrastructure error: {0}")]
    Internal(String),

    #[error("Rate limit exceeded")]
    RateExhausted,
}

#[derive(Clone, Copy)]
pub enum LimiterCategory {
    Middleware,
    GameStream,
    Auth,
    Admin,
}

impl Display for LimiterCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LimiterCategory::Middleware => write!(f, "middleware"),
            LimiterCategory::GameStream => write!(f, "game_stream"),
            LimiterCategory::Auth => write!(f, "auth"),
            LimiterCategory::Admin => write!(f, "admin"),
        }
    }
}

pub type LimitationResult<T> = std::result::Result<T, LimitationError>;

#[tonic::async_trait]
pub trait RateLimitEnforcer: Send + Sync {
    async fn check(&self, category: LimiterCategory, ip_address: &str) -> LimitationResult<()>;
}
