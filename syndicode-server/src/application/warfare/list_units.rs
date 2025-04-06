use crate::{
    application::error::ApplicationResult,
    domain::{unit::model::Unit, unit::repository::UnitRepository},
};
use std::sync::Arc;
use uuid::Uuid;

pub struct ListUnitsUseCase<UNT>
where
    UNT: UnitRepository,
{
    unit_repository: Arc<UNT>,
}

impl<UNT> ListUnitsUseCase<UNT>
where
    UNT: UnitRepository,
{
    pub fn new(unit_repository: Arc<UNT>) -> Self {
        Self { unit_repository }
    }

    pub async fn execute(&self, req_user_uuid: Uuid) -> ApplicationResult<Vec<Unit>> {
        let units = self.unit_repository.list_units(req_user_uuid).await?;

        Ok(units)
    }
}
