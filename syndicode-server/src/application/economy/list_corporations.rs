use crate::{
    application::error::ApplicationResult,
    domain::corporation::{model::Corporation, repository::CorporationRepository},
};
use std::sync::Arc;

pub struct ListCorporationsUseCase<CRP>
where
    CRP: CorporationRepository,
{
    corporation_repo: Arc<CRP>,
}

impl<CRP> ListCorporationsUseCase<CRP>
where
    CRP: CorporationRepository,
{
    pub fn new(corporation_repo: Arc<CRP>) -> Self {
        Self { corporation_repo }
    }

    pub async fn execute(&self) -> ApplicationResult<Vec<Corporation>> {
        let corporations = self.corporation_repo.list_corporations().await?;

        Ok(corporations)
    }
}
