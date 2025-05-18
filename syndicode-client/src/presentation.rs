mod app;
mod event;
mod input;
mod stream;
mod theme;
mod ui;
mod widget;

use crate::application::auth::login::LoginUserUseCase;
use crate::application::auth::resend::ResendVerificationUseCase;
use crate::application::auth::verifiy::VerifyUserUseCase;
use crate::application::game::query_business_listings::QueryBusinessListingsUseCase;
use crate::application::game::stream::PlayStreamUseCase;
use crate::config::load_config;
use crate::trace_dbg;
use crate::{application::auth::register::RegisterUseCase, infrastructure::grpc::GrpcHandler};
use app::{App, CurrentScreen};
use event::InputReader;
use ratatui::widgets::ListState;
use std::sync::Arc;
use stream::StreamHandler;
use tokio::sync::{mpsc, Mutex};
use widget::exit::ExitPopupWidget;
use widget::response_detail::ResponseDetailWidget;
use widget::response_list::ResponseListWidget;
use widget::service::service_detail::ServiceDetailWidget;
use widget::service::service_list::{default_services, ServiceListWidget};
use widget::vim::{Mode, Vim};

pub async fn run_cli() -> anyhow::Result<()> {
    let mut terminal = ratatui::init();

    // Event channel for input events from InputReader
    let (app_event_tx, mut app_event_rx) = mpsc::channel(10);

    let input_reader = InputReader::new();
    // Spawn a task to read input events and send them through the channel
    let read_input_handle = tokio::spawn(input_reader.read_input_events(app_event_tx.clone()));

    // Load configuration
    let config = load_config()?;

    // Initialize gRPC handler (shared among use cases)
    let grpc_handler = Arc::new(Mutex::new(
        GrpcHandler::new(config.grpc.auth_service_address).await?,
    ));

    // Initialize Use Cases
    let register_uc = RegisterUseCase::builder()
        .auth_repository(grpc_handler.clone())
        .build();
    let verify_uc = VerifyUserUseCase::builder()
        .auth_repository(grpc_handler.clone())
        .build();
    let resend_uc = ResendVerificationUseCase::builder()
        .auth_repository(grpc_handler.clone())
        .build();
    let login_uc = LoginUserUseCase::builder()
        .auth_repository(grpc_handler.clone())
        .build();
    let play_stream_uc = PlayStreamUseCase::builder()
        .game_repo(grpc_handler.clone())
        .build();
    let query_business_listings_uc = QueryBusinessListingsUseCase::builder()
        .game_repo(grpc_handler.clone())
        .build();

    // Initialize StreamHandler (owned by App or managed alongside)
    let stream_handler = StreamHandler::new();

    // Initialize Widgets
    let response_list_widget = ResponseListWidget::new();
    let response_detail_widget = ResponseDetailWidget;
    let service_detail_widget = ServiceDetailWidget;
    let exit_popup_widget = ExitPopupWidget;
    let categories = default_services();
    let service_list_widget = ServiceListWidget::new(categories);

    // Vim
    let response_detail_vim = Vim::new(Mode::Normal);

    // Build the main App state
    let mut app = App::builder()
        .should_exit(false)
        .stream_handler(stream_handler)
        .response_detail_vim(response_detail_vim)
        .service_list_widget(service_list_widget)
        .service_list_state(ListState::default().with_selected(Some(0)))
        .response_list_widget(response_list_widget)
        .response_list_state(ListState::default().with_selected(None))
        .response_detail_widget(response_detail_widget)
        .service_detail_widget(service_detail_widget)
        .exit_popup_widget(exit_popup_widget)
        .current_screen(CurrentScreen::default())
        .register_uc(register_uc)
        .verifiy_uc(verify_uc)
        .resend_uc(resend_uc)
        .login_uc(login_uc)
        .play_stream_uc(play_stream_uc)
        .query_business_listings_uc(query_business_listings_uc)
        .is_stream_active(false)
        .build();

    // Spawn the task that listens to server game updates.
    // It gets the sender part of the channel to send responses back.
    app.stream_handler
        .spawn_server_updates_listener(app_event_tx);

    // Run the main application cycle.
    let final_app_result = app.run(&mut terminal, &mut app_event_rx).await;

    // Graceful shutdown sequence
    trace_dbg!("Application loop ended. Shutting down...");

    // Await the completion of the input reader task
    read_input_handle.await?;

    // Restore the terminal to its original state
    ratatui::restore();
    trace_dbg!("Terminal restored. Exiting.");

    final_app_result
}
