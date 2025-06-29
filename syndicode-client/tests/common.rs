use once_cell::sync::OnceCell;
use std::time::Duration;
use syndicode_client::{
    config::{load_config, ClientConfig},
    domain::{
        auth::repository::{AuthenticationRepository, LoginUserReq},
        game::GameRepository,
    },
    infrastructure::grpc::GrpcHandler,
};
use syndicode_proto::syndicode_interface_v1::{game_update::Update, GameUpdate};
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::task::JoinHandle;
use tokio::time::timeout;
use tokio_stream::StreamExt;
use tonic::{Status, Streaming};

static TRACING: OnceCell<()> = OnceCell::new();

// A static guard to ensure crypto initialization happens only once.
static CRYPTO_PROVIDER_INITIALIZED: OnceCell<()> = OnceCell::new();

fn init_tracing() {
    TRACING.get_or_init(|| {
        tracing::info!("Tracing provider initialized.");
        tracing_subscriber::fmt::init();
    });
}

// Handle the one-time crypto initialization.
fn init_crypto_provider() {
    CRYPTO_PROVIDER_INITIALIZED.get_or_init(|| {
        // This closure will only be executed on the very first call to init_crypto_provider()
        // across all tests and all threads in this process.
        rustls::crypto::ring::default_provider()
            .install_default()
            .expect("Failed to install rustls crypto provider on first attempt");
        tracing::info!("rustls crypto provider installed successfully.");
    });
}

pub struct TestSuite {
    pub config: ClientConfig,
    pub grpc_handler: GrpcHandler,
}

pub async fn setup_test_suite() -> anyhow::Result<TestSuite> {
    init_tracing();
    init_crypto_provider();

    let config = load_config()?;

    // Now, this can be called in every test. The problematic code inside it
    // is guarded by the OnceCell and will not be executed a second time.
    let grpc_handler = GrpcHandler::new(config.grpc.server_address.clone(), true).await?;

    Ok(TestSuite {
        config,
        grpc_handler,
    })
}

/// Represents a logged-in user with an active game stream.
/// When this struct is dropped, the receiver and join handle are also dropped,
/// effectively closing the stream-watching task.
pub struct ActiveUser {
    pub jwt: String,
    pub user_uuid: String,
    pub stream_rx: Receiver<Result<GameUpdate, Status>>,
    _stream_handle: JoinHandle<()>, // Is not read, but needs to be kept alive
}

/// Logs in a user and establishes a game stream connection.
pub async fn login_and_setup_stream(
    grpc_handler: &mut GrpcHandler,
    user_name: &str,
    user_password: &str,
) -> Result<ActiveUser, Box<dyn std::error::Error>> {
    // 1. Login user
    let login_response = grpc_handler
        .login_user(LoginUserReq {
            user_name: user_name.to_string(),
            user_password: user_password.to_string(),
        })
        .await?;

    // 2. Get current User
    let get_user_response = grpc_handler
        .get_current_user(login_response.jwt.clone())
        .await?;

    // 3. Setup the play stream
    let stream = grpc_handler.play_stream(login_response.jwt.clone()).await?;

    // 4. Spawn a task to listen to the stream
    let (tx, rx) = mpsc::channel(10);
    let handle = spawn_stream_thread(stream, tx);

    Ok(ActiveUser {
        jwt: login_response.jwt,
        user_uuid: get_user_response.user_uuid,
        stream_rx: rx,
        _stream_handle: handle,
    })
}

/// Waits for a specific `GameUpdate::Update` from the stream, timing out after a duration.
/// It uses a closure (`predicate`) to identify and extract the desired message.
pub async fn wait_for_stream_update<F, T>(
    stream_rx: &mut Receiver<Result<GameUpdate, Status>>,
    description: &str,
    mut predicate: F,
) -> T
where
    F: FnMut(GameUpdate) -> Option<T>,
{
    let wait_duration = Duration::from_secs(5); // Add a timeout to prevent hanging tests
    let operation = async {
        loop {
            match stream_rx.recv().await {
                Some(Ok(game_update)) => {
                    // Ignore ticks, they are just noise for most tests
                    if let Some(Update::TickNotification(_)) = &game_update.update {
                        continue;
                    }
                    // Let the predicate decide if this is the message we want
                    if let Some(result) = predicate(game_update.clone()) {
                        return result;
                    }
                    // If not the one we want, but not a tick, log it for debugging
                    tracing::debug!(
                        "Ignored unexpected game update while waiting: {:?}",
                        game_update
                    );
                }
                Some(Err(status)) => panic!(
                    "Stream returned an error while waiting for {}: {:?}",
                    description, status
                ),
                None => panic!(
                    "Stream closed unexpectedly while waiting for {}",
                    description
                ),
            }
        }
    };

    match timeout(wait_duration, operation).await {
        Ok(result) => result,
        Err(_) => panic!(
            "Timed out after {:?} waiting for {}",
            wait_duration, description
        ),
    }
}

fn spawn_stream_thread(
    mut stream: Streaming<GameUpdate>,
    tx: Sender<Result<GameUpdate, Status>>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        while let Some(result) = stream.next().await {
            if tx.send(result).await.is_err() {
                // Receiver was dropped, so we can stop.
                break;
            }
        }
    })
}
