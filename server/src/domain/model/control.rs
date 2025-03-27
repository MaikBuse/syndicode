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

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "user_role", rename_all = "PascalCase")]
pub enum UserRole {
    Admin,
    User,
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

impl Into<i32> for UserRole {
    fn into(self) -> i32 {
        match self {
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

#[derive(Debug, Clone, PartialEq, sqlx::Type)]
#[sqlx(type_name = "session_state", rename_all = "PascalCase")]
pub enum SessionState {
    Initializing,
    Running,
}

impl Into<i32> for SessionState {
    fn into(self) -> i32 {
        match self {
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
