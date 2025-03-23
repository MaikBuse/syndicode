use sqlx::prelude::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct UnitModel {
    pub uuid: Vec<u8>,
    pub session_uuid: Vec<u8>,
    pub user_uuid: Vec<u8>,
}
