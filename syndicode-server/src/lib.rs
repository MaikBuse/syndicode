mod application;
mod config;
mod domain;
mod infrastructure;
mod presentation;
mod startup;
mod utils;

use anyhow::Result;

pub async fn run() -> Result<()> {
    startup::start_server().await
}
