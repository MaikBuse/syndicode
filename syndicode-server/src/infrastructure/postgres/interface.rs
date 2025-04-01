use super::{DatabaseError, DatabaseResult, PostgresDatabase};
use crate::domain::user::User;
use sqlx::Postgres;
use uuid::Uuid;

impl PostgresDatabase {
    pub async fn create_user<'e, E>(executor: E, user: User) -> DatabaseResult<User>
    where
        E: sqlx::Executor<'e, Database = Postgres> + Send,
    {
        let user_role: i16 = user.role.into();

        match sqlx::query_as!(
            User,
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
        .fetch_one(executor)
        .await
        {
            Ok(user) => Ok(user),
            Err(err) => match err {
                sqlx::Error::Database(database_error) => match database_error.is_unique_violation()
                {
                    true => Err(DatabaseError::UniqueConstraint),
                    false => Err(anyhow::anyhow!("{}", database_error.to_string()).into()),
                },
                _ => Err(err.into()),
            },
        }
    }

    pub async fn get_user<'e, E>(executor: E, user_uuid: Uuid) -> DatabaseResult<User>
    where
        E: sqlx::Executor<'e, Database = Postgres> + Send,
    {
        let user = sqlx::query_as!(
            User,
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
        .fetch_one(executor)
        .await?;

        Ok(user)
    }

    pub async fn get_user_by_name<'e, E>(executor: E, username: String) -> DatabaseResult<User>
    where
        E: sqlx::Executor<'e, Database = Postgres> + Send,
    {
        let user = sqlx::query_as!(
            User,
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
        .fetch_one(executor)
        .await?;

        Ok(user)
    }

    pub async fn delete_user<'e, E>(executor: E, user_uuid: Uuid) -> DatabaseResult<()>
    where
        E: sqlx::Executor<'e, Database = Postgres> + Send,
    {
        sqlx::query!(
            r#"
            DELETE FROM users
            WHERE uuid = $1
            "#,
            user_uuid
        )
        .execute(executor)
        .await?;

        Ok(())
    }
}
