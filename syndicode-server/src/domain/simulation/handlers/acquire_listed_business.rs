use crate::{
    application::action::QueuedActionPayload,
    domain::{
        economy::business_listing::model::BusinessListing, // Import needed for compensation
        outcome::DomainActionOutcome,
        simulation::{game_state::GameState, saga::SagaExecutor, ActionError},
    },
    saga_step,
};
use uuid::Uuid;

pub fn handle_acquire_listed_business(
    state: &mut GameState,
    action: &QueuedActionPayload,
    business_listing_uuid: Uuid,
    next_game_tick: i64,
) -> Result<DomainActionOutcome, ActionError> {
    // --- 1. Pre-Saga Checks and Data Gathering (Immutable) ---
    let req_corporation_uuid = *state
        .get_corporation_uuid_by_user(&action.user_uuid)
        .ok_or(ActionError::RequestingCorporationNotFoundByUser {
            // Use specific error
            user_uuid: action.user_uuid,
        })?;

    // Clone original listing *before* the saga starts for potential compensation
    let original_listing = *state.ref_business_listing(&business_listing_uuid).ok_or(
        ActionError::BusinessListingNotFound {
            // Use specific error
            listing_uuid: business_listing_uuid,
        },
    )?; // Clone here

    let listing_price = original_listing.asking_price;
    let business_uuid = original_listing.business_uuid;
    let seller_corporation_uuid = original_listing.seller_corporation_uuid; // Option<Uuid>

    // Check funds
    let req_corporation = state.ref_corporation(&req_corporation_uuid).ok_or(
        ActionError::CorporationNotFoundDuringChecks {
            // Use specific error
            corporation_uuid: req_corporation_uuid,
        },
    )?;

    if req_corporation.cash_balance < listing_price {
        return Err(ActionError::InsufficientFunds {
            // Use specific error
            corporation_uuid: req_corporation_uuid,
            required: listing_price,
            available: req_corporation.cash_balance,
        });
    }

    // Get original owner UUID *before* saga starts
    let original_owner_uuid = state
        .ref_business(&business_uuid)
        .ok_or(ActionError::BusinessNotFoundDuringChecks {
            // Use specific error
            business_uuid,
        })?
        .owning_corporation_uuid; // Option<Uuid>

    // --- 2. Define and Execute the Saga ---
    let mut executor = SagaExecutor::new(state); // Pass mutable state reference

    // --- Step 1: Debit Buyer ---
    saga_step!(
        executor,
        "Debit Buyer", // Add step description
        // Forward Action (Closure takes |state: &mut GameState|) -> Result<(), ActionError>
        |state: &mut GameState| {
            let corp = state.ref_mut_corporation(&req_corporation_uuid).ok_or(
                ActionError::SagaEntityMissing {
                    // Specific saga error
                    entity_type: "Corporation",
                    entity_id: req_corporation_uuid,
                    step_description: "Debit Buyer",
                },
            )?;
            corp.cash_balance -= listing_price;
            Ok(()) // Indicate success
        },
        // Compensation Action (Closure takes |state: &mut GameState|)
        |state: &mut GameState| {
            if let Some(corp) = state.ref_mut_corporation(&req_corporation_uuid) {
                corp.cash_balance += listing_price;
            } else {
                // Log critical error, compensation failed
                tracing::error!("CRITICAL: Saga Rollback (Debit Buyer) - Failed to find requesting corporation {} to credit back.", req_corporation_uuid);
            }
        }
    );

    // --- Step 2: Credit Seller (if exists) ---
    if let Some(seller_uuid) = seller_corporation_uuid {
        // Clone seller_uuid as it needs to be captured by multiple closures
        let seller_uuid_copy = seller_uuid;
        saga_step!(
            executor,
            "Credit Seller", // Add step description
            // Forward -> Result<(), ActionError>
            move |state: &mut GameState| {
                let selling_corp = state.ref_mut_corporation(&seller_uuid_copy).ok_or(
                    ActionError::SagaEntityMissing {
                        // Specific saga error
                        entity_type: "Corporation",
                        entity_id: seller_uuid_copy,
                        step_description: "Credit Seller",
                    },
                )?;
                selling_corp.cash_balance += listing_price;
                Ok(())
            },
            // Compensate
            move |state: &mut GameState| {
                if let Some(selling_corp) = state.ref_mut_corporation(&seller_uuid_copy) {
                    selling_corp.cash_balance -= listing_price;
                } else {
                    // Log critical error, compensation failed
                    tracing::error!("CRITICAL: Saga Rollback (Credit Seller) - Failed to find selling corporation {} to debit back.", seller_uuid_copy);
                }
            }
        );
    }

    // --- Step 3: Change Ownership ---
    // Capture original_owner_uuid before defining the step
    let original_owner_uuid_clone = original_owner_uuid;
    saga_step!(
        executor,
        "Transfer Ownership",
        // Forward -> Result<(), ActionError>
        |state: &mut GameState| {
            let business =
                state
                    .ref_mut_business(&business_uuid)
                    .ok_or(ActionError::SagaEntityMissing {
                        // Specific saga error
                        entity_type: "Business",
                        entity_id: business_uuid,
                        step_description: "Transfer Ownership",
                    })?;
            business.owning_corporation_uuid = Some(req_corporation_uuid);
            Ok(())
        },
        // Compensate (needs original_owner_uuid)
        move |state: &mut GameState| {
            if let Some(business) = state.ref_mut_business(&business_uuid) {
                business.owning_corporation_uuid = original_owner_uuid_clone;
            } else {
                // Log critical error, compensation failed
                tracing::error!("CRITICAL: Saga Rollback (Transfer Ownership) - Failed to find business {} to restore ownership.", business_uuid);
            }
        }
    );

    // --- Step 4: Remove Listing ---
    // original_listing was already cloned earlier
    let captured_original_listing: BusinessListing = original_listing; // Move capture
    saga_step!(
        executor,
        "Remove Listing", // Add step description
        // Forward -> Result<(), ActionError>
        |state: &mut GameState| {
            state
                .remove_business_listing(&business_listing_uuid)
                .ok_or(ActionError::SagaEntityMissing {
                    // Specific saga error (or maybe SagaStepFailed?)
                    entity_type: "BusinessListing", // Assuming remove returns Option<Listing>
                    entity_id: business_listing_uuid,
                    step_description: "Remove Listing",
                })
                .map(|_| ()) // Discard the removed listing if successful
        },
        // Compensate (needs original_listing)
        move |state: &mut GameState| {
            state.add_business_listing(captured_original_listing); // Re-insert original
                                                                   // Note: If add_business_listing could fail, this compensation isn't guaranteed
        }
    );

    // --- Execute the Saga ---
    // The executor now returns Result<(), ActionError> if any forward step fails
    executor.execute()?; // Use ? to propagate ActionError if execution fails

    // --- 3. Post-Saga: Construct Success Outcome ---
    // If execute() returned Ok, the state changes are committed.
    // Fetch final state if needed for the outcome.
    let final_business = state.ref_business(&business_uuid).ok_or_else(|| {
        // This should ideally be unreachable if the saga succeeded
        ActionError::InternalError(format!(
            "CRITICAL: Business {} disappeared after successful saga execution!",
            business_uuid
        ))
    })?;

    Ok(DomainActionOutcome::ListedBusinessAcquired {
        request_uuid: action.request_uuid,
        tick_effective: next_game_tick,
        user_uuid: action.user_uuid,
        business_uuid,
        market_uuid: final_business.market_uuid,
        owning_corporation_uuid: final_business.owning_corporation_uuid.ok_or_else(|| {
            // This should also be unreachable
            ActionError::InternalError(format!(
                "CRITICAL: Owner UUID missing for business {} after successful saga execution!",
                business_uuid
            ))
        })?,
        name: final_business.name.clone(),
        operational_expenses: final_business.operational_expenses,
    })
}

