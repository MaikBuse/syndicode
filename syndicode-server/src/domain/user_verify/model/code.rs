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

    #[test]
    fn should_be_expired_when_exactly_at_expiration_time() {
        // Create a code that expires exactly now (or in the past by a few milliseconds)
        let now = OffsetDateTime::now_utc();
        let past_time = now - Duration::milliseconds(1);

        let verification_code = VerificationCode {
            code: "code".to_string(),
            expires_at: past_time,
            created_at: past_time - Duration::minutes(30),
        };

        assert!(verification_code.is_expired())
    }

    #[test]
    fn should_not_be_expired_when_just_before_expiration() {
        let future_time = OffsetDateTime::now_utc() + Duration::minutes(1);

        let verification_code = VerificationCode {
            code: "code".to_string(),
            expires_at: future_time,
            created_at: OffsetDateTime::now_utc() - Duration::minutes(29),
        };

        assert!(!verification_code.is_expired())
    }

    #[test]
    fn should_correctly_validate_matching_code() {
        let verification_code = VerificationCode::new();
        let stored_code = verification_code.code.clone();

        assert!(verification_code.is_code_correct(&stored_code))
    }

    #[test]
    fn should_reject_incorrect_code() {
        let verification_code = VerificationCode::new();

        assert!(!verification_code.is_code_correct("wrongcode"));
        assert!(!verification_code.is_code_correct(""));
        assert!(!verification_code.is_code_correct("123456789")); // Different length
    }

    #[test]
    fn should_be_case_sensitive_for_code_validation() {
        let verification_code = VerificationCode {
            code: "AbCdEfGhIj".to_string(),
            expires_at: OffsetDateTime::now_utc() + Duration::minutes(30),
            created_at: OffsetDateTime::now_utc(),
        };

        assert!(verification_code.is_code_correct("AbCdEfGhIj"));
        assert!(!verification_code.is_code_correct("abcdefghij"));
        assert!(!verification_code.is_code_correct("ABCDEFGHIJ"));
    }

    #[test]
    fn new_verification_code_should_have_correct_properties() {
        let verification_code = VerificationCode::new();

        // Should have 10-character code
        assert_eq!(verification_code.code.len(), VERIFICATION_CODE_LENGTH);
        
        // Code should be alphanumeric
        assert!(verification_code.code.chars().all(|c| c.is_alphanumeric()));
        
        // Should not be expired when created
        assert!(!verification_code.is_expired());
        
        // Should expire in approximately 30 minutes
        let now = OffsetDateTime::now_utc();
        let expected_expiry = now + Duration::minutes(30);
        let time_diff = (verification_code.expires_at - expected_expiry).abs();
        
        // Allow for small timing differences (up to 1 second)
        assert!(time_diff < Duration::seconds(1));
    }
}
