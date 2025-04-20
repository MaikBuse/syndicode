use crate::{
    application::error::{ApplicationError, ApplicationResult},
    domain::{user::model::role::UserRole, user::repository::UserRepository},
};
use bon::Builder;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Builder)]
pub struct DeleteUserUseCase<USR>
where
    USR: UserRepository,
{
    user_repo: Arc<USR>,
}

impl<USR> DeleteUserUseCase<USR>
where
    USR: UserRepository,
{
    pub async fn execute(&self, req_user_uuid: Uuid, user_uuid: Uuid) -> ApplicationResult<()> {
        if req_user_uuid != user_uuid {
            let req_user = self.user_repo.get_user(req_user_uuid).await?;

            if req_user.role != UserRole::Admin {
                return Err(ApplicationError::Unauthorized);
            }
        }

        // The corporation automatically gets deleted with the user
        self.user_repo.delete_user(user_uuid).await?;

        Ok(())
    }
}
