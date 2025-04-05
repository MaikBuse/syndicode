use super::{game_tick::PgGameTickRepository, uow::PgTransactionContext};
use crate::domain::{
    repository::RepositoryResult,
    unit::{
        model::Unit,
        repository::{UnitRepository, UnitTxRespository},
    },
};
use sqlx::{Executor, PgPool, Postgres};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct PgUnitRepository;

impl PgUnitRepository {
    /// Inserts a new state record for a unit (representing its existence and ownership)
    /// at a specific game tick.
    pub async fn insert_unit(
        &self,
        executor: impl Executor<'_, Database = Postgres>,
        unit: &Unit,
        game_tick: i64,
    ) -> RepositoryResult<()> {
        sqlx::query!(
            r#"
            INSERT INTO units (
                game_tick,
                uuid,
                user_uuid
            )
            VALUES ($1, $2, $3)
            "#,
            game_tick,
            unit.uuid,
            unit.user_uuid
        )
        .execute(executor)
        .await?;

        Ok(())
    }

    /// Retrieves all units (UUID and owner UUID) belonging to a specific user
    /// at a given game tick.
    pub async fn list_user_units_at_tick(
        &self,
        executor: impl Executor<'_, Database = Postgres>,
        user_uuid: Uuid,
        game_tick: i64,
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
                AND game_tick = $2
            "#,
            user_uuid,
            game_tick
        )
        .fetch_all(executor)
        .await?;

        Ok(units)
    }

    /// Retrieves the state (UUID and owner UUID) of a specific unit
    /// by its UUID at a given game tick.
    pub async fn get_unit_state_by_uuid_at_tick(
        &self,
        executor: impl Executor<'_, Database = Postgres>,
        unit_uuid: Uuid,
        game_tick: i64,
    ) -> RepositoryResult<Option<Unit>> {
        // Return Option<Unit>
        // Select only the columns present in the schema and matching the Unit struct
        let unit = sqlx::query_as!(
            Unit,
            r#"
            SELECT
                uuid,
                user_uuid
            FROM units
            WHERE
                uuid = $1
                AND game_tick = $2
            "#,
            unit_uuid,
            game_tick
        )
        .fetch_optional(executor)
        .await?;

        Ok(unit)
    }

    /// Deletes ALL historical state for a specific unit UUID across ALL ticks.
    pub async fn delete_unit_history<'e>(
        &self,
        executor: impl Executor<'e, Database = Postgres>,
        unit_uuid: Uuid,
    ) -> RepositoryResult<u64> {
        let result = sqlx::query!(
            r#"
             DELETE FROM units
             WHERE uuid = $1
             "#,
            unit_uuid
        )
        .execute(executor)
        .await?;

        Ok(result.rows_affected())
    }
}

pub struct PgUnitService {
    pool: Arc<PgPool>,
    game_tick_repo: PgGameTickRepository,
    unit_repo: PgUnitRepository,
}

impl PgUnitService {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self {
            pool,
            game_tick_repo: PgGameTickRepository,
            unit_repo: PgUnitRepository,
        }
    }
}

#[tonic::async_trait]
impl UnitRepository for PgUnitService {
    async fn list_units(&self, user_uuid: Uuid) -> RepositoryResult<Vec<Unit>> {
        let game_tick = self
            .game_tick_repo
            .get_current_game_tick(&*self.pool)
            .await?;

        self.unit_repo
            .list_user_units_at_tick(&*self.pool, user_uuid, game_tick)
            .await
    }

    async fn insert_unit(&self, unit: &Unit) -> RepositoryResult<()> {
        let game_tick = self
            .game_tick_repo
            .get_current_game_tick(&*self.pool)
            .await?;

        self.unit_repo
            .insert_unit(&*self.pool, unit, game_tick)
            .await
    }
}

#[tonic::async_trait]
impl UnitTxRespository for PgTransactionContext<'_, '_> {
    async fn list_units(&mut self, user_uuid: Uuid) -> RepositoryResult<Vec<Unit>> {
        let game_tick = self
            .game_tick_repo
            .get_current_game_tick(&mut **self.tx)
            .await?;

        self.unit_repo
            .list_user_units_at_tick(&mut **self.tx, user_uuid, game_tick)
            .await
    }

    async fn insert_unit(&mut self, unit: &Unit) -> RepositoryResult<()> {
        let game_tick = self
            .game_tick_repo
            .get_current_game_tick(&mut **self.tx)
            .await?;

        self.unit_repo
            .insert_unit(&mut **self.tx, unit, game_tick)
            .await
    }
}
