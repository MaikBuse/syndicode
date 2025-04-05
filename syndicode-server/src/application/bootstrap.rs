use super::{
    admin::bootstrap_admin::BootstrapAdminUseCase,
    error::{ApplicationError, ApplicationResult},
    ports::{crypto::PasswordHandler, migration::MigrationRunner, uow::UnitOfWork},
};
use crate::utils::read_env_var;
use std::sync::Arc;

const ADMIN_CORPORATION_NAME: &str = "Shinkai Heavyworks";
const ADMIN_USERNAME: &str = "admin";

pub struct Bootstrap<U: UnitOfWork, P: PasswordHandler> {
    pub migrator: Arc<dyn MigrationRunner>,
    bootstrap_admin_uc: Arc<BootstrapAdminUseCase<U, P>>,
}

impl<U, P> Bootstrap<U, P>
where
    U: UnitOfWork,
    P: PasswordHandler,
{
    pub fn new(
        migrator: Arc<dyn MigrationRunner>,
        bootstrap_admin_uc: Arc<BootstrapAdminUseCase<U, P>>,
    ) -> Self {
        Self {
            migrator,
            bootstrap_admin_uc,
        }
    }

    pub async fn run(&self) -> ApplicationResult<()> {
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
