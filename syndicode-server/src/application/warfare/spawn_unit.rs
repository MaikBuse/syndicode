use crate::{
    application::error::ApplicationResult, domain::unit::Unit,
    infrastructure::postgres::PostgresDatabase,
};
use std::sync::Arc;
use uuid::Uuid;

pub struct SpawnUnitUseCase {
    db: Arc<PostgresDatabase>,
}

impl SpawnUnitUseCase {
    pub fn new(db: Arc<PostgresDatabase>) -> Self {
        Self { db }
    }

    pub async fn execute(&self, req_user_uuid: Uuid) -> ApplicationResult<Unit> {
        let unit = Unit {
            uuid: Uuid::now_v7(),
            user_uuid: req_user_uuid,
        };

        Ok(PostgresDatabase::create_unit(&self.db.pool, unit).await?)
    }
}
