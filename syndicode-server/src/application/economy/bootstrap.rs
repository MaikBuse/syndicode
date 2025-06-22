use crate::{
    application::{
        error::ApplicationResult,
        ports::{init::InitializationRepository, uow::UnitOfWork},
    },
    config::ServerConfig,
    domain::economy::{
        building::{model::Building, point::BuildingPoint},
        business::{generator::generate_business_name, model::Business},
        business_listing::model::BusinessListing,
        market::model::{name::MarketName, Market},
    },
};
use bon::Builder;
use geo::{polygon, prelude::Centroid, Distance, Haversine, Point};
use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::record::RowAccessor;
use rand::{seq::IteratorRandom, Rng};
use rstar::RTree;
use std::{collections::HashSet, path::Path};
use std::{fs::File, sync::Arc};
use uuid::Uuid;

const MARKET_NAMES: [MarketName; 10] = [
    MarketName::AutonomousDrone,
    MarketName::VirtualSimSense,
    MarketName::StreetPharm,
    MarketName::ZeroDayExploit,
    MarketName::RestrictedTech,
    MarketName::InfoSecCounterIntel,
    MarketName::WetwareNeural,
    MarketName::AugmentationCybernetics,
    MarketName::SyndicateData,
    MarketName::BlackMarketBio,
];

macro_rules! get_record_value {
    ($record:expr, $method:ident, $idx:expr, $field:expr, $row_group_idx:expr, $record_idx:expr) => {
        match $record.$method($idx) {
            Ok(val) => val,
            Err(_) => {
                return Err(anyhow::anyhow!(
                    "Failed to retrieve '{}' from row group '{}' and record '{}'",
                    $field,
                    $row_group_idx,
                    $record_idx
                ));
            }
        }
    };
}

#[derive(Builder)]
pub struct BootstrapEconomyUseCase<UOW, INI>
where
    UOW: UnitOfWork,
    INI: InitializationRepository,
{
    config: Arc<ServerConfig>,
    uow: Arc<UOW>,
    init_repo: Arc<INI>,
}

impl<UOW, INI> BootstrapEconomyUseCase<UOW, INI>
where
    UOW: UnitOfWork,
    INI: InitializationRepository,
{
    pub async fn execute(&self) -> ApplicationResult<()> {
        if self.init_repo.is_database_initialized().await? {
            tracing::info!("Database initialization flag is already set. Skipping.");

            return Ok(());
        }

        tracing::info!(
            "Database initialization flag not set or missing. Attempting initialization lock..."
        );

        self.init_repo.set_advisory_lock().await?;

        tracing::info!("Acquired initialization advisory lock.");

        let mut markets: Vec<Market> = Vec::with_capacity(MARKET_NAMES.len());

        for name in MARKET_NAMES {
            let market = Market::builder()
                .uuid(Uuid::now_v7())
                .name(name)
                .volume(1000)
                .build();

            markets.push(market);
        }

        let buildings = load_buildings_from_flateau(self.config.bootstrap.parquet_path.as_str())?;
        let central_points = generate_central_points(
            self.config.bootstrap.business_count_x,
            self.config.bootstrap.business_count_y,
            self.config.bootstrap.boundary_min_lon,
            self.config.bootstrap.boundary_max_lon,
            self.config.bootstrap.boundary_min_lat,
            self.config.bootstrap.boundary_max_lat,
        );

        let businesses = assign_buildings_to_businesses(
            &markets,
            &buildings,
            central_points,
            self.config.bootstrap.spread_sigma_meters,
            self.config.bootstrap.max_radius_meters,
        )?;

        let mut business_listings: Vec<BusinessListing> = Vec::with_capacity(businesses.len());

        for business in businesses.iter() {
            business_listings.push(
                BusinessListing::builder()
                    .uuid(Uuid::now_v7())
                    .business_uuid(business.uuid)
                    .asking_price(1000)
                    .build(),
            );
        }

        self.uow
            .execute(|ctx| {
                Box::pin(async move {
                    let is_set_after_lock = ctx.is_database_initialized().await?;

                    if is_set_after_lock {
                        tracing::info!("Initialization flag was set by another instance after lock acquisition. Skipping.");
                        return Ok(());
                    }

                    let game_tick = ctx.get_current_game_tick().await?;

                    ctx.insert_markets_in_tick(game_tick, markets).await?;
                    ctx.insert_businesses_in_tick(game_tick, businesses).await?;
                    ctx.insert_business_listings_in_tick(game_tick, business_listings).await?;
                    ctx.insert_buildings_in_tick(game_tick, buildings).await?;

                    ctx.set_database_initialization_flag().await?;

                    Ok(())
                })
            })
            .await?;

        tracing::info!("Database initialization complete and flag set.");

        self.init_repo.set_advisory_lock().await?;

        tracing::info!("Released initialization advisory lock.");

        Ok(())
    }
}

