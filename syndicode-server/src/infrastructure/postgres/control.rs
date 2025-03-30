use super::PostgresDatabase;
use crate::domain::{
    model::control::UserModel,
    repository::control::{ControlDatabaseError, ControlDatabaseRepository, ControlDatabaseResult},
};
use tonic::async_trait;
use uuid::Uuid;

#[async_trait]
impl ControlDatabaseRepository for PostgresDatabase {
    async fn create_user(&self, user: UserModel) -> ControlDatabaseResult<UserModel> {
        let user_role: i16 = user.role.into();

        match sqlx::query_as!(
            UserModel,
            r#"
            INSERT INTO users (
                uuid,
                name,
                password_hash,
                role
            )
            VALUES ( $1, $2, $3, $4 )
            RETURNING
                uuid,
                name,
                password_hash,
                role
            "#,
            user.uuid,
            user.name,
            user.password_hash,
            user_role
        )
        .fetch_one(&self.pool)
        .await
        {
            Ok(user) => Ok(user),
            Err(err) => match err {
                sqlx::Error::Database(database_error) => {
                    match database_error.is_unique_violation() {
                        true => Err(ControlDatabaseError::UniqueConstraint),
                        false => Err(anyhow::anyhow!("{}", database_error.to_string()).into()),
                    }
                }
                _ => Err(err.into()),
            },
        }
    }

    async fn get_user(&self, user_uuid: Uuid) -> ControlDatabaseResult<UserModel> {
        let user = sqlx::query_as!(
            UserModel,
            r#"
            SELECT
                uuid,
                name,
                password_hash,
                role
            FROM users
            WHERE
                uuid = $1
            "#,
            user_uuid
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn get_user_by_name(&self, username: String) -> ControlDatabaseResult<UserModel> {
        let user = sqlx::query_as!(
            UserModel,
            r#"
            SELECT
                uuid,
                name,
                password_hash,
                role
            FROM users
            WHERE
                name = $1
            "#,
            username
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn delete_user(&self, user_uuid: Uuid) -> ControlDatabaseResult<()> {
        sqlx::query!(
            r#"
            DELETE FROM users
            WHERE uuid = $1
            "#,
            user_uuid
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
