mod application;
mod domain;
mod engine;
mod infrastructure;
mod presentation;
mod startup;

use anyhow::Result;
pub async fn run() -> Result<()> {
    startup::start_server().await
}
