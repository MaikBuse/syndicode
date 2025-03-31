pub mod role;

use role::UserRole;
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct User {
    pub uuid: Uuid,
    pub name: String,
    pub password_hash: String,
    pub role: UserRole,
}
