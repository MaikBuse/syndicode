use crate::{
    application::error::ApplicationResult, domain::corporation::Corporation,
    infrastructure::postgres::PostgresDatabase,
};
use std::sync::Arc;
use uuid::Uuid;

pub struct GetCorporationUseCase {
    db: Arc<PostgresDatabase>,
}

impl GetCorporationUseCase {
    pub fn new(db: Arc<PostgresDatabase>) -> Self {
        Self { db }
    }

    pub async fn execute(&self, user_uuid: Uuid) -> ApplicationResult<Corporation> {
        Ok(PostgresDatabase::get_user_corporation(&self.db.pool, user_uuid).await?)
    }
}
