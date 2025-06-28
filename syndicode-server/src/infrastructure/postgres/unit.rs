use super::{game_tick::PgGameTickRepository, uow::PgTransactionContext, PostgresDatabase};
use crate::domain::{
    repository::RepositoryResult,
    unit::{
        model::Unit,
        repository::{ListUnitsOutcome, UnitRepository, UnitTxRespository},
    },
};
use sqlx::{Executor, Postgres};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct PgUnitRepository;

impl PgUnitRepository {
    pub async fn insert_units_in_tick(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        units: &[Unit],
        game_tick: i64,
    ) -> RepositoryResult<()> {
        // If there are no units, we don't need to do anything.
        if units.is_empty() {
            return Ok(());
        }

        // Prepare separate vectors for each column to be bulk inserted.
        // Pre-allocate capacity for efficiency.
        let count = units.len();
        let mut uuids = Vec::with_capacity(count);
        let mut corporation_uuids = Vec::with_capacity(count);

        for unit in units {
            uuids.push(unit.uuid);
            corporation_uuids.push(unit.corporation_uuid);
        }

        // Execute the bulk insert query using UNNEST
        sqlx::query!(
            r#"
            INSERT INTO units (
                game_tick,
                uuid,
                corporation_uuid
            )
            SELECT
                $1 as game_tick,
                unnest($2::UUID[]) as uuid,
                unnest($3::UUID[]) as corporation_uuid
            "#,
            game_tick,
            &uuids,
            &corporation_uuids,
        )
        .execute(executor)
        .await?;

        Ok(())
    }

    pub async fn list_units_in_tick(
        &self,
        executor: impl Executor<'_, Database = Postgres>,
        game_tick: i64,
    ) -> RepositoryResult<Vec<Unit>> {
        let units = sqlx::query_as!(
            Unit,
            r#"
            SELECT
                uuid,
                corporation_uuid
            FROM units
            WHERE
                game_tick = $1
            "#,
            game_tick
        )
        .fetch_all(executor)
        .await?;

        Ok(units)
    }

    /// Retrieves all units (UUID and owner UUID) belonging to a specific corporation
    /// at a given game tick.
    pub async fn list_corporation_units_at_tick(
        &self,
        executor: impl Executor<'_, Database = Postgres>,
        corporation_uuid: Uuid,
        game_tick: i64,
    ) -> RepositoryResult<Vec<Unit>> {
        let units = sqlx::query_as!(
            Unit,
            r#"
            SELECT
                uuid,
                corporation_uuid
            FROM units
            WHERE
                corporation_uuid = $1
                AND game_tick = $2
            "#,
            corporation_uuid,
            game_tick
        )
        .fetch_all(executor)
        .await?;

        Ok(units)
    }

    pub async fn delete_units_before_tick(
        &self,
        executor: impl Executor<'_, Database = Postgres>,
        game_tick: i64,
    ) -> RepositoryResult<u64> {
        let result = sqlx::query!(
            r#"
             DELETE FROM units
             WHERE
                game_tick < $1
             "#,
            game_tick
        )
        .execute(executor)
        .await?;

        Ok(result.rows_affected())
    }
}

pub struct PgUnitService {
    pg_db: Arc<PostgresDatabase>,
    game_tick_repo: PgGameTickRepository,
    unit_repo: PgUnitRepository,
}

impl PgUnitService {
    pub fn new(pg_db: Arc<PostgresDatabase>) -> Self {
        Self {
            pg_db,
            game_tick_repo: PgGameTickRepository,
            unit_repo: PgUnitRepository,
        }
    }
}

#[tonic::async_trait]
impl UnitRepository for PgUnitService {
    async fn list_units_in_tick(&self, game_tick: i64) -> RepositoryResult<Vec<Unit>> {
        self.unit_repo
            .list_units_in_tick(&self.pg_db.pool, game_tick)
            .await
    }

    async fn list_units_by_corporation(
        &self,
        corporation_uuid: Uuid,
    ) -> RepositoryResult<ListUnitsOutcome> {
        let game_tick = self
            .game_tick_repo
            .get_current_game_tick(&self.pg_db.pool)
            .await?;

        let units = self
            .unit_repo
            .list_corporation_units_at_tick(&self.pg_db.pool, corporation_uuid, game_tick)
            .await?;

        Ok(ListUnitsOutcome { game_tick, units })
    }
}

#[tonic::async_trait]
impl UnitTxRespository for PgTransactionContext<'_, '_> {
    async fn insert_units_in_tick(
        &mut self,
        game_tick: i64,
        units: &[Unit],
    ) -> RepositoryResult<()> {
        self.unit_repo
            .insert_units_in_tick(&mut **self.tx, units, game_tick)
            .await
    }

    async fn delete_units_before_tick(&mut self, game_tick: i64) -> RepositoryResult<u64> {
        self.unit_repo
            .delete_units_before_tick(&mut **self.tx, game_tick)
            .await
    }
}
