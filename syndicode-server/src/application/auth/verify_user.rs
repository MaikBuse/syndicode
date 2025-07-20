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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::ports::uow::MockUnitOfWork;
    use std::sync::Arc;
    use uuid::Uuid;

    #[tokio::test]
    async fn should_verify_user_successfully_with_valid_code() {
        // Arrange
        let mut mock_uow = MockUnitOfWork::new();
        let user_uuid = Uuid::from_u128(12345); // Test UUID
        let user_name = "testuser".to_string();
        let valid_code = "VALID12345".to_string();

        mock_uow
            .expect_execute::<Uuid>()
            .once()
            .returning(move |_callback| Ok(user_uuid));

        let use_case = VerifyUserUseCase::builder().uow(Arc::new(mock_uow)).build();

        // Act
        let result = use_case.execute(user_name, valid_code).await;

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), user_uuid);
    }

    #[tokio::test]
    async fn should_return_error_when_verification_code_expired() {
        // Arrange
        let mut mock_uow = MockUnitOfWork::new();
        let user_name = "testuser".to_string();
        let expired_code = "EXPIRED123".to_string();

        mock_uow
            .expect_execute::<Uuid>()
            .once()
            .returning(move |_callback| Err(ApplicationError::VerificationCodeExpired));

        let use_case = VerifyUserUseCase::builder().uow(Arc::new(mock_uow)).build();

        // Act
        let result = use_case.execute(user_name, expired_code).await;

        // Assert
        assert!(result.is_err());
        match result.err().unwrap() {
            ApplicationError::VerificationCodeExpired => (),
            other_err => panic!("Expected VerificationCodeExpired, got {other_err:?}"),
        }
    }

    #[tokio::test]
    async fn should_return_error_when_verification_code_incorrect() {
        // Arrange
        let mut mock_uow = MockUnitOfWork::new();
        let user_name = "testuser".to_string();
        let wrong_code = "WRONG12345".to_string();

        mock_uow
            .expect_execute::<Uuid>()
            .once()
            .returning(move |_callback| Err(ApplicationError::VerificationCodeFalse));

        let use_case = VerifyUserUseCase::builder().uow(Arc::new(mock_uow)).build();

        // Act
        let result = use_case.execute(user_name, wrong_code).await;

        // Assert
        assert!(result.is_err());
        match result.err().unwrap() {
            ApplicationError::VerificationCodeFalse => (),
            other_err => panic!("Expected VerificationCodeFalse, got {other_err:?}"),
        }
    }
}
