use crate::{
    application::error::{ApplicationError, ApplicationResult},
    domain::{
        user::repository::UserRepository,
        user::{model::role::UserRole, model::User},
    },
};
use std::sync::Arc;
use uuid::Uuid;

pub struct GetUserUseCase<USR>
where
    USR: UserRepository,
{
    user_repo: Arc<USR>,
}

impl<USR> GetUserUseCase<USR>
where
    USR: UserRepository,
{
    pub fn new(user_repo: Arc<USR>) -> Self {
        Self { user_repo }
    }

    pub async fn execute(&self, req_user_uuid: Uuid, user_uuid: Uuid) -> ApplicationResult<User> {
        if req_user_uuid != user_uuid {
            let req_user = self.user_repo.get_user(req_user_uuid).await?;

            if req_user.role != UserRole::Admin {
                return Err(ApplicationError::Unauthorized);
            }
        }

        Ok(self.user_repo.get_user(user_uuid).await?)
    }
}
