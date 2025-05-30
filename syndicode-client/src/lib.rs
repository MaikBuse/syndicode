mod application;
mod config;
mod domain;
mod infrastructure;
mod logging;
mod presentation;

use logging::initialize_logging;

/// Main entry point for the Syndicode client application.
///
/// Initializes the application logging system and starts the CLI interface.
///
/// # Errors
///
/// This function will return an error if:
/// - Logging initialization fails
/// - The CLI interface encounters an error during execution
pub async fn run() -> anyhow::Result<()> {
    initialize_logging().map_err(|err| anyhow::anyhow!("{}", err))?;

    presentation::run_cli().await
}
