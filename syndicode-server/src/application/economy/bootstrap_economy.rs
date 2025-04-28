use crate::{
    application::{
        error::ApplicationResult,
        ports::{init::InitializationRepository, uow::UnitOfWork},
    },
    domain::economy::{
        business::{generator::generate_business_name, model::Business},
        business_listing::model::BusinessListing,
        market::model::{name::MarketName, Market},
    },
};
use bon::Builder;
use std::sync::Arc;
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

const BUSINESSES_PER_MARKET: usize = 50;

#[derive(Builder)]
pub struct BootstrapEconomyUseCase<UOW, INI>
where
    UOW: UnitOfWork,
    INI: InitializationRepository,
{
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

        let mut businesses: Vec<Business> =
            Vec::with_capacity(MARKET_NAMES.len() * BUSINESSES_PER_MARKET);

        let mut business_listings: Vec<BusinessListing> = Vec::with_capacity(businesses.len());

        for market in markets.iter() {
            for _ in 0..BUSINESSES_PER_MARKET {
                let name = generate_business_name(market.name);

                let business = Business::builder()
                    .uuid(Uuid::now_v7())
                    .name(name)
                    .market_uuid(market.uuid)
                    .operational_expenses(10)
                    .build();

                businesses.push(business);
            }
        }

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
