use crate::domain::{
    economy::{
        business::model::Business, business_listing::model::BusinessListing,
        corporation::model::Corporation, market::model::Market,
    },
    unit::model::Unit,
};
use std::collections::HashMap;
use uuid::Uuid;

pub struct GameState {
    pub units_map: HashMap<Uuid, Unit>,
    pub corporations_map: HashMap<Uuid, Corporation>,
    pub markets_map: HashMap<Uuid, Market>,
    pub businesses_map: HashMap<Uuid, Business>,
    pub business_listings_map: HashMap<Uuid, BusinessListing>,

    // Indices
    pub corporation_uuid_by_user_uuid: HashMap<Uuid, Uuid>,
}

impl GameState {
    pub fn build(
        units_vec: Vec<Unit>,
        corporations_vec: Vec<Corporation>,
        markets_vec: Vec<Market>,
        businesses_vec: Vec<Business>,
        business_listings_vec: Vec<BusinessListing>,
    ) -> Self {
        let mut units_map = HashMap::with_capacity(units_vec.len());
        let mut corporations_map = HashMap::with_capacity(corporations_vec.len());
        let mut markets_map = HashMap::with_capacity(markets_vec.len());
        let mut businesses_map = HashMap::with_capacity(businesses_vec.len());
        let mut business_listings_map = HashMap::with_capacity(business_listings_vec.len());
        let mut corporation_uuid_by_user_uuid = HashMap::with_capacity(corporations_vec.len());

        for unit in units_vec {
            units_map.insert(unit.uuid, unit);
        }

        for corporation in corporations_vec {
            corporation_uuid_by_user_uuid.insert(corporation.user_uuid, corporation.uuid);
            corporations_map.insert(corporation.uuid, corporation);
        }

        for market in markets_vec {
            markets_map.insert(market.uuid, market);
        }

        for business in businesses_vec {
            businesses_map.insert(business.uuid, business);
        }

        for business_listing in business_listings_vec {
            business_listings_map.insert(business_listing.uuid, business_listing);
        }

        Self {
            units_map,
            corporations_map,
            corporation_uuid_by_user_uuid,
            markets_map,
            businesses_map,
            business_listings_map,
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

    // Convenience (less needed now)
    pub fn ref_mut_corporation_by_user(&mut self, user_uuid: &Uuid) -> Option<&mut Corporation> {
        let uuid = self.get_corporation_uuid_by_user(user_uuid)?.clone();
        self.ref_mut_corporation(&uuid)
    }
}