/// Loads building data from a Flateau Parquet file.
///
/// This function reads the `gml_id` and the WKB `geometry` for each building,
/// calculates the centroid of the geometry, and returns a list of Buildings.
pub fn load_buildings_from_flateau(path: &str) -> anyhow::Result<Vec<Building>> {
    let file = File::open(Path::new(path))?;
    let reader = SerializedFileReader::new(file)?;

    // Find the column indices from the schema by iterating
    let schema_descr = reader.metadata().file_metadata().schema_descr();

    let get_index_of_column = |column_name: &str| -> anyhow::Result<usize> {
        schema_descr
            .columns()
            .iter()
            .position(|c| c.name() == column_name)
            .ok_or(anyhow::anyhow!("Column '{}' not found", column_name))
    };

    let gml_id_idx = get_index_of_column("gml_id")?;
    let prefecture_idx = get_index_of_column("prefecture")?;
    let class_idx = get_index_of_column("class")?;
    let class_code_idx = get_index_of_column("class_code")?;
    let cal_xmin_idx = get_index_of_column("cal_xmin")?;
    let cal_xmax_idx = get_index_of_column("cal_xmax")?;
    let cal_ymin_idx = get_index_of_column("cal_ymin")?;
    let cal_ymax_idx = get_index_of_column("cal_ymax")?;
    let cal_height_m_idx = get_index_of_column("cal_height_m")?;
    let city_idx = get_index_of_column("city")?;
    let city_code_idx = get_index_of_column("city_code")?;
    let name_idx = get_index_of_column("gml_name")?;
    let address_idx = get_index_of_column("address")?;
    let usage_idx = get_index_of_column("usage")?;
    let usage_code_idx = get_index_of_column("usage_code")?;

    let num_rows = reader.metadata().file_metadata().num_rows() as usize;
    let mut buildings = Vec::with_capacity(num_rows);

    for (row_group_idx, row_group_reader) in reader.get_row_iter(None)?.enumerate() {
        tracing::info!(
            "Importing buildings of group reader with index '{}'",
            row_group_idx
        );
        for (record_idx, record) in row_group_reader.iter().enumerate() {
            let gml_id = get_record_value!(
                record,
                get_string,
                gml_id_idx,
                "gml_id",
                row_group_idx,
                record_idx
            );
            let prefecture = get_record_value!(
                record,
                get_string,
                prefecture_idx,
                "prefecture",
                row_group_idx,
                record_idx
            );
            let class = get_record_value!(
                record,
                get_string,
                class_idx,
                "class",
                row_group_idx,
                record_idx
            );
            let class_code = get_record_value!(
                record,
                get_short,
                class_code_idx,
                "class_code",
                row_group_idx,
                record_idx
            );
            let cal_xmin = get_record_value!(
                record,
                get_double,
                cal_xmin_idx,
                "cal_xmin",
                row_group_idx,
                record_idx
            );
            let cal_xmax = get_record_value!(
                record,
                get_double,
                cal_xmax_idx,
                "cal_xmax",
                row_group_idx,
                record_idx
            );
            let cal_ymin = get_record_value!(
                record,
                get_double,
                cal_ymin_idx,
                "cal_ymin",
                row_group_idx,
                record_idx
            );
            let cal_ymax = get_record_value!(
                record,
                get_double,
                cal_ymax_idx,
                "cal_ymax",
                row_group_idx,
                record_idx
            );
            let cal_height_m = get_record_value!(
                record,
                get_double,
                cal_height_m_idx,
                "cal_height_m",
                row_group_idx,
                record_idx
            );
            let city = get_record_value!(
                record,
                get_string,
                city_idx,
                "city",
                row_group_idx,
                record_idx
            );
            let city_code = get_record_value!(
                record,
                get_string,
                city_code_idx,
                "city_code",
                row_group_idx,
                record_idx
            );
            let name = get_record_value!(
                record,
                get_string,
                name_idx,
                "name",
                row_group_idx,
                record_idx
            );
            let address = get_record_value!(
                record,
                get_string,
                address_idx,
                "address",
                row_group_idx,
                record_idx
            );
            let usage = get_record_value!(
                record,
                get_string,
                usage_idx,
                "usage",
                row_group_idx,
                record_idx
            );
            let usage_code = get_record_value!(
                record,
                get_short,
                usage_code_idx,
                "usage_code",
                row_group_idx,
                record_idx
            );

            let footprint = polygon![
                (x: cal_xmin, y: cal_ymin), // Bottom-left
                (x: cal_xmax, y: cal_ymin), // Bottom-right
                (x: cal_xmax, y: cal_ymax), // Top-right
                (x: cal_xmin, y: cal_ymax), // Top-left
            ];

            let Some(centroid) = footprint.centroid() else {
                return Err(anyhow::anyhow!(
                    "Failed to retrieve 'centroid' from polygon of row group '{}' and record '{}'",
                    row_group_idx,
                    record_idx
                ));
            };

            let uuid = Uuid::now_v7();

            buildings.push(Building {
                uuid,
                gml_id: gml_id.to_owned(),
                name: name.to_owned(),
                address: address.to_owned(),
                city: city.to_owned(),
                city_code: city_code.to_owned(),
                center: centroid,
                footprint,
                height: cal_height_m,
                owning_business_uuid: None,
                class: class.to_owned(),
                class_code,
                usage: usage.to_owned(),
                usage_code,
                prefecture: prefecture.to_owned(),
            });
        }
    }

    Ok(buildings)
}

