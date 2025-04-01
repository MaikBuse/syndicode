use sqlx::prelude::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Validate, FromRow)]
pub struct Corporation {
    pub uuid: Uuid,
    pub user_uuid: Uuid,
    #[validate(length(min = 1, max = 20))]
    pub name: String,
    pub balance: i64,
}
