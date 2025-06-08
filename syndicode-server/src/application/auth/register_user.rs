use crate::{
    application::{
        action::{ActionDetails, QueuedActionPayload},
        error::{ApplicationError, ApplicationResult},
        ports::{
            crypto::PasswordHandler, queuer::ActionQueueable, uow::UnitOfWork,
            verification::VerificationSendable,
        },
    },
    domain::{
        economy::corporation::{model::name::CorporationName, repository::CorporationRepository},
        repository::RepositoryError,
        user::model::{
            email::UserEmail, name::UserName, password::UserPassword, role::UserRole,
            status::UserStatus, User,
        },
        user_verify::model::UserVerification,
    },
};
use bon::{bon, Builder};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Builder)]
pub struct RegisterUserUseCase<Q, UOW, P, VS, CRP>
where
    Q: ActionQueueable,
    UOW: UnitOfWork,
    P: PasswordHandler,
    VS: VerificationSendable,
    CRP: CorporationRepository,
{
    pw: Arc<P>,
    verification: Arc<VS>,
    action_queuer: Arc<Q>,
    uow: Arc<UOW>,
    corp_repo: Arc<CRP>,
}

#[bon]
impl<Q, UOW, P, VS, CRP> RegisterUserUseCase<Q, UOW, P, VS, CRP>
where
    Q: ActionQueueable,
    UOW: UnitOfWork,
    P: PasswordHandler,
    VS: VerificationSendable,
    CRP: CorporationRepository,
{
    #[builder]
    pub async fn execute(
        &self,
        user_name: String,
        password: String,
        user_email: String,
        corporation_name: String,
    ) -> ApplicationResult<User> {
        let user_name = UserName::new(user_name)?;
        let user_password = UserPassword::new(password)?;
        let user_email = UserEmail::new(user_email)?;

        let password_hash = self.pw.hash_user_password(user_password)?;

        let user_uuid = Uuid::now_v7();

        let user = User {
            uuid: user_uuid,
            name: user_name,
            password_hash: password_hash.to_string(),
            email: user_email,
            role: UserRole::Player,
            status: UserStatus::Pending,
        };

        let user_verification = UserVerification::new(user.uuid);
        let verfication_code_clone = user_verification.clone_code();

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

        let user_created = self
            .uow
            .execute(|ctx| {
                Box::pin(async move {
                    let user_to_create = user.clone();

                    if let Err(err) = ctx.create_user(&user_to_create).await {
                        match err {
                            RepositoryError::UniqueConstraint => {
                                return Err(ApplicationError::UniqueConstraint)
                            }
                            _ => return Err(ApplicationError::from(err)),
                        }
                    }

                    ctx.create_user_verification(&user_verification).await?;

                    Ok(user_to_create)
                })
            })
            .await?;

        self.verification
            .send_verification_email(
                user_created.email.clone().into_inner(),
                user_created.name.clone().into_inner(),
                verfication_code_clone,
            )
            .await?;

        // Queue an action to create the user's corporation
        let action = QueuedActionPayload::builder()
            .req_user_uuid(user_uuid)
            .request_uuid(user_uuid)
            .details(ActionDetails::CreateCorporation {
                user_uuid: user_created.uuid,
                corporation_name,
            })
            .build();

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

        Ok(user_created)
    }
}
