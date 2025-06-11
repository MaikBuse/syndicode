use std::{cell::RefCell, rc::Rc};

use bon::builder;
use uuid::Uuid;

use crate::{
    application::action::QueuedActionPayload,
    domain::{
        economy::{business_offer::model::BusinessOffer, corporation::model::Corporation},
        outcome::DomainActionOutcome,
        simulation::{game_state::GameState, saga::SagaExecutor, ActionError},
        unit::model::Unit,
    },
    saga_step,
};

#[builder]
pub fn handle_delete_corporation(
    state: &mut GameState,
    action_payload: &QueuedActionPayload,
    next_game_tick: i64,
    corporation_uuid: Uuid,
    req_user_uuid: Uuid,
) -> Result<DomainActionOutcome, ActionError> {
    let mut executor = SagaExecutor::new(state);

    // Step1: Change owner in businesses
    let captured_business_uuids: Rc<RefCell<Vec<Uuid>>> = Rc::new(RefCell::new(Vec::new()));
    saga_step!(
        executor,
        "Change owner in businesses",
        {
            let captured_business_uuids = Rc::clone(&captured_business_uuids);

            move |state: &mut GameState| {
                if let Some(business_uuids) = state
                    .business_uuids_by_corporation_uuid
                    .remove(&corporation_uuid)
                {
                    captured_business_uuids
                        .borrow_mut()
                        .extend(business_uuids.iter());

                    for business_uuid in business_uuids {
                        let Some(business) = state.ref_mut_business(&business_uuid) else {
                            return Err(ActionError::BusinessNotFound { business_uuid });
                        };

                        business.owning_corporation_uuid = None;
                    }
                }
                Ok(())
            }
        },
        move |state: &mut GameState| {
            for business_uuid in captured_business_uuids.take().iter() {
                if let Some(business) = state.ref_mut_business(business_uuid) {
                    business.owning_corporation_uuid = Some(corporation_uuid);

                    let index_business_uuids = state
                        .business_uuids_by_corporation_uuid
                        .entry(corporation_uuid)
                        .or_default();

                    index_business_uuids.push(*business_uuid);
                };
            }
        }
    );

    // Step2: Set selling corporation in business listings to none
    let captured_business_listing_uuids: Rc<RefCell<Vec<Uuid>>> = Rc::new(RefCell::new(Vec::new()));
    saga_step!(
        executor,
        "Set selling corporation in business listings to none",
        {
            let captured_business_listing_uuids = Rc::clone(&captured_business_listing_uuids);

            move |state: &mut GameState| {
                if let Some(business_listing_uuids) = state
                    .business_listing_uuids_by_corporation_uuid
                    .remove(&corporation_uuid)
                {
                    captured_business_listing_uuids
                        .borrow_mut()
                        .extend(business_listing_uuids.iter());

                    for business_listing_uuid in business_listing_uuids {
                        let Some(business_listing) =
                            state.ref_mut_business_listing(&business_listing_uuid)
                        else {
                            return Err(ActionError::BusinessListingNotFound {
                                listing_uuid: business_listing_uuid,
                            });
                        };

                        business_listing.seller_corporation_uuid = None;
                    }
                }
                Ok(())
            }
        },
        move |state: &mut GameState| {
            for business_listing_uuid in captured_business_listing_uuids.take().iter() {
                if let Some(business_listing) =
                    state.ref_mut_business_listing(business_listing_uuid)
                {
                    business_listing.seller_corporation_uuid = Some(corporation_uuid);

                    let index_business_listing_uuids = state
                        .business_listing_uuids_by_corporation_uuid
                        .entry(corporation_uuid)
                        .or_default();

                    index_business_listing_uuids.push(*business_listing_uuid);
                };
            }
        }
    );

    // Step3: Delete business offers
    let captured_business_offer_uuids: Rc<RefCell<Vec<Uuid>>> = Rc::new(RefCell::new(Vec::new()));
    let captured_business_offers: Rc<RefCell<Vec<BusinessOffer>>> =
        Rc::new(RefCell::new(Vec::new()));

    saga_step!(
        executor,
        "Delete business offers",
        {
            let captured_business_offer_uuids = Rc::clone(&captured_business_offer_uuids);
            let captured_business_offers = Rc::clone(&captured_business_offers);

            move |state: &mut GameState| {
                if let Some(business_offer_uuids) = state
                    .business_offer_uuids_by_corporation_uuid
                    .remove(&corporation_uuid)
                {
                    captured_business_offer_uuids
                        .borrow_mut()
                        .extend(business_offer_uuids.iter());

                    for business_offer_uuid in business_offer_uuids {
                        let Some(business_offer) =
                            state.business_offers_map.remove(&business_offer_uuid)
                        else {
                            return Err(ActionError::BusinessOfferNotFound {
                                offer_uuid: business_offer_uuid,
                            });
                        };

                        captured_business_offers.borrow_mut().push(business_offer);
                    }
                }
                Ok(())
            }
        },
        move |state: &mut GameState| {
            for business_offer in captured_business_offers.take().iter() {
                state
                    .business_offers_map
                    .insert(business_offer.uuid, *business_offer);
            }

            let captured_business_offer_uuids = captured_business_offer_uuids.take();

            state
                .business_offer_uuids_by_corporation_uuid
                .insert(corporation_uuid, captured_business_offer_uuids);
        }
    );

    // Step4: Delete units
    let captured_unit_uuids: Rc<RefCell<Vec<Uuid>>> = Rc::new(RefCell::new(Vec::new()));
    let captured_units: Rc<RefCell<Vec<Unit>>> = Rc::new(RefCell::new(Vec::new()));

    saga_step!(
        executor,
        "Delete units",
        {
            let captured_unit_uuids = Rc::clone(&captured_unit_uuids);
            let captured_units = Rc::clone(&captured_units);

            move |state: &mut GameState| {
                if let Some(unit_uuids) = state
                    .unit_uuids_by_corporation_uuid
                    .remove(&corporation_uuid)
                {
                    captured_unit_uuids.borrow_mut().extend(unit_uuids.iter());

                    for unit_uuid in unit_uuids {
                        let Some(unit) = state.units_map.remove(&unit_uuid) else {
                            return Err(ActionError::UnitNotFound { unit_uuid });
                        };

                        captured_units.borrow_mut().push(unit);
                    }
                }
                Ok(())
            }
        },
        move |state: &mut GameState| {
            for unit in captured_units.take().iter() {
                state.units_map.insert(unit.uuid, *unit);
            }

            let captured_unit_uuids = captured_unit_uuids.take();

            state
                .unit_uuids_by_corporation_uuid
                .insert(corporation_uuid, captured_unit_uuids);
        }
    );

    // Step5: Delete corporation
    let captured_corporation: Rc<RefCell<Option<Corporation>>> = Rc::new(RefCell::new(None));

    saga_step!(
        executor,
        "Delete corporation",
        {
            let captured_corporation = Rc::clone(&captured_corporation);

            move |state: &mut GameState| {
                match state.corporations_map.remove(&corporation_uuid) {
                    Some(corporation) => {
                        state
                            .corporation_uuid_by_user_uuid
                            .remove(&corporation.user_uuid);
                        state.corporation_names.remove(corporation.name.as_str());

                        *captured_corporation.borrow_mut() = Some(corporation);
                    }
                    None => {
                        return Err(ActionError::CorporationNotFound { corporation_uuid });
                    }
                }

                Ok(())
            }
        },
        {
            let captured_corporation = Rc::clone(&captured_corporation);

            move |state: &mut GameState| {
                if let Some(corporation) = captured_corporation.take() {
                    state
                        .corporation_uuid_by_user_uuid
                        .insert(corporation.user_uuid, corporation.uuid);
                    state.corporation_names.insert(corporation.name.to_string());

                    state.corporations_map.insert(corporation_uuid, corporation);
                }
            }
        }
    );

    // Execute the saga
    executor.execute()?;

    let Some(corporation) = captured_corporation.take() else {
        return Err(ActionError::CorporationNotCaptured { corporation_uuid });
    };

    Ok(DomainActionOutcome::CorporationDeleted {
        corporation_uuid,
        user_uuid: corporation.user_uuid,
        request_uuid: action_payload.request_uuid,
        tick_effective: next_game_tick,
        req_user_uuid,
    })
}
