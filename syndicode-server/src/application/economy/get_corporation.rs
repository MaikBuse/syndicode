use crate::{
    application::error::ApplicationResult,
    domain::{corporation::model::Corporation, corporation::repository::CorporationRepository},
};
use std::sync::Arc;
use uuid::Uuid;

pub struct GetCorporationUseCase {
    corporation_repo: Arc<dyn CorporationRepository>,
}

impl GetCorporationUseCase {
    pub fn new(corporation_repo: Arc<dyn CorporationRepository>) -> Self {
        Self { corporation_repo }
    }

    pub async fn execute(&self, user_uuid: Uuid) -> ApplicationResult<Corporation> {
        Ok(self
            .corporation_repo
            .get_corporation_by_user(user_uuid)
            .await?)
    }
}
