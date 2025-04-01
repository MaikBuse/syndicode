use crate::{
    application::error::ApplicationResult, domain::unit::Unit,
    infrastructure::postgres::PostgresDatabase,
};
use std::sync::Arc;
use uuid::Uuid;

pub struct ListUnitsUseCase {
    db: Arc<PostgresDatabase>,
}

impl ListUnitsUseCase {
    pub fn new(db: Arc<PostgresDatabase>) -> Self {
        Self { db }
    }

    pub async fn execute(&self, req_user_uuid: Uuid) -> ApplicationResult<Vec<Unit>> {
        let units = PostgresDatabase::list_user_units(&self.db.pool, req_user_uuid).await?;

        Ok(units)
    }
}
