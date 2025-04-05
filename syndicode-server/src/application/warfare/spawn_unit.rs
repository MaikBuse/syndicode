use crate::{
    application::error::ApplicationResult,
    domain::{unit::model::Unit, unit::repository::UnitRepository},
};
use std::sync::Arc;
use uuid::Uuid;

pub struct SpawnUnitUseCase {
    unit_repository: Arc<dyn UnitRepository>,
}

impl SpawnUnitUseCase {
    pub fn new(unit_repository: Arc<dyn UnitRepository>) -> Self {
        Self { unit_repository }
    }

    pub async fn execute(&self, req_user_uuid: Uuid) -> ApplicationResult<Unit> {
        let unit = Unit {
            uuid: Uuid::now_v7(),
            user_uuid: req_user_uuid,
        };

        self.unit_repository.insert_unit(&unit).await?;

        Ok(unit)
    }
}
