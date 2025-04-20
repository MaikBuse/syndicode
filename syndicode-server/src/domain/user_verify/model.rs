pub mod code;

use bon::Builder;
use code::VerificationCode;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Builder)]
pub struct UserVerification {
    user_uuid: Uuid,
    code: VerificationCode,
}

impl UserVerification {
    pub fn new(user_uuid: Uuid) -> Self {
        Self {
            user_uuid,
            code: VerificationCode::new(),
        }
    }

    pub fn get_user_uuid(&self) -> Uuid {
        self.user_uuid
    }

    pub fn into_code(self) -> VerificationCode {
        self.code
    }

    pub fn clone_code(&self) -> VerificationCode {
        self.code.clone()
    }

    pub fn get_code(&self) -> &str {
        self.code.get_code()
    }

    pub fn get_expires_at(&self) -> OffsetDateTime {
        self.code.get_expires_at()
    }

    pub fn get_created_at(&self) -> OffsetDateTime {
        self.code.get_created_at()
    }

    pub fn is_expired(&self) -> bool {
        self.code.is_expired()
    }

    pub fn is_code_correct(&self, code: &str) -> bool {
        self.code.is_code_correct(code)
    }
}
