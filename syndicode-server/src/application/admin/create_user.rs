use crate::{
    application::{
        action::{ActionDetails, QueuedActionPayload},
        error::{ApplicationError, ApplicationResult},
        ports::{crypto::PasswordHandler, queuer::ActionQueueable},
    },
    domain::{
        economy::corporation::{model::name::CorporationName, repository::CorporationRepository},
        repository::RepositoryError,
        user::{
            model::{
                email::UserEmail, name::UserName, password::UserPassword, role::UserRole,
                status::UserStatus, User,
            },
            repository::UserRepository,
        },
    },
};
use bon::{bon, Builder};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Builder)]
pub struct CreateUserUseCase<Q, P, USR, CRP>
where
    Q: ActionQueueable,
    P: PasswordHandler,
    USR: UserRepository,
    CRP: CorporationRepository,
{
    pw: Arc<P>,
    user_repo: Arc<USR>,
    corp_repo: Arc<CRP>,
    action_queuer: Arc<Q>,
}

#[bon]
impl<Q, P, USR, CRP> CreateUserUseCase<Q, P, USR, CRP>
where
    Q: ActionQueueable,
    P: PasswordHandler,
    USR: UserRepository,
    CRP: CorporationRepository,
{
    #[builder]
    pub async fn execute(
        &self,
        request_uuid: Uuid,
        req_user_uuid: Uuid,
        user_name: String,
        password: String,
        user_email: String,
        user_role: UserRole,
        corporation_name: String,
    ) -> ApplicationResult<User> {
        let user_name = UserName::new(user_name)?;
        let user_password = UserPassword::new(password)?;
        let user_email = UserEmail::new(user_email)?;

        // check authorization
        let req_user = self.user_repo.get_user(req_user_uuid).await?;
        if req_user.role != UserRole::Admin || req_user.status != UserStatus::Active {
            return Err(ApplicationError::Unauthorized);
        }

        // Check the syntactical validity of the corporation name
        let corporation_name = CorporationName::new(corporation_name)?;

        // Check if the corporation name is already taken
        match self
            .corp_repo
            .get_corporation_by_name(corporation_name.to_string())
            .await
        {
            Ok(_) => {
                return Err(ApplicationError::CorporationNameAlreadyTaken);
            }
            Err(err) => match err {
                // This is the expected result, since we don't want the name to be taken yet
                RepositoryError::NotFound => {}
                _ => return Err(ApplicationError::from(err)),
            },
        };

        let password_hash = self.pw.hash_user_password(user_password)?;

        let user = User {
            uuid: Uuid::now_v7(),
            name: user_name,
            password_hash: password_hash.to_string(),
            email: user_email,
            role: user_role,
            status: UserStatus::Active,
        };

        self.user_repo
            .create_user(&user)
            .await
            .map_err(ApplicationError::from)?;

        let action = QueuedActionPayload::builder()
            .request_uuid(request_uuid)
            .req_user_uuid(req_user_uuid)
            .details(ActionDetails::CreateCorporation {
                user_uuid: user.uuid,
                corporation_name,
            })
            .build();

        // Queue an action to create the user's corporation
        match self.action_queuer.enqueue_action(action).await {
            Ok(entry_id) => {
                tracing::debug!(
                    "Successfully enqueued CreateCorporation action with ID: {}",
                    entry_id
                );
            }
            Err(err) => {
                tracing::error!(
                    "Failed to enqueue CreateCorporation action with error: {:?}",
                    err
                );

                return Err(err.into());
            }
        };

        Ok(user)
    }
}
