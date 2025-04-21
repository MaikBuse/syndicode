use crate::{
    domain::{
        economy::business::{model::Business, repository::BusinessTxRepository},
        repository::RepositoryResult,
    },
    infrastructure::postgres::uow::PgTransactionContext,
};
use sqlx::Postgres;
use uuid::Uuid;

#[derive(Clone)]
pub struct PgBusinessRepository;

impl PgBusinessRepository {
    /// Inserts a new state record for a business at a specific game tick.
    /// This is used when advancing the game state from N to N+1.
    /// The business input should contain the state calculated for the *new* tick.
    pub async fn insert_business(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        business: &Business,
        game_tick: i64,
    ) -> RepositoryResult<()> {
        sqlx::query!(
            r#"
            INSERT INTO businesses (
                game_tick,
                uuid,
                market_uuid,
                owning_corporation_uuid,
                name,
                operational_expenses
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            game_tick,
            business.uuid,
            business.market_uuid,
            business.owning_corporation_uuid,
            business.name,
            business.operational_expenses
        )
        .execute(executor)
        .await?;

        Ok(())
    }

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

        for business in businesses {
            uuids.push(business.uuid);
            market_uuids.push(business.market_uuid);
            owning_corporation_uuids.push(business.owning_corporation_uuid);
            names.push(business.name);
            operational_expenses.push(business.operational_expenses);
        }

        sqlx::query(
            r#"
            INSERT INTO businesses (
                game_tick,
                uuid,
                market_uuid,
                owning_corporation_uuid,
                name,
                operational_expenses
            )
            SELECT $1, u.*
            FROM unnest($2::UUID[], $3::UUID[], $4::UUID[], $5::TEXT[], $6::BIGINT[])
            AS u(uuid, market_uuid, owning_corporation_uuid, name, operational_expenses)
            "#,
        )
        .bind(game_tick) // Binds to tick_number column via $1
        .bind(&uuids) // Binds to $2 -> u.uuid -> uuid column
        .bind(&market_uuids) // Binds to $3 -> u.market_uuid -> market_uuid column
        .bind(&owning_corporation_uuids) // Binds to $4 -> u.owning_corporation_uuid -> owning_corporation_uuid column
        .bind(&names) // Binds to $5 -> u.name -> name column
        .bind(&operational_expenses) // Binds to $6 -> u.operational_expenses -> operational_expenses column
        .execute(executor)
        .await?;

        Ok(())
    }

    /// Retrieves the state of a specific user's business at a given game tick.
    /// This is typically used by clients reading the "current" game state.
    pub async fn get_business_by_user_at_tick(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        owning_corporation_uuid: Uuid,
        game_tick: i64,
    ) -> RepositoryResult<Business> {
        let business = sqlx::query_as!(
            Business,
            r#"
            SELECT
                uuid,
                market_uuid,
                owning_corporation_uuid,
                name,
                operational_expenses
            FROM businesses
            WHERE
                owning_corporation_uuid = $1
                AND game_tick = $2
            "#,
            owning_corporation_uuid,
            game_tick
        )
        .fetch_one(executor)
        .await?;

        Ok(business)
    }

    /// Retrieves the state of a specific business (by its UUID) at a given game tick.
    pub async fn get_business_by_uuid_at_tick(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        business_uuid: Uuid,
        game_tick: i64,
    ) -> RepositoryResult<Business> {
        let business = sqlx::query_as!(
            Business,
            r#"
            SELECT
                uuid,
                market_uuid,
                owning_corporation_uuid,
                name,
                operational_expenses
            FROM businesses
            WHERE
                uuid = $1
                AND game_tick = $2
            "#,
            business_uuid,
            game_tick
        )
        .fetch_one(executor)
        .await?;

        Ok(business)
    }

    pub async fn list_businesses_at_tick(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        game_tick: i64,
    ) -> RepositoryResult<Vec<Business>> {
        let businesses = sqlx::query_as!(
            Business,
            r#"
            SELECT
                uuid,
                market_uuid,
                owning_corporation_uuid,
                name,
                operational_expenses
            FROM businesses
            WHERE
                game_tick = $1
            "#,
            game_tick
        )
        .fetch_all(executor)
        .await?;

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
}
