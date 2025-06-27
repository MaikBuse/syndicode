use crate::{
    domain::{
        economy::building::{model::Building, repository::BuildingTxRepository},
        repository::RepositoryResult,
    },
    infrastructure::postgres::{
        from_geo_point_to_pg_point, from_geo_polygon_to_pg_points, uow::PgTransactionContext,
    },
};
use sqlx::Postgres;

#[derive(Clone)]
pub struct PgBuildingRepository;

impl PgBuildingRepository {
    /// This leverages PostgreSQL's UNNEST function for efficiency.
    /// CARE: This is not compile time checked
    pub async fn insert_buildings(
        &self,
        executor: impl sqlx::Executor<'_, Database = Postgres>,
        buildings: Vec<Building>,
    ) -> RepositoryResult<()> {
        if buildings.is_empty() {
            return Ok(());
        }

        let count = buildings.len();

        let mut uuid_vec = Vec::with_capacity(count);
        let mut gml_id_vec = Vec::with_capacity(count);
        let mut name_vec = Vec::with_capacity(count);
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
                    uuid,
                    gml_id,
                    name,
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
                )
                SELECT *
                FROM unnest(
                    $1::UUID[],
                    $2::TEXT[],
                    $3::TEXT[],
                    $4::TEXT[],
                    $5::TEXT[],
                    $6::SMALLINT[],
                    $7::TEXT[],
                    $8::SMALLINT[],
                    $9::TEXT[],
                    $10::TEXT[],
                    $11::geometry[],
                    $12::geometry[],
                    $13::DOUBLE PRECISION[],
                    $14::TEXT[]
                )
                AS u(
                    uuid,
                    gml_id,
                    name,
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
                )
                "#,
        )
        .bind(&uuid_vec)
        .bind(&gml_id_vec)
        .bind(&name_vec)
        .bind(&address_vec)
        .bind(&usage_vec)
        .bind(&usage_code_vec) // This is now Vec<Option<i16>>
        .bind(&class_vec)
        .bind(&class_code_vec) // This is now Vec<Option<i16>>
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
}

#[tonic::async_trait]
impl BuildingTxRepository for PgTransactionContext<'_, '_> {
    async fn insert_buildings(&mut self, buildings: Vec<Building>) -> RepositoryResult<()> {
        self.building_repo
            .insert_buildings(&mut **self.tx, buildings)
            .await
    }
}
