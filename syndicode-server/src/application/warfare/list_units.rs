use crate::{
    application::error::ApplicationResult,
    domain::{unit::model::Unit, unit::repository::UnitRepository},
};
use bon::Builder;
use std::sync::Arc;

#[derive(Builder)]
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
    pub async fn execute(&self) -> ApplicationResult<Vec<Unit>> {
        let units = self.unit_repository.list_units().await?;

        Ok(units)
    }
}