#[cfg(test)]
mod tests {
    use super::*; // Import the handler function
    use crate::application::action::{ActionDetails, QueuedActionPayload};
    use crate::domain::economy::business::model::Business;
    use crate::domain::economy::business_listing::model::BusinessListing;
    use crate::domain::economy::corporation::model::name::CorporationName;
    use crate::domain::economy::corporation::model::Corporation;
    use uuid::Uuid; // Use matches crate for cleaner assertions

    // --- Test Helper Functions --- (remain the same)
    fn setup_test_state() -> (GameState, Uuid, Uuid, Uuid, Uuid, Uuid) {
        // Added business_uuid return
        let buyer_user_uuid = Uuid::now_v7();
        let buyer_corp_uuid = Uuid::now_v7();
        let seller_user_uuid = Uuid::now_v7();
        let seller_corp_uuid = Uuid::now_v7();
        let market_uuid = Uuid::now_v7();
        let business_uuid = Uuid::now_v7();
        let listing_uuid = Uuid::now_v7();

        let buyer_corp = Corporation {
            uuid: buyer_corp_uuid,
            user_uuid: buyer_user_uuid,
            name: CorporationName::new("Buyer Corp".to_string()).unwrap(),
            cash_balance: 10000,
        };
        let seller_corp = Corporation {
            uuid: seller_corp_uuid,
            user_uuid: seller_user_uuid,
            name: CorporationName::new("Seller Corp".to_string()).unwrap(),
            cash_balance: 5000,
        };
        let business = Business {
            uuid: business_uuid,
            market_uuid,
            owning_corporation_uuid: Some(seller_corp_uuid), // Initially owned by seller
            name: "Test Biz".to_string(),
            operational_expenses: 100,
        };
        let listing = BusinessListing {
            uuid: listing_uuid,
            business_uuid,
            seller_corporation_uuid: Some(seller_corp_uuid),
            asking_price: 7500,
        };

        let state = GameState::build(
            vec![],
            vec![buyer_corp.clone(), seller_corp.clone()],
            vec![],
            vec![business.clone()],
            vec![listing],
        );

        (
            state,
            buyer_user_uuid,
            buyer_corp_uuid,
            seller_corp_uuid,
            listing_uuid,
            business_uuid, // return business_uuid
        )
    }

