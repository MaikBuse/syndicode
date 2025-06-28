use crate::domain::{
    economy::{
        building_ownership::model::BuildingOwnership, business::model::Business,
        business_listing::model::BusinessListing, business_offer::model::BusinessOffer,
        corporation::model::Corporation, market::model::Market,
    },
    unit::model::Unit,
};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

pub struct GameState {
    /// The last tick number that was successfully processed and persisted.
    pub last_processed_tick: i64,
    // Game State Maps
    pub units_map: HashMap<Uuid, Unit>,
    pub corporations_map: HashMap<Uuid, Corporation>,
    pub markets_map: HashMap<Uuid, Market>,
    pub businesses_map: HashMap<Uuid, Business>,
    pub business_listings_map: HashMap<Uuid, BusinessListing>,
    pub business_offers_map: HashMap<Uuid, BusinessOffer>,
    pub building_ownerships_map: HashMap<Uuid, BuildingOwnership>,

    // Aggregates
    pub total_operation_expenses_by_market_uuid: HashMap<Uuid, i64>,

    // Indices
    pub business_uuids_by_market_uuid: HashMap<Uuid, Vec<Uuid>>,
    pub corporation_uuid_by_user_uuid: HashMap<Uuid, Uuid>,
    pub corporation_names: HashSet<String>,
    pub business_uuids_by_corporation_uuid: HashMap<Uuid, Vec<Uuid>>,
    pub business_listing_uuids_by_corporation_uuid: HashMap<Uuid, Vec<Uuid>>,
    pub business_offer_uuids_by_corporation_uuid: HashMap<Uuid, Vec<Uuid>>,
    pub unit_uuids_by_corporation_uuid: HashMap<Uuid, Vec<Uuid>>,
}

