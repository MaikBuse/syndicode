use super::{DatabaseResult, PostgresDatabase};
use crate::domain::unit::Unit;
use sqlx::Postgres;
use uuid::Uuid;

impl PostgresDatabase {
    pub async fn create_unit<'e, E>(executor: E, unit: Unit) -> DatabaseResult<Unit>
    where
        E: sqlx::Executor<'e, Database = Postgres> + Send,
    {
        let unit = sqlx::query_as!(
            Unit,
            r#"
            INSERT INTO units (
                uuid,
                user_uuid
            )
            VALUES ( $1, $2 )
            RETURNING uuid, user_uuid
            "#,
            unit.uuid,
            unit.user_uuid
        )
        .fetch_one(executor)
        .await?;

        Ok(unit)
    }

    pub async fn list_user_units<'e, E>(executor: E, user_uuid: Uuid) -> DatabaseResult<Vec<Unit>>
    where
        E: sqlx::Executor<'e, Database = Postgres> + Send,
    {
        let units = sqlx::query_as!(
            Unit,
            r#"
            SELECT
                uuid,
                user_uuid
            FROM units
            WHERE
                user_uuid = $1
            "#,
            user_uuid
        )
        .fetch_all(executor)
        .await?;

        Ok(units)
    }
}
