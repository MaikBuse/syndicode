use crate::{
    application::economy::get_corporation::GetCorporationUseCase,
    domain::corporation::repository::CorporationRepository,
};
use std::sync::Arc;
use syndicode_proto::{
    syndicode_economy_v1::{Corporation, GetCorporationRequest, GetCorporationResponse},
    syndicode_interface_v1::{game_update::Update, GameUpdate},
};
use tonic::{Code, Status};
use uuid::Uuid;

pub async fn get_corporation<CRP>(
    _: GetCorporationRequest,
    get_corporation_uc: Arc<GetCorporationUseCase<CRP>>,
    user_uuid: Uuid,
) -> Result<GameUpdate, Status>
where
    CRP: CorporationRepository,
{
    match get_corporation_uc.execute(user_uuid).await {
        Ok(corporation) => Ok(GameUpdate {
            update: Some(Update::GetCorporation(GetCorporationResponse {
                corporation: Some(Corporation {
                    uuid: corporation.uuid.to_string(),
                    user_uuid: corporation.user_uuid.to_string(),
                    name: corporation.name,
                    balance: corporation.balance,
                }),
            })),
        }),
        Err(err) => Err(Status::new(Code::Internal, err.to_string())),
    }
}
