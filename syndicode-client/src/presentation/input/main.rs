use crate::{
    domain::{
        admin::AdminRepository, auth::repository::AuthenticationRepository, game::GameRepository,
        response::DomainResponse,
    },
    presentation::{
        app::{App, CurrentScreen, CurrentScreenMain},
        widget::{
            service::{
                selected_service::SelectedService,
                service_list::{default_services, ServiceAction, ServiceListWidget},
            },
            vim::Mode,
        },
    },
};
use crossterm::event::{Event, KeyCode};
use tui_textarea::TextArea;

pub(super) async fn handle_main<AUTH, ADMIN, GAME>(
    app: &mut App<'_, AUTH, ADMIN, GAME>,
    event: Event,
) where
    AUTH: AuthenticationRepository,
    ADMIN: AdminRepository,
    GAME: GameRepository,
{
    if let Event::Key(key_event) = event {
        if !key_event.is_press() {
            return;
        }
        match key_event.code {
            KeyCode::Char('e') => {
                app.hide_game_tick_notification = !app.hide_game_tick_notification;

                if let CurrentScreen::Main(CurrentScreenMain::Responses) = app.current_screen {
                    app.response_list_state.select(Some(0));
                }
            }
            KeyCode::Char('h') | KeyCode::Left => {
                app.service_list_state.select(Some(0));
                app.response_list_state.select(None);
                app.current_screen = CurrentScreen::Main(CurrentScreenMain::Services);
            }
            KeyCode::Char('l') | KeyCode::Right => {
                if !app.response_list_widget.responses.is_empty() {
                    app.service_list_state.select(None);
                    app.response_list_state.select(Some(0));
                    app.current_screen = CurrentScreen::Main(CurrentScreenMain::Responses);
                }
            }
            KeyCode::Char('j') | KeyCode::Down => {
                match app.current_screen {
                    CurrentScreen::Main(CurrentScreenMain::Services) => {
                        app.service_list_state.select_next()
                    }
                    CurrentScreen::Main(CurrentScreenMain::Responses) => {
                        app.response_list_state.select_previous()
                    }
                    _ => {}
                };
            }
            KeyCode::Char('k') | KeyCode::Up => {
                match app.current_screen {
                    CurrentScreen::Main(CurrentScreenMain::Services) => {
                        app.service_list_state.select_previous()
                    }
                    CurrentScreen::Main(CurrentScreenMain::Responses) => {
                        app.response_list_state.select_next()
                    }
                    _ => {}
                };
            }
            KeyCode::Char('g') | KeyCode::Home => {
                match app.current_screen {
                    CurrentScreen::Main(CurrentScreenMain::Services) => {
                        app.service_list_state.select_first()
                    }
                    CurrentScreen::Main(CurrentScreenMain::Responses) => {
                        app.response_list_state.select_last()
                    }
                    _ => {}
                };
            }
            KeyCode::Char('G') | KeyCode::End => {
                match app.current_screen {
                    CurrentScreen::Main(CurrentScreenMain::Services) => {
                        app.service_list_state.select_last()
                    }
                    CurrentScreen::Main(CurrentScreenMain::Responses) => {
                        app.response_list_state.select_first()
                    }
                    _ => {}
                };
            }
            KeyCode::Enter => handle_enter(app).await,
            KeyCode::Esc => app.current_screen = CurrentScreen::Exiting,
            _ => {}
        };
    }
}

async fn handle_enter<AUTH, ADMIN, GAME>(app: &mut App<'_, AUTH, ADMIN, GAME>)
where
    AUTH: AuthenticationRepository,
    ADMIN: AdminRepository,
    GAME: GameRepository,
{
    if let CurrentScreen::Main(CurrentScreenMain::Services) = app.current_screen {
        if let Some(service_action) = app
            .service_list_widget
            .get_selected_action(&app.service_list_state)
        {
            match service_action {
                ServiceAction::GetCurrentUser => {
                    if let Some(token) = app.maybe_token.clone() {
                        let response: DomainResponse = app
                            .get_current_user_uc
                            .execute()
                            .token(token)
                            .call()
                            .await
                            .into();

                        app.response_list_widget.push(response);

                        return;
                    }
                }
                ServiceAction::PlayStream => {
                    if let Some(token) = app.maybe_token.clone() {
                        let server_updates_stream = match app.play_stream_uc.execute(token).await {
                            Ok(inner) => inner,
                            Err(err) => {
                                tracing::error!("Failed to retrieve play stream: {}", err);
                                return;
                            }
                        };

                        app.stream_handler
                            .set_server_updates_stream(server_updates_stream)
                            .await;

                        app.stream_handler.signal_start_processing();

                        let categories = default_services()
                            .is_stream_active(true)
                            .is_logged_in(true)
                            .call();
                        app.service_list_widget = ServiceListWidget::new(categories);
                        app.is_stream_active = true;
                    }
                    return;
                }
                ServiceAction::GetCorporation => {
                    if let Err(err) = app.get_corporation_uc.execute().await {
                        tracing::error!("Failed to retrieve corporation: {}", err);
                        return;
                    }

                    return;
                }
                _ => {}
            };

            let selected = SelectedService::from(service_action);

            app.maybe_selected_service = Some(selected);
            app.current_screen = CurrentScreen::ServiceDetail;
        }
    }

    if let CurrentScreen::Main(CurrentScreenMain::Responses) = app.current_screen {
        let Some(current_index) = app.response_list_state.selected() else {
            tracing::error!("Failed to retrieve the current index of the reponse list state");
            return;
        };

        let Some(response) = app
            .response_list_widget
            .get_response(current_index, app.hide_game_tick_notification)
        else {
            tracing::error!("Failed to retrieve response from response list by index");
            return;
        };

        let span = response.message.lines();

        let mut response_detail_textarea = TextArea::from(span);
        response_detail_textarea.set_block(Mode::Normal.block());
        response_detail_textarea.set_cursor_style(Mode::Normal.cursor_style());

        app.maybe_response_detail_textarea = Some(response_detail_textarea);

        app.current_screen = CurrentScreen::ResponseDetail;
    }
}
