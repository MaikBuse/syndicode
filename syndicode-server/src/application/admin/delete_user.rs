use crate::{
    application::error::{ApplicationError, ApplicationResult},
    domain::user::role::UserRole,
    infrastructure::postgres::PostgresDatabase,
};
use std::sync::Arc;
use uuid::Uuid;

pub struct DeleteUserUseCase {
    db: Arc<PostgresDatabase>,
}

impl DeleteUserUseCase {
    pub fn new(db: Arc<PostgresDatabase>) -> Self {
        Self { db }
    }

    pub async fn execute(&self, req_user_uuid: Uuid, user_uuid: Uuid) -> ApplicationResult<()> {
        if req_user_uuid != user_uuid {
            let req_user = PostgresDatabase::get_user(&self.db.pool, req_user_uuid).await?;

            if req_user.role != UserRole::Admin {
                return Err(ApplicationError::Unauthorized);
            }
        }

        // The corporation automatically gets deleted with the user
        PostgresDatabase::delete_user(&self.db.pool, user_uuid).await?;

        Ok(())
    }
}
