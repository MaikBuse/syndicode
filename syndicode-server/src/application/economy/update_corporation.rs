use crate::{
    application::{action::QueuedAction, error::ApplicationResult, ports::queue::ActionQueuer},
    domain::corporation::{model::name::CorporationName, repository::CorporationRepository},
};
use std::sync::Arc;
use uuid::Uuid;

pub struct UpdateCorporationUseCase<CRP, Q>
where
    CRP: CorporationRepository,
    Q: ActionQueuer,
{
    corporation_repo: Arc<CRP>,
    action_queuer: Arc<Q>,
}

impl<CRP, Q> UpdateCorporationUseCase<CRP, Q>
where
    CRP: CorporationRepository,
    Q: ActionQueuer,
{
    pub fn new(corporation_repo: Arc<CRP>, action_queuer: Arc<Q>) -> Self {
        Self {
            corporation_repo,
            action_queuer,
        }
    }

    pub async fn execute(&self, req_user_uuid: Uuid, name: String) -> ApplicationResult<()> {
        let corporation_name = CorporationName::new(name)?;

        let mut corporation = self
            .corporation_repo
            .get_corporation_by_user(req_user_uuid)
            .await?;

        corporation.name = corporation_name;

        let action = QueuedAction::UpdateCorporation { corporation };

        match self.action_queuer.enqueue_action(action).await {
            Ok(entry_id) => {
                tracing::info!(
                    "Successfully enqueued UpdateCorporation action with ID: {}",
                    entry_id
                );
                Ok(())
            }
            Err(err) => {
                tracing::error!("Failed to enqueue UpdateCorporation action: {:?}", err);

                Err(err.into())
            }
        }
    }
}
