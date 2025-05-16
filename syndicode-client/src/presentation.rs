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
use crate::domain::game::GameRepository;
use crate::domain::response::Response;
use crate::trace_dbg;
use crate::{
    application::auth::register::RegisterUseCase, domain::auth::AuthenticationRepository,
    infrastructure::grpc::GrpcHandler,
};
use bon::Builder;
use event::InputReader;
use input::handle_crossterm_event;
use ratatui::style::Style;
use ratatui::widgets::{Block, Borders};
use ratatui::{widgets::ListState, DefaultTerminal};
use std::sync::Arc;
use stream::StreamHandler;
use tokio::sync::{mpsc, Mutex};
use tui_textarea::TextArea;
use ui::DrawStateData;
use widget::editing::{
    EditingPopupWidget, SelectedBlockLogin, SelectedBlockRegister, SelectedBlockResend,
    SelectedBlockVerify,
};
use widget::exit::ExitPopupWidget;
use widget::response_list::ResponseListWidget;
use widget::service_list::{default_services, ServiceAction, ServiceListWidget};

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
    let editing_popup_widget = EditingPopupWidget;
    let exit_popup_widget = ExitPopupWidget;
    let categories = default_services();
    let service_list_widget = ServiceListWidget::new(categories);

    // Build the main App state
    let mut app = App::builder()
        .should_exit(false)
        .stream_handler(stream_handler)
        .service_list_widget(service_list_widget)
        .service_list_state(ListState::default().with_selected(Some(0)))
        .response_list(response_list_widget)
        .editing_popup(editing_popup_widget)
        .exit_popup(exit_popup_widget)
        .current_screen(CurrentScreen::Main)
        .register_uc(register_uc)
        .verifiy_uc(verify_uc)
        .resend_uc(resend_uc)
        .login_uc(login_uc)
        .play_stream_uc(play_stream_uc)
        .query_business_listings_uc(query_business_listings_uc)
        .build();

    // Spawn the task that listens to server game updates.
    // It gets the sender part of the channel to send responses back.
    let stream_listener_handle = app
        .stream_handler
        .spawn_server_updates_listener(app_event_tx);

    // Run the main application cycle.
    let final_app_result = app.run(&mut terminal, &mut app_event_rx).await;

    // Graceful shutdown sequence
    trace_dbg!("Application loop ended. Shutting down...");

    // Await the completion of the input reader task
    read_input_handle.await?;

    // Await the completion of the stream listener task
    stream_listener_handle.await?;

    // Restore the terminal to its original state
    ratatui::restore();
    trace_dbg!("Terminal restored. Exiting.");

    final_app_result
}

pub enum AppEvent {
    Crossterm(ratatui::crossterm::event::Event),
    StreamUpdate(Response),
}

#[derive(Debug)]
pub enum SelectedService<'a> {
    Register {
        selected: SelectedBlockRegister,
        user_name_textarea: TextArea<'a>,
        user_password_textarea: TextArea<'a>,
        corporation_name_textarea: TextArea<'a>,
        email_textarea: TextArea<'a>,
    },
    VerifyRegistration {
        selected: SelectedBlockVerify,
        user_name_textarea: TextArea<'a>,
        code_textarea: TextArea<'a>,
    },
    ResendVerification {
        selected: SelectedBlockResend,
        user_name_textarea: TextArea<'a>,
    },
    Login {
        selected: SelectedBlockLogin,
        user_name_textarea: TextArea<'a>,
        user_password_textarea: TextArea<'a>,
    },
}

