use syndicode_client::run;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run().await
}
