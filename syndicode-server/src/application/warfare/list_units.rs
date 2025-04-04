use crate::{
    application::error::ApplicationResult,
    domain::{repository::unit::UnitRespository, unit::Unit},
};
use std::sync::Arc;
use uuid::Uuid;

pub struct ListUnitsUseCase {
    unit_repository: Arc<dyn UnitRespository>,
}

impl ListUnitsUseCase {
    pub fn new(unit_repository: Arc<dyn UnitRespository>) -> Self {
        Self { unit_repository }
    }

    pub async fn execute(&self, req_user_uuid: Uuid) -> ApplicationResult<Vec<Unit>> {
        let units = self.unit_repository.list_units(req_user_uuid).await?;

        Ok(units)
    }
}
