use crate::{
    application::{
        ports::queue::ActionQueuer,
        warfare::{list_units_by_user::ListUnitsByUserUseCase, spawn_unit::SpawnUnitUseCase},
    },
    domain::unit::repository::UnitRepository,
    presentation::common::application_error_into_status,
    utils::timestamp_now,
};
use std::sync::Arc;
use syndicode_proto::{
    syndicode_interface_v1::{game_update::Update, ActionInitResponse, GameUpdate},
    syndicode_warfare_v1::{ListUnitsResponse, Unit},
};
use tonic::{Code, Result, Status};
use uuid::Uuid;

pub async fn spawn_unit<Q>(
    spawn_unit_uc: Arc<SpawnUnitUseCase<Q>>,
    req_user_uuid: Uuid,
) -> Result<GameUpdate, Status>
where
    Q: ActionQueuer,
{
    if let Err(err) = spawn_unit_uc.execute(req_user_uuid).await {
        return Err(application_error_into_status(err));
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