    fn create_test_action(user_uuid: Uuid, business_uuid: Uuid) -> QueuedActionPayload {
        // Changed param name
        QueuedActionPayload {
            request_uuid: Uuid::now_v7(),
            user_uuid,
            details: ActionDetails::AcquireListedBusiness { business_uuid },
        }
    }

    // --- Test Cases ---

    #[test]
    fn test_acquire_success() {
        let (mut state, buyer_user_uuid, buyer_corp_uuid, seller_corp_uuid, listing_uuid, business_uuid) = // capture business_uuid
            setup_test_state();
        let buyer_initial_cash = state
            .ref_corporation(&buyer_corp_uuid)
            .unwrap()
            .cash_balance;
        let seller_initial_cash = state
            .ref_corporation(&seller_corp_uuid)
            .unwrap()
            .cash_balance;
        let listing_price = state
            .ref_business_listing(&listing_uuid)
            .unwrap()
            .asking_price;
        // business_uuid already captured

        let action = create_test_action(buyer_user_uuid, business_uuid);
        let tick = 10;

        let result = handle_acquire_listed_business(&mut state, &action, listing_uuid, tick);

        // Assert success
        assert!(result.is_ok());
        assert!(matches!(
            result,
            Ok(DomainActionOutcome::ListedBusinessAcquired { owning_corporation_uuid, .. }) if owning_corporation_uuid == buyer_corp_uuid,
        ));

        // Assert state changes
        assert_eq!(
            state
                .ref_corporation(&buyer_corp_uuid)
                .unwrap()
                .cash_balance,
            buyer_initial_cash - listing_price
        );
        assert_eq!(
            state
                .ref_corporation(&seller_corp_uuid)
                .unwrap()
                .cash_balance,
            seller_initial_cash + listing_price
        );
        assert_eq!(
            state
                .ref_business(&business_uuid)
                .unwrap()
                .owning_corporation_uuid,
            Some(buyer_corp_uuid)
        );
        assert!(
            state.ref_business_listing(&listing_uuid).is_none(),
            "Listing should be removed"
        );
    }

