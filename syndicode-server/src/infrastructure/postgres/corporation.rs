use super::uow::PgTransactionContext;
use crate::domain::{
    corporation::Corporation,
    repository::{
        corporation::{CorporationRepository, CorporationTxRepository},
        RepositoryResult,
    },
};
use sqlx::{PgPool, Postgres};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct PgCorporationRepository;

impl PgCorporationRepository {
    pub async fn create_corporation(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        corporation: Corporation,
    ) -> RepositoryResult<Corporation> {
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

    pub async fn get_corporation_by_user(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        user_uuid: Uuid,
    ) -> RepositoryResult<Corporation> {
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

    pub async fn update_corporation(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        corporation: Corporation,
    ) -> RepositoryResult<Corporation> {
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

pub struct PgCorporationService {
    pool: Arc<PgPool>,
    corporation_repo: PgCorporationRepository,
}

impl PgCorporationService {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self {
            pool,
            corporation_repo: PgCorporationRepository,
        }
    }
}

#[tonic::async_trait]
impl CorporationRepository for PgCorporationService {
    async fn get_corporation_by_user(&self, user_uuid: Uuid) -> RepositoryResult<Corporation> {
        self.corporation_repo
            .get_corporation_by_user(&*self.pool, user_uuid)
            .await
    }

    async fn create_corporation(&self, corporation: Corporation) -> RepositoryResult<Corporation> {
        self.corporation_repo
            .create_corporation(&*self.pool, corporation)
            .await
    }

    async fn update_corporation(&self, corporation: Corporation) -> RepositoryResult<Corporation> {
        self.corporation_repo
            .update_corporation(&*self.pool, corporation)
            .await
    }
}

#[tonic::async_trait]
impl<'a, 'tx> CorporationTxRepository for PgTransactionContext<'a, 'tx> {
    async fn get_corporation_by_user(&mut self, user_uuid: Uuid) -> RepositoryResult<Corporation> {
        self.corporation_repo
            .get_corporation_by_user(&mut **self.tx, user_uuid)
            .await
    }

    async fn create_corporation(
        &mut self,
        corporation: Corporation,
    ) -> RepositoryResult<Corporation> {
        self.corporation_repo
            .create_corporation(&mut **self.tx, corporation)
            .await
    }

    async fn update_corporation(
        &mut self,
        corporation: Corporation,
    ) -> RepositoryResult<Corporation> {
        self.corporation_repo
            .update_corporation(&mut **self.tx, corporation)
            .await
    }
}
