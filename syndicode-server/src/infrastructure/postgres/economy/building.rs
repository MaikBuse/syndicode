use std::sync::Arc;

use crate::{
    domain::{
        economy::building::{
            model::Building,
            repository::{BuildingRepository, BuildingTxRepository},
        },
        repository::RepositoryResult,
    },
    infrastructure::postgres::{
        from_geo_point_to_pg_point, from_geo_polygon_to_pg_points, from_pg_point_to_geo_point,
        from_pg_polygon_to_geo_polygon, uow::PgTransactionContext, PostgresDatabase,
    },
};
use sqlx::Postgres;

#[derive(Clone)]
pub struct PgBuildingRepository;

impl PgBuildingRepository {
    /// This leverages PostgreSQL's UNNEST function for efficiency.
    /// CARE: This is not compile time checked
    pub async fn insert_buildings_in_tick(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        buildings: Vec<Building>,
        game_tick: i64,
    ) -> RepositoryResult<()> {
        if buildings.is_empty() {
            return Ok(());
        }

        let count = buildings.len();

        let mut uuid_vec = Vec::with_capacity(count);
        let mut gml_id_vec = Vec::with_capacity(count);
        let mut name_vec = Vec::with_capacity(count);
        let mut owning_business_uuid_vec = Vec::with_capacity(count);
        let mut address_vec = Vec::with_capacity(count);
        let mut usage_vec = Vec::with_capacity(count);
        let mut usage_code_vec = Vec::with_capacity(count);
        let mut class_vec = Vec::with_capacity(count);
        let mut class_code_vec = Vec::with_capacity(count);
        let mut city_vec = Vec::with_capacity(count);
        let mut city_code_vec = Vec::with_capacity(count);
        let mut center_vec = Vec::with_capacity(count);
        let mut footprint_vec = Vec::with_capacity(count);
        let mut height_vec = Vec::with_capacity(count);
        let mut prefecture_vec = Vec::with_capacity(count);

        for building in buildings {
            uuid_vec.push(building.uuid);
            gml_id_vec.push(building.gml_id);
            name_vec.push(building.name);
            owning_business_uuid_vec.push(building.owning_business_uuid);
            address_vec.push(building.address);
            usage_vec.push(building.usage);
            usage_code_vec.push(building.usage_code);
            class_vec.push(building.class);
            class_code_vec.push(building.class_code);
            city_vec.push(building.city);
            city_code_vec.push(building.city_code);
            center_vec.push(from_geo_point_to_pg_point(building.center));
            footprint_vec.push(from_geo_polygon_to_pg_points(building.footprint));
            height_vec.push(building.height);
            prefecture_vec.push(building.prefecture);
        }

        sqlx::query(
            r#"
            INSERT INTO buildings (
                game_tick,
                uuid,
                gml_id,
                name,
                owning_business_uuid,
                address,
                usage,
                usage_code,
                class,
                class_code,
                city,
                city_code,
                center,
                footprint,
                height,
                prefecture,
            )
            SELECT $1, u.*
            FROM unnest(
                $2::UUID[],
                $3::TEXT[],
                $4::TEXT[],
                $5::UUID[],
                $6::TEXT[],
                $7::TEXT[],
                $8::SMALLINT[],
                $9::TEXT[],
                $10::TEXT[],
                $11::TEXT[],
                $12::TEXT[],
                $13::geometry[],
                $14::geometry[],
                $15::DOUBLE PRECISION[],
                $16::TEXT[]
            )
            AS u(
                game_tick,
                uuid,
                gml_id,
                name,
                owning_business_uuid,
                address,
                usage,
                usage_code,
                class,
                class_code,
                city,
                city_code,
                center,
                footprint,
                height,
                prefecture,
            )
            "#,
        )
        .bind(game_tick)
        .bind(&uuid_vec)
        .bind(&gml_id_vec)
        .bind(&name_vec)
        .bind(&owning_business_uuid_vec)
        .bind(&address_vec)
        .bind(&usage_vec)
        .bind(&usage_code_vec)
        .bind(&class_vec)
        .bind(&class_code_vec)
        .bind(&city_vec)
        .bind(&city_code_vec)
        .bind(&center_vec)
        .bind(&footprint_vec)
        .bind(&height_vec)
        .bind(&prefecture_vec)
        .execute(executor)
        .await?;

        Ok(())
    }

    pub async fn list_buildings_in_tick(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        game_tick: i64,
    ) -> RepositoryResult<Vec<Building>> {
        let records = sqlx::query!(
            r#"
            SELECT
                game_tick,
                uuid,
                gml_id,
                name,
                owning_business_uuid,
                address,
                usage,
                usage_code,
                class,
                class_code,
                city,
                city_code,
                center,
                footprint,
                height,
                prefecture
            FROM buildings
            WHERE
                game_tick = $1
            "#,
            game_tick
        )
        .fetch_all(executor)
        .await?;

        let mut buildings = Vec::with_capacity(records.len());
        for record in records {
            buildings.push(Building {
                uuid: record.uuid,
                gml_id: record.gml_id,
                name: record.name,
                owning_business_uuid: record.owning_business_uuid,
                address: record.address,
                usage: record.usage,
                usage_code: record.usage_code,
                class: record.class,
                class_code: record.class_code,
                city: record.city,
                city_code: record.city_code,
                center: from_pg_point_to_geo_point(record.center),
                footprint: from_pg_polygon_to_geo_polygon(record.footprint),
                height: record.height,
                prefecture: record.prefecture,
            });
        }

        Ok(buildings)
    }

    pub async fn delete_buildings_before_tick(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        game_tick: i64,
    ) -> RepositoryResult<u64> {
        let result = sqlx::query!(
            r#"
            DELETE FROM buildings
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

pub struct PgBuildingService {
    pg_db: Arc<PostgresDatabase>,
    building_repo: PgBuildingRepository,
}

impl PgBuildingService {
    pub fn new(pg_db: Arc<PostgresDatabase>) -> Self {
        Self {
            pg_db,
            building_repo: PgBuildingRepository,
        }
    }
}

#[tonic::async_trait]
impl BuildingRepository for PgBuildingService {
    async fn list_buildings_in_tick(&self, game_tick: i64) -> RepositoryResult<Vec<Building>> {
        self.building_repo
            .list_buildings_in_tick(&self.pg_db.pool, game_tick)
            .await
    }
}

#[tonic::async_trait]
impl BuildingTxRepository for PgTransactionContext<'_, '_> {
    async fn insert_buildings_in_tick(
        &mut self,
        game_tick: i64,
        buildings: Vec<Building>,
    ) -> RepositoryResult<()> {
        self.building_repo
            .insert_buildings_in_tick(&mut **self.tx, buildings, game_tick)
            .await
    }

    async fn delete_buildings_before_tick(&mut self, game_tick: i64) -> RepositoryResult<u64> {
        self.building_repo
            .delete_buildings_before_tick(&mut **self.tx, game_tick)
            .await
    }
}
