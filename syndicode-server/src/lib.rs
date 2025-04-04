mod application;
mod config;
mod domain;
mod engine;
mod infrastructure;
mod presentation;
mod startup;
mod utils;

use anyhow::Result;

pub async fn run() -> Result<()> {
    startup::start_server().await
}
