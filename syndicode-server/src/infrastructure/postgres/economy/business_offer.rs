use crate::{
    domain::{
        economy::business_offer::{
            model::BusinessOffer,
            repository::{BusinessOfferRepository, BusinessOfferTxRepository},
        },
        repository::RepositoryResult,
    },
    infrastructure::postgres::{uow::PgTransactionContext, PostgresDatabase},
};
use sqlx::Postgres;
use std::sync::Arc;

#[derive(Clone)]
pub struct PgBusinessOfferRepository;

impl PgBusinessOfferRepository {
    /// This leverages PostgreSQL's UNNEST function for efficiency.
    pub async fn insert_business_offers_in_tick(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        business_offers: &[BusinessOffer],
        game_tick: i64,
    ) -> RepositoryResult<()> {
        if business_offers.is_empty() {
            return Ok(());
        }

        // Prepare separate vectors for each column to be bulk inserted.
        // Pre-allocate capacity for efficiency.
        let count = business_offers.len();
        let mut uuids = Vec::with_capacity(count);
        let mut business_uuids = Vec::with_capacity(count);
        let mut offering_corporation_uuids = Vec::with_capacity(count);
        let mut target_corporation_uuids = Vec::with_capacity(count);
        let mut offer_prices = Vec::with_capacity(count);

        for bo in business_offers {
            uuids.push(bo.uuid);
            business_uuids.push(bo.business_uuid);
            offering_corporation_uuids.push(bo.offering_corporation_uuid);
            target_corporation_uuids.push(bo.target_corporation_uuid);
            offer_prices.push(bo.offer_price);
        }

        // Execute the bulk insert query using UNNEST
        sqlx::query(
            r#"
            INSERT INTO business_listings (
                game_tick,
                uuid,
                business_uuid,
                offering_corporation_uuid,
                target_corporation_uuid,
                offer_price
            )
            SELECT $1, u.*
            FROM unnest($2::UUID[], $3::UUID[], $4::UUID[], $5::UUID[],  $6::BIGINT[])
            AS u(uuid, business_uuid, offering_corporation_uuid, target_corporation_uuid, offer_price)
            "#,
        )
        .bind(game_tick) // Binds to tick_number column via $1
        .bind(&uuids) // Binds to $2 -> u.uuid -> uuid column
        .bind(&business_uuids) // Binds to $3 -> u.market_uuid -> market_uuid column
        .bind(&offering_corporation_uuids) // Binds to $4 -> u.owning_corporation_uuid -> owning_corporation_uuid column
        .bind(&target_corporation_uuids) // Binds to $4 -> u.owning_corporation_uuid -> owning_corporation_uuid column
        .bind(&offer_prices) // Binds to $6 -> u.operational_expenses -> operational_expenses column
        .execute(executor)
        .await?;

        Ok(())
    }

    pub async fn list_business_offers_in_tick(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        game_tick: i64,
    ) -> RepositoryResult<Vec<BusinessOffer>> {
        let business_offers = sqlx::query_as!(
            BusinessOffer,
            r#"
            SELECT
                uuid,
                business_uuid,
                offering_corporation_uuid,
                target_corporation_uuid,
                offer_price
            FROM business_offers
            WHERE
                game_tick = $1
            "#,
            game_tick
        )
        .fetch_all(executor)
        .await?;

        Ok(business_offers)
    }

    pub async fn delete_business_offers_before_tick(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        game_tick: i64,
    ) -> RepositoryResult<u64> {
        let result = sqlx::query!(
            r#"
            DELETE FROM business_offers
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

pub struct PgBusinessOfferService {
    pg_db: Arc<PostgresDatabase>,
    business_offer_repo: PgBusinessOfferRepository,
}

impl PgBusinessOfferService {
    pub fn new(pg_db: Arc<PostgresDatabase>) -> Self {
        Self {
            pg_db,
            business_offer_repo: PgBusinessOfferRepository,
        }
    }
}

#[tonic::async_trait]
impl BusinessOfferRepository for PgBusinessOfferService {
    async fn list_business_offers_in_tick(
        &self,
        game_tick: i64,
    ) -> RepositoryResult<Vec<BusinessOffer>> {
        self.business_offer_repo
            .list_business_offers_in_tick(&self.pg_db.pool, game_tick)
            .await
    }
}

#[tonic::async_trait]
impl BusinessOfferTxRepository for PgTransactionContext<'_, '_> {
    async fn insert_business_offers_in_tick(
        &mut self,
        game_tick: i64,
        business_offers: &[BusinessOffer],
    ) -> RepositoryResult<()> {
        self.business_offer_repo
            .insert_business_offers_in_tick(&mut **self.tx, business_offers, game_tick)
            .await
    }

    async fn delete_business_offers_before_tick(
        &mut self,
        game_tick: i64,
    ) -> RepositoryResult<u64> {
        self.business_offer_repo
            .delete_business_offers_before_tick(&mut **self.tx, game_tick)
            .await
    }
}
