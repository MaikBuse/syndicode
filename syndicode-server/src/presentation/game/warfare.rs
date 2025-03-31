use crate::{application::warfare::list_units::ListUnitsUseCase, engine::Job};
use std::{collections::VecDeque, sync::Arc};
use syndicode_proto::{
    syndicode_interface_v1::{game_update::Update, GameUpdate},
    syndicode_warfare_v1::{ListUnitsResponse, SpawnUnitResponse, Unit},
};
use tokio::sync::Mutex;
use tonic::{Code, Result, Status};
use uuid::Uuid;

pub async fn spawn_unit(
    jobs: Arc<Mutex<VecDeque<Job>>>,
    req_user_uuid: Uuid,
) -> Result<GameUpdate, Status> {
    let mut jobs = jobs.lock().await;

    jobs.push_front(Job::UnitSpawn {
        user_uuid: req_user_uuid,
    });

    Ok(GameUpdate {
        update: Some(Update::SpawnUnit(SpawnUnitResponse {})),
    })
}

pub async fn list_units(
    list_units_uc: Arc<ListUnitsUseCase>,
    req_user_uuid: Uuid,
) -> Result<GameUpdate, Status> {
    let units = match list_units_uc.execute(req_user_uuid).await {
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
