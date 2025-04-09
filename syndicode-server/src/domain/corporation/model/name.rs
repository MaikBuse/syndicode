use crate::application::error::{ApplicationError, ApplicationResult};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, ops::Deref};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CorporationName(String);

impl CorporationName {
    pub fn new(name: String) -> ApplicationResult<Self> {
        let name_len = name.len();

        if name_len < 4 {
            return Err(ApplicationError::CorporationNameTooShort(4));
        }

        if name_len > 25 {
            return Err(ApplicationError::CorporationNameTooLong(25));
        }

        Ok(Self(name))
    }
}

impl Deref for CorporationName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for CorporationName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Display for CorporationName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
