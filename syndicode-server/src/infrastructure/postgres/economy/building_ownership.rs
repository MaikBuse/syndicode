use std::sync::Arc;

use crate::{
    domain::{
        economy::building_ownership::{
            model::BuildingOwnership,
            repository::{
                BuildingOwnershipDetails, BuildingOwnershipRepository,
                BuildingOwnershipTxRepository, QueryBuildingOwnershipsRequest,
            },
        },
        repository::RepositoryResult,
    },
    infrastructure::postgres::{
        game_tick::PgGameTickRepository, uow::PgTransactionContext, PostgresDatabase,
    },
};
use sqlx::{Postgres, QueryBuilder};

#[derive(Clone)]
pub struct PgBuildingOwnershipRepository;

impl PgBuildingOwnershipRepository {
    /// This leverages PostgreSQL's UNNEST function for efficiency.
    /// CARE: This is not compile time checked
    pub async fn insert_building_ownerships_in_tick(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        building_ownerships: &[BuildingOwnership],
        game_tick: i64,
    ) -> RepositoryResult<()> {
        if building_ownerships.is_empty() {
            return Ok(());
        }

        let count = building_ownerships.len();

        let mut building_uuid_vec = Vec::with_capacity(count);
        let mut owning_business_uuid_vec = Vec::with_capacity(count);

        for building_ownership in building_ownerships {
            building_uuid_vec.push(building_ownership.building_uuid);
            owning_business_uuid_vec.push(building_ownership.owning_business_uuid);
        }

        sqlx::query(
            r#"
            INSERT INTO building_ownerships (
                game_tick,
                building_uuid,
                owning_business_uuid
            )
            SELECT $1, u.*
            FROM unnest(
                $2::UUID[],
                $3::UUID[]
            )
            AS u(
                building_uuid,
                owning_business_uuid
            )
            "#,
        )
        .bind(game_tick)
        .bind(&building_uuid_vec)
        .bind(&owning_business_uuid_vec)
        .execute(executor)
        .await?;

        Ok(())
    }

    pub async fn query_building_ownerships(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres> + Copy,
        game_tick: i64,
        req: QueryBuildingOwnershipsRequest,
    ) -> RepositoryResult<Vec<BuildingOwnershipDetails>> {
        let mut qb = QueryBuilder::new(
            r#"
        SELECT
            bui.gml_id
        FROM building_ownerships bo
        JOIN businesses b ON b.uuid = bo.owning_business_uuid AND b.game_tick = 
        "#,
        );
        qb.push_bind(game_tick);

        // The 'buildings' table is static and does not have a game_tick column.
        qb.push(" JOIN buildings bui ON bui.uuid = bo.building_uuid ");

        qb.push(" WHERE bo.game_tick = ");
        qb.push_bind(game_tick);

        // --- Build WHERE clause dynamically based on optional request parameters ---

        if let Some(corp_uuid) = req.owning_corporation_uuid {
            qb.push(" AND b.owning_corporation_uuid = ");
            qb.push_bind(corp_uuid);
        }

        // A valid bounding box requires all four coordinate values.
        if let (Some(min_lon), Some(min_lat), Some(max_lon), Some(max_lat)) =
            (req.min_lon, req.min_lat, req.max_lon, req.max_lat)
        {
            // Use the '&&' operator for an efficient, index-based bounding box check.
            // ST_MakeEnvelope(min_lon, min_lat, max_lon, max_lat, srid)
            qb.push(" AND bui.center && ST_MakeEnvelope(");
            qb.push_bind(min_lon);
            qb.push(", ");
            qb.push_bind(min_lat);
            qb.push(", ");
            qb.push_bind(max_lon);
            qb.push(", ");
            qb.push_bind(max_lat);
            qb.push(", 4326)"); // SRID 4326 for WGS 84
        }

        // --- Add Pagination ---
        let limit = req.limit.unwrap_or(10).min(250);
        qb.push(" LIMIT ");
        qb.push_bind(limit);

        // --- Execute Query ---
        let query = qb.build_query_as::<BuildingOwnershipDetails>();

        let ownership_details = query.fetch_all(executor).await?;

        Ok(ownership_details)
    }

    pub async fn list_building_ownerships_in_tick(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        game_tick: i64,
    ) -> RepositoryResult<Vec<BuildingOwnership>> {
        let records = sqlx::query!(
            r#"
            SELECT
                game_tick,
                building_uuid,
                owning_business_uuid
            FROM building_ownerships
            WHERE
                game_tick = $1
            "#,
            game_tick
        )
        .fetch_all(executor)
        .await?;

        let mut building_ownerships = Vec::with_capacity(records.len());
        for record in records {
            building_ownerships.push(BuildingOwnership {
                building_uuid: record.building_uuid,
                owning_business_uuid: record.owning_business_uuid,
            });
        }

        Ok(building_ownerships)
    }

    pub async fn delete_building_ownerships_before_tick(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        game_tick: i64,
    ) -> RepositoryResult<u64> {
        let result = sqlx::query!(
            r#"
            DELETE FROM building_ownerships
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

pub struct PgBuildingOwnershipService {
    pg_db: Arc<PostgresDatabase>,
    game_tick_repo: PgGameTickRepository,
    building_ownership_repo: PgBuildingOwnershipRepository,
}

impl PgBuildingOwnershipService {
    pub fn new(pg_db: Arc<PostgresDatabase>) -> Self {
        Self {
            pg_db,
            game_tick_repo: PgGameTickRepository,
            building_ownership_repo: PgBuildingOwnershipRepository,
        }
    }
}

#[tonic::async_trait]
impl BuildingOwnershipRepository for PgBuildingOwnershipService {
    async fn list_building_ownerships_in_tick(
        &self,
        game_tick: i64,
    ) -> RepositoryResult<Vec<BuildingOwnership>> {
        self.building_ownership_repo
            .list_building_ownerships_in_tick(&self.pg_db.pool, game_tick)
            .await
    }

    async fn query_building_ownerships(
        &self,
        req: QueryBuildingOwnershipsRequest,
    ) -> RepositoryResult<(i64, Vec<BuildingOwnershipDetails>)> {
        let game_tick = self
            .game_tick_repo
            .get_current_game_tick(&self.pg_db.pool)
            .await?;

        let result = self
            .building_ownership_repo
            .query_building_ownerships(&self.pg_db.pool, game_tick, req)
            .await?;

        Ok((game_tick, result))
    }
}

#[tonic::async_trait]
impl BuildingOwnershipTxRepository for PgTransactionContext<'_, '_> {
    async fn insert_building_ownerships_in_tick(
        &mut self,
        game_tick: i64,
        building_ownerships: &[BuildingOwnership],
    ) -> RepositoryResult<()> {
        self.building_ownerships_repo
            .insert_building_ownerships_in_tick(&mut **self.tx, building_ownerships, game_tick)
            .await
    }

    async fn delete_building_ownerships_before_tick(
        &mut self,
        game_tick: i64,
    ) -> RepositoryResult<u64> {
        self.building_ownerships_repo
            .delete_building_ownerships_before_tick(&mut **self.tx, game_tick)
            .await
    }
}
