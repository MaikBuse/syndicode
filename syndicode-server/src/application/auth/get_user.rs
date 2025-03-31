use crate::{
    application::error::{ApplicationError, ApplicationResult},
    domain::user::{role::UserRole, User},
    infrastructure::postgres::PostgresDatabase,
};
use std::sync::Arc;
use uuid::Uuid;

pub struct GetUserUseCase {
    db: Arc<PostgresDatabase>,
}

impl GetUserUseCase {
    pub fn new(db: Arc<PostgresDatabase>) -> Self {
        Self { db }
    }

    pub async fn execute(&self, req_user_uuid: Uuid, user_uuid: Uuid) -> ApplicationResult<User> {
        if req_user_uuid != user_uuid {
            let req_user = PostgresDatabase::get_user(&self.db.pool, req_user_uuid).await?;

            if req_user.role != UserRole::Admin {
                return Err(ApplicationError::Unauthorized);
            }
        }

        Ok(PostgresDatabase::get_user(&self.db.pool, user_uuid).await?)
    }
}
