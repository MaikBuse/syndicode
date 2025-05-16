use super::{
    widget::{
        editing::{
            SelectedBlockLogin, SelectedBlockRegister, SelectedBlockResend, SelectedBlockVerify,
        },
        service_list::ServiceAction,
    },
    App, CurrentScreen, SelectedService,
};
use crate::domain::{auth::AuthenticationRepository, game::GameRepository};
use ratatui::crossterm::event::{Event, KeyCode};

pub(super) async fn handle_crossterm_event<AUTH, GAME>(
    app: &mut App<'_, AUTH, GAME>,
    event: Event,
) -> anyhow::Result<()>
where
    AUTH: AuthenticationRepository,
    GAME: GameRepository,
{
    match app.current_screen {
        CurrentScreen::Main => {
            handle_main(app, event).await;
        }
        CurrentScreen::Editing => {
            handle_editing(app, event).await?;
        }
        CurrentScreen::Exiting => {
            handle_exit(app, event);
        }
    };

    Ok(())
}

fn handle_exit<AUTH, GAME>(app: &mut App<'_, AUTH, GAME>, event: Event)
where
    AUTH: AuthenticationRepository,
    GAME: GameRepository,
{
    if let Event::Key(key_event) = event {
        match key_event.code {
            KeyCode::Char('y') => {
                app.should_exit = true;
            }
            KeyCode::Char('n') => {
                app.current_screen = CurrentScreen::Main;
            }
            _ => {}
        }
    }
}

async fn handle_main<AUTH, GAME>(app: &mut App<'_, AUTH, GAME>, event: Event)
where
    AUTH: AuthenticationRepository,
    GAME: GameRepository,
{
    if let Event::Key(key_event) = event {
        match key_event.code {
            KeyCode::Char('h') | KeyCode::Left => app.service_list_state.select(None),
            KeyCode::Char('j') | KeyCode::Down => app.service_list_state.select_next(),
            KeyCode::Char('k') | KeyCode::Up => app.service_list_state.select_previous(),
            KeyCode::Char('g') | KeyCode::Home => app.service_list_state.select_first(),
            KeyCode::Char('G') | KeyCode::End => app.service_list_state.select_last(),
            KeyCode::Enter => {
                if let Some(service_action) = app
                    .service_list_widget
                    .get_selected_action(&app.service_list_state)
                {
                    match service_action {
                        ServiceAction::PlayStream => {
                            if let Some(token) = app.maybe_token.clone() {
                                let server_updates_stream =
                                    app.play_stream_uc.execute(token).await.unwrap();

                                app.stream_handler
                                    .set_server_updates_stream(server_updates_stream)
                                    .await;

                                app.stream_handler.signal_start_processing();
                            }
                            return;
                        }
                        ServiceAction::QueryBusinessListings => {
                            app.query_business_listings_uc.execute().await.unwrap();
                            return;
                        }
                        _ => {}
                    }

                    let selected = SelectedService::from(service_action);

                    app.maybe_selected_service = Some(selected);
                    app.current_screen = CurrentScreen::Editing;
                }
            }
            KeyCode::Esc => app.current_screen = CurrentScreen::Exiting,
            _ => {}
        };
    }
}

async fn handle_editing<AUTH, GAME>(
    app: &mut App<'_, AUTH, GAME>,
    event: Event,
) -> anyhow::Result<()>
where
    AUTH: AuthenticationRepository,
    GAME: GameRepository,
{
    let Some(selected_service) = &mut app.maybe_selected_service else {
        return Err(anyhow::anyhow!("Failed to retrieve selected service"));
    };

    if let Event::Key(key_event) = event {
        match key_event.code {
            KeyCode::Enter => {
                match selected_service {
                    SelectedService::Register {
                        selected: _,
                        user_name_textarea,
                        user_password_textarea,
                        corporation_name_textarea,
                        email_textarea,
                    } => {
                        let user_name = user_name_textarea.lines().first().ok_or_else(|| {
                            anyhow::anyhow!("Failed to retrieve user name from textarea")
                        })?;
                        let user_password =
                            user_password_textarea.lines().first().ok_or_else(|| {
                                anyhow::anyhow!("Failed to retrieve user password from textarea")
                            })?;
                        let email = email_textarea.lines().first().ok_or_else(|| {
                            anyhow::anyhow!("Failed to retrieve email from textarea")
                        })?;
                        let corporation_name =
                            corporation_name_textarea.lines().first().ok_or_else(|| {
                                anyhow::anyhow!("Failed to retrieve corporation name from textarea")
                            })?;

                        let response = app
                            .register_uc
                            .execute()
                            .user_name(user_name.to_owned())
                            .user_password(user_password.to_owned())
                            .email(email.to_owned())
                            .corporation_name(corporation_name.to_owned())
                            .call()
                            .await?;

                        app.response_list.push(response);
                        app.maybe_selected_service = None;
                        app.current_screen = CurrentScreen::Main;
                    }
                    SelectedService::VerifyRegistration {
                        selected: _,
                        user_name_textarea,
                        code_textarea,
                    } => {
                        let user_name = user_name_textarea.lines().first().ok_or_else(|| {
                            anyhow::anyhow!("Failed to retrieve user name from textarea")
                        })?;
                        let code = code_textarea.lines().first().ok_or_else(|| {
                            anyhow::anyhow!("Failed to retrieve code from textarea")
                        })?;

                        let response = app
                            .verifiy_uc
                            .execute()
                            .user_name(user_name.to_owned())
                            .code(code.to_owned())
                            .call()
                            .await?;

                        app.response_list.push(response);
                        app.maybe_selected_service = None;
                        app.current_screen = CurrentScreen::Main;
                    }
                    SelectedService::ResendVerification {
                        selected: _,
                        user_name_textarea,
                    } => {
                        let user_name = user_name_textarea.lines().first().ok_or_else(|| {
                            anyhow::anyhow!("Failed to retrieve user name from textarea")
                        })?;

                        let response = app
                            .resend_uc
                            .execute()
                            .user_name(user_name.to_owned())
                            .call()
                            .await?;

                        app.response_list.push(response);
                        app.maybe_selected_service = None;
                        app.current_screen = CurrentScreen::Main;
                    }
                    SelectedService::Login {
                        selected: _,
                        user_name_textarea,
                        user_password_textarea,
                    } => {
                        let user_name = user_name_textarea.lines().first().ok_or_else(|| {
                            anyhow::anyhow!("Failed to retrieve user name from textarea")
                        })?;
                        let user_password =
                            user_password_textarea.lines().first().ok_or_else(|| {
                                anyhow::anyhow!("Failed to retrieve user password from textarea")
                            })?;

                        let (jwt, response) = app
                            .login_uc
                            .execute()
                            .user_name(user_name.to_owned())
                            .user_password(user_password.to_owned())
                            .call()
                            .await?;

                        app.maybe_token = Some(jwt);
                        app.maybe_username = Some(user_name.to_owned());

                        app.response_list.push(response);
                        app.maybe_selected_service = None;
                        app.current_screen = CurrentScreen::Main;
                    }
                };
            }
            KeyCode::Esc => app.current_screen = CurrentScreen::Main,
            KeyCode::Tab => {
                if let Some(selected_service) = &mut app.maybe_selected_service {
                    match selected_service {
                        SelectedService::Register { selected, .. } => selected.advance(),
                        SelectedService::VerifyRegistration { selected, .. } => selected.advance(),
                        SelectedService::ResendVerification { selected, .. } => selected.advance(),
                        SelectedService::Login { selected, .. } => selected.advance(),
                    }
                };
            }
            KeyCode::BackTab => {
                if let Some(selected_service) = &mut app.maybe_selected_service {
                    match selected_service {
                        SelectedService::Register { selected, .. } => selected.previous(),
                        SelectedService::VerifyRegistration { selected, .. } => selected.previous(),
                        SelectedService::ResendVerification { selected, .. } => selected.previous(),
                        SelectedService::Login { selected, .. } => selected.previous(),
                    }
                };
            }
            _ => match selected_service {
                SelectedService::Register {
                    selected,
                    user_name_textarea,
                    user_password_textarea,
                    corporation_name_textarea,
                    email_textarea,
                } => match selected {
                    SelectedBlockRegister::UserName => {
                        user_name_textarea.input(event);
                    }
                    SelectedBlockRegister::UserPassword => {
                        user_password_textarea.input(event);
                    }
                    SelectedBlockRegister::Email => {
                        email_textarea.input(event);
                    }
                    SelectedBlockRegister::CorporationName => {
                        corporation_name_textarea.input(event);
                    }
                },
                SelectedService::VerifyRegistration {
                    selected,
                    user_name_textarea,
                    code_textarea,
                } => match selected {
                    SelectedBlockVerify::UserName => {
                        user_name_textarea.input(event);
                    }
                    SelectedBlockVerify::Code => {
                        code_textarea.input(event);
                    }
                },
                SelectedService::ResendVerification {
                    selected,
                    user_name_textarea,
                } => match selected {
                    SelectedBlockResend::UserName => {
                        user_name_textarea.input(event);
                    }
                },
                SelectedService::Login {
                    selected,
                    user_name_textarea,
                    user_password_textarea,
                } => match selected {
                    SelectedBlockLogin::UserName => {
                        user_name_textarea.input(event);
                    }
                    SelectedBlockLogin::UserPassword => {
                        user_password_textarea.input(event);
                    }
                },
            },
        };
    }

    Ok(())
}