    #[test]
    fn test_acquire_fail_insufficient_funds() {
        let (mut state, buyer_user_uuid, buyer_corp_uuid, seller_corp_uuid, listing_uuid, business_uuid) = // capture business_uuid
            setup_test_state();
        let buyer_initial_cash = 100; // Not enough cash
        state
            .ref_mut_corporation(&buyer_corp_uuid)
            .unwrap()
            .cash_balance = buyer_initial_cash;
        let seller_initial_cash = state
            .ref_corporation(&seller_corp_uuid)
            .unwrap()
            .cash_balance;
        // business_uuid captured
        let initial_owner = state
            .ref_business(&business_uuid)
            .unwrap()
            .owning_corporation_uuid;

        let action = create_test_action(buyer_user_uuid, business_uuid);
        let tick = 10;

        let result = handle_acquire_listed_business(&mut state, &action, listing_uuid, tick);

        // Assert failure with specific error
        assert!(
            matches!(result, Err(ActionError::InsufficientFunds { corporation_uuid, .. }) if corporation_uuid == buyer_corp_uuid )
        );

        // Assert state unchanged (no rollback needed as checks fail first)
        assert_eq!(
            state
                .ref_corporation(&buyer_corp_uuid)
                .unwrap()
                .cash_balance,
            buyer_initial_cash
        );
        assert_eq!(
            state
                .ref_corporation(&seller_corp_uuid)
                .unwrap()
                .cash_balance,
            seller_initial_cash
        );
        assert_eq!(
            state
                .ref_business(&business_uuid)
                .unwrap()
                .owning_corporation_uuid,
            initial_owner
        );
        assert!(
            state.ref_business_listing(&listing_uuid).is_some(),
            "Listing should still exist"
        );
    }

    #[test]
    fn test_acquire_fail_listing_not_found() {
        let (mut state, buyer_user_uuid, _, _, _, business_uuid) = setup_test_state();
        let non_existent_listing_uuid = Uuid::now_v7();
        let action = create_test_action(buyer_user_uuid, business_uuid);
        let tick = 10;

        let result =
            handle_acquire_listed_business(&mut state, &action, non_existent_listing_uuid, tick);

        assert!(
            matches!(result, Err(ActionError::BusinessListingNotFound { listing_uuid }) if listing_uuid == non_existent_listing_uuid)
        );
    }

    #[test]
    fn test_acquire_fail_buyer_corp_not_found_by_user() {
        // More specific test name
        let (mut state, _, _, _, listing_uuid, business_uuid) = setup_test_state();
        let non_existent_user_uuid = Uuid::now_v7(); // User not linked to any corp in state
        let action = create_test_action(non_existent_user_uuid, business_uuid);
        let tick = 10;

        let result = handle_acquire_listed_business(&mut state, &action, listing_uuid, tick);

        assert!(matches!(
            result,
            Err(ActionError::RequestingCorporationNotFoundByUser { user_uuid }) if user_uuid == non_existent_user_uuid,
        ));
    }

    #[test]
    fn test_acquire_rollback_if_seller_disappears_mid_saga() {
        let (mut state, buyer_user_uuid, buyer_corp_uuid, seller_corp_uuid, listing_uuid, business_uuid) = // capture business_uuid
            setup_test_state();
        let buyer_initial_cash = state
            .ref_corporation(&buyer_corp_uuid)
            .unwrap()
            .cash_balance;
        // Seller initial cash doesn't matter as it won't be found
        // business_uuid captured
        let initial_owner = state
            .ref_business(&business_uuid)
            .unwrap()
            .owning_corporation_uuid;
        let initial_listing = state.ref_business_listing(&listing_uuid).cloned(); // Clone for check later

        // *** Simulate seller disappearing BEFORE the handler tries to credit them ***
        // This causes the Saga's step 2 (Credit Seller) forward action to fail
        state.corporations_map.remove(&seller_corp_uuid); // Remove *after* setup, before handler call

        let action = create_test_action(buyer_user_uuid, business_uuid);
        let tick = 10;

        let result = handle_acquire_listed_business(&mut state, &action, listing_uuid, tick);

        // Assert failure occurred during the saga's "Credit Seller" step
        assert!(matches!(
        result,
        Err(ActionError::SagaEntityMissing { entity_type: "Corporation", entity_id, step_description: "Credit Seller" }) if entity_id == seller_corp_uuid
        ));

        // *** Assert Rollback Occurred ***
        // Buyer's cash should be restored
        assert_eq!(
            state
                .ref_corporation(&buyer_corp_uuid)
                .expect("Buyer corp disappeared during test?") // Should exist
                .cash_balance,
            buyer_initial_cash,
            "Buyer cash not rolled back"
        );
        // Seller doesn't exist to check
        // Business ownership should be restored
        assert_eq!(
            state
                .ref_business(&business_uuid)
                .expect("Business disappeared during test?") // Should exist
                .owning_corporation_uuid,
            initial_owner,
            "Business owner not rolled back"
        );
        // Listing should have been re-added by compensation
        assert_eq!(
            state.ref_business_listing(&listing_uuid).unwrap(),
            initial_listing.as_ref().unwrap(),
            "Listing not rolled back (should exist and match original)"
        );
    }

