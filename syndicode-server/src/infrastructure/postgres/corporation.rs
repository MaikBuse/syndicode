use super::{game_tick::PgGameTickRepository, uow::PgTransactionContext};
use crate::domain::{
    corporation::{
        model::Corporation,
        repository::{CorporationRepository, CorporationTxRepository, GetCorporationOutcome},
    },
    repository::RepositoryResult,
};
use sqlx::{PgPool, Postgres};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct PgCorporationRepository;

impl PgCorporationRepository {
    /// Inserts a new state record for a corporation at a specific game tick.
    /// This is used when advancing the game state from N to N+1.
    /// The Corporation input should contain the state calculated for the *new* tick.
    pub async fn insert_corporation(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        corporation: &Corporation,
        game_tick: i64,
    ) -> RepositoryResult<()> {
        sqlx::query!(
            r#"
            INSERT INTO corporations (
                game_tick,
                uuid,
                user_uuid,
                name,
                balance
            )
            VALUES ($1, $2, $3, $4, $5)
            "#,
            game_tick,
            corporation.uuid,
            corporation.user_uuid,
            corporation.name.to_string(),
            corporation.balance
        )
        .execute(executor)
        .await?;

        Ok(())
    }

    /// This leverages PostgreSQL's UNNEST function for efficiency.
    pub async fn insert_corporations_in_tick(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        corporations: Vec<Corporation>, // Take ownership for efficiency
        game_tick: i64,                 // Use i64 to match insert_corporation and DB schema
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
        let mut balances = Vec::with_capacity(count);

        for corp in corporations {
            uuids.push(corp.uuid);
            user_uuids.push(corp.user_uuid);
            names.push(corp.name.to_string());
            balances.push(corp.balance);
        }

        // Execute the bulk insert query using UNNEST
        sqlx::query!(
            r#"
            INSERT INTO corporations (
                game_tick,
                uuid,
                user_uuid,
                name,
                balance
            )
            SELECT
                $1 as game_tick,
                unnest($2::UUID[]) as uuid,
                unnest($3::UUID[]) as user_uuid,
                unnest($4::TEXT[]) as name,
                unnest($5::BIGINT[]) as balance
            "#,
            game_tick,
            &uuids,
            &user_uuids,
            &names,
            &balances
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
                balance
            FROM corporations
            WHERE
                user_uuid = $1
                AND game_tick = $2
            "#,
            user_uuid,
            game_tick
        )
        .fetch_one(executor)
        .await?;

        Ok(corporation)
    }

    /// Retrieves the state of a specific corporation (by its UUID) at a given game tick.
    pub async fn get_corporation_by_uuid_at_tick(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        corporation_uuid: Uuid,
        game_tick: i64,
    ) -> RepositoryResult<Corporation> {
        // Return Option<>
        let corporation = sqlx::query_as!(
            Corporation,
            r#"
            SELECT
                uuid, user_uuid, name, balance
            FROM corporations
            WHERE
                uuid = $1
                AND game_tick = $2
            "#,
            corporation_uuid,
            game_tick
        )
        .fetch_one(executor)
        .await?;

        Ok(corporation)
    }

    pub async fn list_corporations_at_tick(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        game_tick: i64,
    ) -> RepositoryResult<Vec<Corporation>> {
        let corporations = sqlx::query_as!(
            Corporation,
            r#"
            SELECT
                uuid, user_uuid, name, balance
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
    pool: Arc<PgPool>,
    game_tick_repo: PgGameTickRepository,
    corporation_repo: PgCorporationRepository,
}

impl PgCorporationService {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self {
            pool,
            game_tick_repo: PgGameTickRepository,
            corporation_repo: PgCorporationRepository,
        }
    }
}

#[tonic::async_trait]
impl CorporationRepository for PgCorporationService {
    async fn insert_corporation(&self, corporation: &Corporation) -> RepositoryResult<()> {
        let game_tick = self
            .game_tick_repo
            .get_current_game_tick(&*self.pool)
            .await?;

        self.corporation_repo
            .insert_corporation(&*self.pool, corporation, game_tick)
            .await
    }

    async fn get_corporation_by_user(
        &self,
        user_uuid: Uuid,
    ) -> RepositoryResult<GetCorporationOutcome> {
        let game_tick = self
            .game_tick_repo
            .get_current_game_tick(&*self.pool)
            .await?;

        let corporation = self
            .corporation_repo
            .get_corporation_by_user_at_tick(&*self.pool, user_uuid, game_tick)
            .await?;

        Ok(GetCorporationOutcome {
            game_tick,
            corporation,
        })
    }

    async fn get_corporation_by_uuid(
        &self,
        corporation_uuid: Uuid,
    ) -> RepositoryResult<Corporation> {
        let game_tick = self
            .game_tick_repo
            .get_current_game_tick(&*self.pool)
            .await?;

        self.corporation_repo
            .get_corporation_by_uuid_at_tick(&*self.pool, corporation_uuid, game_tick)
            .await
    }

    async fn list_corporations(&self) -> RepositoryResult<Vec<Corporation>> {
        let game_tick = self
            .game_tick_repo
            .get_current_game_tick(&*self.pool)
            .await?;

        self.corporation_repo
            .list_corporations_at_tick(&*self.pool, game_tick)
            .await
    }
}

#[tonic::async_trait]
impl CorporationTxRepository for PgTransactionContext<'_, '_> {
    async fn insert_corporation(&mut self, corporation: &Corporation) -> RepositoryResult<()> {
        let game_tick = self
            .game_tick_repo
            .get_current_game_tick(&mut **self.tx)
            .await?;

        self.corporation_repo
            .insert_corporation(&mut **self.tx, corporation, game_tick)
            .await
    }

    async fn get_corporation_by_user(&mut self, user_uuid: Uuid) -> RepositoryResult<Corporation> {
        let game_tick = self
            .game_tick_repo
            .get_current_game_tick(&mut **self.tx)
            .await?;

        self.corporation_repo
            .get_corporation_by_user_at_tick(&mut **self.tx, user_uuid, game_tick)
            .await
    }

    async fn get_corporation_by_uuid(
        &mut self,
        corporation_uuid: Uuid,
    ) -> RepositoryResult<Corporation> {
        let game_tick = self
            .game_tick_repo
            .get_current_game_tick(&mut **self.tx)
            .await?;

        self.corporation_repo
            .get_corporation_by_uuid_at_tick(&mut **self.tx, corporation_uuid, game_tick)
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
