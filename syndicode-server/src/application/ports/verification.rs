#[cfg(test)]
use mockall::{automock, predicate::*};

use crate::domain::user_verify::model::code::VerificationCode;

#[derive(thiserror::Error, Clone, Debug)]
pub enum VerificationSendableError {
    #[error("Failed to send email via SMTP transport: {0}")]
    SendEmail(String),

    #[error("Failed to initialize SMTP relay: {0}")]
    InitSMTP(String),

    #[error("Failed to build email message: {0}")]
    BuildEmail(String),

    #[error("Failed to parse recipient email address: {0}")]
    ParseRecipient(String),
}

pub type VerificationSendableResult<T> = Result<T, VerificationSendableError>;

#[cfg_attr(test, automock)]
#[tonic::async_trait]
pub trait VerificationSendable: Send + Sync {
    async fn send_verification_email(
        &self,
        recipient_email: String,
        recipient_name: String,
        verification_code: VerificationCode,
    ) -> VerificationSendableResult<()>;
}
