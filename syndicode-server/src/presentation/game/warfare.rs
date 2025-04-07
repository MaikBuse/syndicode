use crate::{
    application::{
        action::{ActionHandler, QueuedAction},
        ports::queue::ActionQueuer,
        warfare::list_units_by_user::ListUnitsByUserUseCase,
    },
    domain::unit::repository::UnitRepository,
    utils::timestamp_now,
};
use std::sync::Arc;
use syndicode_proto::{
    syndicode_interface_v1::{game_update::Update, ActionInitResponse, GameUpdate},
    syndicode_warfare_v1::{ListUnitsResponse, Unit},
};
use tonic::{Code, Result, Status};
use uuid::Uuid;

pub async fn spawn_unit<A>(
    action_handler: Arc<ActionHandler<A>>,
    req_user_uuid: Uuid,
) -> Result<GameUpdate, Status>
where
    A: ActionQueuer,
{
    let payload = QueuedAction::SpawnUnit { req_user_uuid };

    if let Err(err) = action_handler.submit_action(payload).await {
        return Err(Status::internal(err.to_string()));
    }

    let now = timestamp_now().map_err(|err| {
        tracing::error!("Failed to create timestamp now: {}", err);
        Status::internal("Internal server error")
    })?;

    Ok(GameUpdate {
        update: Some(Update::ActionInitResponse(ActionInitResponse {
            confirmation_message: "Successfully initiated action to spawn a unit".to_string(),
            initiated_at: Some(now),
        })),
    })
}

pub async fn list_units<UNT>(
    list_units_by_user_uc: Arc<ListUnitsByUserUseCase<UNT>>,
    req_user_uuid: Uuid,
) -> Result<GameUpdate, Status>
where
    UNT: UnitRepository,
{
    let units = match list_units_by_user_uc.execute(req_user_uuid).await {
        Ok(units) => units,
        Err(err) => return Err(Status::new(Code::Internal, err.to_string())),
    };

    let mut unit_infos = Vec::<Unit>::with_capacity(units.len());
    for u in units {
        unit_infos.push(Unit {
            uuid: u.uuid.to_string(),
            user_uuid: u.user_uuid.to_string(),
        });
    }

    Ok(GameUpdate {
        update: Some(Update::ListUnits(ListUnitsResponse { units: unit_infos })),
    })
}
