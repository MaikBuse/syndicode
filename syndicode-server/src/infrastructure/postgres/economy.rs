use crate::domain::{
    model::economy::CorporationModel,
    repository::economy::{EconomyDatabaseRepository, EconomyDatabaseResult},
};
use tonic::async_trait;
use uuid::Uuid;

use super::PostgresDatabase;

#[async_trait]
impl EconomyDatabaseRepository for PostgresDatabase {
    async fn create_corporation(
        &self,
        corporation: CorporationModel,
    ) -> EconomyDatabaseResult<CorporationModel> {
        let corporation = sqlx::query_as!(
            CorporationModel,
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
        .fetch_one(&self.pool)
        .await?;

        Ok(corporation)
    }

    async fn get_user_corporation(
        &self,
        user_uuid: Uuid,
    ) -> EconomyDatabaseResult<CorporationModel> {
        let corporation = sqlx::query_as!(
            CorporationModel,
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
        .fetch_one(&self.pool)
        .await?;

        Ok(corporation)
    }

    async fn update_corporation(
        &self,
        corporation: CorporationModel,
    ) -> EconomyDatabaseResult<CorporationModel> {
        let corporation = sqlx::query_as!(
            CorporationModel,
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
        .fetch_one(&self.pool)
        .await?;

        Ok(corporation)
    }
}
