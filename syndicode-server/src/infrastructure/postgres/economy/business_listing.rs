use crate::{
    domain::{
        economy::business_listing::{
            model::BusinessListing,
            repository::{
                BusinessListingDetails, BusinessListingRepository, BusinessListingTxRepository,
                DomainBusinessListingSortBy, QueryBusinessListingsRequest,
            },
        },
        repository::RepositoryResult,
    },
    infrastructure::postgres::{
        game_tick::PgGameTickRepository, uow::PgTransactionContext, PostgresDatabase,
    },
};
use sqlx::{Execute, Postgres, QueryBuilder};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct PgBusinessListingRepository;

impl PgBusinessListingRepository {
    /// This leverages PostgreSQL's UNNEST function for efficiency.
    pub async fn insert_business_listings_in_tick(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        business_listings: &[BusinessListing],
        game_tick: i64,
    ) -> RepositoryResult<()> {
        if business_listings.is_empty() {
            return Ok(());
        }

        // Prepare separate vectors for each column to be bulk inserted.
        // Pre-allocate capacity for efficiency.
        let count = business_listings.len();
        let mut uuids = Vec::with_capacity(count);
        let mut business_uuids = Vec::with_capacity(count);
        let mut seller_corporation_uuids = Vec::with_capacity(count);
        let mut asking_prices = Vec::with_capacity(count);

        for bl in business_listings {
            uuids.push(bl.uuid);
            business_uuids.push(bl.business_uuid);
            seller_corporation_uuids.push(bl.seller_corporation_uuid);
            asking_prices.push(bl.asking_price);
        }

        // Execute the bulk insert query using UNNEST
        sqlx::query(
            r#"
            INSERT INTO business_listings (
                game_tick,
                uuid,
                business_uuid,
                seller_corporation_uuid,
                asking_price
            )
            SELECT $1, u.*
            FROM unnest($2::UUID[], $3::UUID[], $4::UUID[], $5::BIGINT[])
            AS u(uuid, business_uuid, seller_corporation_uuid, asking_price)
            "#,
        )
        .bind(game_tick) // Binds to tick_number column via $1
        .bind(&uuids) // Binds to $2 -> u.uuid -> uuid column
        .bind(&business_uuids) // Binds to $3 -> u.market_uuid -> market_uuid column
        .bind(&seller_corporation_uuids) // Binds to $4 -> u.owning_corporation_uuid -> owning_corporation_uuid column
        .bind(&asking_prices) // Binds to $6 -> u.operational_expenses -> operational_expenses column
        .execute(executor)
        .await?;

        Ok(())
    }

