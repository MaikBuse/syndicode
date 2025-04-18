use super::{
    admin::bootstrap_admin::BootstrapAdminUseCase,
    error::{ApplicationError, ApplicationResult},
    ports::{crypto::PasswordHandler, migration::MigrationRunner, uow::UnitOfWork},
};
use crate::utils::read_env_var;
use std::sync::Arc;

const ADMIN_CORPORATION_NAME: &str = "Shinkai Heavyworks";
const ADMIN_USERNAME: &str = "admin";

pub struct Bootstrap<UOW, P, M>
where
    UOW: UnitOfWork,
    P: PasswordHandler,
    M: MigrationRunner,
{
    pub migrator: Arc<M>,
    bootstrap_admin_uc: Arc<BootstrapAdminUseCase<UOW, P>>,
}

impl<UOW, P, M> Bootstrap<UOW, P, M>
where
    UOW: UnitOfWork,
    P: PasswordHandler,
    M: MigrationRunner,
{
    pub fn new(migrator: Arc<M>, bootstrap_admin_uc: Arc<BootstrapAdminUseCase<UOW, P>>) -> Self {
        Self {
            migrator,
            bootstrap_admin_uc,
        }
    }

    pub async fn run(&self) -> ApplicationResult<()> {
        tracing::info!("Bootstrapping server...");

        let admin_password = read_env_var("ADMIN_PASSWORD")?;

        self.migrator.run_migration().await?;

        if let Err(err) = self
            .bootstrap_admin_uc
            .execute(
                ADMIN_USERNAME.to_string(),
                admin_password.to_string(),
                ADMIN_CORPORATION_NAME.to_string(),
            )
            .await
        {
            match err {
                ApplicationError::UniqueConstraint => {}
                _ => return Err(err),
            };
        };

        Ok(())
    }
}
