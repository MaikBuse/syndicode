use crate::{
    domain::{
        economy::building::{model::Building, repository::BuildingTxRepository},
        repository::RepositoryResult,
    },
    infrastructure::postgres::{uow::PgTransactionContext, SRID},
};
use sqlx::{Executor, Postgres};
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

        // --- GEOMETRY VECTORS ARE NOW STRINGS ---
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
}

#[tonic::async_trait]
impl BuildingTxRepository for PgTransactionContext<'_, '_> {
    async fn insert_buildings(&mut self, buildings: Vec<Building>) -> RepositoryResult<()> {
        self.building_repo
            .insert_buildings(&mut **self.tx, buildings)
            .await
    }
}
