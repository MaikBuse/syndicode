use super::error::ServiceResult;
use crate::domain::{
    model::economy::CorporationModel,
    repository::{control::ControlDatabaseRepository, economy::EconomyDatabaseRepository},
};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub struct EconomyService {
    control_db: Arc<dyn ControlDatabaseRepository>,
    economy_db: Arc<dyn EconomyDatabaseRepository>,
}

impl EconomyService {
    pub fn new(
        control_db: Arc<dyn ControlDatabaseRepository>,
        economy_db: Arc<dyn EconomyDatabaseRepository>,
    ) -> Self {
        Self {
            control_db,
            economy_db,
        }
    }

    pub async fn get_corporation(
        &self,
        session_uuid: Uuid,
        user_uuid: Uuid,
    ) -> ServiceResult<CorporationModel> {
        Ok(self
            .economy_db
            .get_user_corporation(session_uuid, user_uuid)
            .await?)
    }
}
