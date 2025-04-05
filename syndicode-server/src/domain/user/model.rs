pub mod password;
pub mod role;

use role::UserRole;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Validate, PartialEq)]
pub struct User {
    pub uuid: Uuid,
    #[validate(length(min = 1, max = 20))]
    pub name: String,
    pub password_hash: String,
    pub role: UserRole,
}
