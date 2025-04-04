use syndicode_server::run;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    run().await
}
