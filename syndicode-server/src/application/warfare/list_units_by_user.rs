use uuid::Uuid;

use crate::{
    application::error::ApplicationResult,
    domain::{unit::model::Unit, unit::repository::UnitRepository},
};
use std::sync::Arc;

pub struct ListUnitsByUserUseCase<UNT>
where
    UNT: UnitRepository,
{
    unit_repository: Arc<UNT>,
}

impl<UNT> ListUnitsByUserUseCase<UNT>
where
    UNT: UnitRepository,
{
    pub fn new(unit_repository: Arc<UNT>) -> Self {
        Self { unit_repository }
    }

    pub async fn execute(&self, user_uuid: Uuid) -> ApplicationResult<Vec<Unit>> {
        let units = self.unit_repository.list_units_by_user(user_uuid).await?;

        Ok(units)
    }
}
