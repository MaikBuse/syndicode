use crate::{
    application::{
        action::{ActionDetails, QueuedActionPayload},
        error::{ApplicationError, ApplicationResult},
        ports::queuer::ActionQueueable,
    },
    domain::{
        economy::corporation::repository::CorporationRepository,
        repository::RepositoryError,
        user::{
            model::{role::UserRole, status::UserStatus},
            repository::UserRepository,
        },
    },
};
use bon::{bon, Builder};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Builder)]
pub struct DeleteUserUseCase<Q, USR, CRP>
where
    Q: ActionQueueable,
    USR: UserRepository,
    CRP: CorporationRepository,
{
    user_repo: Arc<USR>,
    corporation_repo: Arc<CRP>,
    action_queuer: Arc<Q>,
}

#[bon]
impl<Q, USR, CRP> DeleteUserUseCase<Q, USR, CRP>
where
    Q: ActionQueueable,
    USR: UserRepository,
    CRP: CorporationRepository,
{
    #[builder]
    pub async fn execute(
        &self,
        request_uuid: Uuid,
        req_user_uuid: Uuid,
        user_uuid: Uuid,
    ) -> ApplicationResult<()> {
        if req_user_uuid != user_uuid {
            let req_user = self.user_repo.get_user(req_user_uuid).await?;

            if req_user.role != UserRole::Admin || req_user.status != UserStatus::Active {
                return Err(ApplicationError::Unauthorized);
            }
        }

        // Check if the corporation exists and get the id
        let get_corp_outcome = self
            .corporation_repo
            .get_corporation_by_user(user_uuid)
            .await
            .map_err(|err| match err {
                RepositoryError::NotFound => ApplicationError::CorporationForUserNotFound,
                _ => ApplicationError::from(err),
            })?;

        let action = QueuedActionPayload::builder()
            .request_uuid(request_uuid)
            .req_user_uuid(req_user_uuid)
            .details(ActionDetails::DeleteCorporation {
                corporation_uuid: get_corp_outcome.corporation.uuid,
            })
            .build();

        // Queue action the delete the user's corporation
        match self.action_queuer.enqueue_action(action).await {
            Ok(entry_id) => {
                tracing::info!(
                    "Successfully enqueued DeleteCorporation action with ID: {}",
                    entry_id
                );
            }
            Err(err) => {
                tracing::error!(
                    "Failed to enqueue DeleteCorporation action with error: {:?}",
                    err
                );

                return Err(err.into());
            }
        };

        // Delete the user once everything passed
        self.user_repo.delete_user(user_uuid).await?;

        Ok(())
    }
}
