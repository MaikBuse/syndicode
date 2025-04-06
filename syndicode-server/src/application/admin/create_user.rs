use crate::{
    application::{
        error::{ApplicationError, ApplicationResult},
        ports::{crypto::PasswordHandler, uow::UnitOfWork},
    },
    domain::{
        corporation::model::Corporation,
        repository::RepositoryError,
        user::{
            model::{password::UserPassword, role::UserRole, User},
            repository::UserRepository,
        },
    },
};
use std::sync::Arc;
use uuid::Uuid;

pub struct CreateUserUseCase<P, UOW, USR>
where
    P: PasswordHandler,
    UOW: UnitOfWork,
    USR: UserRepository,
{
    pw: Arc<P>,
    uow: Arc<UOW>,
    user_repo: Arc<USR>,
}

impl<P, UOW, USR> CreateUserUseCase<P, UOW, USR>
where
    P: PasswordHandler,
    UOW: UnitOfWork,
    USR: UserRepository,
{
    pub fn new(pw: Arc<P>, uow: Arc<UOW>, user_repo: Arc<USR>) -> Self {
        Self { pw, uow, user_repo }
    }

    pub async fn execute(
        &self,
        maybe_req_user_uuid: Option<Uuid>,
        user_name: String,
        password: String,
        user_role: UserRole,
        corporation_name: String,
    ) -> ApplicationResult<User> {
        if user_name.is_empty() {
            return Err(ApplicationError::UsernameInvalid);
        }

        let user_password = UserPassword::new(password)?;

        if user_role == UserRole::Admin {
            let Some(req_user_uuid) = maybe_req_user_uuid else {
                return Err(ApplicationError::MissingAuthentication);
            };

            let req_user = self.user_repo.get_user(req_user_uuid).await?;

            if req_user.role != UserRole::Admin {
                return Err(ApplicationError::Unauthorized);
            }
        }

        let password_hash = self.pw.hash_user_password(user_password)?;

        let user = User {
            uuid: Uuid::now_v7(),
            name: user_name,
            password_hash: password_hash.to_string(),
            role: user_role,
        };

        let user_created = match self
            .uow
            .execute(|ctx| {
                Box::pin(async move {
                    let user_to_create = user.clone();

                    ctx.create_user(&user_to_create).await?;

                    let corporation = Corporation::new(user_to_create.uuid, corporation_name);

                    ctx.insert_corporation(&corporation).await?;

                    Ok(user_to_create)
                })
            })
            .await
        {
            Ok(user_created) => user_created,
            Err(err) => match err {
                RepositoryError::UniqueConstraint => {
                    return Err(ApplicationError::UniqueConstraint)
                }
                _ => return Err(err.into()),
            },
        };

        Ok(user_created)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        application::ports::{crypto::MockPasswordHandler, uow::MockUnitOfWork},
        domain::user::repository::MockUserRepository,
    };
    use mockall::predicate::*;

    struct TestFixture {
        mock_pw: MockPasswordHandler,
        mock_uow: MockUnitOfWork,
        mock_user_repo: MockUserRepository,
    }

    impl TestFixture {
        fn new() -> Self {
            TestFixture {
                mock_pw: MockPasswordHandler::new(),
                mock_uow: MockUnitOfWork::new(),
                mock_user_repo: MockUserRepository::new(),
            }
        }

        /// Configures mocks for the standard user success scenario
        fn expect_standard_user_success(
            &mut self,
            input_password: String,
            expected_hash: String,
            expected_user_output: User, // The user returned by UoW
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

            // No user repo calls expected for standard user creation
            self.mock_user_repo.expect_get_user().never();
        }

        /// Configures mocks for the admin unauthorized scenario
        fn expect_admin_unauthorized(
            &mut self,
            requester_uuid: Uuid,
            non_admin_requester: User, // The user object returned by get_user
        ) {
            // Expect the user repo to be called to check the requester's role
            self.mock_user_repo
                .expect_get_user()
                .with(eq(requester_uuid))
                .times(1)
                .return_once(move |_| Ok(non_admin_requester.clone())); // Return the non-admin user

            // IMPORTANT: Do NOT expect hash_password or execute, as the code should fail before them
            self.mock_pw.expect_hash_user_password().never(); // Explicitly expect NO call
            self.mock_uow.expect_execute::<User>().never(); // Explicitly expect NO call
        }

        /// Builds the use case instance, consuming the fixture
        fn build_use_case(
            self,
        ) -> CreateUserUseCase<MockPasswordHandler, MockUnitOfWork, MockUserRepository> {
            CreateUserUseCase::new(
                Arc::new(self.mock_pw),
                Arc::new(self.mock_uow),
                Arc::new(self.mock_user_repo),
            )
        }
    }

    #[tokio::test]
    async fn should_create_user() {
        // 1. Arrange
        let mut fixture = TestFixture::new(); // Create fixture

        // Define test data
        let input_user_name = "testuser".to_string();
        let input_password = "password123".to_string();
        let input_role = UserRole::Player;
        let input_corp_name = "TestCorp".to_string();
        let expected_hashed_password = "mock_hashed_password".to_string();
        let expected_user_uuid = Uuid::now_v7();

        let expected_user_output = User {
            uuid: expected_user_uuid,
            name: input_user_name.clone(),
            password_hash: expected_hashed_password.clone(),
            role: input_role.clone(),
        };

        // Configure mocks using the fixture method
        fixture.expect_standard_user_success(
            input_password.clone(),
            expected_hashed_password,
            expected_user_output.clone(),
        );

        // Build the use case from the fixture
        let uc = fixture.build_use_case();

        // 2. Act
        let result = uc
            .execute(
                None, // Standard user, no requester UUID
                input_user_name.clone(),
                input_password.clone(),
                input_role,
                input_corp_name.clone(),
            )
            .await;

        // 3. Assert
        assert!(result.is_ok());
        let created_user = result.unwrap();
        assert_eq!(created_user, expected_user_output);
    }

    #[tokio::test]
    async fn should_fail_creating_admin_user_when_requester_not_admin() {
        let mut fixture = TestFixture::new();

        let requesting_user_uuid = Uuid::now_v7();
        let input_user_name = "newadmin".to_string();
        let input_password = "password456".to_string();
        let input_role = UserRole::Admin;
        let input_corp_name = "AdminCorp".to_string();

        // Simulate the user fetched from the repo (NOT an admin)
        let non_admin_requester = User {
            uuid: requesting_user_uuid,
            name: "standard_requester".to_string(),
            password_hash: "somehash".to_string(),
            role: UserRole::Player, // <-- Crucial: Requester is Standard
        };

        fixture.expect_admin_unauthorized(requesting_user_uuid, non_admin_requester.clone());

        let uc = fixture.build_use_case();

        let result = uc
            .execute(
                Some(requesting_user_uuid), // Provide the requester's UUID
                input_user_name.clone(),
                input_password.clone(),
                input_role,
                input_corp_name.clone(),
            )
            .await;

        assert!(result.is_err());
        // Check for the specific error type
        match result.err().unwrap() {
            ApplicationError::Unauthorized => (),
            other_err => panic!(
                "Expected ApplicationError::Unauthorized, got {:?}",
                other_err
            ),
        }
    }
}
