use crate::{
    application::error::{ApplicationError, ApplicationResult},
    domain::{
        repository::user::UserRepository,
        user::{role::UserRole, User},
    },
};
use std::sync::Arc;
use uuid::Uuid;

pub struct GetUserUseCase {
    user_repo: Arc<dyn UserRepository>,
}

impl GetUserUseCase {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
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
