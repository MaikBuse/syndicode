use anyhow::Context;
use lazy_static::lazy_static;
use std::path::PathBuf;
use tracing_error::ErrorLayer;
use tracing_subscriber::{self, layer::SubscriberExt, util::SubscriberInitExt, Layer};

lazy_static! {
    pub static ref PROJECT_NAME: String = env!("CARGO_CRATE_NAME").to_uppercase().to_string();
    pub static ref DATA_FOLDER: Option<PathBuf> =
        std::env::var(format!("{}_DATA", PROJECT_NAME.clone()))
            .ok()
            .map(PathBuf::from);
    pub static ref LOG_ENV: String = format!("{}_LOGLEVEL", PROJECT_NAME.clone());
    pub static ref LOG_FILE: String = format!("{}.log", env!("CARGO_PKG_NAME"));
}

pub fn initialize_logging() -> anyhow::Result<()> {
    let exe_path = std::env::current_exe()
        .context("Failed to determine the path of the running executable")?;

    let Some(exe_dir) = exe_path.parent() else {
        return Err(anyhow::anyhow!(
            "Failed to get the parent directory of the executable"
        ));
    };

    let log_path = exe_dir.join(LOG_FILE.clone());

    let log_file = std::fs::File::create(&log_path)
        .with_context(|| format!("Failed to create log file at: {log_path:?}"))?;

    let file_subscriber = tracing_subscriber::fmt::layer()
        .with_file(true) // Log the file name where the event occurred
        .with_line_number(true) // Log the line number
        .with_writer(log_file) // Write to our dynamically located log file
        .with_target(false) // Don't log the event's target (module path)
        .with_ansi(false) // No ANSI color codes in the file
        .with_filter(tracing_subscriber::filter::EnvFilter::from_default_env());

    tracing_subscriber::registry()
        .with(file_subscriber)
        .with(ErrorLayer::default())
        .init();

    Ok(())
}
