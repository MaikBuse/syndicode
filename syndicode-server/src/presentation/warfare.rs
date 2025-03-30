use crate::{engine::Job, service::warfare::WarfareService};
use std::{collections::VecDeque, sync::Arc};
use syndicode_proto::{
    control::{game_update::ResponseEnum, GameUpdate},
    warfare::{ListUnitsResponse, SpawnUnitResponse, UnitInfo},
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
        response_enum: Some(ResponseEnum::SpawnUnit(SpawnUnitResponse {})),
    })
}

pub async fn list_units(
    warfare_service: Arc<WarfareService>,
    req_user_uuid: Uuid,
) -> Result<GameUpdate, Status> {
    let units = match warfare_service.list_units(req_user_uuid).await {
        Ok(units) => units,
        Err(err) => return Err(Status::new(Code::Internal, err.to_string())),
    };

    let mut unit_infos = Vec::<UnitInfo>::with_capacity(units.len());
    for u in units {
        unit_infos.push(UnitInfo {
            uuid: u.uuid.to_string(),
            user_uuid: u.user_uuid.to_string(),
        });
    }

    Ok(GameUpdate {
        response_enum: Some(ResponseEnum::ListUnits(ListUnitsResponse {
            units: unit_infos,
        })),
    })
}
