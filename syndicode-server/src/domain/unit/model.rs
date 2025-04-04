use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Unit {
    pub uuid: Uuid,
    pub user_uuid: Uuid,
}
