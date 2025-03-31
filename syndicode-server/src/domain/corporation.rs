use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Corporation {
    pub uuid: Uuid,
    pub user_uuid: Uuid,
    pub name: String,
    pub balance: i64,
}
