use super::error::ServiceResult;
use crate::domain::{
    model::economy::CorporationModel, repository::economy::EconomyDatabaseRepository,
};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub struct EconomyService {
    economy_db: Arc<dyn EconomyDatabaseRepository>,
}

impl EconomyService {
    pub fn new(economy_db: Arc<dyn EconomyDatabaseRepository>) -> Self {
        Self { economy_db }
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
