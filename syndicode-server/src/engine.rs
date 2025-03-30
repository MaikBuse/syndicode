use crate::service::{control::ControlService, warfare::WarfareService};
use std::{collections::VecDeque, sync::Arc};
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug)]
pub enum Job {
    UnitSpawn { user_uuid: Uuid },
}

pub struct Engine {
    jobs: Arc<Mutex<VecDeque<Job>>>,
    control_service: Arc<ControlService>,
    warfare_service: Arc<WarfareService>,
}

impl Engine {
    pub fn init(
        jobs: Arc<Mutex<VecDeque<Job>>>,
        control_service: Arc<ControlService>,
        warfare_service: Arc<WarfareService>,
    ) -> Self {
        Self {
            jobs,
            control_service,
            warfare_service,
        }
    }

    pub async fn advance_epoch(&mut self) -> anyhow::Result<()> {
        let mut jobs = self.jobs.lock().await;

        'while_job: while let Some(job) = jobs.pop_back() {
            match job {
                Job::UnitSpawn { user_uuid } => {
                    if let Err(err) = self.warfare_service.create_unit(user_uuid).await {
                        tracing::error!("{}", err.to_string());

                        continue 'while_job;
                    };
                }
            }
        }

        Ok(())
    }
}
