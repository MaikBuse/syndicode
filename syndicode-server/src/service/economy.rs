use super::error::ServiceResult;
use crate::{
    domain::model::economy::CorporationModel,
    infrastructure::postgres::{economy, PostgresDatabase},
};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub struct EconomyService {
    postgres_db: Arc<PostgresDatabase>,
}

impl EconomyService {
    pub fn new(postgres_db: Arc<PostgresDatabase>) -> Self {
        Self { postgres_db }
    }

    pub async fn get_corporation(&self, user_uuid: Uuid) -> ServiceResult<CorporationModel> {
        Ok(economy::get_user_corporation(&self.postgres_db.pool, user_uuid).await?)
    }
}
