use crate::{
    domain::{auth::AuthenticationRepository, game::GameRepository},
    presentation::{
        app::{App, CurrentScreen, CurrentScreenMain},
        widget::service::{
            selected_block::{
                SelectedBlockLogin, SelectedBlockQueryBusinessListings, SelectedBlockRegister,
                SelectedBlockResend, SelectedBlockVerify,
            },
            selected_service::SelectedService,
        },
    },
};
use ratatui::crossterm::event::{Event, KeyCode};

pub(super) async fn handle_service_detail<AUTH, GAME>(
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
                    SelectedService::QueryBusinessListings {
                        selected: _,
                        min_asking_price,
                        max_asking_price,
                        seller_corporation_uuid,
                        market_uuid,
                        min_operational_expenses,
                        max_operational_expenses,
                        sort_by,
                        sort_direction,
                        limit,
                        offset,
                    } => {
                        let maybe_min_asking_price: Option<i64> = min_asking_price
                            .textarea
                            .lines()
                            .first()
                            .map(|x| x.parse::<i64>().unwrap());
                        let maybe_max_asking_price: Option<i64> = max_asking_price
                            .textarea
                            .lines()
                            .first()
                            .map(|x| x.parse::<i64>().unwrap());
                        let maybe_seller_corporation_uuid: Option<String> = seller_corporation_uuid
                            .textarea
                            .lines()
                            .first()
                            .map(|x| x.to_owned());
                        let maybe_market_uuid: Option<String> =
                            market_uuid.textarea.lines().first().map(|x| x.to_owned());
                        let maybe_min_operational_expenses: Option<i64> = min_operational_expenses
                            .textarea
                            .lines()
                            .first()
                            .map(|x| x.parse::<i64>().unwrap());
                        let maybe_max_operational_expenses: Option<i64> = max_operational_expenses
                            .textarea
                            .lines()
                            .first()
                            .map(|x| x.parse::<i64>().unwrap());
                        let sort_by: String = sort_by
                            .textarea
                            .lines()
                            .first()
                            .map(|x| x.to_owned())
                            .unwrap_or(String::new());
                        let sort_direction: i32 = sort_direction
                            .textarea
                            .lines()
                            .first()
                            .map(|x| x.parse::<i32>().unwrap())
                            .unwrap_or_default();
                        let maybe_limit: Option<i64> = limit
                            .textarea
                            .lines()
                            .first()
                            .map(|x| x.parse::<i64>().unwrap());
                        let maybe_offset: Option<i64> = offset
                            .textarea
                            .lines()
                            .first()
                            .map(|x| x.parse::<i64>().unwrap());

                        app.query_business_listings_uc
                            .execute()
                            .maybe_min_asking_price(maybe_min_asking_price)
                            .maybe_max_asking_price(maybe_max_asking_price)
                            .maybe_seller_corporation_uuid(maybe_seller_corporation_uuid)
                            .maybe_market_uuid(maybe_market_uuid)
                            .maybe_min_operational_expenses(maybe_min_operational_expenses)
                            .maybe_max_operational_expenses(maybe_max_operational_expenses)
                            .sort_by(sort_by)
                            .sort_direction(sort_direction)
                            .maybe_limit(maybe_limit)
                            .maybe_offset(maybe_offset)
                            .call()
                            .await?;
                    }
                    SelectedService::Register {
                        selected: _,
                        user_name,
                        user_password,
                        corporation_name,
                        email,
                    } => {
                        let user_name = user_name.textarea.lines().first().ok_or_else(|| {
                            anyhow::anyhow!("Failed to retrieve user name from textarea")
                        })?;
                        let user_password =
                            user_password.textarea.lines().first().ok_or_else(|| {
                                anyhow::anyhow!("Failed to retrieve user password from textarea")
                            })?;
                        let email = email.textarea.lines().first().ok_or_else(|| {
                            anyhow::anyhow!("Failed to retrieve email from textarea")
                        })?;
                        let corporation_name =
                            corporation_name.textarea.lines().first().ok_or_else(|| {
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

                        app.response_list_widget.push(response);
                        app.maybe_selected_service = None;
                        app.current_screen = CurrentScreen::Main(CurrentScreenMain::Services);
                    }
                    SelectedService::VerifyRegistration {
                        selected: _,
                        user_name,
                        code,
                    } => {
                        let user_name = user_name.textarea.lines().first().ok_or_else(|| {
                            anyhow::anyhow!("Failed to retrieve user name from textarea")
                        })?;
                        let code = code.textarea.lines().first().ok_or_else(|| {
                            anyhow::anyhow!("Failed to retrieve code from textarea")
                        })?;

                        let response = app
                            .verifiy_uc
                            .execute()
                            .user_name(user_name.to_owned())
                            .code(code.to_owned())
                            .call()
                            .await?;

                        app.response_list_widget.push(response);
                        app.maybe_selected_service = None;
                        app.current_screen = CurrentScreen::Main(CurrentScreenMain::Services);
                    }
                    SelectedService::ResendVerification {
                        selected: _,
                        user_name,
                    } => {
                        let user_name = user_name.textarea.lines().first().ok_or_else(|| {
                            anyhow::anyhow!("Failed to retrieve user name from textarea")
                        })?;

                        let response = app
                            .resend_uc
                            .execute()
                            .user_name(user_name.to_owned())
                            .call()
                            .await?;

                        app.response_list_widget.push(response);
                        app.maybe_selected_service = None;
                        app.current_screen = CurrentScreen::Main(CurrentScreenMain::Services);
                    }
                    SelectedService::Login {
                        selected: _,
                        user_name,
                        user_password,
                    } => {
                        let user_name = user_name.textarea.lines().first().ok_or_else(|| {
                            anyhow::anyhow!("Failed to retrieve user name from textarea")
                        })?;
                        let user_password =
                            user_password.textarea.lines().first().ok_or_else(|| {
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

                        app.response_list_widget.push(response);
                        app.maybe_selected_service = None;
                        app.current_screen = CurrentScreen::Main(CurrentScreenMain::Services);
                    }
                };
            }
            KeyCode::Esc => app.current_screen = CurrentScreen::Main(CurrentScreenMain::Services),
            KeyCode::Tab => {
                if let Some(selected_service) = &mut app.maybe_selected_service {
                    match selected_service {
                        SelectedService::Register { selected, .. } => selected.advance(),
                        SelectedService::VerifyRegistration { selected, .. } => selected.advance(),
                        SelectedService::ResendVerification { selected, .. } => selected.advance(),
                        SelectedService::Login { selected, .. } => selected.advance(),
                        SelectedService::QueryBusinessListings { selected, .. } => {
                            selected.advance()
                        }
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
                        SelectedService::QueryBusinessListings { selected, .. } => {
                            selected.previous()
                        }
                    }
                };
            }
            _ => match selected_service {
                SelectedService::Register {
                    selected,
                    user_name,
                    user_password,
                    corporation_name,
                    email,
                } => match selected {
                    SelectedBlockRegister::UserName => {
                        user_name.textarea.input(event);
                    }
                    SelectedBlockRegister::UserPassword => {
                        user_password.textarea.input(event);
                    }
                    SelectedBlockRegister::Email => {
                        email.textarea.input(event);
                    }
                    SelectedBlockRegister::CorporationName => {
                        corporation_name.textarea.input(event);
                    }
                },
                SelectedService::VerifyRegistration {
                    selected,
                    user_name,
                    code,
                } => match selected {
                    SelectedBlockVerify::UserName => {
                        user_name.textarea.input(event);
                    }
                    SelectedBlockVerify::Code => {
                        code.textarea.input(event);
                    }
                },
                SelectedService::ResendVerification {
                    selected,
                    user_name,
                } => match selected {
                    SelectedBlockResend::UserName => {
                        user_name.textarea.input(event);
                    }
                },
                SelectedService::Login {
                    selected,
                    user_name,
                    user_password,
                } => match selected {
                    SelectedBlockLogin::UserName => {
                        user_name.textarea.input(event);
                    }
                    SelectedBlockLogin::UserPassword => {
                        user_password.textarea.input(event);
                    }
                },
                SelectedService::QueryBusinessListings {
                    selected,
                    min_asking_price,
                    max_asking_price,
                    seller_corporation_uuid,
                    market_uuid,
                    min_operational_expenses,
                    max_operational_expenses,
                    sort_by,
                    sort_direction,
                    limit,
                    offset,
                } => match selected {
                    SelectedBlockQueryBusinessListings::MinAskingPrice => {
                        min_asking_price.textarea.input(event);
                    }
                    SelectedBlockQueryBusinessListings::MaxAskingPrice => {
                        max_asking_price.textarea.input(event);
                    }
                    SelectedBlockQueryBusinessListings::SellerCorporationUuid => {
                        seller_corporation_uuid.textarea.input(event);
                    }
                    SelectedBlockQueryBusinessListings::MarketUuid => {
                        market_uuid.textarea.input(event);
                    }
                    SelectedBlockQueryBusinessListings::MinOperationalExpenses => {
                        min_operational_expenses.textarea.input(event);
                    }
                    SelectedBlockQueryBusinessListings::MaxOperationalExpenses => {
                        max_operational_expenses.textarea.input(event);
                    }
                    SelectedBlockQueryBusinessListings::SortBy => {
                        sort_by.textarea.input(event);
                    }
                    SelectedBlockQueryBusinessListings::SortDirection => {
                        sort_direction.textarea.input(event);
                    }
                    SelectedBlockQueryBusinessListings::Limit => {
                        limit.textarea.input(event);
                    }
                    SelectedBlockQueryBusinessListings::Offset => {
                        offset.textarea.input(event);
                    }
                },
            },
        };
    }

    Ok(())
}
