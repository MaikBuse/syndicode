use crate::{
    domain::model::control::SessionState,
    service::{control::ControlService, economy::EconomyService, warfare::WarfareService},
};
use dashmap::DashMap;
use std::{collections::VecDeque, sync::Arc};
use uuid::Uuid;

#[derive(Debug)]
pub enum Job {
    UnitSpawn { user_uuid: Uuid },
}

pub struct Engine {
    jobs: Arc<DashMap<Uuid, VecDeque<Job>>>,
    control_service: Arc<ControlService>,
    economy_service: Arc<EconomyService>,
    warfare_service: Arc<WarfareService>,
}

impl Engine {
    pub fn init(
        jobs: Arc<DashMap<Uuid, VecDeque<Job>>>,
        control_service: Arc<ControlService>,
        economy_service: Arc<EconomyService>,
        warfare_service: Arc<WarfareService>,
    ) -> Self {
        Self {
            jobs,
            control_service,
            warfare_service,
            economy_service,
        }
    }

    pub async fn advance_epoch(&mut self) -> anyhow::Result<()> {
        let sessions = self.control_service.list_sessions().await?;

        'for_session: for session in sessions.into_iter() {
            let state = match SessionState::try_from(session.state) {
                Ok(state) => state,
                Err(err) => {
                    tracing::error!("{}", err.to_string());

                    continue 'for_session;
                }
            };

            if state != SessionState::Running {
                continue 'for_session;
            }

            let mut session_jobs = self.jobs.entry(session.uuid.clone()).or_default();

            'while_job: while let Some(job) = session_jobs.pop_back() {
                match job {
                    Job::UnitSpawn { user_uuid } => {
                        if let Err(err) = self
                            .warfare_service
                            .create_unit(session.uuid.clone(), user_uuid)
                            .await
                        {
                            tracing::error!("{}", err.to_string());

                            continue 'while_job;
                        };
                    }
                }
            }

            if let Err(err) = self
                .control_service
                .advance_session_interval(session.uuid)
                .await
            {
                tracing::error!("{}", err.to_string());
            }
        }

        Ok(())
    }
}
