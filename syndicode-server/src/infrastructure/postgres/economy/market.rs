use std::sync::Arc;

use crate::{
    domain::{
        economy::market::{
            model::Market,
            repository::{MarketRepository, MarketTxRepository},
        },
        repository::RepositoryResult,
    },
    infrastructure::postgres::{uow::PgTransactionContext, PostgresDatabase},
};
use sqlx::Postgres;

#[derive(Clone)]
pub struct PgMarketRepository;

impl PgMarketRepository {
    /// This leverages PostgreSQL's UNNEST function for efficiency.
    pub async fn insert_markets_in_tick(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        markets: &[Market],
        game_tick: i64,
    ) -> RepositoryResult<()> {
        if markets.is_empty() {
            return Ok(());
        }

        // Prepare separate vectors for each column to be bulk inserted.
        // Pre-allocate capacity for efficiency.
        let count = markets.len();
        let mut uuids = Vec::with_capacity(count);
        let mut names: Vec<i16> = Vec::with_capacity(count);
        let mut volumes = Vec::with_capacity(count);

        for market in markets {
            uuids.push(market.uuid);
            names.push(market.name.into());
            volumes.push(market.volume);
        }

        // Execute the bulk insert query using UNNEST
        sqlx::query!(
            r#"
            INSERT INTO markets (
                game_tick,
                uuid,
                name,
                volume
            )
            SELECT
                $1 as game_tick,
                unnest($2::UUID[]) as uuid,
                unnest($3::SMALLINT[]) as name,
                unnest($4::BIGINT[]) as volume
            "#,
            game_tick,
            &uuids,
            &names,
            &volumes
        )
        .execute(executor)
        .await?;

        Ok(())
    }

    pub async fn list_markets_at_tick(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        game_tick: i64,
    ) -> RepositoryResult<Vec<Market>> {
        let markets = sqlx::query_as!(
            Market,
            r#"
            SELECT
                uuid,
                name,
                volume
            FROM markets
            WHERE
                game_tick = $1
            "#,
            game_tick
        )
        .fetch_all(executor)
        .await?;

        Ok(markets)
    }

    pub async fn delete_markets_before_tick(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        game_tick: i64,
    ) -> RepositoryResult<u64> {
        let result = sqlx::query!(
            r#"
            DELETE FROM markets
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

pub struct PgMarketService {
    pg_db: Arc<PostgresDatabase>,
    market_repo: PgMarketRepository,
}

impl PgMarketService {
    pub fn new(pg_db: Arc<PostgresDatabase>) -> Self {
        Self {
            pg_db,
            market_repo: PgMarketRepository,
        }
    }
}

#[tonic::async_trait]
impl MarketRepository for PgMarketService {
    async fn list_markets_in_tick(&self, game_tick: i64) -> RepositoryResult<Vec<Market>> {
        self.market_repo
            .list_markets_at_tick(&self.pg_db.pool, game_tick)
            .await
    }
}

#[tonic::async_trait]
impl MarketTxRepository for PgTransactionContext<'_, '_> {
    async fn insert_markets_in_tick(
        &mut self,
        game_tick: i64,
        markets: &[Market],
    ) -> RepositoryResult<()> {
        self.market_repo
            .insert_markets_in_tick(&mut **self.tx, markets, game_tick)
            .await
    }

    async fn delete_markets_before_tick(&mut self, game_tick: i64) -> RepositoryResult<u64> {
        self.market_repo
            .delete_markets_before_tick(&mut **self.tx, game_tick)
            .await
    }
}
