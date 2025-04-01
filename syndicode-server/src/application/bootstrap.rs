use super::{
    admin::create_user::CreateUserUseCase,
    error::{ApplicationError, ApplicationResult},
    migration::MigrationRunner,
    uow::UnitOfWork,
};
use crate::domain::user::role::UserRole;
use std::sync::Arc;

const ADMIN_CORPORATION_NAME: &str = "Shinkai Heavyworks";
const ADMIN_USERNAME: &str = "admin";

pub struct Bootstrap<U: UnitOfWork> {
    pub migrator: Arc<dyn MigrationRunner>,
    pub create_user_uc: Arc<CreateUserUseCase<U>>,
}

impl<U: UnitOfWork> Bootstrap<U> {
    pub async fn run(&self) -> ApplicationResult<()> {
        let admin_password = std::env::var("ADMIN_PASSWORD")
            .expect("Environment variable 'ADMIN_PASSWORD' must be set");

        self.migrator.run_migration().await?;

        if let Err(err) = self
            .create_user_uc
            .execute(
                None,
                ADMIN_USERNAME.to_string(),
                admin_password.to_string(),
                UserRole::Admin,
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
