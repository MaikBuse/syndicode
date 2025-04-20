pub mod email;
pub mod name;
pub mod password;
pub mod role;
pub mod status;

use email::UserEmail;
use name::UserName;
use role::UserRole;
use status::UserStatus;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub uuid: Uuid,
    pub name: UserName,
    pub password_hash: String,
    pub email: UserEmail,
    pub role: UserRole,
    pub status: UserStatus,
}
