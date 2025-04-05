use super::{game_tick::PgGameTickRepository, uow::PgTransactionContext};
use crate::domain::{
    corporation::{
        model::Corporation,
        repository::{CorporationRepository, CorporationTxRepository},
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
            corporation.name,
            corporation.balance
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

    async fn get_corporation_by_user(&self, user_uuid: Uuid) -> RepositoryResult<Corporation> {
        let game_tick = self
            .game_tick_repo
            .get_current_game_tick(&*self.pool)
            .await?;

        self.corporation_repo
            .get_corporation_by_user_at_tick(&*self.pool, user_uuid, game_tick)
            .await
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
}
