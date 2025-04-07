use crate::{
    application::error::ApplicationResult,
    domain::{unit::model::Unit, unit::repository::UnitRepository},
};
use std::sync::Arc;

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

    pub async fn execute(&self) -> ApplicationResult<Vec<Unit>> {
        let units = self.unit_repository.list_units().await?;

        Ok(units)
    }
}
