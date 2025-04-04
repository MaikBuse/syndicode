use crate::application::error::{ApplicationError, ApplicationResult};
use std::ops::Deref;

const MINIMAL_LENGTH: usize = 8;
const MAXIMUM_LENGTH: usize = 128;

#[derive(Debug, PartialEq)]
pub struct UserPassword(String);

impl UserPassword {
    pub fn new(password: String) -> ApplicationResult<Self> {
        let password_len = password.len();

        if password_len < MINIMAL_LENGTH {
            return Err(ApplicationError::PasswordTooShort(MINIMAL_LENGTH));
        }

        if password_len > MAXIMUM_LENGTH {
            return Err(ApplicationError::PasswordTooLong(MAXIMUM_LENGTH));
        }

        Ok(Self(password))
    }
}

impl Deref for UserPassword {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_user_password() {
        let input = String::from("testpassword");

        let result = UserPassword::new(input.clone());

        assert!(result.is_ok());
        assert_eq!(input, result.unwrap().0);
    }

    #[test]
    fn should_fail_when_password_too_short() {
        let input = String::from("test");

        let result = UserPassword::new(input.clone());

        assert!(result.is_err());
        // Check for the specific error type
        match result.err().unwrap() {
            ApplicationError::PasswordTooShort(_) => (),
            other_err => panic!(
                "Expected ApplicationError::PasswordTooShort, got {:?}",
                other_err
            ),
        }
    }

    #[test]
    fn should_fail_when_password_too_long() {
        let input =
            String::from("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b85582f231898694e893389f7fc7f0d4b2ae1ddfb69ed2d59295603593c8529db673x");

        let result = UserPassword::new(input.clone());

        assert!(result.is_err());
        // Check for the specific error type
        match result.err().unwrap() {
            ApplicationError::PasswordTooLong(_) => (),
            other_err => panic!(
                "Expected ApplicationError::PasswordTooLong, got {:?}",
                other_err
            ),
        }
    }
}