    #[test]
    fn test_acquire_fail_if_business_disappears_before_checks() {
        // Adjusted test name and expectation
        let (mut state, buyer_user_uuid, buyer_corp_uuid, seller_corp_uuid, listing_uuid, business_uuid) = // capture business_uuid
            setup_test_state();
        let buyer_initial_cash = state
            .ref_corporation(&buyer_corp_uuid)
            .unwrap()
            .cash_balance;
        let seller_initial_cash = state
            .ref_corporation(&seller_corp_uuid)
            .unwrap()
            .cash_balance;
        let initial_listing = state.ref_business_listing(&listing_uuid).cloned();

        // *** Simulate business disappearing BEFORE the handler is called ***
        // This causes the pre-saga check for the business to fail.
        state.businesses_map.remove(&business_uuid);

        let action = create_test_action(buyer_user_uuid, business_uuid);
        let tick = 10;

        let result = handle_acquire_listed_business(&mut state, &action, listing_uuid, tick);

        // Assert failure occurred during the pre-saga checks phase
        // The saga does not start, so no rollback is needed or tested here.
        assert!(matches!(
        result,
        Err(ActionError::BusinessNotFoundDuringChecks { business_uuid: err_biz_uuid }) if err_biz_uuid == business_uuid
        ));

        // *** Assert State Unchanged (because checks failed before saga) ***
        assert_eq!(
            state
                .ref_corporation(&buyer_corp_uuid)
                .unwrap()
                .cash_balance,
            buyer_initial_cash,
            "Buyer cash should be unchanged"
        );
        assert_eq!(
            state
                .ref_corporation(&seller_corp_uuid)
                .unwrap()
                .cash_balance,
            seller_initial_cash,
            "Seller cash should be unchanged"
        );
        // Business doesn't exist to check owner
        assert_eq!(
            state.ref_business_listing(&listing_uuid),
            initial_listing.as_ref(),
            "Listing should be unchanged"
        );
    }

    #[test]
    fn test_acquire_success_no_seller_corp() {
        let (mut state, buyer_user_uuid, buyer_corp_uuid, _, listing_uuid, business_uuid) = // capture business_uuid
            setup_test_state(); // Ignore seller corp UUID from setup
        let buyer_initial_cash = state
            .ref_corporation(&buyer_corp_uuid)
            .unwrap()
            .cash_balance;
        let listing_price = state
            .ref_business_listing(&listing_uuid)
            .unwrap()
            .asking_price;
        // business_uuid captured

        // Modify listing and business to have no seller/owner initially
        state
            .ref_mut_business_listing(&listing_uuid)
            .unwrap()
            .seller_corporation_uuid = None;
        state
            .ref_mut_business(&business_uuid)
            .unwrap()
            .owning_corporation_uuid = None; // Or owned by system? Test assumes None ok

        let action = create_test_action(buyer_user_uuid, business_uuid);
        let tick = 10;

        let result = handle_acquire_listed_business(&mut state, &action, listing_uuid, tick);

        // Assert success
        assert!(result.is_ok());
        assert!(
            matches!(result, Ok(DomainActionOutcome::ListedBusinessAcquired { owning_corporation_uuid, .. }) if owning_corporation_uuid == buyer_corp_uuid)
        );

        // Assert state changes (only buyer and business)
        assert_eq!(
            state
                .ref_corporation(&buyer_corp_uuid)
                .unwrap()
                .cash_balance,
            buyer_initial_cash - listing_price
        );
        assert_eq!(
            state
                .ref_business(&business_uuid)
                .unwrap()
                .owning_corporation_uuid,
            Some(buyer_corp_uuid)
        );
        assert!(
            state.ref_business_listing(&listing_uuid).is_none(),
            "Listing should be removed"
        );
        // No change to seller cash as there was no seller
    }
} // End tests module
