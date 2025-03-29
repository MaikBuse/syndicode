use anyhow::anyhow;
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

#[derive(Debug, Clone, PartialEq)]
#[repr(i16)]
pub enum SessionState {
    Initializing,
    Running,
}

impl From<i16> for SessionState {
    fn from(value: i16) -> Self {
        match value {
            1 => Self::Initializing,
            _ => Self::Running,
        }
    }
}

impl From<SessionState> for i16 {
    fn from(val: SessionState) -> Self {
        match val {
            SessionState::Initializing => 1,
            SessionState::Running => 2,
        }
    }
}

impl From<i32> for SessionState {
    fn from(value: i32) -> Self {
        match value {
            1 => Self::Initializing,
            _ => Self::Running,
        }
    }
}

impl From<SessionState> for i32 {
    fn from(val: SessionState) -> Self {
        match val {
            SessionState::Initializing => 1,
            SessionState::Running => 2,
        }
    }
}

impl TryFrom<String> for SessionState {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Initializing" => Ok(Self::Initializing),
            "Running" => Ok(Self::Running),
            _ => Err(anyhow!(
                "Failed to parse session state '{}' from string",
                value
            )),
        }
    }
}

impl Display for SessionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionState::Initializing => write!(f, "Initializing"),
            SessionState::Running => write!(f, "Running"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, FromRow)]
pub struct SessionModel {
    pub uuid: Uuid,
    pub interval: i64,
    pub state: SessionState,
}

#[derive(Debug, Clone, PartialEq, FromRow)]
pub struct SessionUser {
    pub uuid: Uuid,
    pub session_uuid: Uuid,
    pub user_uuid: Uuid,
}
