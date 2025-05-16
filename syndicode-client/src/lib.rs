mod application;
mod config;
mod domain;
mod infrastructure;
mod logging;
mod presentation;

use logging::initialize_logging;

pub async fn run() -> anyhow::Result<()> {
    initialize_logging().map_err(|err| anyhow::anyhow!("{}", err))?;

    presentation::run_cli().await
}
