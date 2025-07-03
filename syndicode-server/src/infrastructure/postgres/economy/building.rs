use std::sync::Arc;

use crate::{
    domain::{
        economy::building::{
            model::Building,
            repository::{
                BuildingDetails, BuildingRepository, BuildingTxRepository, QueryBuildingsRequest,
            },
        },
        repository::RepositoryResult,
    },
    infrastructure::postgres::{
        game_tick::PgGameTickRepository, uow::PgTransactionContext, PostgresDatabase, SRID,
    },
};
use sqlx::{Executor, Postgres, QueryBuilder};
use wkt::ToWkt;

#[derive(Clone)]
pub struct PgBuildingRepository;

impl PgBuildingRepository {
    pub async fn insert_buildings(
        &self,
        executor: impl Executor<'_, Database = Postgres>,
        buildings: Vec<Building>,
    ) -> RepositoryResult<()> {
        if buildings.is_empty() {
            return Ok(());
        }

        let count = buildings.len();

        let mut uuid_vec = Vec::with_capacity(count);
        let mut gml_id_vec = Vec::with_capacity(count);
        let mut name_vec: Vec<Option<String>> = Vec::with_capacity(count);
        let mut address_vec: Vec<Option<String>> = Vec::with_capacity(count);
        let mut usage_vec: Vec<Option<String>> = Vec::with_capacity(count);
        let mut usage_code_vec: Vec<Option<String>> = Vec::with_capacity(count);
        let mut class_vec: Vec<Option<String>> = Vec::with_capacity(count);
        let mut class_code_vec: Vec<Option<String>> = Vec::with_capacity(count);
        let mut city_vec: Vec<Option<String>> = Vec::with_capacity(count);
        let mut city_code_vec: Vec<Option<String>> = Vec::with_capacity(count);
        let mut height_vec = Vec::with_capacity(count);
        let mut prefecture_vec: Vec<Option<String>> = Vec::with_capacity(count);

        let mut center_wkt_vec: Vec<String> = Vec::with_capacity(count);
        let mut footprint_wkt_vec: Vec<String> = Vec::with_capacity(count);

        for building in buildings {
            uuid_vec.push(building.uuid);
            gml_id_vec.push(building.gml_id);
            name_vec.push(building.name);
            address_vec.push(building.address);
            usage_vec.push(building.usage);
            usage_code_vec.push(building.usage_code);
            class_vec.push(building.class);
            class_code_vec.push(building.class_code);
            city_vec.push(building.city);
            city_code_vec.push(building.city_code);
            height_vec.push(building.height);
            prefecture_vec.push(building.prefecture);

            center_wkt_vec.push(building.center.to_wkt().to_string());
            footprint_wkt_vec.push(building.footprint.to_wkt().to_string());
        }
        sqlx::query(
            r#"
            INSERT INTO buildings (
                uuid, gml_id, name, address, usage, usage_code, class, class_code,
                city, city_code, center, footprint, height, prefecture
            )
            SELECT
                u.uuid, u.gml_id, u.name, u.address, u.usage, u.usage_code, u.class, u.class_code,
                u.city, u.city_code,
                -- Use ST_GeomFromText to convert WKT strings to geometry
                ST_SetSRID(ST_GeomFromText(u.center), $15),
                ST_SetSRID(ST_GeomFromText(u.footprint), $15),
                u.height, u.prefecture
            FROM unnest(
                $1::UUID[],
                $2::TEXT[],
                $3::TEXT[],
                $4::TEXT[],
                $5::TEXT[],
                $6::TEXT[],
                $7::TEXT[],
                $8::TEXT[],
                $9::TEXT[],
                $10::TEXT[],
                $11::TEXT[],
                $12::TEXT[],
                $13::DOUBLE PRECISION[],
                $14::TEXT[]
            ) AS u(
                uuid, gml_id, name, address, usage, usage_code, class, class_code,
                city, city_code, center, footprint, height, prefecture
            )
        "#,
        )
        .bind(&uuid_vec)
        .bind(&gml_id_vec)
        .bind(&name_vec)
        .bind(&address_vec)
        .bind(&usage_vec)
        .bind(&usage_code_vec)
        .bind(&class_vec)
        .bind(&class_code_vec)
        .bind(&city_vec)
        .bind(&city_code_vec)
        .bind(&center_wkt_vec)
        .bind(&footprint_wkt_vec)
        .bind(&height_vec)
        .bind(&prefecture_vec)
        .bind(SRID)
        .execute(executor)
        .await?;

        Ok(())
    }

    pub async fn query_buildings(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres> + Copy,
        game_tick: i64,
        req: QueryBuildingsRequest,
    ) -> RepositoryResult<Vec<BuildingDetails>> {
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
        let query = qb.build_query_as::<BuildingDetails>();

        let ownership_details = query.fetch_all(executor).await?;

        Ok(ownership_details)
    }
}

pub struct PgBuildingService {
    pg_db: Arc<PostgresDatabase>,
    game_tick_repo: PgGameTickRepository,
    building_repo: PgBuildingRepository,
}

impl PgBuildingService {
    pub fn new(pg_db: Arc<PostgresDatabase>) -> Self {
        Self {
            pg_db,
            game_tick_repo: PgGameTickRepository,
            building_repo: PgBuildingRepository,
        }
    }
}

#[tonic::async_trait]
impl BuildingRepository for PgBuildingService {
    async fn query_buildings(
        &self,
        req: QueryBuildingsRequest,
    ) -> RepositoryResult<(i64, Vec<BuildingDetails>)> {
        let game_tick = self
            .game_tick_repo
            .get_current_game_tick(&self.pg_db.pool)
            .await?;

        let result = self
            .building_repo
            .query_buildings(&self.pg_db.pool, game_tick, req)
            .await?;

        Ok((game_tick, result))
    }
}

#[tonic::async_trait]
impl BuildingTxRepository for PgTransactionContext<'_, '_> {
    async fn insert_buildings(&mut self, buildings: Vec<Building>) -> RepositoryResult<()> {
        self.building_repo
            .insert_buildings(&mut **self.tx, buildings)
            .await
    }
}
