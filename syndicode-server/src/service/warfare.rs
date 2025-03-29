use super::error::ServiceResult;
use crate::domain::{model::warfare::UnitModel, repository::warfare::WarfareDatabaseRepository};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub struct WarfareService {
    warfare_db: Arc<dyn WarfareDatabaseRepository>,
}

impl WarfareService {
    pub fn new(warfare_db: Arc<dyn WarfareDatabaseRepository>) -> Self {
        Self { warfare_db }
    }

    pub async fn create_unit(
        &self,
        session_uuid: Uuid,
        user_uuid: Uuid,
    ) -> ServiceResult<UnitModel> {
        let unit = UnitModel {
            uuid: Uuid::now_v7(),
            session_uuid,
            user_uuid,
        };

        Ok(self.warfare_db.create_unit(unit).await?)
    }

    pub async fn list_units(
        &self,
        session_uuid: Uuid,
        user_uuid: Uuid,
    ) -> ServiceResult<Vec<UnitModel>> {
        let units = self
            .warfare_db
            .list_user_units(session_uuid, user_uuid)
            .await?;

        Ok(units)
    }
}
