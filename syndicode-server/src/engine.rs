use std::{collections::VecDeque, sync::Arc};
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::application::warfare::spawn_unit::SpawnUnitUseCase;

#[derive(Debug)]
pub enum Job {
    UnitSpawn { user_uuid: Uuid },
}

pub struct Engine {
    jobs: Arc<Mutex<VecDeque<Job>>>,
    spawn_unit_uc: Arc<SpawnUnitUseCase>,
}

impl Engine {
    pub fn init(jobs: Arc<Mutex<VecDeque<Job>>>, spawn_unit_uc: Arc<SpawnUnitUseCase>) -> Self {
        Self {
            jobs,
            spawn_unit_uc,
        }
    }

    pub async fn advance_epoch(&mut self) -> anyhow::Result<()> {
        let mut jobs = self.jobs.lock().await;

        'while_job: while let Some(job) = jobs.pop_back() {
            match job {
                Job::UnitSpawn { user_uuid } => {
                    if let Err(err) = self.spawn_unit_uc.execute(user_uuid).await {
                        tracing::error!("{}", err.to_string());

                        continue 'while_job;
                    };
                }
            }
        }

        Ok(())
    }
}
