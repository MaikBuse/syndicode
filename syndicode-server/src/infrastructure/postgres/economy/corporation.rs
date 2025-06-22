use crate::{
    domain::{
        economy::corporation::{
            model::Corporation,
            repository::{CorporationRepository, CorporationTxRepository, GetCorporationOutcome},
        },
        repository::{RepositoryError, RepositoryResult},
    },
    infrastructure::postgres::{
        game_tick::PgGameTickRepository, uow::PgTransactionContext, PostgresDatabase,
    },
};
use sqlx::Postgres;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct PgCorporationRepository;

impl PgCorporationRepository {
    pub async fn create_corporation(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        corporation: &Corporation,
        game_tick: i64,
    ) -> RepositoryResult<()> {
        let result = sqlx::query!(
            r#"
            INSERT INTO corporations (
                game_tick,
                uuid,
                user_uuid,
                name,
                cash_balance
            )
            VALUES ($1, $2, $3, $4, $5)
            "#,
            game_tick,
            corporation.uuid,
            corporation.user_uuid,
            corporation.name.to_string(),
            corporation.cash_balance
        )
        .execute(executor)
        .await;

        if let Err(err) = result {
            tracing::warn!("Failed to create new corporation with error: {}", err);

            return Err(err.into());
        }

        Ok(())
    }

    /// Inserts the new state corporations at a specific game tick.
    /// This is used when advancing the game state from N to N+1.
    pub async fn insert_corporations_in_tick(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        corporations: Vec<Corporation>,
        game_tick: i64,
    ) -> RepositoryResult<()> {
        // If there are no corporations, we don't need to do anything.
        if corporations.is_empty() {
            return Ok(());
        }

        // Prepare separate vectors for each column to be bulk inserted.
        // Pre-allocate capacity for efficiency.
        let count = corporations.len();
        let mut uuids = Vec::with_capacity(count);
        let mut user_uuids = Vec::with_capacity(count);
        let mut names = Vec::with_capacity(count);
        let mut cash_balances = Vec::with_capacity(count);

        for corp in corporations {
            uuids.push(corp.uuid);
            user_uuids.push(corp.user_uuid);
            names.push(corp.name.to_string());
            cash_balances.push(corp.cash_balance);
        }

        // Execute the bulk insert query using UNNEST
        sqlx::query!(
            r#"
            INSERT INTO corporations (
                game_tick,
                uuid,
                user_uuid,
                name,
                cash_balance
            )
            SELECT
                $1 as game_tick,
                unnest($2::UUID[]) as uuid,
                unnest($3::UUID[]) as user_uuid,
                unnest($4::TEXT[]) as name,
                unnest($5::BIGINT[]) as cash_balance
            "#,
            game_tick,
            &uuids,
            &user_uuids,
            &names,
            &cash_balances
        )
        .execute(executor)
        .await?;

        Ok(())
    }

    /// Retrieves the state of a specific user's corporation at a given game tick.
    /// This is typically used by clients reading the "current" game state.
    pub async fn get_corporation_by_user_at_tick(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        user_uuid: Uuid,
        game_tick: i64,
    ) -> RepositoryResult<Corporation> {
        let corporation = sqlx::query_as!(
            Corporation,
            r#"
            SELECT
                uuid,
                user_uuid,
                name,
                cash_balance
            FROM corporations
            WHERE
                user_uuid = $1
                AND game_tick = $2
            "#,
            user_uuid,
            game_tick
        )
        .fetch_one(executor)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => todo!(),
            _ => RepositoryError::from(err),
        })?;

        Ok(corporation)
    }

    /// Retrieves the state of a specific corporation (by its name) at a given game tick.
    pub async fn get_corporation_by_name_at_tick(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        corporation_name: String,
        game_tick: i64,
    ) -> RepositoryResult<Corporation> {
        // Return Option<>
        let corporation = sqlx::query_as!(
            Corporation,
            r#"
            SELECT
                uuid,
                user_uuid,
                name,
                cash_balance
            FROM corporations
            WHERE
                name = $1
                AND game_tick = $2
            "#,
            corporation_name,
            game_tick
        )
        .fetch_one(executor)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => RepositoryError::NotFound,
            _ => RepositoryError::from(err),
        })?;

        Ok(corporation)
    }

    pub async fn list_corporations_in_tick(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        game_tick: i64,
    ) -> RepositoryResult<Vec<Corporation>> {
        let corporations = sqlx::query_as!(
            Corporation,
            r#"
            SELECT
                uuid,
                user_uuid,
                name,
                cash_balance
            FROM corporations
            WHERE
                game_tick = $1
            "#,
            game_tick
        )
        .fetch_all(executor)
        .await?;

        Ok(corporations)
    }

    pub async fn delete_corporations_before_tick(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        game_tick: i64,
    ) -> RepositoryResult<u64> {
        let result = sqlx::query!(
            r#"
            DELETE FROM corporations
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

pub struct PgCorporationService {
    pg_db: Arc<PostgresDatabase>,
    game_tick_repo: PgGameTickRepository,
    corporation_repo: PgCorporationRepository,
}

impl PgCorporationService {
    pub fn new(pg_db: Arc<PostgresDatabase>) -> Self {
        Self {
            pg_db,
            game_tick_repo: PgGameTickRepository,
            corporation_repo: PgCorporationRepository,
        }
    }
}

#[tonic::async_trait]
impl CorporationRepository for PgCorporationService {
    async fn get_corporation_by_user(
        &self,
        user_uuid: Uuid,
    ) -> RepositoryResult<GetCorporationOutcome> {
        let game_tick = self
            .game_tick_repo
            .get_current_game_tick(&self.pg_db.pool)
            .await?;

        let corporation = self
            .corporation_repo
            .get_corporation_by_user_at_tick(&self.pg_db.pool, user_uuid, game_tick)
            .await?;

        Ok(GetCorporationOutcome {
            game_tick,
            corporation,
        })
    }
    async fn get_corporation_by_name(
        &self,
        corporation_name: String,
    ) -> RepositoryResult<Corporation> {
        let game_tick = self
            .game_tick_repo
            .get_current_game_tick(&self.pg_db.pool)
            .await?;

        self.corporation_repo
            .get_corporation_by_name_at_tick(&self.pg_db.pool, corporation_name, game_tick)
            .await
    }

    async fn list_corporations_in_tick(
        &self,
        game_tick: i64,
    ) -> RepositoryResult<Vec<Corporation>> {
        self.corporation_repo
            .list_corporations_in_tick(&self.pg_db.pool, game_tick)
            .await
    }
}

#[tonic::async_trait]
impl CorporationTxRepository for PgTransactionContext<'_, '_> {
    async fn create_corporation(&mut self, corporation: &Corporation) -> RepositoryResult<()> {
        let game_tick = self
            .game_tick_repo
            .get_current_game_tick(&mut **self.tx)
            .await?;

        self.corporation_repo
            .create_corporation(&mut **self.tx, corporation, game_tick)
            .await
    }

    async fn insert_corporations_in_tick(
        &mut self,
        game_tick: i64,
        corporations: Vec<Corporation>,
    ) -> RepositoryResult<()> {
        self.corporation_repo
            .insert_corporations_in_tick(&mut **self.tx, corporations, game_tick)
            .await
    }

    async fn delete_corporations_before_tick(&mut self, game_tick: i64) -> RepositoryResult<u64> {
        self.corporation_repo
            .delete_corporations_before_tick(&mut **self.tx, game_tick)
            .await
    }
}
