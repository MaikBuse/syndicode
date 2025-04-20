use bon::bon;
use rand::{distr::Alphanumeric, Rng};
use time::{Duration, OffsetDateTime};

const VERIFICATION_CODE_LENGTH: usize = 10;

#[derive(Debug, Clone)]
pub struct VerificationCode {
    code: String,
    expires_at: OffsetDateTime,
    created_at: OffsetDateTime,
}

#[bon]
impl VerificationCode {
    pub fn new() -> Self {
        let code: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(VERIFICATION_CODE_LENGTH)
            .map(char::from)
            .collect();

        let created_at = OffsetDateTime::now_utc();

        let expires_at = OffsetDateTime::now_utc() + Duration::minutes(30);

        Self {
            code,
            expires_at,
            created_at,
        }
    }

    #[builder]
    pub fn from_input(
        code: String,
        expires_at: OffsetDateTime,
        created_at: OffsetDateTime,
    ) -> Self {
        Self {
            code,
            expires_at,
            created_at,
        }
    }

    pub fn get_code(&self) -> &str {
        &self.code
    }

    pub fn get_expires_at(&self) -> OffsetDateTime {
        self.expires_at
    }

    pub fn get_created_at(&self) -> OffsetDateTime {
        self.created_at
    }

    pub fn is_expired(&self) -> bool {
        let now = OffsetDateTime::now_utc();

        now > self.expires_at
    }

    /// Returns true if the provided code is correct
    pub fn is_code_correct(&self, code: &str) -> bool {
        self.code.as_str() == code
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_not_be_expired() {
        let verification_code = VerificationCode::new();

        assert!(!verification_code.is_expired())
    }

    #[test]
    fn should_be_expired() {
        let yesterday = OffsetDateTime::now_utc() - Duration::hours(24);

        let verification_code = VerificationCode {
            code: "code".to_string(),
            expires_at: yesterday,
            created_at: yesterday,
        };

        assert!(verification_code.is_expired())
    }
}
