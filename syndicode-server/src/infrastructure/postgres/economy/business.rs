use std::sync::Arc;

use crate::{
    domain::{
        economy::business::{
            model::Business,
            repository::{BusinessRepository, BusinessTxRepository},
        },
        repository::RepositoryResult,
    },
    infrastructure::postgres::{
        from_geo_point_to_pg_point, uow::PgTransactionContext, PostgresDatabase,
    },
};
use geo::Point;
use sqlx::Postgres;
use uuid::Uuid;

#[derive(Clone)]
pub struct PgBusinessRepository;

impl PgBusinessRepository {
    /// This leverages PostgreSQL's UNNEST function for efficiency.
    /// CARE: This is not compile time checked
    pub async fn insert_businesses_in_tick(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        businesses: Vec<Business>,
        game_tick: i64,
    ) -> RepositoryResult<()> {
        if businesses.is_empty() {
            return Ok(());
        }

        let count = businesses.len();
        let mut uuids = Vec::with_capacity(count);
        let mut market_uuids = Vec::with_capacity(count);
        let mut owning_corporation_uuids: Vec<Option<Uuid>> = Vec::with_capacity(count);
        let mut names = Vec::with_capacity(count);
        let mut operational_expenses = Vec::with_capacity(count);
        let mut centers = Vec::with_capacity(count);

        for business in businesses {
            uuids.push(business.uuid);
            market_uuids.push(business.market_uuid);
            owning_corporation_uuids.push(business.owning_corporation_uuid);
            names.push(business.name);
            operational_expenses.push(business.operational_expenses);
            centers.push(from_geo_point_to_pg_point(business.center));
        }

        sqlx::query(
            r#"
            INSERT INTO businesses (
                game_tick,
                uuid,
                market_uuid,
                owning_corporation_uuid,
                name,
                operational_expenses,
                center
            )
            SELECT $1, u.*
            FROM unnest(
                $2::UUID[],
                $3::UUID[],
                $4::UUID[],
                $5::TEXT[],
                $6::BIGINT[],
                $7::geometry[]
            )
            AS u(
                uuid,
                market_uuid,
                owning_corporation_uuid,
                name,
                operational_expenses,
                center
            )
            "#,
        )
        .bind(game_tick)
        .bind(&uuids)
        .bind(&market_uuids)
        .bind(&owning_corporation_uuids)
        .bind(&names)
        .bind(&operational_expenses)
        .bind(&centers)
        .execute(executor)
        .await?;

        Ok(())
    }

    pub async fn list_businesses_in_tick(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        game_tick: i64,
    ) -> RepositoryResult<Vec<Business>> {
        let records = sqlx::query!(
            r#"
            SELECT
                uuid,
                market_uuid,
                owning_corporation_uuid,
                name,
                operational_expenses,
                center
            FROM businesses
            WHERE
                game_tick = $1
            "#,
            game_tick
        )
        .fetch_all(executor)
        .await?;

        let mut businesses = Vec::with_capacity(records.len());
        for record in records {
            businesses.push(Business {
                uuid: record.uuid,
                market_uuid: record.market_uuid,
                owning_corporation_uuid: record.owning_corporation_uuid,
                name: record.name,
                operational_expenses: record.operational_expenses,
                center: Point::new(record.center.x, record.center.y),
            });
        }

        Ok(businesses)
    }

    pub async fn delete_businesses_before_tick(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        game_tick: i64,
    ) -> RepositoryResult<u64> {
        let result = sqlx::query!(
            r#"
            DELETE FROM businesses
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

pub struct PgBusinessService {
    pg_db: Arc<PostgresDatabase>,
    business_repo: PgBusinessRepository,
}

impl PgBusinessService {
    pub fn new(pg_db: Arc<PostgresDatabase>) -> Self {
        Self {
            pg_db,
            business_repo: PgBusinessRepository,
        }
    }
}

#[tonic::async_trait]
impl BusinessRepository for PgBusinessService {
    async fn list_businesses_in_tick(&self, game_tick: i64) -> RepositoryResult<Vec<Business>> {
        self.business_repo
            .list_businesses_in_tick(&self.pg_db.pool, game_tick)
            .await
    }
}

#[tonic::async_trait]
impl BusinessTxRepository for PgTransactionContext<'_, '_> {
    async fn insert_businesses_in_tick(
        &mut self,
        game_tick: i64,
        businesses: Vec<Business>,
    ) -> RepositoryResult<()> {
        self.business_repo
            .insert_businesses_in_tick(&mut **self.tx, businesses, game_tick)
            .await
    }

    async fn delete_businesses_before_tick(&mut self, game_tick: i64) -> RepositoryResult<u64> {
        self.business_repo
            .delete_businesses_before_tick(&mut **self.tx, game_tick)
            .await
    }
}
