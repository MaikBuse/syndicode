use super::uow::PgTransactionContext;
use crate::domain::{
    repository::RepositoryResult,
    unit::{
        model::Unit,
        repository::{UnitRepository, UnitTxRespository},
    },
};
use sqlx::{PgPool, Postgres};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct PgUnitRepository;

impl PgUnitRepository {
    pub async fn create_unit(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        unit: Unit,
    ) -> RepositoryResult<Unit> {
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

    pub async fn list_units(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        user_uuid: Uuid,
    ) -> RepositoryResult<Vec<Unit>> {
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

pub struct PgUnitService {
    pool: Arc<PgPool>,
    unit_repo: PgUnitRepository,
}

impl PgUnitService {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self {
            pool,
            unit_repo: PgUnitRepository,
        }
    }
}

#[tonic::async_trait]
impl UnitRepository for PgUnitService {
    async fn list_units(&self, user_uuid: Uuid) -> RepositoryResult<Vec<Unit>> {
        self.unit_repo.list_units(&*self.pool, user_uuid).await
    }

    async fn create_unit(&self, unit: Unit) -> RepositoryResult<Unit> {
        self.unit_repo.create_unit(&*self.pool, unit).await
    }
}

#[tonic::async_trait]
impl<'a, 'tx> UnitTxRespository for PgTransactionContext<'a, 'tx> {
    async fn list_units(&mut self, user_uuid: Uuid) -> RepositoryResult<Vec<Unit>> {
        self.unit_repo.list_units(&mut **self.tx, user_uuid).await
    }

    async fn create_unit(&mut self, unit: Unit) -> RepositoryResult<Unit> {
        self.unit_repo.create_unit(&mut **self.tx, unit).await
    }
}
