use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use std::fmt::Display;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub role: String,
}

#[derive(Debug, Clone)]
#[repr(i16)]
pub enum UserRole {
    Admin,
    User,
}

impl From<i16> for UserRole {
    fn from(value: i16) -> Self {
        match value {
            1 => Self::Admin,
            _ => Self::User,
        }
    }
}

impl From<UserRole> for i16 {
    fn from(val: UserRole) -> Self {
        match val {
            UserRole::Admin => 1,
            UserRole::User => 2,
        }
    }
}

impl TryFrom<String> for UserRole {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Admin" => Ok(Self::Admin),
            "User" => Ok(Self::User),
            _ => Err(anyhow::anyhow!(
                "Failed to parse user role '{}' from string",
                value
            )),
        }
    }
}

impl From<UserRole> for i32 {
    fn from(val: UserRole) -> Self {
        match val {
            UserRole::Admin => 1,
            UserRole::User => 2,
        }
    }
}

impl Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::Admin => write!(f, "Admin"),
            UserRole::User => write!(f, "User"),
        }
    }
}

#[derive(Debug, FromRow)]
pub struct UserModel {
    pub uuid: Uuid,
    pub name: String,
    pub password_hash: String,
    pub role: UserRole,
}
