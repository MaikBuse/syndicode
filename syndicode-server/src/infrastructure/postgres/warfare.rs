use super::PostgresDatabase;
use crate::domain::{
    model::warfare::UnitModel,
    repository::warfare::{WarfareDatabaseRepository, WarfareDatabaseResult},
};
use tonic::async_trait;
use uuid::Uuid;

#[async_trait]
impl WarfareDatabaseRepository for PostgresDatabase {
    async fn create_unit(&self, unit: UnitModel) -> WarfareDatabaseResult<UnitModel> {
        let unit = sqlx::query_as!(
            UnitModel,
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
        .fetch_one(&self.pool)
        .await?;

        Ok(unit)
    }

    async fn list_user_units(&self, user_uuid: Uuid) -> WarfareDatabaseResult<Vec<UnitModel>> {
        let units = sqlx::query_as!(
            UnitModel,
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
        .fetch_all(&self.pool)
        .await?;

        Ok(units)
    }
}
