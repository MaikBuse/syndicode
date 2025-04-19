use crate::{
    application::error::ApplicationResult,
    domain::unit::repository::{ListUnitsOutcome, UnitRepository},
};
use bon::{bon, Builder};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Builder)]
pub struct ListUnitsByUserUseCase<UNT>
where
    UNT: UnitRepository,
{
    unit_repository: Arc<UNT>,
}

#[bon]
impl<UNT> ListUnitsByUserUseCase<UNT>
where
    UNT: UnitRepository,
{
    #[builder]
    pub async fn execute(&self, user_uuid: Uuid) -> ApplicationResult<ListUnitsOutcome> {
        Ok(self.unit_repository.list_units_by_user(user_uuid).await?)
    }
}
