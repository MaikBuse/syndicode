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
            session_uuid,
            user_uuid,
            name,
            balance
        )
        VALUES (
            $1, $2, $3, $4, $5
        )
        RETURNING uuid, session_uuid, user_uuid, name, balance
        "#,
            corporation.uuid,
            corporation.session_uuid,
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
        session_uuid: Uuid,
        user_uuid: Uuid,
    ) -> EconomyDatabaseResult<CorporationModel> {
        let corporation = sqlx::query_as!(
            CorporationModel,
            r#"
            SELECT
                uuid,
                session_uuid,
                user_uuid,
                name,
                balance
            FROM corporations
            WHERE
                session_uuid = $1
                AND user_uuid = $2
            "#,
            session_uuid,
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
                session_uuid = $2,
                user_uuid = $3,
                name = $4,
                balance = $5
            WHERE uuid = $1
            RETURNING uuid, session_uuid, user_uuid, name, balance
            "#,
            corporation.uuid,
            corporation.session_uuid,
            corporation.user_uuid,
            corporation.name,
            corporation.balance
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(corporation)
    }
}
