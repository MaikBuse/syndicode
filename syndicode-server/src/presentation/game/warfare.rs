use crate::{
    application::{
        get_game_tick::GetGameTickUseCase,
        ports::{game_tick::GameTickRepository, queuer::ActionQueueable},
        warfare::{list_units_by_user::ListUnitsByUserUseCase, spawn_unit::SpawnUnitUseCase},
    },
    domain::unit::repository::UnitRepository,
    presentation::error::PresentationError,
};
use bon::builder;
use std::sync::Arc;
use syndicode_proto::{
    syndicode_interface_v1::{game_update::Update, ActionInitResponse, GameUpdate},
    syndicode_warfare_v1::{ListUnitsResponse, Unit},
};
use tonic::{Result, Status};
use uuid::Uuid;

#[builder]
pub async fn spawn_unit<Q, GTR>(
    get_game_tick_uc: Arc<GetGameTickUseCase<GTR>>,
    spawn_unit_uc: Arc<SpawnUnitUseCase<Q, GTR>>,
    request_uuid: Uuid,
    req_user_uuid: Uuid,
) -> Result<GameUpdate, Status>
where
    Q: ActionQueueable,
    GTR: GameTickRepository,
{
    match spawn_unit_uc
        .execute()
        .req_user_uuid(req_user_uuid)
        .request_uuid(request_uuid)
        .call()
        .await
    {
        Ok(game_tick) => Ok(GameUpdate {
            game_tick,
            update: Some(Update::ActionInitResponse(ActionInitResponse {
                request_uuid: request_uuid.to_string(),
            })),
        }),
        Err(err) => {
            let game_tick = get_game_tick_uc.execute().await.unwrap_or_default();

            Ok(PresentationError::from(err).into_game_update(game_tick, request_uuid.to_string()))
        }
    }
}

#[builder]
pub async fn list_units<GTR, UNT>(
    get_game_tick_uc: Arc<GetGameTickUseCase<GTR>>,
    list_units_by_user_uc: Arc<ListUnitsByUserUseCase<UNT>>,
    req_user_uuid: Uuid,
    request_uuid: Uuid,
) -> Result<GameUpdate, Status>
where
    GTR: GameTickRepository,
    UNT: UnitRepository,
{
    match list_units_by_user_uc
        .execute()
        .user_uuid(req_user_uuid)
        .call()
        .await
    {
        Ok(outcome) => {
            let mut unit_infos = Vec::<Unit>::with_capacity(outcome.units.len());
            for u in outcome.units {
                unit_infos.push(Unit {
                    uuid: u.uuid.to_string(),
                    user_uuid: u.user_uuid.to_string(),
                });
            }

            Ok(GameUpdate {
                game_tick: outcome.game_tick,
                update: Some(Update::ListUnits(ListUnitsResponse {
                    units: unit_infos,
                    request_uuid: request_uuid.to_string(),
                })),
            })
        }
        Err(err) => {
            let game_tick = get_game_tick_uc.execute().await.unwrap_or_default();

            Ok(PresentationError::from(err).into_game_update(game_tick, request_uuid.to_string()))
        }
    }
}
