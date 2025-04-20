use crate::{
    application::error::ApplicationResult,
    domain::corporation::{model::Corporation, repository::CorporationRepository},
};
use bon::Builder;
use std::sync::Arc;

#[derive(Builder)]
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
    pub async fn execute(&self) -> ApplicationResult<Vec<Corporation>> {
        let corporations = self.corporation_repo.list_corporations().await?;

        Ok(corporations)
    }
}
