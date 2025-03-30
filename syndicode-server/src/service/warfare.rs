use super::error::ServiceResult;
use crate::{
    domain::model::warfare::UnitModel,
    infrastructure::postgres::{warfare, PostgresDatabase},
};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub struct WarfareService {
    postgres_db: Arc<PostgresDatabase>,
}

impl WarfareService {
    pub fn new(postgres_db: Arc<PostgresDatabase>) -> Self {
        Self { postgres_db }
    }

    pub async fn create_unit(&self, req_user_uuid: Uuid) -> ServiceResult<UnitModel> {
        let unit = UnitModel {
            uuid: Uuid::now_v7(),
            user_uuid: req_user_uuid,
        };

        Ok(warfare::create_unit(&self.postgres_db.pool, unit).await?)
    }

    pub async fn list_units(&self, req_user_uuid: Uuid) -> ServiceResult<Vec<UnitModel>> {
        let units = warfare::list_user_units(&self.postgres_db.pool, req_user_uuid).await?;

        Ok(units)
    }
}
