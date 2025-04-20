use crate::{
    application::{
        error::{ApplicationError, ApplicationResult},
        ports::{uow::UnitOfWork, verification::VerificationSendable},
    },
    domain::{
        user::model::{status::UserStatus, User},
        user_verify::model::UserVerification,
    },
};
use bon::Builder;
use std::sync::Arc;

#[derive(Builder)]
pub struct ResendVerificationUseCase<UOW, VS>
where
    UOW: UnitOfWork,
    VS: VerificationSendable,
{
    uow: Arc<UOW>,
    verification: Arc<VS>,
}

impl<UOW, VS> ResendVerificationUseCase<UOW, VS>
where
    UOW: UnitOfWork,
    VS: VerificationSendable,
{
    pub async fn execute(&self, user_name: String) -> ApplicationResult<User> {
        let (user, code) = self
            .uow
            .execute(|ctx| {
                Box::pin(async move {
                    let user_to_verify = ctx
                        .get_user_by_name(user_name)
                        .await
                        .map_err(ApplicationError::from)?;

                    if user_to_verify.status != UserStatus::Pending {
                        return Err(ApplicationError::UserNotPending);
                    }

                    let user_verification = UserVerification::new(user_to_verify.uuid);

                    ctx.update_user_verification(&user_verification).await?;

                    Ok((user_to_verify, user_verification.into_code()))
                })
            })
            .await?;

        self.verification
            .send_verification_email(
                user.email.clone().into_inner(),
                user.name.clone().into_inner(),
                code,
            )
            .await?;

        Ok(user)
    }
}
