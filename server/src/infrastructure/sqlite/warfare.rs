use super::SqliteDatabase;
use crate::domain::{
    model::warfare::UnitModel,
    repository::warfare::{WarfareDatabaseRepository, WarfareDatabaseResult},
};
use tonic::async_trait;

#[async_trait]
impl WarfareDatabaseRepository for SqliteDatabase {
    async fn create_unit(&self, unit: UnitModel) -> WarfareDatabaseResult<UnitModel> {
        let unit = sqlx::query_as!(
            UnitModel,
            r#"
            INSERT INTO units (
                uuid,
                session_uuid,
                user_uuid
            )
            VALUES ( ?1, ?2, ?3 )
            RETURNING uuid, session_uuid, user_uuid
            "#,
            unit.uuid,
            unit.session_uuid,
            unit.user_uuid
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(unit)
    }

    async fn list_user_units(
        &self,
        session_uuid: Vec<u8>,
        user_uuid: Vec<u8>,
    ) -> WarfareDatabaseResult<Vec<UnitModel>> {
        let units = sqlx::query_as!(
            UnitModel,
            r#"
            SELECT
                uuid,
                session_uuid,
                user_uuid
            FROM units
            WHERE
                session_uuid = ?1
                AND user_uuid = ?2
            "#,
            session_uuid,
            user_uuid
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(units)
    }
}
