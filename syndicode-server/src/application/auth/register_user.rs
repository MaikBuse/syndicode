use crate::{
    application::{
        error::{ApplicationError, ApplicationResult},
        ports::{crypto::PasswordHandler, uow::UnitOfWork, verification::VerificationSendable},
    },
    domain::{
        economy::corporation::model::{name::CorporationName, Corporation},
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
pub struct RegisterUserUseCase<P, UOW, VS>
where
    P: PasswordHandler,
    UOW: UnitOfWork,
    VS: VerificationSendable,
{
    pw: Arc<P>,
    uow: Arc<UOW>,
    verification: Arc<VS>,
}

#[bon]
impl<P, UOW, VS> RegisterUserUseCase<P, UOW, VS>
where
    P: PasswordHandler,
    UOW: UnitOfWork,
    VS: VerificationSendable,
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

        let user = User {
            uuid: Uuid::now_v7(),
            name: user_name,
            password_hash: password_hash.to_string(),
            email: user_email,
            role: UserRole::Player,
            status: UserStatus::Pending,
        };

        let user_verification = UserVerification::new(user.uuid);
        let verfication_code_clone = user_verification.clone_code();

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

                    let corporation_name = CorporationName::new(corporation_name)?;

                    let corporation = Corporation::new(user_to_create.uuid, corporation_name);

                    ctx.insert_corporation(&corporation)
                        .await
                        .map_err(ApplicationError::from)?;

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

        Ok(user_created)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        application::ports::{
            crypto::MockPasswordHandler, uow::MockUnitOfWork,
            verification::MockVerificationSendable,
        },
        domain::user::repository::MockUserRepository,
    };
    use mockall::predicate::*;

    struct TestFixture {
        mock_pw: MockPasswordHandler,
        mock_uow: MockUnitOfWork,
        mock_user_repo: MockUserRepository,
        mock_verification: MockVerificationSendable,
    }

    impl TestFixture {
        fn new() -> Self {
            TestFixture {
                mock_pw: MockPasswordHandler::new(),
                mock_uow: MockUnitOfWork::new(),
                mock_user_repo: MockUserRepository::new(),
                mock_verification: MockVerificationSendable::new(),
            }
        }

        /// Configures mocks for the standard user success scenario
        fn expect_standard_user_success(
            &mut self,
            input_password: String,
            expected_hash: String,
            expected_user_output: User, // The user returned by UoW
            receipient_email: String,
            receipient_name: String,
        ) {
            let input_user_password = UserPassword::new(input_password).unwrap();

            // Expect password hashing
            self.mock_pw
                .expect_hash_user_password()
                .with(eq(input_user_password)) // Match specific password
                .times(1)
                .return_once(move |_| Ok(expected_hash.to_string())); // Use return_once for clarity

            // Expect UnitOfWork execution
            self.mock_uow
                .expect_execute::<User>()
                .times(1)
                .with(mockall::predicate::always()) // Ignore the closure argument
                .return_once(move |_| {
                    // Use return_once
                    // Simulate UoW success, returning the fully formed user
                    let user_to_return = expected_user_output.clone();
                    Ok(user_to_return)
                });

            self.mock_verification
                .expect_send_verification_email()
                .times(1)
                .with(
                    eq(receipient_email),
                    eq(receipient_name),
                    mockall::predicate::always(),
                )
                .return_once(move |_, _, _| Ok(()));

            // No user repo calls expected for standard user creation
            self.mock_user_repo.expect_get_user().never();
        }

        /// Builds the use case instance, consuming the fixture
        fn build_use_case(
            self,
        ) -> RegisterUserUseCase<MockPasswordHandler, MockUnitOfWork, MockVerificationSendable>
        {
            RegisterUserUseCase::builder()
                .pw(Arc::new(self.mock_pw))
                .uow(Arc::new(self.mock_uow))
                .verification(Arc::new(self.mock_verification))
                .build()
        }
    }

    #[tokio::test]
    async fn should_create_user() {
        // 1. Arrange
        let mut fixture = TestFixture::new(); // Create fixture

        // Define test data
        let input_user_name = UserName::new("testuser".to_string()).unwrap();
        let input_password = "password123".to_string();
        let input_role = UserRole::Player;
        let input_email = UserEmail::new("contact@maikbuse.com".to_string()).unwrap();
        let input_corp_name = "TestCorp".to_string();
        let expected_hashed_password = "mock_hashed_password".to_string();
        let expected_user_uuid = Uuid::now_v7();

        let expected_user_output = User {
            uuid: expected_user_uuid,
            name: input_user_name.clone(),
            password_hash: expected_hashed_password.clone(),
            email: input_email.clone(),
            role: input_role,
            status: UserStatus::Pending,
        };

        // Configure mocks using the fixture method
        fixture.expect_standard_user_success(
            input_password.clone(),
            expected_hashed_password,
            expected_user_output.clone(),
            input_email.clone().into_inner(),
            input_user_name.clone().into_inner(),
        );

        // Build the use case from the fixture
        let uc = fixture.build_use_case();

        // 2. Act
        let result = uc
            .execute()
            .user_name(input_user_name.into_inner())
            .password(input_password.clone())
            .user_email(input_email.into_inner())
            .corporation_name(input_corp_name.clone())
            .call()
            .await;

        // 3. Assert
        assert!(result.is_ok());
        let created_user = result.unwrap();
        assert_eq!(created_user, expected_user_output);
    }
}
