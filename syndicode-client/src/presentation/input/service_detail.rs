use crate::{
    domain::{admin::AdminRepository, auth::AuthenticationRepository, game::GameRepository},
    presentation::{
        app::{App, CurrentScreen, CurrentScreenMain},
        widget::service::{
            selected_block::{
                SelectedBlockAcquireBusinessListing, SelectedBlockCreateUser,
                SelectedBlockDeleteUser, SelectedBlockGetUser, SelectedBlockLogin,
                SelectedBlockQueryBusinessListings, SelectedBlockRegister, SelectedBlockResend,
                SelectedBlockVerify,
            },
            selected_service::SelectedService,
            service_list::{default_services, ServiceListWidget},
        },
    },
};
use ratatui::crossterm::event::{Event, KeyCode, KeyModifiers};

pub(super) async fn handle_service_detail<AUTH, ADMIN, GAME>(
    app: &mut App<'_, AUTH, ADMIN, GAME>,
    event: Event,
) -> anyhow::Result<()>
where
    AUTH: AuthenticationRepository,
    ADMIN: AdminRepository,
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
                        let maybe_min_asking_price: Option<i64> =
                            min_asking_price.textarea.lines().first().and_then(|x| {
                                match x.is_empty() {
                                    true => None,
                                    false => Some(x.parse::<i64>().unwrap()),
                                }
                            });
                        let maybe_max_asking_price: Option<i64> =
                            max_asking_price.textarea.lines().first().and_then(|x| {
                                match x.is_empty() {
                                    true => None,
                                    false => Some(x.parse::<i64>().unwrap()),
                                }
                            });
                        let maybe_seller_corporation_uuid: Option<String> = seller_corporation_uuid
                            .textarea
                            .lines()
                            .first()
                            .and_then(|x| match x.is_empty() {
                                true => None,
                                false => Some(x.to_owned()),
                            });
                        let maybe_market_uuid: Option<String> = market_uuid
                            .textarea
                            .lines()
                            .first()
                            .and_then(|x| match x.is_empty() {
                                true => None,
                                false => Some(x.to_owned()),
                            });
                        let maybe_min_operational_expenses: Option<i64> = min_operational_expenses
                            .textarea
                            .lines()
                            .first()
                            .and_then(|x| match x.is_empty() {
                                true => None,
                                false => Some(x.parse::<i64>().unwrap()),
                            });
                        let maybe_max_operational_expenses: Option<i64> = max_operational_expenses
                            .textarea
                            .lines()
                            .first()
                            .and_then(|x| match x.is_empty() {
                                true => None,
                                false => Some(x.parse::<i64>().unwrap()),
                            });
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
                            .map(|x| x.parse::<i32>().unwrap_or_default())
                            .unwrap_or_default();
                        let maybe_limit: Option<i64> =
                            limit
                                .textarea
                                .lines()
                                .first()
                                .and_then(|x| match x.is_empty() {
                                    true => None,
                                    false => Some(x.parse::<i64>().unwrap()),
                                });
                        let maybe_offset: Option<i64> =
                            offset
                                .textarea
                                .lines()
                                .first()
                                .and_then(|x| match x.is_empty() {
                                    true => None,
                                    false => Some(x.parse::<i64>().unwrap()),
                                });

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

                        app.maybe_selected_service = None;
                        app.current_screen = CurrentScreen::Main(CurrentScreenMain::Services);
                    }
                    SelectedService::AcquireBusinessListing {
                        selected: _,
                        business_listing_uuid,
                    } => {
                        let business_listing_uuid = business_listing_uuid
                            .textarea
                            .lines()
                            .first()
                            .ok_or_else(|| {
                                anyhow::anyhow!(
                                    "Failed to retrieve business listing uuid from textarea"
                                )
                            })?;

                        app.acquire_business_listing_uc
                            .execute()
                            .business_listing_uuid(business_listing_uuid.to_owned())
                            .call()
                            .await?;

                        app.maybe_selected_service = None;
                        app.current_screen = CurrentScreen::Main(CurrentScreenMain::Services);
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

                        let categories = default_services()
                            .is_stream_active(false)
                            .is_logged_in(true)
                            .call();
                        app.service_list_widget = ServiceListWidget::new(categories);

                        app.current_screen = CurrentScreen::Main(CurrentScreenMain::Services);
                    }
                    SelectedService::CreateUser {
                        selected: _,
                        user_name,
                        user_password,
                        user_email,
                        user_role,
                        corporation_name,
                    } => {
                        let user_name = user_name.textarea.lines().first().ok_or_else(|| {
                            anyhow::anyhow!("Failed to retrieve user name from textarea")
                        })?;
                        let user_password =
                            user_password.textarea.lines().first().ok_or_else(|| {
                                anyhow::anyhow!("Failed to retrieve user password from textarea")
                            })?;
                        let user_email = user_email.textarea.lines().first().ok_or_else(|| {
                            anyhow::anyhow!("Failed to retrieve email from textarea")
                        })?;
                        let maybe_user_role: Option<i32> = user_role
                            .textarea
                            .lines()
                            .first()
                            .and_then(|x| match x.is_empty() {
                                true => None,
                                false => Some(x.parse::<i32>().unwrap()),
                            });
                        let corporation_name =
                            corporation_name.textarea.lines().first().ok_or_else(|| {
                                anyhow::anyhow!("Failed to retrieve corporation name from textarea")
                            })?;

                        let response = app
                            .create_user_uc
                            .execute()
                            .token(app.maybe_token.clone().unwrap_or_default())
                            .user_name(user_name.to_owned())
                            .user_password(user_password.to_owned())
                            .user_email(user_email.to_owned())
                            .user_role(maybe_user_role.unwrap_or_default())
                            .corporation_name(corporation_name.to_owned())
                            .call()
                            .await?;

                        app.response_list_widget.push(response);
                        app.maybe_selected_service = None;
                        app.current_screen = CurrentScreen::Main(CurrentScreenMain::Services);
                    }
                    SelectedService::GetUser {
                        selected: _,
                        user_uuid,
                    } => {
                        let user_uuid = user_uuid.textarea.lines().first().ok_or_else(|| {
                            anyhow::anyhow!("Failed to retrieve user uuid from textarea")
                        })?;

                        let response = app
                            .get_user_uc
                            .execute()
                            .token(app.maybe_token.clone().unwrap_or_default())
                            .user_uuid(user_uuid.to_owned())
                            .call()
                            .await?;

                        app.response_list_widget.push(response);
                        app.maybe_selected_service = None;
                        app.current_screen = CurrentScreen::Main(CurrentScreenMain::Services);
                    }
                    SelectedService::DeleteUser {
                        selected: _,
                        user_uuid,
                    } => {
                        let user_uuid = user_uuid.textarea.lines().first().ok_or_else(|| {
                            anyhow::anyhow!("Failed to retrieve user uuid from textarea")
                        })?;

                        let response = app
                            .delete_user_uc
                            .execute()
                            .token(app.maybe_token.clone().unwrap_or_default())
                            .user_uuid(user_uuid.to_owned())
                            .call()
                            .await?;

                        app.response_list_widget.push(response);
                        app.maybe_selected_service = None;
                        app.current_screen = CurrentScreen::Main(CurrentScreenMain::Services);
                    }
                };
            }
            KeyCode::Char('p') if key_event.modifiers.contains(KeyModifiers::ALT) => {
                if let Some(selected_service) = &mut app.maybe_selected_service {
                    match selected_service {
                        SelectedService::Register {
                            selected,
                            user_name,
                            user_password,
                            corporation_name,
                            email,
                        } => match selected {
                            SelectedBlockRegister::UserName => {
                                user_name.textarea.insert_str(app.yank_buffer.clone());
                            }
                            SelectedBlockRegister::UserPassword => {
                                user_password.textarea.insert_str(app.yank_buffer.clone());
                            }
                            SelectedBlockRegister::Email => {
                                email.textarea.insert_str(app.yank_buffer.clone());
                            }
                            SelectedBlockRegister::CorporationName => {
                                corporation_name
                                    .textarea
                                    .insert_str(app.yank_buffer.clone());
                            }
                        },
                        SelectedService::VerifyRegistration {
                            selected,
                            user_name,
                            code,
                        } => match selected {
                            SelectedBlockVerify::UserName => {
                                user_name.textarea.insert_str(app.yank_buffer.clone());
                            }
                            SelectedBlockVerify::Code => {
                                code.textarea.insert_str(app.yank_buffer.clone());
                            }
                        },
                        SelectedService::ResendVerification {
                            selected,
                            user_name,
                        } => match selected {
                            SelectedBlockResend::UserName => {
                                user_name.textarea.insert_str(app.yank_buffer.clone());
                            }
                        },
                        SelectedService::Login {
                            selected,
                            user_name,
                            user_password,
                        } => match selected {
                            SelectedBlockLogin::UserName => {
                                user_name.textarea.insert_str(app.yank_buffer.clone());
                            }
                            SelectedBlockLogin::UserPassword => {
                                user_password.textarea.insert_str(app.yank_buffer.clone());
                            }
                        },
                        SelectedService::CreateUser {
                            selected,
                            user_name,
                            user_password,
                            user_email,
                            user_role,
                            corporation_name,
                        } => match selected {
                            SelectedBlockCreateUser::UserName => {
                                user_name.textarea.insert_str(app.yank_buffer.clone());
                            }
                            SelectedBlockCreateUser::UserPassword => {
                                user_password.textarea.insert_str(app.yank_buffer.clone());
                            }
                            SelectedBlockCreateUser::UserEmail => {
                                user_email.textarea.insert_str(app.yank_buffer.clone());
                            }
                            SelectedBlockCreateUser::UserRole => {
                                user_role.textarea.insert_str(app.yank_buffer.clone());
                            }
                            SelectedBlockCreateUser::CorporationName => {
                                corporation_name
                                    .textarea
                                    .insert_str(app.yank_buffer.clone());
                            }
                        },
                        SelectedService::GetUser {
                            selected: _,
                            user_uuid,
                        } => {
                            user_uuid.textarea.insert_str(app.yank_buffer.clone());
                        }
                        SelectedService::DeleteUser {
                            selected: _,
                            user_uuid,
                        } => {
                            user_uuid.textarea.insert_str(app.yank_buffer.clone());
                        }
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
                                min_asking_price
                                    .textarea
                                    .insert_str(app.yank_buffer.clone());
                            }
                            SelectedBlockQueryBusinessListings::MaxAskingPrice => {
                                max_asking_price
                                    .textarea
                                    .insert_str(app.yank_buffer.clone());
                            }
                            SelectedBlockQueryBusinessListings::SellerCorporationUuid => {
                                seller_corporation_uuid
                                    .textarea
                                    .insert_str(app.yank_buffer.clone());
                            }
                            SelectedBlockQueryBusinessListings::MarketUuid => {
                                market_uuid.textarea.insert_str(app.yank_buffer.clone());
                            }
                            SelectedBlockQueryBusinessListings::MinOperationalExpenses => {
                                min_operational_expenses
                                    .textarea
                                    .insert_str(app.yank_buffer.clone());
                            }
                            SelectedBlockQueryBusinessListings::MaxOperationalExpenses => {
                                max_operational_expenses
                                    .textarea
                                    .insert_str(app.yank_buffer.clone());
                            }
                            SelectedBlockQueryBusinessListings::SortBy => {
                                sort_by.textarea.insert_str(app.yank_buffer.clone());
                            }
                            SelectedBlockQueryBusinessListings::SortDirection => {
                                sort_direction.textarea.insert_str(app.yank_buffer.clone());
                            }
                            SelectedBlockQueryBusinessListings::Limit => {
                                limit.textarea.insert_str(app.yank_buffer.clone());
                            }
                            SelectedBlockQueryBusinessListings::Offset => {
                                offset.textarea.insert_str(app.yank_buffer.clone());
                            }
                        },
                        SelectedService::AcquireBusinessListing {
                            selected: _,
                            business_listing_uuid,
                        } => {
                            business_listing_uuid
                                .textarea
                                .insert_str(app.yank_buffer.clone());
                        }
                    }
                }
            }
            KeyCode::Esc => app.current_screen = CurrentScreen::Main(CurrentScreenMain::Services),
            KeyCode::Tab => {
                if let Some(selected_service) = &mut app.maybe_selected_service {
                    match selected_service {
                        SelectedService::Register { selected, .. } => selected.advance(),
                        SelectedService::VerifyRegistration { selected, .. } => selected.advance(),
                        SelectedService::ResendVerification { selected, .. } => selected.advance(),
                        SelectedService::Login { selected, .. } => selected.advance(),
                        SelectedService::CreateUser { selected, .. } => selected.advance(),
                        SelectedService::GetUser { selected, .. } => selected.advance(),
                        SelectedService::DeleteUser { selected, .. } => selected.advance(),
                        SelectedService::QueryBusinessListings { selected, .. } => {
                            selected.advance()
                        }
                        SelectedService::AcquireBusinessListing { selected, .. } => {
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
                        SelectedService::CreateUser { selected, .. } => selected.previous(),
                        SelectedService::GetUser { selected, .. } => selected.previous(),
                        SelectedService::DeleteUser { selected, .. } => selected.previous(),
                        SelectedService::QueryBusinessListings { selected, .. } => {
                            selected.previous()
                        }
                        SelectedService::AcquireBusinessListing { selected, .. } => {
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
                SelectedService::CreateUser {
                    selected,
                    user_name,
                    user_password,
                    user_email,
                    user_role,
                    corporation_name,
                } => match selected {
                    SelectedBlockCreateUser::UserName => {
                        user_name.textarea.input(event);
                    }
                    SelectedBlockCreateUser::UserPassword => {
                        user_password.textarea.input(event);
                    }
                    SelectedBlockCreateUser::UserEmail => {
                        user_email.textarea.input(event);
                    }
                    SelectedBlockCreateUser::UserRole => {
                        user_role.textarea.input(event);
                    }
                    SelectedBlockCreateUser::CorporationName => {
                        corporation_name.textarea.input(event);
                    }
                },
                SelectedService::GetUser {
                    selected,
                    user_uuid,
                } => match selected {
                    SelectedBlockGetUser::UserUuid => {
                        user_uuid.textarea.input(event);
                    }
                },
                SelectedService::DeleteUser {
                    selected,
                    user_uuid,
                } => match selected {
                    SelectedBlockDeleteUser::UserUuid => {
                        user_uuid.textarea.input(event);
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
                SelectedService::AcquireBusinessListing {
                    selected,
                    business_listing_uuid,
                } => match selected {
                    SelectedBlockAcquireBusinessListing::BusinessListingUuid => {
                        business_listing_uuid.textarea.input(event);
                    }
                },
            },
        };
    }

    Ok(())
}