/// Generates a grid of central points with random jitter within a bounding box.
pub fn generate_central_points(
    count_x: usize,
    count_y: usize,
    min_lon: f64,
    max_lon: f64,
    min_lat: f64,
    max_lat: f64,
) -> Vec<Point<f64>> {
    let mut points = Vec::new();
    let mut rng = rand::rng();

    let step_x = (max_lon - min_lon) / (count_x as f64);
    let step_y = (max_lat - min_lat) / (count_y as f64);

    for i in 0..count_x {
        for j in 0..count_y {
            let jitter_x = rng.random_range(-step_x / 4.0..step_x / 4.0);
            let jitter_y = rng.random_range(-step_y / 4.0..step_y / 4.0);

            let lon = min_lon + (i as f64 * step_x) + step_x / 2.0 + jitter_x;
            let lat = min_lat + (j as f64 * step_y) + step_y / 2.0 + jitter_y;
            points.push(Point::new(lon, lat));
        }
    }
    points
}

/// Assigns buildings to businesses ensuring each building is claimed only once.
pub fn assign_buildings_to_businesses(
    markets: &[Market],
    buildings: &[Building],
    central_points: Vec<Point<f64>>,
    sigma_meters: f64,
    max_radius_meters: f64,
) -> anyhow::Result<Vec<Business>> {
    let mut businesses = Vec::new();
    let mut rng = rand::rng();
    let mut assigned_building_uuids: HashSet<Uuid> = HashSet::new();
    let variance = sigma_meters.powi(2);

    let building_points: Vec<BuildingPoint> = buildings
        .iter()
        .map(|b| BuildingPoint { building: b })
        .collect();

    let rtree = RTree::bulk_load(building_points);

    for center in central_points {
        let Some(market) = markets.iter().choose(&mut rng) else {
            return Err(anyhow::anyhow!("Markets slice is empty"));
        };

        let mut assigned_buildings_for_this_business = Vec::new();

        // We find all buildings within the max_radius. This is extremely fast.
        let search_center = [center.x(), center.y()];
        let max_radius_sq = max_radius_meters.powi(2);

        for building_point in rtree.locate_within_distance(search_center, max_radius_sq) {
            let building = building_point.building;

            if assigned_building_uuids.contains(&building.uuid) {
                continue;
            }

            // We still need the precise Haversine distance for the probability calc.
            let distance = Haversine.distance(building.center, center);

            // Gaussian probability falloff
            let probability = (-distance.powi(2) / (2.0 * variance)).exp();
            if rng.random::<f64>() < probability {
                assigned_buildings_for_this_business.push(building.uuid);
                assigned_building_uuids.insert(building.uuid);
            }
        }

        if !assigned_buildings_for_this_business.is_empty() {
            let business_name = generate_business_name(market.name);

            let business = Business::builder()
                .center(center)
                .uuid(Uuid::now_v7())
                .name(business_name)
                .operational_expenses(0)
                .market_uuid(market.uuid)
                .build();

            businesses.push(business);
        }
    }

    Ok(businesses)
}