impl GameState {
    pub fn build(
        last_processed_tick: i64,
        units_vec: Vec<Unit>,
        corporations_vec: Vec<Corporation>,
        markets_vec: Vec<Market>,
        businesses_vec: Vec<Business>,
        business_listings_vec: Vec<BusinessListing>,
        business_offers_vec: Vec<BusinessOffer>,
        building_ownerships_vec: Vec<BuildingOwnership>,
    ) -> Self {
        // Game State Maps
        let mut units_map = HashMap::with_capacity(units_vec.len());
        let mut corporations_map = HashMap::with_capacity(corporations_vec.len());
        let mut markets_map = HashMap::with_capacity(markets_vec.len());
        let mut businesses_map = HashMap::with_capacity(businesses_vec.len());
        let mut business_listings_map = HashMap::with_capacity(business_listings_vec.len());
        let mut business_offers_map = HashMap::with_capacity(business_offers_vec.len());
        let mut building_ownerships_map = HashMap::with_capacity(building_ownerships_vec.len());

        // Aggregates
        let mut total_operation_expenses_by_market_uuid = HashMap::with_capacity(markets_vec.len());

        // Indices
        let mut business_uuids_by_market_uuid = HashMap::with_capacity(markets_vec.len());
        let mut corporation_names = HashSet::with_capacity(corporations_vec.len());
        let mut corporation_uuid_by_user_uuid = HashMap::with_capacity(corporations_vec.len());
        let mut business_uuids_by_corporation_uuid = HashMap::with_capacity(corporations_vec.len());
        let mut business_listing_uuids_by_corporation_uuid =
            HashMap::with_capacity(corporations_vec.len());
        let mut business_offer_uuids_by_corporation_uuid =
            HashMap::with_capacity(corporations_vec.len());
        let mut unit_uuids_by_corporation_uuid = HashMap::with_capacity(corporations_vec.len());

        for unit in units_vec {
            // unit_uuids_by_corporation_uuid
            let unit_uuids: &mut Vec<Uuid> = unit_uuids_by_corporation_uuid
                .entry(unit.corporation_uuid)
                .or_default();
            unit_uuids.push(unit.uuid);

            // units_map
            units_map.insert(unit.uuid, unit);
        }

        for corporation in corporations_vec {
            // corporation_names
            corporation_names.insert(corporation.name.to_string());

            // corporation_uuid_by_user_uuid
            corporation_uuid_by_user_uuid.insert(corporation.user_uuid, corporation.uuid);

            // corporations_map
            corporations_map.insert(corporation.uuid, corporation);
        }

        for market in markets_vec {
            // markets_map
            markets_map.insert(market.uuid, market);
        }

        for business in businesses_vec {
            // total_operation_expenses_by_market_uuid
            let total_expenses: &mut i64 = total_operation_expenses_by_market_uuid
                .entry(business.market_uuid)
                .or_default();
            *total_expenses += business.operational_expenses;

            // business_uuids_by_corporation_uuid
            if let Some(owning_corporation_uuid) = business.owning_corporation_uuid {
                let business_uuids: &mut Vec<Uuid> = business_uuids_by_corporation_uuid
                    .entry(owning_corporation_uuid)
                    .or_default();
                business_uuids.push(business.uuid);
            }

            // business_uuids_by_market_uuid
            let business_uuids: &mut Vec<Uuid> = business_uuids_by_market_uuid
                .entry(business.market_uuid)
                .or_default();
            business_uuids.push(business.uuid);

            // businesses_map
            businesses_map.insert(business.uuid, business);
        }

        for business_listing in business_listings_vec {
            // business_listing_uuids_by_corporation_uuid
            if let Some(seller_corporation_uuid) = business_listing.seller_corporation_uuid {
                let business_listing_uuids: &mut Vec<Uuid> =
                    business_listing_uuids_by_corporation_uuid
                        .entry(seller_corporation_uuid)
                        .or_default();
                business_listing_uuids.push(business_listing.uuid);
            }

            // business_listings_map
            business_listings_map.insert(business_listing.uuid, business_listing);
        }

        for business_offer in business_offers_vec {
            // business_offer_uuids_by_corporation_uuid
            let business_offer_uuids = business_offer_uuids_by_corporation_uuid
                .entry(business_offer.offering_corporation_uuid)
                .or_insert(Vec::new());
            business_offer_uuids.push(business_offer.uuid);

            // business_offers_map
            business_offers_map.insert(business_offer.uuid, business_offer);
        }

        for building_ownership in building_ownerships_vec {
            building_ownerships_map.insert(building_ownership.building_uuid, building_ownership);
        }

        Self {
            last_processed_tick,
            units_map,
            corporations_map,
            corporation_names,
            markets_map,
            businesses_map,
            business_listings_map,
            business_offers_map,
            building_ownerships_map,
            total_operation_expenses_by_market_uuid,
            corporation_uuid_by_user_uuid,
            business_uuids_by_market_uuid,
            business_uuids_by_corporation_uuid,
            business_listing_uuids_by_corporation_uuid,
            business_offer_uuids_by_corporation_uuid,
            unit_uuids_by_corporation_uuid,
        }
    }

    // --- Mutators ---
    pub fn add_unit(&mut self, unit: Unit) {
        self.units_map.insert(unit.uuid, unit);
    }
    pub fn remove_business_listing(&mut self, uuid: &Uuid) -> Option<BusinessListing> {
        self.business_listings_map.remove(uuid)
    }
    pub fn add_business_listing(&mut self, listing: BusinessListing) {
        self.business_listings_map.insert(listing.uuid, listing);
    }

    // --- Mutable Accessors ---
    pub fn ref_mut_corporation(&mut self, uuid: &Uuid) -> Option<&mut Corporation> {
        self.corporations_map.get_mut(uuid)
    }
    pub fn ref_mut_business(&mut self, uuid: &Uuid) -> Option<&mut Business> {
        self.businesses_map.get_mut(uuid)
    }
    pub fn ref_mut_business_listing(&mut self, uuid: &Uuid) -> Option<&mut BusinessListing> {
        self.business_listings_map.get_mut(uuid)
    }

    // --- Immutable Accessors ---
    pub fn get_corporation_uuid_by_user(&self, user_uuid: &Uuid) -> Option<&Uuid> {
        self.corporation_uuid_by_user_uuid.get(user_uuid)
    }
    pub fn ref_corporation(&self, uuid: &Uuid) -> Option<&Corporation> {
        self.corporations_map.get(uuid)
    }
    pub fn ref_business(&self, uuid: &Uuid) -> Option<&Business> {
        self.businesses_map.get(uuid)
    }
    pub fn ref_business_listing(&self, uuid: &Uuid) -> Option<&BusinessListing> {
        self.business_listings_map.get(uuid)
    }
}
