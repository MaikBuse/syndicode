use crate::{
    application::{
        error::ApplicationResult,
        ports::{init::InitializationRepository, uow::UnitOfWork},
    },
    config::ServerConfig,
    domain::economy::{
        building::{model::Building, point::BuildingPoint},
        building_ownership::model::BuildingOwnership,
        business::{generator::generate_business_name, model::Business},
        business_listing::model::BusinessListing,
        market::model::{name::MarketName, Market},
    },
};
use arrow::array::Array;
use bon::Builder;
use geo::{polygon, prelude::Centroid, Distance, Haversine, Point};
use indicatif::{ProgressBar, ProgressStyle};
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use rand::Rng;
use rayon::prelude::*;
use rstar::RTree;
use std::{collections::HashSet, path::Path, time::Instant};
use std::{fs::File, sync::Arc};
use uuid::Uuid;

const PARQUET_BASE_PATH: &str = "assets/parquet";

const PARQUET_FILES: [&str; 23] = [
    "13101_chiyoda-ku_2023_building_lod0.parquet",
    "13102_chuo-ku_2023_building_lod0.parquet",
    "13103_minato-ku_2023_building_lod0.parquet",
    "13104_shinjuku-ku_2023_building_lod0.parquet",
    "13105_bunkyo-ku_2023_building_lod0.parquet",
    "13106_taito-ku_2023_building_lod0.parquet",
    "13107_sumida-ku_2023_building_lod0.parquet",
    "13108_koto-ku_2023_building_lod0.parquet",
    "13109_shinagawa-ku_2023_building_lod0.parquet",
    "13110_meguro-ku_2023_building_lod0.parquet",
    "13111_ota-ku_2023_building_lod0.parquet",
    "13112_setagaya-ku_2023_building_lod0.parquet",
    "13113_shibuya-ku_2023_building_lod0.parquet",
    "13114_nakano-ku_2023_building_lod0.parquet",
    "13115_suginami-ku_2023_building_lod0.parquet",
    "13116_toshima-ku_2023_building_lod0.parquet",
    "13117_kita-ku_2023_building_lod0.parquet",
    "13118_arakawa-ku_2023_building_lod0.parquet",
    "13119_itabashi-ku_2023_building_lod0.parquet",
    "13120_nerima-ku_2023_building_lod0.parquet",
    "13121_adachi-ku_2023_building_lod0.parquet",
    "13122_katsushika-ku_2023_building_lod0.parquet",
    "13123_edogawa-ku_2023_building_lod0.parquet",
];

const PROGRESS_BAR_TEMPLATE: &str =
    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})";
