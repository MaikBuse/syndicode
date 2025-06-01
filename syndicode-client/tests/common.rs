use syndicode_client::{
    config::{load_config, ClientConfig},
    infrastructure::grpc::GrpcHandler,
};

pub struct TestSuite {
    pub config: ClientConfig,
    pub grpc_handler: GrpcHandler,
}

pub async fn setup_test_suite() -> anyhow::Result<TestSuite> {
    let config = load_config()?;

    let grpc_handler = GrpcHandler::new(config.grpc.server_address.clone()).await?;

    Ok(TestSuite {
        config,
        grpc_handler,
    })
}
