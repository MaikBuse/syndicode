use crate::{
    application::error::ApplicationResult,
    domain::corporation::repository::{CorporationRepository, GetCorporationOutcome},
};
use std::sync::Arc;
use uuid::Uuid;

pub struct GetCorporationUseCase<CRP>
where
    CRP: CorporationRepository,
{
    corporation_repo: Arc<CRP>,
}

impl<CRP> GetCorporationUseCase<CRP>
where
    CRP: CorporationRepository,
{
    pub fn new(corporation_repo: Arc<CRP>) -> Self {
        Self { corporation_repo }
    }

    pub async fn execute(&self, user_uuid: Uuid) -> ApplicationResult<GetCorporationOutcome> {
        Ok(self
            .corporation_repo
            .get_corporation_by_user(user_uuid)
            .await?)
    }
}
