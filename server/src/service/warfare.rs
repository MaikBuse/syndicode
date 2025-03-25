use super::error::ServiceResult;
use crate::domain::{
    model::warfare::UnitModel,
    repository::{control::ControlDatabaseRepository, warfare::WarfareDatabaseRepository},
};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub struct WarfareService {
    control_db: Arc<dyn ControlDatabaseRepository>,
    warfare_db: Arc<dyn WarfareDatabaseRepository>,
}

impl WarfareService {
    pub fn new(
        control_db: Arc<dyn ControlDatabaseRepository>,
        warfare_db: Arc<dyn WarfareDatabaseRepository>,
    ) -> Self {
        Self {
            control_db,
            warfare_db,
        }
    }

    pub async fn create_unit(
        &self,
        session_uuid: Vec<u8>,
        user_uuid: Vec<u8>,
    ) -> ServiceResult<UnitModel> {
        let unit = UnitModel {
            uuid: Uuid::now_v7().into(),
            session_uuid,
            user_uuid,
        };

        Ok(self.warfare_db.create_unit(unit).await?)
    }

    pub async fn list_units(
        &self,
        session_uuid: Vec<u8>,
        user_uuid: Vec<u8>,
    ) -> ServiceResult<Vec<UnitModel>> {
        let units = self
            .warfare_db
            .list_user_units(session_uuid, user_uuid)
            .await?;

        Ok(units)
    }
}
