use crate::application::warfare::spawn_unit::SpawnUnitUseCase;
use std::sync::Arc;

pub struct Engine {
    spawn_unit_uc: Arc<SpawnUnitUseCase>,
}

impl Engine {
    pub fn init(spawn_unit_uc: Arc<SpawnUnitUseCase>) -> Self {
        Self { spawn_unit_uc }
    }

    pub async fn advance_epoch(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}
