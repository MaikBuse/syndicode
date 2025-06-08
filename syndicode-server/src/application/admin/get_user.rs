use crate::{
    application::error::{ApplicationError, ApplicationResult},
    domain::user::{
        model::{role::UserRole, status::UserStatus, User},
        repository::UserRepository,
    },
};
use bon::{bon, Builder};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Builder)]
pub struct GetUserUseCase<USR>
where
    USR: UserRepository,
{
    user_repo: Arc<USR>,
}

#[bon]
impl<USR> GetUserUseCase<USR>
where
    USR: UserRepository,
{
    #[builder]
    pub async fn execute(&self, req_user_uuid: Uuid, user_uuid: Uuid) -> ApplicationResult<User> {
        if req_user_uuid != user_uuid {
            let req_user = self.user_repo.get_user(req_user_uuid).await?;

            if req_user.role != UserRole::Admin || req_user.status != UserStatus::Active {
                return Err(ApplicationError::Unauthorized);
            }
        }

        Ok(self.user_repo.get_user(user_uuid).await?)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::error::ApplicationError;
    use crate::domain::user::model::email::UserEmail;
    use crate::domain::user::model::name::UserName;
    use crate::domain::user::model::status::UserStatus;
    use crate::domain::user::model::{role::UserRole, User};
    use crate::domain::user::repository::MockUserRepository;
    use mockall::predicate::*;
    use std::sync::Arc;
    use uuid::Uuid;

    fn make_user(uuid: Uuid, role: UserRole) -> User {
        User {
            uuid,
            email: UserEmail::new("test@example.com".to_string()).unwrap(),
            role,
            name: UserName::new("Some-Name".to_string()).unwrap(),
            password_hash: "Password-Hash".to_string(),
            status: UserStatus::Active,
        }
    }

    #[tokio::test]
    async fn test_execute_as_admin() {
        let admin_uuid = Uuid::now_v7();
        let user_uuid = Uuid::now_v7();
        let admin = make_user(admin_uuid, UserRole::Admin);
        let user = make_user(user_uuid, UserRole::Player);

        let mut mock = MockUserRepository::new();
        mock.expect_get_user()
            .with(eq(admin_uuid))
            .returning(move |_| Ok(admin.clone()));
        mock.expect_get_user()
            .with(eq(user_uuid))
            .returning(move |_| Ok(user.clone()));

        let use_case = GetUserUseCase::builder().user_repo(Arc::new(mock)).build();
        let result = use_case
            .execute()
            .req_user_uuid(admin_uuid)
            .user_uuid(user_uuid)
            .call()
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().uuid, user_uuid);
    }

    #[tokio::test]
    async fn test_execute_as_self() {
        let user_uuid = Uuid::now_v7();
        let user = make_user(user_uuid, UserRole::Player);

        let mut mock = MockUserRepository::new();
        mock.expect_get_user()
            .with(eq(user_uuid))
            .returning(move |_| Ok(user.clone()));

        let use_case = GetUserUseCase::builder().user_repo(Arc::new(mock)).build();
        let result = use_case
            .execute()
            .req_user_uuid(user_uuid)
            .user_uuid(user_uuid)
            .call()
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().uuid, user_uuid);
    }

    #[tokio::test]
    async fn test_execute_unauthorized() {
        let req_user_uuid = Uuid::now_v7();
        let user_uuid = Uuid::now_v7();
        let req_user = make_user(req_user_uuid, UserRole::Player);

        let mut mock = MockUserRepository::new();
        mock.expect_get_user()
            .with(eq(req_user_uuid))
            .returning(move |_| Ok(req_user.clone()));

        let use_case = GetUserUseCase::builder().user_repo(Arc::new(mock)).build();
        let result = use_case
            .execute()
            .req_user_uuid(req_user_uuid)
            .user_uuid(user_uuid)
            .call()
            .await;
        assert!(matches!(result, Err(ApplicationError::Unauthorized)));
    }
}
