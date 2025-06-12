use std::sync::Arc;

use super::{
    input::handle_crossterm_event,
    stream::StreamHandler,
    ui,
    widget::{
        exit::ExitPopupWidget,
        response_detail::ResponseDetailWidget,
        response_list::ResponseListWidget,
        service::{
            selected_service::SelectedService, service_detail::ServiceDetailWidget,
            service_list::ServiceListWidget,
        },
        vim::Vim,
    },
};
use crate::{
    application::{
        admin::{
            create_user::CreateUserUseCase, delete_user::DeleteUserUseCase,
            get_user::GetUserUseCase,
        },
        auth::{
            get_current_user::GetCurrentUserUseCase, login::LoginUserUseCase,
            register::RegisterUseCase, resend::ResendVerificationUseCase,
            verifiy::VerifyUserUseCase,
        },
        game::{
            acquire_listed_business::AcquireListedBusinessUseCase,
            get_corporation::GetCorporationUseCase,
            query_business_listings::QueryBusinessListingsUseCase, stream::PlayStreamUseCase,
        },
    },
    domain::{
        admin::AdminRepository,
        auth::repository::AuthenticationRepository,
        game::GameRepository,
        response::{DomainResponse, ResponseType},
    },
};
use bon::Builder;
use ratatui::{widgets::ListState, DefaultTerminal};
use tokio::sync::{mpsc, Notify};
use tui_textarea::TextArea;

pub enum AppEvent {
    Crossterm(ratatui::crossterm::event::Event),
    StreamUpdate(DomainResponse),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CurrentScreenMain {
    Services,
    Responses,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CurrentScreen {
    Main(CurrentScreenMain),
    ServiceDetail,
    ResponseDetail,
    Exiting,
}

impl Default for CurrentScreen {
    fn default() -> Self {
        Self::Main(CurrentScreenMain::Services)
    }
}

#[derive(Debug, Builder)]
pub struct App<'a, AUTH, ADMIN, GAME>
where
    AUTH: AuthenticationRepository,
    ADMIN: AdminRepository,
    GAME: GameRepository,
{
    pub maybe_username: Option<String>,
    pub maybe_token: Option<String>,
    pub yank_buffer: String,
    pub is_stream_active: bool,
    pub hide_game_tick_notification: bool,
    pub current_screen: CurrentScreen,
    pub should_exit: bool,
    pub shutdown_signal: Arc<Notify>,
    pub stream_handler: StreamHandler,
    pub maybe_response_detail_textarea: Option<TextArea<'a>>,
    pub response_detail_vim: Vim,
    pub maybe_selected_service: Option<SelectedService<'a>>,
    pub service_list_widget: ServiceListWidget,
    pub service_list_state: ListState,
    pub response_list_widget: ResponseListWidget,
    pub response_list_state: ListState,
    pub response_detail_widget: ResponseDetailWidget,
    pub service_detail_widget: ServiceDetailWidget,
    pub exit_popup_widget: ExitPopupWidget,
    pub register_uc: RegisterUseCase<AUTH>,
    pub verifiy_uc: VerifyUserUseCase<AUTH>,
    pub resend_uc: ResendVerificationUseCase<AUTH>,
    pub login_uc: LoginUserUseCase<AUTH>,
    pub get_current_user_uc: GetCurrentUserUseCase<AUTH>,
    pub create_user_uc: CreateUserUseCase<ADMIN>,
    pub get_user_uc: GetUserUseCase<ADMIN>,
    pub delete_user_uc: DeleteUserUseCase<ADMIN>,
    pub play_stream_uc: PlayStreamUseCase<GAME>,
    pub get_corporation_uc: GetCorporationUseCase<GAME>,
    pub query_business_listings_uc: QueryBusinessListingsUseCase<GAME>,
    pub acquire_business_listing_uc: AcquireListedBusinessUseCase<GAME>,
}

impl<'a, AUTH, ADMIN, GAME> App<'a, AUTH, ADMIN, GAME>
where
    AUTH: AuthenticationRepository,
    ADMIN: AdminRepository,
    GAME: GameRepository,
{
    pub async fn run(
        &'a mut self,
        terminal: &mut DefaultTerminal,
        rx: &mut mpsc::Receiver<AppEvent>,
    ) -> anyhow::Result<()> {
        'app_loop: loop {
            terminal.draw(|frame| {
                ui::draw(frame, self);
            })?;

            // --- Event Handling Phase ---
            match rx.recv().await {
                Some(AppEvent::Crossterm(event)) => {
                    // Previous borrows are released, `self` can be mutably borrowed.
                    handle_crossterm_event(self, event).await?;
                }
                Some(AppEvent::StreamUpdate(response)) => {
                    let is_main_screen = self.current_screen
                        == CurrentScreen::Main(CurrentScreenMain::Responses)
                        || self.current_screen == CurrentScreen::ResponseDetail;

                    match response.response_type {
                        ResponseType::GameTickeNotification => {
                            self.response_list_widget.push(response);

                            if !self.hide_game_tick_notification && is_main_screen {
                                self.response_list_state.select_next();
                            }
                        }
                        _ => {
                            self.response_list_widget.push(response);

                            if is_main_screen {
                                self.response_list_state.select_next();
                            }
                        }
                    };
                }
                None => {
                    tracing::info!("App::run: Event channel closed, exiting loop.");
                    break 'app_loop;
                }
            };

            if self.should_exit {
                tracing::info!("App::run: Exiting loop due to should_exit flag.");

                self.shutdown_signal.notify_waiters();

                break 'app_loop;
            };
        }
        Ok(())
    }
}
