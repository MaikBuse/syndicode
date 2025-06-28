use std::sync::Arc;

use crate::{
    domain::{
        economy::business::{
            model::Business,
            repository::{BusinessRepository, BusinessTxRepository},
        },
        repository::{RepositoryError, RepositoryResult},
    },
    infrastructure::postgres::{uow::PgTransactionContext, PostgresDatabase, SRID},
};
use geo::Geometry;
use sqlx::Postgres;
use uuid::Uuid;
use wkt::ToWkt;

#[derive(sqlx::FromRow)]
struct BusinessRow {
    uuid: Uuid,
    market_uuid: Uuid,
    owning_corporation_uuid: Option<Uuid>,
    name: String,
    operational_expenses: i64,
    center: geozero::wkb::Decode<Geometry<f64>>,
}

impl BusinessRow {
    fn into_business(self) -> RepositoryResult<Business> {
        let Geometry::Point(center) = self
            .center
            .geometry
            .ok_or(RepositoryError::GeometryMissing)?
        else {
            return Err(RepositoryError::PointCasting);
        };

        Ok(Business::builder()
            .uuid(self.uuid)
            .maybe_owning_corporation_uuid(self.owning_corporation_uuid)
            .name(self.name)
            .market_uuid(self.market_uuid)
            .operational_expenses(self.operational_expenses)
            .center(center)
            .build())
    }
}

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
        let mut center_wkt_vec: Vec<String> = Vec::with_capacity(count);

        for business in businesses {
            uuids_vec.push(business.uuid);
            market_uuids_vec.push(business.market_uuid);
            owning_corporation_uuids_vec.push(business.owning_corporation_uuid);
            names_vec.push(business.name.clone());
            operational_expenses_vec.push(business.operational_expenses);
            center_wkt_vec.push(business.center.to_wkt().to_string());
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
            SELECT
                $1,
                u.uuid,
                u.market_uuid,
                u.owning_corporation_uuid,
                u.name,
                u.operational_expenses,
                ST_SetSRID(ST_GeomFromText(u.center), $8)
            FROM unnest(
                $2::UUID[],
                $3::UUID[],
                $4::UUID[],
                $5::TEXT[],
                $6::BIGINT[],
                $7::TEXT[]
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
        .bind(&uuids_vec)
        .bind(&market_uuids_vec)
        .bind(&owning_corporation_uuids_vec)
        .bind(&names_vec)
        .bind(&operational_expenses_vec)
        .bind(&center_wkt_vec)
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
        let rows = sqlx::query_as::<_, BusinessRow>(
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
        )
        .bind(game_tick)
        .fetch_all(executor)
        .await?;

        let mut businesses = Vec::with_capacity(rows.len());

        for row in rows {
            businesses.push(row.into_business()?);
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