impl From<ServiceAction> for SelectedService<'_> {
    fn from(value: ServiceAction) -> Self {
        match value {
            ServiceAction::Register => {
                let mut textarea = TextArea::default();
                textarea.set_cursor_line_style(Style::default());

                let mut user_name_textarea = textarea.clone();
                user_name_textarea.set_placeholder_text("Mc_Lovin");
                user_name_textarea
                    .set_block(Block::default().borders(Borders::ALL).title("Username"));

                let mut user_password_textarea = textarea.clone();
                user_password_textarea.set_placeholder_text("my-secret-password");
                user_password_textarea
                    .set_block(Block::default().borders(Borders::ALL).title("Password"));

                let mut email_textarea = textarea.clone();
                email_textarea.set_placeholder_text("name@domain.com");
                email_textarea.set_block(Block::default().borders(Borders::ALL).title("Email"));

                let mut corporation_name_textarea = textarea.clone();
                corporation_name_textarea.set_placeholder_text("Lima Hammersmith Inc.");
                corporation_name_textarea.set_block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Corporation Name"),
                );

                Self::Register {
                    selected: SelectedBlockRegister::UserName,
                    user_name_textarea,
                    user_password_textarea,
                    corporation_name_textarea,
                    email_textarea,
                }
            }
            ServiceAction::VerifyRegistration => {
                let mut textarea = TextArea::default();
                textarea.set_cursor_line_style(Style::default());

                let mut user_name_textarea = textarea.clone();
                user_name_textarea.set_placeholder_text("Mc_Lovin");
                user_name_textarea
                    .set_block(Block::default().borders(Borders::ALL).title("Username"));

                let mut code_textarea = textarea.clone();
                code_textarea.set_placeholder_text("8FA3FLI91");
                code_textarea.set_block(Block::default().borders(Borders::ALL).title("Code"));

                Self::VerifyRegistration {
                    selected: SelectedBlockVerify::UserName,
                    user_name_textarea,
                    code_textarea,
                }
            }
            ServiceAction::ResendVerification => {
                let mut textarea = TextArea::default();
                textarea.set_cursor_line_style(Style::default());

                let mut user_name_textarea = textarea.clone();
                user_name_textarea.set_placeholder_text("Mc_Lovin");
                user_name_textarea
                    .set_block(Block::default().borders(Borders::ALL).title("Username"));

                Self::ResendVerification {
                    selected: SelectedBlockResend::UserName,
                    user_name_textarea,
                }
            }
            _ => {
                let mut textarea = TextArea::default();
                textarea.set_cursor_line_style(Style::default());

                let mut user_name_textarea = textarea.clone();
                user_name_textarea.set_placeholder_text("Mc_Lovin");
                user_name_textarea
                    .set_block(Block::default().borders(Borders::ALL).title("Username"));

                let mut user_password_textarea = textarea.clone();
                user_password_textarea.set_placeholder_text("my-secret-password");
                user_password_textarea
                    .set_block(Block::default().borders(Borders::ALL).title("Password"));

                Self::Login {
                    selected: SelectedBlockLogin::UserName,
                    user_name_textarea,
                    user_password_textarea,
                }
            }
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum CurrentScreen {
    #[default]
    Main,
    Editing,
    Exiting,
}

#[derive(Debug, Builder)]
pub struct App<'a, AUTH, GAME>
where
    AUTH: AuthenticationRepository,
    GAME: GameRepository,
{
    maybe_username: Option<String>,
    maybe_token: Option<String>,
    current_screen: CurrentScreen,
    should_exit: bool,
    stream_handler: StreamHandler,
    maybe_selected_service: Option<SelectedService<'a>>,
    service_list_widget: ServiceListWidget,
    service_list_state: ListState,
    response_list: ResponseListWidget,
    editing_popup: EditingPopupWidget,
    exit_popup: ExitPopupWidget,
    register_uc: RegisterUseCase<AUTH>,
    verifiy_uc: VerifyUserUseCase<AUTH>,
    resend_uc: ResendVerificationUseCase<AUTH>,
    login_uc: LoginUserUseCase<AUTH>,
    play_stream_uc: PlayStreamUseCase<GAME>,
    query_business_listings_uc: QueryBusinessListingsUseCase<GAME>,
}

impl<'app_lifetime, AUTH, GAME> App<'app_lifetime, AUTH, GAME>
where
    AUTH: AuthenticationRepository,
    GAME: GameRepository,
{
    pub async fn run(
        &'app_lifetime mut self,
        terminal: &mut DefaultTerminal,
        rx: &mut mpsc::Receiver<AppEvent>,
    ) -> anyhow::Result<()> {
        'app_loop: loop {
            let prepared_ui_data = {
                DrawStateData {
                    maybe_username: self.maybe_username.clone(),
                    service_list_widget: &self.service_list_widget,
                    current_screen: self.current_screen,
                    editing_popup: &self.editing_popup,
                    exit_popup: &self.exit_popup,
                }
            };

            // Explicitly borrow mutable parts needed for drawing BEFORE the closure
            let service_list_state_mut = &mut self.service_list_state;
            let maybe_selected_service_mut = &mut self.maybe_selected_service;

            terminal.draw(|frame| {
                ui::draw(
                    frame,
                    &prepared_ui_data,
                    service_list_state_mut,
                    maybe_selected_service_mut,
                    &self.response_list,
                );
            })?;

            // --- Event Handling Phase ---
            match rx.recv().await {
                Some(AppEvent::Crossterm(event)) => {
                    // Previous borrows are released, `self` can be mutably borrowed.
                    handle_crossterm_event(self, event).await?;
                }
                Some(AppEvent::StreamUpdate(response)) => {
                    self.response_list.push(response);
                }
                None => {
                    trace_dbg!("App::run: Event channel closed, exiting loop.");
                    break 'app_loop;
                }
            };

            if self.should_exit {
                trace_dbg!("App::run: Exiting loop due to should_exit flag.");

                if self.stream_handler.is_processing().await {
                    self.stream_handler.signal_stop_processing();
                }

                break 'app_loop;
            };
        }
        Ok(())
    }
}
