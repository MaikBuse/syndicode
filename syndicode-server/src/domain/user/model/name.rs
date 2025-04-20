use std::{fmt::Display, ops::Deref};

use crate::application::error::{ApplicationError, ApplicationResult};

const MINIMAL_LENGTH: usize = 3;
const MAXIMUM_LENGTH: usize = 20;

#[derive(Clone, Debug, PartialEq)]
pub struct UserName(String);

impl UserName {
    pub fn new(name: String) -> ApplicationResult<Self> {
        let name_len = name.len();

        if name_len < MINIMAL_LENGTH {
            return Err(ApplicationError::UserNameTooShort(MINIMAL_LENGTH));
        }

        if name_len > MAXIMUM_LENGTH {
            return Err(ApplicationError::UserNameTooLong(MAXIMUM_LENGTH));
        }

        Ok(Self(name))
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl Display for UserName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for UserName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Deref for UserName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
