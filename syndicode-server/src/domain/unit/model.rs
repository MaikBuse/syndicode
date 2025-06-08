use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, FromRow)]
pub struct Unit {
    pub uuid: Uuid,
    pub corporation_uuid: Uuid,
}
