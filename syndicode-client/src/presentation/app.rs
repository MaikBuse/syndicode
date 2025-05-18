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
        auth::{
            login::LoginUserUseCase, register::RegisterUseCase, resend::ResendVerificationUseCase,
            verifiy::VerifyUserUseCase,
        },
        game::{query_business_listings::QueryBusinessListingsUseCase, stream::PlayStreamUseCase},
    },
    domain::{auth::AuthenticationRepository, game::GameRepository, response::Response},
    trace_dbg,
};
use bon::Builder;
use ratatui::{widgets::ListState, DefaultTerminal};
use tokio::sync::mpsc;
use tui_textarea::TextArea;

pub enum AppEvent {
    Crossterm(ratatui::crossterm::event::Event),
    StreamUpdate(Response),
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
    ListDetail,
    Exiting,
}

impl Default for CurrentScreen {
    fn default() -> Self {
        Self::Main(CurrentScreenMain::Services)
    }
}

#[derive(Debug, Builder)]
pub struct App<'a, AUTH, GAME>
where
    AUTH: AuthenticationRepository,
    GAME: GameRepository,
{
    pub maybe_username: Option<String>,
    pub maybe_token: Option<String>,
    pub is_stream_active: bool,
    pub current_screen: CurrentScreen,
    pub should_exit: bool,
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
    pub play_stream_uc: PlayStreamUseCase<GAME>,
    pub query_business_listings_uc: QueryBusinessListingsUseCase<GAME>,
}

impl<'a, AUTH, GAME> App<'a, AUTH, GAME>
where
    AUTH: AuthenticationRepository,
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
                    self.response_list_widget.push(response);

                    if self.current_screen == CurrentScreen::Main(CurrentScreenMain::Responses) {
                        self.response_list_state.select_next();
                    }
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