const PROGRESS_BAR_CHARS: &str = "#>-";

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

        let buildings = load_buildings_from_parquet()?;
        let central_points = generate_central_points(
            self.config.bootstrap.business_count_x,
            self.config.bootstrap.business_count_y,
            self.config.bootstrap.boundary_min_lon,
            self.config.bootstrap.boundary_max_lon,
            self.config.bootstrap.boundary_min_lat,
            self.config.bootstrap.boundary_max_lat,
        );

        let (businesses, building_ownerships) = assign_buildings_to_businesses(
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

                    tracing::info!("Inserting markets...");
                    ctx.insert_markets_in_tick(game_tick, markets).await?;

                    tracing::info!("Inserting businesses...");
                    ctx.insert_businesses_in_tick(game_tick, businesses).await?;

                    tracing::info!("Inserting business_listings...");
                    ctx.insert_business_listings_in_tick(game_tick, business_listings).await?;

                    tracing::info!("Inserting buildings...");
                    ctx.insert_buildings(buildings).await?;

                    tracing::info!("Inserting building ownerships...");
                    ctx.insert_building_ownerships_in_tick(game_tick, building_ownerships).await?;

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

/// Loads building data from a Flateau Parquet file using a high-performance
/// columnar and parallel approach.
pub fn load_buildings_from_parquet() -> anyhow::Result<Vec<Building>> {
    let mut prefecture_buildings = Vec::new();

    for name in PARQUET_FILES {
        tracing::info!("Reading buildings from parquet: {}", name);

        let start_time = Instant::now();

        let path = Path::new(PARQUET_BASE_PATH).join(name);

        let file = File::open(path)?;

        // Use the Arrow-based RecordBatchReader for columnar access
        let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;

        // Get total rows for the progress bar
        let total_rows = builder.metadata().file_metadata().num_rows() as u64;

        let reader = builder.build()?;

        // Set up the progress bar
        let pb = ProgressBar::new(total_rows);
        pb.set_style(
            ProgressStyle::default_bar()
                .template(PROGRESS_BAR_TEMPLATE)?
                .progress_chars(PROGRESS_BAR_CHARS),
        );

        // Process record batches in parallel using Rayon
        let ward_buildings: Vec<Building> = reader
            .into_iter()
            .filter_map(Result::ok) // Ignore batches that fail to read
            .par_bridge() // Switch to a parallel iterator
            .flat_map(|batch| {
                // Increment the progress bar after processing a batch
                // It's thread-safe and designed for this.
                pb.inc(batch.num_rows() as u64);

                // Cast columns to their concrete Arrow Array types. This is very fast.
                // This looks verbose, but it's much more efficient than row-by-row access.
                let gml_id_arr = batch
                    .column_by_name("gml_id")
                    .unwrap()
                    .as_any()
                    .downcast_ref::<arrow::array::StringArray>()
                    .unwrap();
                let prefecture_arr = batch
                    .column_by_name("prefecture")
                    .unwrap()
                    .as_any()
                    .downcast_ref::<arrow::array::StringArray>()
                    .unwrap();
                let class_arr = batch
                    .column_by_name("class")
                    .unwrap()
                    .as_any()
                    .downcast_ref::<arrow::array::StringArray>()
                    .unwrap();
                let class_code_arr = batch
                    .column_by_name("class_code")
                    .unwrap()
                    .as_any()
                    .downcast_ref::<arrow::array::StringArray>()
                    .unwrap();
                let cal_xmin_arr = batch
                    .column_by_name("cal_xmin")
                    .unwrap()
                    .as_any()
                    .downcast_ref::<arrow::array::Float64Array>()
                    .unwrap();
                let cal_xmax_arr = batch
                    .column_by_name("cal_xmax")
                    .unwrap()
                    .as_any()
                    .downcast_ref::<arrow::array::Float64Array>()
                    .unwrap();
                let cal_ymin_arr = batch
                    .column_by_name("cal_ymin")
                    .unwrap()
                    .as_any()
                    .downcast_ref::<arrow::array::Float64Array>()
                    .unwrap();
                let cal_ymax_arr = batch
                    .column_by_name("cal_ymax")
                    .unwrap()
                    .as_any()
                    .downcast_ref::<arrow::array::Float64Array>()
                    .unwrap();
                let cal_height_m_arr = batch
                    .column_by_name("cal_height_m")
                    .unwrap()
                    .as_any()
                    .downcast_ref::<arrow::array::Float64Array>()
                    .unwrap();
                let city_arr = batch
                    .column_by_name("city")
                    .unwrap()
                    .as_any()
                    .downcast_ref::<arrow::array::StringArray>()
                    .unwrap();
                let city_code_arr = batch
                    .column_by_name("city_code")
                    .unwrap()
                    .as_any()
                    .downcast_ref::<arrow::array::StringArray>()
                    .unwrap();
                let name_arr = batch
                    .column_by_name("gml_name")
                    .unwrap()
                    .as_any()
                    .downcast_ref::<arrow::array::StringArray>()
                    .unwrap();
                let address_arr = batch
                    .column_by_name("address")
                    .unwrap()
                    .as_any()
                    .downcast_ref::<arrow::array::StringArray>()
                    .unwrap();
                let usage_arr = batch
                    .column_by_name("usage")
                    .unwrap()
                    .as_any()
                    .downcast_ref::<arrow::array::StringArray>()
                    .unwrap();
                let usage_code_arr = batch
                    .column_by_name("usage_code")
                    .unwrap()
                    .as_any()
                    .downcast_ref::<arrow::array::StringArray>()
                    .unwrap();

                // Iterate from 0 to num_rows and extract data by index.
                // This is cache-friendly and avoids repeated lookups.
                (0..batch.num_rows())
                    .into_par_iter() // You can even parallelize within a batch for very large batches
                    .map(|i| {
                        let cal_xmin = cal_xmin_arr.value(i);
                        let cal_xmax = cal_xmax_arr.value(i);
                        let cal_ymin = cal_ymin_arr.value(i);
                        let cal_ymax = cal_ymax_arr.value(i);

                        let footprint = polygon![
                            (x: cal_xmin, y: cal_ymin),
                            (x: cal_xmax, y: cal_ymin),
                            (x: cal_xmax, y: cal_ymax),
                            (x: cal_xmin, y: cal_ymax),
                        ];

                        // The centroid calculation is safe for a rectangle
                        let centroid = footprint.centroid().unwrap();

                        // Helper to reduce repetition for optional string fields
                        // This gets a &str slice, avoiding allocation until .to_string()
                        let get_opt_string = |arr: &arrow::array::StringArray, idx: usize| {
                            if arr.is_valid(idx) {
                                Some(arr.value(idx).to_string())
                            } else {
                                None
                            }
                        };

                        Building {
                            uuid: Uuid::now_v7(),
                            gml_id: gml_id_arr.value(i).to_string(),
                            height: cal_height_m_arr.value(i),
                            footprint,
                            center: centroid,
                            name: get_opt_string(name_arr, i),
                            address: get_opt_string(address_arr, i),
                            city: get_opt_string(city_arr, i),
                            city_code: get_opt_string(city_code_arr, i),
                            class: get_opt_string(class_arr, i),
                            class_code: get_opt_string(class_code_arr, i),
                            usage: get_opt_string(usage_arr, i),
                            usage_code: get_opt_string(usage_code_arr, i),
                            prefecture: get_opt_string(prefecture_arr, i),
                        }
                    })
                    .collect::<Vec<Building>>()
            })
            .collect();

        prefecture_buildings.extend(ward_buildings);

        //Finish the progress bar and stop the timer
        pb.finish();
        let duration = start_time.elapsed();
        tracing::info!(
            "Successfully loaded {} buildings in {:.2?}.",
            prefecture_buildings.len(),
            duration
        );
    }

    Ok(prefecture_buildings)
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
    tracing::info!("Generating central points for businesses");

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

// A temporary struct to hold the results from the parallel processing
struct BusinessProposal {
    center: Point<f64>,
    market_uuid: Uuid,
    claimed_building_uuids: Vec<Uuid>,
}

pub fn assign_buildings_to_businesses(
    markets: &[Market],
    buildings: &[Building],
    central_points: Vec<Point<f64>>,
    sigma_meters: f64,
    max_radius_meters: f64,
) -> anyhow::Result<(Vec<Business>, Vec<BuildingOwnership>)> {
    tracing::info!("Assigning buildings to businesses");

    if markets.is_empty() {
        return Err(anyhow::anyhow!("Markets slice is empty"));
    }

    let building_points: Vec<BuildingPoint> = buildings
        .iter()
        .map(|b| BuildingPoint { building: b })
        .collect();
    let rtree = RTree::bulk_load(building_points);

    let variance = sigma_meters.powi(2);
    let max_radius_sq = max_radius_meters.powi(2);

    tracing::info!("Phase 1: Mapping buildings...");

    // Set up the progress bar
    let pb = ProgressBar::new(central_points.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(PROGRESS_BAR_TEMPLATE)?
            .progress_chars(PROGRESS_BAR_CHARS),
    );

    // Parallel "Map" phase
    let proposals: Vec<BusinessProposal> = central_points
        .into_par_iter()
        .enumerate()
        .filter_map(|(i, center)| {
            // Increment the progress bar after processing a batch
            // It's thread-safe and designed for this.
            let mut rng = rand::rng();
            let market = &markets[i % markets.len()];

            let search_center = [center.x(), center.y()];
            let mut claimed_building_uuids = Vec::new();

            for building_point in rtree.locate_within_distance(search_center, max_radius_sq) {
                let building = building_point.building;
                let distance = Haversine.distance(building.center, center);

                let probability = (-distance.powi(2) / (2.0 * variance)).exp();
                if rng.random::<f64>() < probability {
                    claimed_building_uuids.push(building.uuid);
                }
            }

            let result = if claimed_building_uuids.is_empty() {
                None
            } else {
                Some(BusinessProposal {
                    center,
                    market_uuid: market.uuid,
                    claimed_building_uuids,
                })
            };

            pb.inc(1);

            result
        })
        .collect();

    pb.finish();

    tracing::info!(
        "Phase 2: Resolving {} business proposals...",
        proposals.len()
    );

    // Pass 1: Count total successful assignments to determine exact capacity
    let mut temp_assigned_ids: HashSet<Uuid> = HashSet::new();
    let mut total_successful_assignments = 0;
    for proposal in &proposals {
        for building_uuid in &proposal.claimed_building_uuids {
            if temp_assigned_ids.insert(*building_uuid) {
                total_successful_assignments += 1;
            }
        }
    }
    // Clear the temporary set to free its memory
    drop(temp_assigned_ids);

    tracing::info!(
        "Found {} unique buildings to be assigned.",
        total_successful_assignments
    );

    // Pass 2: Allocate with perfect capacity and populate
    let mut businesses = Vec::with_capacity(proposals.len());
    let mut building_ownerships = Vec::with_capacity(total_successful_assignments);
    let mut assigned_building_uuids = HashSet::with_capacity(total_successful_assignments);

    // This map is needed to get market names later.
    let market_map: std::collections::HashMap<Uuid, MarketName> =
        markets.iter().map(|m| (m.uuid, m.name)).collect();

    for proposal in proposals {
        let mut successfully_assigned_buildings = Vec::new();
        for building_uuid in proposal.claimed_building_uuids {
            if assigned_building_uuids.insert(building_uuid) {
                successfully_assigned_buildings.push(building_uuid);
            }
        }

        if !successfully_assigned_buildings.is_empty() {
            let business_uuid = Uuid::now_v7();
            let market_name = market_map.get(&proposal.market_uuid).unwrap();
            let business_name = generate_business_name(*market_name);

            businesses.push(
                Business::builder()
                    .center(proposal.center)
                    .uuid(business_uuid)
                    .name(business_name)
                    .operational_expenses(0)
                    .market_uuid(proposal.market_uuid)
                    .build(),
            );

            for building_uuid in successfully_assigned_buildings {
                building_ownerships.push(
                    BuildingOwnership::builder()
                        .owning_business_uuid(business_uuid)
                        .building_uuid(building_uuid)
                        .build(),
                );
            }
        }
    }

    Ok((businesses, building_ownerships))
}
