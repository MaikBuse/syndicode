use crate::{
    application::{
        error::{ApplicationError, ApplicationResult},
        ports::uow::UnitOfWork,
    },
    domain::user::model::status::UserStatus,
};
use bon::Builder;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Builder)]
pub struct VerifyUserUseCase<UOW>
where
    UOW: UnitOfWork,
{
    uow: Arc<UOW>,
}

impl<UOW> VerifyUserUseCase<UOW>
where
    UOW: UnitOfWork,
{
    pub async fn execute(
        &self,
        user_name: String,
        provided_code: String,
    ) -> ApplicationResult<Uuid> {
        let user_uuid = self
            .uow
            .execute(|ctx| {
                Box::pin(async move {
                    let mut user_to_update = ctx
                        .get_user_by_name(user_name)
                        .await
                        .map_err(ApplicationError::from)?;

                    let user_verification = ctx
                        .get_user_verification(user_to_update.uuid)
                        .await
                        .map_err(ApplicationError::from)?;

                    // Check if the verification code has expired
                    if user_verification.is_expired() {
                        return Err(ApplicationError::VerificationCodeExpired);
                    }

                    // Compare the verification code
                    if !user_verification.is_code_correct(provided_code.as_str()) {
                        return Err(ApplicationError::VerificationCodeFalse);
                    }

                    user_to_update.status = UserStatus::Active;

                    ctx.update_user(&user_to_update)
                        .await
                        .map_err(ApplicationError::from)?;

                    ctx.delete_user_verification(user_to_update.uuid)
                        .await
                        .map_err(ApplicationError::from)?;

                    Ok(user_to_update.uuid)
                })
            })
            .await?;

        Ok(user_uuid)
    }
}
