mod app;
mod input;
mod reader;
mod stream;
mod theme;
mod ui;
mod widget;

use crate::application::admin::create_user::CreateUserUseCase;
use crate::application::admin::delete_user::DeleteUserUseCase;
use crate::application::admin::get_user::GetUserUseCase;
use crate::application::auth::get_current_user::GetCurrentUserUseCase;
use crate::application::auth::login::LoginUserUseCase;
use crate::application::auth::resend::ResendVerificationUseCase;
use crate::application::auth::verifiy::VerifyUserUseCase;
use crate::application::game::acquire_listed_business::AcquireListedBusinessUseCase;
use crate::application::game::query_business_listings::QueryBusinessListingsUseCase;
use crate::application::game::stream::PlayStreamUseCase;
use crate::config::load_config;
use crate::{application::auth::register::RegisterUseCase, infrastructure::grpc::GrpcHandler};
use app::{App, AppEvent, CurrentScreen};
use ratatui::widgets::ListState;
use reader::Reader;
use std::sync::Arc;
use stream::StreamHandler;
use tokio::sync::{mpsc, Mutex, Notify};
use widget::exit::ExitPopupWidget;
use widget::response_detail::ResponseDetailWidget;
use widget::response_list::ResponseListWidget;
use widget::service::service_detail::ServiceDetailWidget;
use widget::service::service_list::{default_services, ServiceListWidget};
use widget::vim::{Mode, Vim};

pub async fn run_cli() -> anyhow::Result<()> {
    // Load configuration
    let config = load_config()?;

    // Initialize gRPC handler (shared among use cases)
    let grpc_handler = Arc::new(Mutex::new(
        GrpcHandler::new(config.grpc.server_address, config.general.is_local_test).await?,
    ));

    let mut terminal = ratatui::init();

    // Event channel for input events from InputReader
    let (app_event_tx, mut app_event_rx) = mpsc::channel(10);
    let app_event_tx_clone: mpsc::Sender<AppEvent> = app_event_tx.clone();

    let shutdown_signal = Arc::new(Notify::new());

    // Spawn a task to read input events and send them through the channel
    let input_reader = Reader::new(Arc::clone(&shutdown_signal), app_event_tx_clone);
    let reader_handle = input_reader.spawn_read_input_events();

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
    let get_current_user_uc = GetCurrentUserUseCase::builder()
        .auth_repository(grpc_handler.clone())
        .build();
    let create_user_uc = CreateUserUseCase::builder()
        .admin_repository(grpc_handler.clone())
        .build();
    let get_user_uc = GetUserUseCase::builder()
        .admin_repository(grpc_handler.clone())
        .build();
    let delete_user_uc = DeleteUserUseCase::builder()
        .admin_repository(grpc_handler.clone())
        .build();
    let play_stream_uc = PlayStreamUseCase::builder()
        .game_repo(grpc_handler.clone())
        .build();
    let get_corporation_uc =
        crate::application::game::get_corporation::GetCorporationUseCase::builder()
            .game_repo(grpc_handler.clone())
            .build();
    let query_business_listings_uc = QueryBusinessListingsUseCase::builder()
        .game_repo(grpc_handler.clone())
        .build();
    let acquire_listed_business_uc = AcquireListedBusinessUseCase::builder()
        .game_repo(grpc_handler.clone())
        .build();

    // Initialize StreamHandler (owned by App or managed alongside)
    let stream_handler = StreamHandler::new(shutdown_signal.clone());

    // Initialize Widgets
    let response_list_widget = ResponseListWidget::new();
    let response_detail_widget = ResponseDetailWidget;
    let service_detail_widget = ServiceDetailWidget;
    let exit_popup_widget = ExitPopupWidget;
    let categories = default_services()
        .is_stream_active(false)
        .is_logged_in(false)
        .call();
    let service_list_widget = ServiceListWidget::new(categories);

    // Vim
    let response_detail_vim = Vim::new(Mode::Normal);

    // Build the main App state
    let mut app = App::builder()
        .should_exit(false)
        .hide_game_tick_notification(false)
        .yank_buffer(String::new())
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
        .get_current_user_uc(get_current_user_uc)
        .create_user_uc(create_user_uc)
        .get_user_uc(get_user_uc)
        .delete_user_uc(delete_user_uc)
        .play_stream_uc(play_stream_uc)
        .get_corporation_uc(get_corporation_uc)
        .query_business_listings_uc(query_business_listings_uc)
        .acquire_business_listing_uc(acquire_listed_business_uc)
        .is_stream_active(false)
        .shutdown_signal(shutdown_signal)
        .build();

    // Spawn the task that listens to server game updates.
    // It gets the sender part of the channel to send responses back.
    let stream_handle = app
        .stream_handler
        .spawn_server_updates_listener(app_event_tx);

    // Run the main application cycle.
    let final_app_result = app.run(&mut terminal, &mut app_event_rx).await;

    // Graceful shutdown sequence
    tracing::debug!("Application loop ended. Shutting down...");

    // Await the completion of the tasks
    reader_handle.await?;
    stream_handle.await?;

    // Restore the terminal to its original state
    ratatui::restore();
    tracing::debug!("Terminal restored. Exiting.");

    final_app_result
}