    pub async fn query_business_listings(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres> + Copy,
        game_tick: i64,
        req: &QueryBusinessListingsRequest,
    ) -> RepositoryResult<Vec<BusinessListingDetails>> {
        let mut qb = QueryBuilder::new(
            r#"
            SELECT
                bl.uuid AS listing_uuid,
                bl.business_uuid,
                b.name AS business_name,
                m.uuid AS market_uuid,
                bl.seller_corporation_uuid,
                bl.asking_price,
                b.operational_expenses,
                hb.gml_id AS headquarter_building_gml_id,
                ST_X(hb.center) AS headquarter_longitude,
                ST_Y(hb.center) AS headquarter_latitude
            FROM business_listings bl
            JOIN businesses b ON bl.business_uuid = b.uuid AND b.game_tick = "#,
        );
        qb.push_bind(game_tick);
        qb.push(" JOIN markets m ON b.market_uuid = m.uuid AND m.game_tick = ");
        qb.push_bind(game_tick);
        qb.push(" JOIN buildings hb ON b.headquarter_building_uuid = hb.uuid");
        qb.push(" WHERE bl.game_tick = ");
        qb.push_bind(game_tick);

        // Build WHERE clause dynamically
        if let Some(min_price) = req.min_asking_price {
            qb.push(" AND bl.asking_price >= ");
            qb.push_bind(min_price); // Access value from wrapper
        }
        if let Some(max_price) = req.max_asking_price {
            qb.push(" AND bl.asking_price <= ");
            qb.push_bind(max_price);
        }
        if let Some(seller_uuid_val) = &req.seller_corporation_uuid {
            match Uuid::parse_str(&seller_uuid_val.to_string()) {
                Ok(uuid) => {
                    qb.push(" AND bl.seller_corporation_uuid = ");
                    qb.push_bind(uuid);
                }
                Err(_) => {
                    tracing::warn!(
                        "Invalid seller_corporation_uuid format: {}",
                        seller_uuid_val
                    );
                }
            }
        }
        if let Some(market_uuid) = &req.market_uuid {
            // Assuming market name is stored as text/varchar
            qb.push(" AND m.uuid = ");
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

        // --- Clone the builder for COUNT(*) query before adding ORDER BY, LIMIT, OFFSET ---
        // --- Add Sorting ---
        let sort_column = match req.sort_by.as_ref().unwrap_or_default() {
            DomainBusinessListingSortBy::Price => "bl.asking_price",
            DomainBusinessListingSortBy::Name => "b.name",
            DomainBusinessListingSortBy::OperationExpenses => "b.operational_expenses",
            DomainBusinessListingSortBy::MarketVolume => "m.volume",
        };

        let sort_direction = req.sort_direction.unwrap_or_default().to_string();

        qb.push(format!(" ORDER BY {sort_column} {sort_direction}")); // Safe because sort_column comes from match

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

        // --- Execute Queries ---
        // Fetch the results
        let query = qb.build_query_as::<BusinessListingDetails>();
        tracing::debug!("Executing query: {}", query.sql()); // Log the query for debugging
        let listings = query.fetch_all(executor).await?;

        Ok(listings)
    }

    pub async fn list_business_listings_in_tick(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        game_tick: i64,
    ) -> RepositoryResult<Vec<BusinessListing>> {
        let business_listings = sqlx::query_as!(
            BusinessListing,
            r#"
            SELECT
                uuid,
                business_uuid,
                seller_corporation_uuid as "seller_corporation_uuid?",
                asking_price
            FROM business_listings
            WHERE
                game_tick = $1
            "#,
            game_tick
        )
        .fetch_all(executor)
        .await?;

        Ok(business_listings)
    }

    pub async fn delete_business_listings_before_tick(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        game_tick: i64,
    ) -> RepositoryResult<u64> {
        let result = sqlx::query!(
            r#"
            DELETE FROM business_listings
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

pub struct PgBusinessListingService {
    pg_db: Arc<PostgresDatabase>,
    game_tick_repo: PgGameTickRepository,
    business_listing_repo: PgBusinessListingRepository,
}

impl PgBusinessListingService {
    pub fn new(pg_db: Arc<PostgresDatabase>) -> Self {
        Self {
            pg_db,
            game_tick_repo: PgGameTickRepository,
            business_listing_repo: PgBusinessListingRepository,
        }
    }
}

#[tonic::async_trait]
impl BusinessListingRepository for PgBusinessListingService {
    async fn list_business_listings_in_tick(
        &self,
        game_tick: i64,
    ) -> RepositoryResult<Vec<BusinessListing>> {
        self.business_listing_repo
            .list_business_listings_in_tick(&self.pg_db.pool, game_tick)
            .await
    }

    async fn query_business_listings(
        &self,
        req: &QueryBusinessListingsRequest,
    ) -> RepositoryResult<(i64, Vec<BusinessListingDetails>)> {
        let game_tick = self
            .game_tick_repo
            .get_current_game_tick(&self.pg_db.pool)
            .await?;

        let result = self
            .business_listing_repo
            .query_business_listings(&self.pg_db.pool, game_tick, req)
            .await?;

        Ok((game_tick, result))
    }
}

#[tonic::async_trait]
impl BusinessListingTxRepository for PgTransactionContext<'_, '_> {
    async fn insert_business_listings_in_tick(
        &mut self,
        game_tick: i64,
        business_listings: &[BusinessListing],
    ) -> RepositoryResult<()> {
        self.business_listing_repo
            .insert_business_listings_in_tick(&mut **self.tx, business_listings, game_tick)
            .await
    }

    async fn delete_business_listings_before_tick(
        &mut self,
        game_tick: i64,
    ) -> RepositoryResult<u64> {
        self.business_listing_repo
            .delete_business_listings_before_tick(&mut **self.tx, game_tick)
            .await
    }
}
