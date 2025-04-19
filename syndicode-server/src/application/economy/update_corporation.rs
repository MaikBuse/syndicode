use crate::{
    application::{
        action::{ActionDetails, QueuedActionPayload},
        error::ApplicationResult,
        ports::queuer::ActionQueueable,
    },
    domain::corporation::{model::name::CorporationName, repository::CorporationRepository},
};
use bon::bon;
use std::sync::Arc;
use uuid::Uuid;

pub struct UpdateCorporationUseCase<CRP, Q>
where
    CRP: CorporationRepository,
    Q: ActionQueueable,
{
    corporation_repo: Arc<CRP>,
    action_queuer: Arc<Q>,
}

#[bon]
impl<CRP, Q> UpdateCorporationUseCase<CRP, Q>
where
    CRP: CorporationRepository,
    Q: ActionQueueable,
{
    pub fn new(corporation_repo: Arc<CRP>, action_queuer: Arc<Q>) -> Self {
        Self {
            corporation_repo,
            action_queuer,
        }
    }

    #[builder]
    pub async fn execute(
        &self,
        request_uuid: Uuid,
        req_user_uuid: Uuid,
        name: String,
    ) -> ApplicationResult<i64> {
        let corporation_name = CorporationName::new(name)?;

        let mut outcome = self
            .corporation_repo
            .get_corporation_by_user(req_user_uuid)
            .await?;

        outcome.corporation.name = corporation_name;

        let action = QueuedActionPayload::builder()
            .user_uuid(req_user_uuid)
            .request_uuid(request_uuid)
            .details(ActionDetails::UpdateCorporation {
                corporation: outcome.corporation,
            })
            .build();

        match self.action_queuer.enqueue_action(action).await {
            Ok(entry_id) => {
                tracing::info!(
                    "Successfully enqueued UpdateCorporation action with ID: {}",
                    entry_id
                );
            }
            Err(err) => {
                tracing::error!("Failed to enqueue UpdateCorporation action: {:?}", err);

                return Err(err.into());
            }
        };

        Ok(outcome.game_tick)
    }
}
