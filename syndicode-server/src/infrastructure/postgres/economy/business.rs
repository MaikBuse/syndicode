use std::sync::Arc;

use crate::{
    domain::{
        economy::business::{
            model::Business,
            repository::{
                BusinessDetails, BusinessRepository, BusinessTxRepository, DomainBusinessSortBy,
                QueryBusinessesRequest,
            },
        },
        repository::RepositoryResult,
    },
    infrastructure::postgres::{
        game_tick::PgGameTickRepository, uow::PgTransactionContext, PostgresDatabase, SRID,
    },
};
use sqlx::{Execute, Postgres, QueryBuilder};
use uuid::Uuid;

#[derive(Clone)]
pub struct PgBusinessRepository;

impl PgBusinessRepository {
    /// This leverages PostgreSQL's UNNEST function for efficiency.
    /// CARE: This is not compile time checked
    pub async fn insert_businesses_in_tick(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        businesses: &[Business],
        game_tick: i64,
    ) -> RepositoryResult<()> {
        if businesses.is_empty() {
            return Ok(());
        }

        let count = businesses.len();
        let mut uuids_vec = Vec::with_capacity(count);
        let mut market_uuids_vec = Vec::with_capacity(count);
        let mut owning_corporation_uuids_vec: Vec<Option<Uuid>> = Vec::with_capacity(count);
        let mut names_vec = Vec::with_capacity(count);
        let mut operational_expenses_vec = Vec::with_capacity(count);
        let mut headquarter_business_uuids: Vec<Uuid> = Vec::with_capacity(count);

        for business in businesses {
            uuids_vec.push(business.uuid);
            market_uuids_vec.push(business.market_uuid);
            owning_corporation_uuids_vec.push(business.owning_corporation_uuid);
            names_vec.push(business.name.clone());
            operational_expenses_vec.push(business.operational_expenses);
            headquarter_business_uuids.push(business.headquarter_building_uuid);
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
                headquarter_building_uuid
            )
            SELECT
                $1,
                u.uuid,
                u.market_uuid,
                u.owning_corporation_uuid,
                u.name,
                u.operational_expenses,
                u.headquarter_building_uuid
            FROM unnest(
                $2::UUID[],
                $3::UUID[],
                $4::UUID[],
                $5::TEXT[],
                $6::BIGINT[],
                $7::TEXT[]
                $8::UUID[],
            )
            AS u(
                uuid,
                market_uuid,
                owning_corporation_uuid,
                name,
                operational_expenses,
                headquarter_building_uuid
            )
            "#,
        )
        .bind(game_tick)
        .bind(&uuids_vec)
        .bind(&market_uuids_vec)
        .bind(&owning_corporation_uuids_vec)
        .bind(&names_vec)
        .bind(&operational_expenses_vec)
        .bind(&headquarter_business_uuids)
        .bind(SRID)
        .execute(executor)
        .await?;

        Ok(())
    }

    pub async fn list_businesses_in_tick(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        game_tick: i64,
    ) -> RepositoryResult<Vec<Business>> {
        let businesses = sqlx::query_as::<_, Business>(
            r#"
        SELECT
            uuid,
            market_uuid,
            owning_corporation_uuid,
            name,
            operational_expenses,
            headquarter_building_uuid
        FROM businesses
        WHERE
            game_tick = $1
        "#,
        )
        .bind(game_tick)
        .fetch_all(executor)
        .await?;

        Ok(businesses)
    }

    pub async fn query_businesses(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres> + Copy,
        game_tick: i64,
        req: &QueryBusinessesRequest,
    ) -> RepositoryResult<Vec<BusinessDetails>> {
        let mut qb = QueryBuilder::new(
            r#"
            SELECT
                b.uuid AS business_uuid,
                b.name AS business_name,
                b.owning_corporation_uuid,
                b.market_uuid,
                b.operational_expenses,
                b.headquarter_building_uuid,
                m.volume AS market_volume
            FROM businesses b
            JOIN markets m ON b.market_uuid = m.uuid AND m.game_tick = "#,
        );
        qb.push_bind(game_tick);
        qb.push(" WHERE b.game_tick = ");
        qb.push_bind(game_tick);

        // Build WHERE clause dynamically
        if let Some(owning_corporation_uuid) = &req.owning_corporation_uuid {
            qb.push(" AND b.owning_corporation_uuid = ");
            qb.push_bind(owning_corporation_uuid);
        }

        if let Some(market_uuid) = &req.market_uuid {
            qb.push(" AND b.market_uuid = ");
            qb.push_bind(market_uuid);
        }

        if let Some(min_op_ex) = req.min_operational_expenses {
            qb.push(" AND b.operational_expenses >= ");
            qb.push_bind(min_op_ex);
        }

        if let Some(max_op_ex) = req.max_operational_expenses {
            qb.push(" AND b.operational_expenses <= ");
            qb.push_bind(max_op_ex);
        }

        // --- Add Sorting ---
        let sort_column = match req.sort_by.as_ref().unwrap_or_default() {
            DomainBusinessSortBy::Name => "b.name",
            DomainBusinessSortBy::OperationExpenses => "b.operational_expenses",
            DomainBusinessSortBy::MarketVolume => "m.volume",
        };

        let sort_direction = req.sort_direction.unwrap_or_default().to_string();

        qb.push(format!(" ORDER BY {sort_column} {sort_direction}"));

        // --- Add Pagination ---
        let limit = req.limit.unwrap_or(10).min(100);
        qb.push(" LIMIT ");
        qb.push_bind(limit);

        if let Some(offset_val) = req.offset {
            if offset_val > 0 {
                qb.push(" OFFSET ");
                qb.push_bind(offset_val);
            }
        }

        // --- Execute Query ---
        let query = qb.build_query_as::<BusinessDetails>();
        tracing::debug!("Executing query: {}", query.sql());
        let businesses = query.fetch_all(executor).await?;

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
    game_tick_repo: PgGameTickRepository,
    business_repo: PgBusinessRepository,
}

impl PgBusinessService {
    pub fn new(pg_db: Arc<PostgresDatabase>) -> Self {
        Self {
            pg_db,
            game_tick_repo: PgGameTickRepository,
            business_repo: PgBusinessRepository,
        }
    }
}

#[tonic::async_trait]
impl BusinessRepository for PgBusinessService {
    async fn query_businesses(
        &self,
        req: &QueryBusinessesRequest,
    ) -> RepositoryResult<(i64, Vec<BusinessDetails>)> {
        let game_tick = self
            .game_tick_repo
            .get_current_game_tick(&self.pg_db.pool)
            .await?;

        let result = self
            .business_repo
            .query_businesses(&self.pg_db.pool, game_tick, req)
            .await?;

        Ok((game_tick, result))
    }

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
        businesses: &[Business],
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
