use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

pub fn init() {
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env()) // reads RUST_LOG env var
        .with(fmt::layer().pretty()) // use .json() instead of .pretty() for JSON logs
        .init();
}
