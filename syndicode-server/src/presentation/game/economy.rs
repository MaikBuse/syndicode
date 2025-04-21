use crate::{
    application::economy::get_corporation::GetCorporationUseCase,
    domain::economy::corporation::repository::CorporationRepository,
};
use bon::builder;
use std::sync::Arc;
use syndicode_proto::{
    syndicode_economy_v1::{Corporation, GetCorporationResponse},
    syndicode_interface_v1::{game_update::Update, GameUpdate},
};
use tonic::{Code, Status};
use uuid::Uuid;

#[builder]
pub async fn get_corporation<CRP>(
    get_corporation_uc: Arc<GetCorporationUseCase<CRP>>,
    user_uuid: Uuid,
    request_uuid: Uuid,
) -> Result<GameUpdate, Status>
where
    CRP: CorporationRepository,
{
    match get_corporation_uc.execute(user_uuid).await {
        Ok(outcome) => Ok(GameUpdate {
            request_uuid: request_uuid.to_string(),
            game_tick: outcome.game_tick,
            update: Some(Update::GetCorporation(GetCorporationResponse {
                corporation: Some(Corporation {
                    uuid: outcome.corporation.uuid.to_string(),
                    user_uuid: outcome.corporation.user_uuid.to_string(),
                    name: outcome.corporation.name.to_string(),
                    balance: outcome.corporation.cash_balance,
                }),
            })),
        }),
        Err(err) => Err(Status::new(Code::Internal, err.to_string())),
    }
}
