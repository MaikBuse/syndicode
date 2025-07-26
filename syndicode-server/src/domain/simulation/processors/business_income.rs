use uuid::Uuid;

use crate::domain::simulation::game_state::GameState;

pub fn calculate_business_income(state: &mut GameState) {
    let market_uuids: Vec<Uuid> = state
        .markets_map
        .values()
        .map(|market| market.uuid)
        .collect();

    for market_uuid in market_uuids {
        let business_uuids: Vec<Uuid> = state
            .business_uuids_by_market_uuid
            .get(&market_uuid)
            .cloned()
            .unwrap_or_default();

        'for_business: for business_uuid in business_uuids {
            let Some(business) = state.businesses_map.get(&business_uuid) else {
                tracing::error!("Failed to retrieve business with UUID '{}'", business_uuid);

                continue 'for_business;
            };

            // We don't need to calculate the income of a business that does not have an owner
            let Some(owning_corporation_uuid) = business.owning_corporation_uuid else {
                continue 'for_business;
            };

            let Some(corporation_cash_balance) = state
                .ref_corporation(&owning_corporation_uuid)
                .map(|c| c.cash_balance)
            else {
                tracing::error!(
                    "Failed to retrieve owning corporation with UUID '{}'",
                    owning_corporation_uuid
                );
                continue 'for_business;
            };

            let real_op_exp = business.operational_expenses.min(corporation_cash_balance);

            let Some(total_expenses) = state
                .total_operation_expenses_by_market_uuid
                .get(&business.market_uuid)
            else {
                tracing::error!(
                    "Failed to retrieve total operational expenses of market with UUID '{}'",
                    business.market_uuid
                );
                continue 'for_business;
            };

            let market_share: f64 = if *total_expenses > 0 {
                // The common, safe case
                real_op_exp as f64 / *total_expenses as f64
            } else {
                // The total_expenses is 0
                if real_op_exp == 0 {
                    // If this business also has 0 expense, its share is 0. This is fine.
                    0.0
                } else {
                    // A business has expenses but the market total is 0. This is a data inconsistency.
                    tracing::error!(
                      "The operational expense of business '{}' is not null, but the total expense of its market '{}' is null. This state should not happen",
                      business_uuid,
                      business.market_uuid
                    );
                    continue 'for_business;
                }
            };

            let business_income: i64 = match market_share == 0. {
                true => 0,
                false => {
                    let Some(market) = state.markets_map.get(&business.market_uuid) else {
                        tracing::error!(
                            "Failed to retrieve market with UUID '{}'",
                            &business.market_uuid
                        );
                        continue 'for_business;
                    };

                    let business_income = market_share * market.volume as f64;
                    business_income.round() as i64
                }
            };

            let Some(corporation) = state.ref_mut_corporation(&owning_corporation_uuid) else {
                tracing::error!(
                    "Failed to retrieve owning corporation with UUID '{}' for mutation",
                    owning_corporation_uuid
                );

                continue 'for_business;
            };

            // Add the business income
            corporation.cash_balance += business_income;

            // Deduct the operating expense
            corporation.cash_balance -= real_op_exp;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use uuid::Uuid;

    use crate::domain::{
        economy::{
            business::model::Business,
            corporation::model::{name::CorporationName, Corporation},
            market::model::{name::MarketName, Market},
        },
        simulation::game_state::GameState,
    };

    use super::calculate_business_income;

    #[test]
    fn should_calculate_income() {
        let corporation_uuid = Uuid::now_v7();
        let corporation_name = CorporationName::unchecked("test-name".to_string());

        let corporation = Corporation {
            uuid: corporation_uuid,
            user_uuid: Uuid::now_v7(),
            name: corporation_name,
            cash_balance: 100,
        };
        let mut corporations_map = HashMap::new();
        corporations_map.insert(corporation_uuid, corporation);

        let market_uuid = Uuid::now_v7();
        let market_name = MarketName::NeurochemicalAdjustments;
        let market = Market {
            uuid: market_uuid,
            name: market_name,
            volume: 100,
        };
        let mut markets_map = HashMap::new();
        markets_map.insert(market_uuid, market);

        let business_one_uuid = Uuid::now_v7();
        let business_one_op_exp = 10_i64;
        let business_one = Business {
            uuid: business_one_uuid,
            market_uuid,
            owning_corporation_uuid: Some(corporation_uuid),
            name: "business-one".to_string(),
            operational_expenses: business_one_op_exp,
            headquarter_building_uuid: Uuid::now_v7(),
            image_number: 1,
        };

        let business_two_uuid = Uuid::now_v7();
        let business_two_op_exp = 10_i64;
        let business_two = Business {
            uuid: business_two_uuid,
            market_uuid,
            owning_corporation_uuid: None,
            name: "business-two".to_string(),
            operational_expenses: business_two_op_exp,
            headquarter_building_uuid: Uuid::now_v7(),
            image_number: 2,
        };
        let mut businesses_map = HashMap::new();
        businesses_map.insert(business_one.uuid, business_one);
        businesses_map.insert(business_two.uuid, business_two);

        let total_op_exp = business_one_op_exp + business_two_op_exp;
        let mut total_operation_expenses_by_market_uuid = HashMap::new();
        total_operation_expenses_by_market_uuid.insert(market_uuid, total_op_exp);

        let mut business_uuids_by_market_uuid = HashMap::new();
        business_uuids_by_market_uuid
            .insert(market_uuid, vec![business_one_uuid, business_two_uuid]);

        let mut state = GameState {
            last_processed_tick: 0,
            units_map: HashMap::new(),
            corporations_map,
            markets_map,
            businesses_map,
            building_ownerships_map: HashMap::new(),
            business_listings_map: HashMap::new(),
            business_offers_map: HashMap::new(),
            total_operation_expenses_by_market_uuid,
            business_uuids_by_market_uuid,
            corporation_uuid_by_user_uuid: HashMap::new(),
            corporation_names: HashSet::new(),
            business_uuids_by_corporation_uuid: HashMap::new(),
            business_listing_uuids_by_corporation_uuid: HashMap::new(),
            business_offer_uuids_by_corporation_uuid: HashMap::new(),
            unit_uuids_by_corporation_uuid: HashMap::new(),
        };

        calculate_business_income(&mut state);

        let updated_corporation = state
            .ref_corporation(&corporation_uuid)
            .expect("Failed to retrieve corporation");

        let expected_cash_balance = 140_i64;

        assert_eq!(expected_cash_balance, updated_corporation.cash_balance);
    }
}
