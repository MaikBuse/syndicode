use crate::application::error::{ApplicationError, ApplicationResult};
use email_address::*;

#[derive(Clone, Debug, PartialEq)]
pub struct UserEmail(String);

impl UserEmail {
    pub fn new(email: String) -> ApplicationResult<Self> {
        if !EmailAddress::is_valid(&email) {
            return Err(ApplicationError::EmailInvalid(email));
        }

        Ok(Self(email))
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl From<String> for UserEmail {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_validate_email() {
        let email = "test@syndicode.com".to_string();

        let result = UserEmail::new(email.clone());

        assert!(result.is_ok());

        let user_name = result.unwrap();

        assert_eq!(email, user_name.into_inner());
    }

    #[test]
    fn should_not_validate_email() {
        let email = "syndicode.com@".to_string();

        let result = UserEmail::new(email.clone());

        assert!(result.is_err())
    }
}
