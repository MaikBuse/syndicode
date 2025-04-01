use super::{DatabaseResult, PostgresDatabase};
use crate::domain::corporation::Corporation;
use sqlx::Postgres;
use uuid::Uuid;

impl PostgresDatabase {
    pub async fn create_corporation<'e, E>(
        executor: E,
        corporation: Corporation,
    ) -> DatabaseResult<Corporation>
    where
        E: sqlx::Executor<'e, Database = Postgres> + Send,
    {
        let corporation = sqlx::query_as!(
            Corporation,
            r#"
            INSERT INTO corporations (
            uuid,
            user_uuid,
            name,
            balance
        )
        VALUES (
            $1, $2, $3, $4
        )
        RETURNING uuid, user_uuid, name, balance
        "#,
            corporation.uuid,
            corporation.user_uuid,
            corporation.name,
            corporation.balance
        )
        .fetch_one(executor)
        .await?;

        Ok(corporation)
    }

    pub async fn get_user_corporation<'e, E>(
        executor: E,
        user_uuid: Uuid,
    ) -> DatabaseResult<Corporation>
    where
        E: sqlx::Executor<'e, Database = Postgres> + Send,
    {
        let corporation = sqlx::query_as!(
            Corporation,
            r#"
            SELECT
                uuid,
                user_uuid,
                name,
                balance
            FROM corporations
            WHERE
                user_uuid = $1
            "#,
            user_uuid
        )
        .fetch_one(executor)
        .await?;

        Ok(corporation)
    }

    pub async fn update_corporation<'e, E>(
        executor: E,
        corporation: Corporation,
    ) -> DatabaseResult<Corporation>
    where
        E: sqlx::Executor<'e, Database = Postgres> + Send,
    {
        let corporation = sqlx::query_as!(
            Corporation,
            r#"
            UPDATE corporations
            SET
                uuid = $1,
                user_uuid = $2,
                name = $3,
                balance = $4
            WHERE uuid = $1
            RETURNING uuid, user_uuid, name, balance
            "#,
            corporation.uuid,
            corporation.user_uuid,
            corporation.name,
            corporation.balance
        )
        .fetch_one(executor)
        .await?;

        Ok(corporation)
    }
}
