use crate::{
    domain::{
        admin::AdminRepository, auth::repository::AuthenticationRepository, game::GameRepository,
    },
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
use crossterm::event::{Event, KeyCode, KeyModifiers};

use syndicode_proto::{
    syndicode_economy_v1::BusinessListingSortBy,
    syndicode_interface_v1::{SortDirection, UserRole},
};
use uuid::Uuid;

use super::utils::from_crossterm_into_ratatui;

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
        if !key_event.is_press() {
            return Ok(());
        }
        match key_event.code {
            KeyCode::Enter => {
                match selected_service {
                    SelectedService::QueryBusinessListings(data) => {
                        let maybe_min_asking_price: Option<i64> = data
                            .min_asking_price
                            .textarea
                            .lines()
                            .first()
                            .and_then(|x| match x.is_empty() {
                                true => None,
                                false => Some(x.parse::<i64>().unwrap()),
                            });
                        let maybe_max_asking_price: Option<i64> = data
                            .max_asking_price
                            .textarea
                            .lines()
                            .first()
                            .and_then(|x| match x.is_empty() {
                                true => None,
                                false => Some(x.parse::<i64>().unwrap()),
                            });
                        let maybe_seller_corporation_uuid: Option<String> = data
                            .seller_corporation_uuid
                            .textarea
                            .lines()
                            .first()
                            .and_then(|x| match x.is_empty() {
                                true => None,
                                false => Some(x.to_owned()),
                            });
                        let maybe_market_uuid: Option<String> =
                            data.market_uuid.textarea.lines().first().and_then(|x| {
                                match x.is_empty() {
                                    true => None,
                                    false => Some(x.to_owned()),
                                }
                            });
                        let maybe_min_operational_expenses: Option<i64> = data
                            .min_operational_expenses
                            .textarea
                            .lines()
                            .first()
                            .and_then(|x| match x.is_empty() {
                                true => None,
                                false => Some(x.parse::<i64>().unwrap()),
                            });
                        let maybe_max_operational_expenses: Option<i64> = data
                            .max_operational_expenses
                            .textarea
                            .lines()
                            .first()
                            .and_then(|x| match x.is_empty() {
                                true => None,
                                false => Some(x.parse::<i64>().unwrap()),
                            });

                        let sort_by = data
                            .sort_by
                            .textarea
                            .lines()
                            .first()
                            .and_then(|string| {
                                BusinessListingSortBy::from_str_name(string.as_str())
                            })
                            .map(|sort_by| sort_by as i32)
                            .unwrap_or_default();
                        let sort_direction = data
                            .sort_direction
                            .textarea
                            .lines()
                            .first()
                            .and_then(|string| SortDirection::from_str_name(string.as_str()))
                            .map(|sort_dir| sort_dir as i32)
                            .unwrap_or_default();
                        let maybe_limit: Option<i64> = data
                            .limit
                            .textarea
                            .lines()
                            .first()
                            .and_then(|x| match x.is_empty() {
                                true => None,
                                false => Some(x.parse::<i64>().unwrap()),
                            });
                        let maybe_offset: Option<i64> = data
                            .offset
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
                    SelectedService::AcquireBusinessListing(data) => {
                        let business_listing_uuid = data
                            .business_listing_uuid
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
                    SelectedService::Register(data) => {
                        let user_name =
                            data.user_name.textarea.lines().first().ok_or_else(|| {
                                anyhow::anyhow!("Failed to retrieve user name from textarea")
                            })?;
                        let user_password =
                            data.user_password.textarea.lines().first().ok_or_else(|| {
                                anyhow::anyhow!("Failed to retrieve user password from textarea")
                            })?;
                        let email = data.email.textarea.lines().first().ok_or_else(|| {
                            anyhow::anyhow!("Failed to retrieve email from textarea")
                        })?;
                        let corporation_name = data
                            .corporation_name
                            .textarea
                            .lines()
                            .first()
                            .ok_or_else(|| {
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
                    SelectedService::VerifyRegistration(data) => {
                        let user_name =
                            data.user_name.textarea.lines().first().ok_or_else(|| {
                                anyhow::anyhow!("Failed to retrieve user name from textarea")
                            })?;
                        let code = data.code.textarea.lines().first().ok_or_else(|| {
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
                    SelectedService::ResendVerification(data) => {
                        let user_name =
                            data.user_name.textarea.lines().first().ok_or_else(|| {
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
                    SelectedService::Login(data) => {
                        let user_name =
                            data.user_name.textarea.lines().first().ok_or_else(|| {
                                anyhow::anyhow!("Failed to retrieve user name from textarea")
                            })?;
                        let user_password =
                            data.user_password.textarea.lines().first().ok_or_else(|| {
                                anyhow::anyhow!("Failed to retrieve user password from textarea")
                            })?;

                        let result = app
                            .login_uc
                            .execute()
                            .user_name(user_name.to_owned())
                            .user_password(user_password.to_owned())
                            .call()
                            .await;

                        if let Ok(login_response) = &result {
                            app.maybe_token = Some(login_response.jwt.clone());
                            app.maybe_username = Some(user_name.to_owned());

                            let categories = default_services()
                                .is_stream_active(false)
                                .is_logged_in(true)
                                .call();
                            app.service_list_widget = ServiceListWidget::new(categories);
                        }

                        app.maybe_selected_service = None;
                        app.response_list_widget.push(result.into());

                        app.current_screen = CurrentScreen::Main(CurrentScreenMain::Services);
                    }
                    SelectedService::CreateUser(data) => {
                        let user_name =
                            data.user_name.textarea.lines().first().ok_or_else(|| {
                                anyhow::anyhow!("Failed to retrieve user name from textarea")
                            })?;
                        let user_password =
                            data.user_password.textarea.lines().first().ok_or_else(|| {
                                anyhow::anyhow!("Failed to retrieve user password from textarea")
                            })?;
                        let user_email =
                            data.user_email.textarea.lines().first().ok_or_else(|| {
                                anyhow::anyhow!("Failed to retrieve email from textarea")
                            })?;
                        let user_role = data
                            .user_role
                            .textarea
                            .lines()
                            .first()
                            .and_then(|string| UserRole::from_str_name(string.as_str()))
                            .map(|user_role| user_role as i32)
                            .unwrap_or(2);
                        let corporation_name = data
                            .corporation_name
                            .textarea
                            .lines()
                            .first()
                            .ok_or_else(|| {
                                anyhow::anyhow!("Failed to retrieve corporation name from textarea")
                            })?;

                        let request_uuid = Uuid::now_v7();

                        let response = app
                            .create_user_uc
                            .execute()
                            .request_uuid(request_uuid.to_string())
                            .token(app.maybe_token.clone().unwrap_or_default())
                            .user_name(user_name.to_owned())
                            .user_password(user_password.to_owned())
                            .user_email(user_email.to_owned())
                            .user_role(user_role)
                            .corporation_name(corporation_name.to_owned())
                            .call()
                            .await;

                        app.response_list_widget.push(response.into());
                        app.maybe_selected_service = None;
                        app.current_screen = CurrentScreen::Main(CurrentScreenMain::Services);
                    }
                    SelectedService::GetUser(data) => {
                        let user_uuid =
                            data.user_uuid.textarea.lines().first().ok_or_else(|| {
                                anyhow::anyhow!("Failed to retrieve user uuid from textarea")
                            })?;

                        let response = app
                            .get_user_uc
                            .execute()
                            .token(app.maybe_token.clone().unwrap_or_default())
                            .user_uuid(user_uuid.to_owned())
                            .call()
                            .await;

                        app.response_list_widget.push(response.into());
                        app.maybe_selected_service = None;
                        app.current_screen = CurrentScreen::Main(CurrentScreenMain::Services);
                    }
                    SelectedService::DeleteUser(data) => {
                        let user_uuid =
                            data.user_uuid.textarea.lines().first().ok_or_else(|| {
                                anyhow::anyhow!("Failed to retrieve user uuid from textarea")
                            })?;

                        let request_uuid = Uuid::now_v7().to_string();

                        let response = app
                            .delete_user_uc
                            .execute()
                            .token(app.maybe_token.clone().unwrap_or_default())
                            .request_uuid(request_uuid)
                            .user_uuid(user_uuid.to_owned())
                            .call()
                            .await;

                        app.response_list_widget.push(response.into());
                        app.maybe_selected_service = None;
                        app.current_screen = CurrentScreen::Main(CurrentScreenMain::Services);
                    }
                };
            }
            KeyCode::Char('p') if key_event.modifiers.contains(KeyModifiers::ALT) => {
                if let Some(selected_service) = &mut app.maybe_selected_service {
                    match selected_service {
                        SelectedService::Register(data) => match data.selected {
                            SelectedBlockRegister::UserName => {
                                data.user_name.textarea.insert_str(app.yank_buffer.clone());
                            }
                            SelectedBlockRegister::UserPassword => {
                                data.user_password
                                    .textarea
                                    .insert_str(app.yank_buffer.clone());
                            }
                            SelectedBlockRegister::Email => {
                                data.email.textarea.insert_str(app.yank_buffer.clone());
                            }
                            SelectedBlockRegister::CorporationName => {
                                data.corporation_name
                                    .textarea
                                    .insert_str(app.yank_buffer.clone());
                            }
                        },
                        SelectedService::VerifyRegistration(data) => match data.selected {
                            SelectedBlockVerify::UserName => {
                                data.user_name.textarea.insert_str(app.yank_buffer.clone());
                            }
                            SelectedBlockVerify::Code => {
                                data.code.textarea.insert_str(app.yank_buffer.clone());
                            }
                        },
                        SelectedService::ResendVerification(data) => match data.selected {
                            SelectedBlockResend::UserName => {
                                data.user_name.textarea.insert_str(app.yank_buffer.clone());
                            }
                        },
                        SelectedService::Login(data) => match data.selected {
                            SelectedBlockLogin::UserName => {
                                data.user_name.textarea.insert_str(app.yank_buffer.clone());
                            }
                            SelectedBlockLogin::UserPassword => {
                                data.user_password
                                    .textarea
                                    .insert_str(app.yank_buffer.clone());
                            }
                        },
                        SelectedService::CreateUser(data) => match data.selected {
                            SelectedBlockCreateUser::UserName => {
                                data.user_name.textarea.insert_str(app.yank_buffer.clone());
                            }
                            SelectedBlockCreateUser::UserPassword => {
                                data.user_password
                                    .textarea
                                    .insert_str(app.yank_buffer.clone());
                            }
                            SelectedBlockCreateUser::UserEmail => {
                                data.user_email.textarea.insert_str(app.yank_buffer.clone());
                            }
                            SelectedBlockCreateUser::UserRole => {
                                data.user_role.textarea.insert_str(app.yank_buffer.clone());
                            }
                            SelectedBlockCreateUser::CorporationName => {
                                data.corporation_name
                                    .textarea
                                    .insert_str(app.yank_buffer.clone());
                            }
                        },
                        SelectedService::GetUser(data) => {
                            data.user_uuid.textarea.insert_str(app.yank_buffer.clone());
                        }
                        SelectedService::DeleteUser(data) => {
                            data.user_uuid.textarea.insert_str(app.yank_buffer.clone());
                        }
                        SelectedService::QueryBusinessListings(data) => match data.selected {
                            SelectedBlockQueryBusinessListings::MinAskingPrice => {
                                data.min_asking_price
                                    .textarea
                                    .insert_str(app.yank_buffer.clone());
                            }
                            SelectedBlockQueryBusinessListings::MaxAskingPrice => {
                                data.max_asking_price
                                    .textarea
                                    .insert_str(app.yank_buffer.clone());
                            }
                            SelectedBlockQueryBusinessListings::SellerCorporationUuid => {
                                data.seller_corporation_uuid
                                    .textarea
                                    .insert_str(app.yank_buffer.clone());
                            }
                            SelectedBlockQueryBusinessListings::MarketUuid => {
                                data.market_uuid
                                    .textarea
                                    .insert_str(app.yank_buffer.clone());
                            }
                            SelectedBlockQueryBusinessListings::MinOperationalExpenses => {
                                data.min_operational_expenses
                                    .textarea
                                    .insert_str(app.yank_buffer.clone());
                            }
                            SelectedBlockQueryBusinessListings::MaxOperationalExpenses => {
                                data.max_operational_expenses
                                    .textarea
                                    .insert_str(app.yank_buffer.clone());
                            }
                            SelectedBlockQueryBusinessListings::SortBy => {
                                data.sort_by.textarea.insert_str(app.yank_buffer.clone());
                            }
                            SelectedBlockQueryBusinessListings::SortDirection => {
                                data.sort_direction
                                    .textarea
                                    .insert_str(app.yank_buffer.clone());
                            }
                            SelectedBlockQueryBusinessListings::Limit => {
                                data.limit.textarea.insert_str(app.yank_buffer.clone());
                            }
                            SelectedBlockQueryBusinessListings::Offset => {
                                data.offset.textarea.insert_str(app.yank_buffer.clone());
                            }
                        },
                        SelectedService::AcquireBusinessListing(data) => {
                            data.business_listing_uuid
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
                        SelectedService::Register(data) => data.selected.advance(),
                        SelectedService::VerifyRegistration(data) => data.selected.advance(),
                        SelectedService::ResendVerification(data) => data.selected.advance(),
                        SelectedService::Login(data) => data.selected.advance(),
                        SelectedService::CreateUser(data) => data.selected.advance(),
                        SelectedService::GetUser(data) => data.selected.advance(),
                        SelectedService::DeleteUser(data) => data.selected.advance(),
                        SelectedService::QueryBusinessListings(data) => data.selected.advance(),
                        SelectedService::AcquireBusinessListing(data) => data.selected.advance(),
                    }
                };
            }
            KeyCode::BackTab => {
                if let Some(selected_service) = &mut app.maybe_selected_service {
                    match selected_service {
                        SelectedService::Register(data) => data.selected.previous(),
                        SelectedService::VerifyRegistration(data) => data.selected.previous(),
                        SelectedService::ResendVerification(data) => data.selected.previous(),
                        SelectedService::Login(data) => data.selected.previous(),
                        SelectedService::CreateUser(data) => data.selected.previous(),
                        SelectedService::GetUser(data) => data.selected.previous(),
                        SelectedService::DeleteUser(data) => data.selected.previous(),
                        SelectedService::QueryBusinessListings(data) => data.selected.previous(),
                        SelectedService::AcquireBusinessListing(data) => data.selected.previous(),
                    }
                };
            }
            _ => match selected_service {
                SelectedService::Register(data) => match data.selected {
                    SelectedBlockRegister::UserName => {
                        if let Ok(event) = from_crossterm_into_ratatui(event) {
                            data.user_name.textarea.input(event);
                        }
                    }
                    SelectedBlockRegister::UserPassword => {
                        if let Ok(event) = from_crossterm_into_ratatui(event) {
                            data.user_password.textarea.input(event);
                        }
                    }
                    SelectedBlockRegister::Email => {
                        if let Ok(event) = from_crossterm_into_ratatui(event) {
                            data.email.textarea.input(event);
                        }
                    }
                    SelectedBlockRegister::CorporationName => {
                        if let Ok(event) = from_crossterm_into_ratatui(event) {
                            data.corporation_name.textarea.input(event);
                        }
                    }
                },
                SelectedService::VerifyRegistration(data) => match data.selected {
                    SelectedBlockVerify::UserName => {
                        if let Ok(event) = from_crossterm_into_ratatui(event) {
                            data.user_name.textarea.input(event);
                        }
                    }
                    SelectedBlockVerify::Code => {
                        if let Ok(event) = from_crossterm_into_ratatui(event) {
                            data.code.textarea.input(event);
                        }
                    }
                },
                SelectedService::ResendVerification(data) => match data.selected {
                    SelectedBlockResend::UserName => {
                        if let Ok(event) = from_crossterm_into_ratatui(event) {
                            data.user_name.textarea.input(event);
                        }
                    }
                },
                SelectedService::Login(data) => match data.selected {
                    SelectedBlockLogin::UserName => {
                        if let Ok(event) = from_crossterm_into_ratatui(event) {
                            data.user_name.textarea.input(event);
                        }
                    }
                    SelectedBlockLogin::UserPassword => {
                        if let Ok(event) = from_crossterm_into_ratatui(event) {
                            data.user_password.textarea.input(event);
                        }
                    }
                },
                SelectedService::CreateUser(data) => match data.selected {
                    SelectedBlockCreateUser::UserName => {
                        if let Ok(event) = from_crossterm_into_ratatui(event) {
                            data.user_name.textarea.input(event);
                        }
                    }
                    SelectedBlockCreateUser::UserPassword => {
                        if let Ok(event) = from_crossterm_into_ratatui(event) {
                            data.user_password.textarea.input(event);
                        }
                    }
                    SelectedBlockCreateUser::UserEmail => {
                        if let Ok(event) = from_crossterm_into_ratatui(event) {
                            data.user_email.textarea.input(event);
                        }
                    }
                    SelectedBlockCreateUser::UserRole => {
                        if let Ok(event) = from_crossterm_into_ratatui(event) {
                            data.user_role.textarea.input(event);
                        }
                    }
                    SelectedBlockCreateUser::CorporationName => {
                        if let Ok(event) = from_crossterm_into_ratatui(event) {
                            data.corporation_name.textarea.input(event);
                        }
                    }
                },
                SelectedService::GetUser(data) => match data.selected {
                    SelectedBlockGetUser::UserUuid => {
                        if let Ok(event) = from_crossterm_into_ratatui(event) {
                            data.user_uuid.textarea.input(event);
                        }
                    }
                },
                SelectedService::DeleteUser(data) => match data.selected {
                    SelectedBlockDeleteUser::UserUuid => {
                        if let Ok(event) = from_crossterm_into_ratatui(event) {
                            data.user_uuid.textarea.input(event);
                        }
                    }
                },
                SelectedService::QueryBusinessListings(data) => match data.selected {
                    SelectedBlockQueryBusinessListings::MinAskingPrice => {
                        if let Ok(event) = from_crossterm_into_ratatui(event) {
                            data.min_asking_price.textarea.input(event);
                        }
                    }
                    SelectedBlockQueryBusinessListings::MaxAskingPrice => {
                        if let Ok(event) = from_crossterm_into_ratatui(event) {
                            data.max_asking_price.textarea.input(event);
                        }
                    }
                    SelectedBlockQueryBusinessListings::SellerCorporationUuid => {
                        if let Ok(event) = from_crossterm_into_ratatui(event) {
                            data.seller_corporation_uuid.textarea.input(event);
                        }
                    }
                    SelectedBlockQueryBusinessListings::MarketUuid => {
                        if let Ok(event) = from_crossterm_into_ratatui(event) {
                            data.market_uuid.textarea.input(event);
                        }
                    }
                    SelectedBlockQueryBusinessListings::MinOperationalExpenses => {
                        if let Ok(event) = from_crossterm_into_ratatui(event) {
                            data.min_operational_expenses.textarea.input(event);
                        }
                    }
                    SelectedBlockQueryBusinessListings::MaxOperationalExpenses => {
                        if let Ok(event) = from_crossterm_into_ratatui(event) {
                            data.max_operational_expenses.textarea.input(event);
                        }
                    }
                    SelectedBlockQueryBusinessListings::SortBy => {
                        if let Ok(event) = from_crossterm_into_ratatui(event) {
                            data.sort_by.textarea.input(event);
                        }
                    }
                    SelectedBlockQueryBusinessListings::SortDirection => {
                        if let Ok(event) = from_crossterm_into_ratatui(event) {
                            data.sort_direction.textarea.input(event);
                        }
                    }
                    SelectedBlockQueryBusinessListings::Limit => {
                        if let Ok(event) = from_crossterm_into_ratatui(event) {
                            data.limit.textarea.input(event);
                        }
                    }
                    SelectedBlockQueryBusinessListings::Offset => {
                        if let Ok(event) = from_crossterm_into_ratatui(event) {
                            data.offset.textarea.input(event);
                        }
                    }
                },
                SelectedService::AcquireBusinessListing(data) => match data.selected {
                    SelectedBlockAcquireBusinessListing::BusinessListingUuid => {
                        if let Ok(event) = from_crossterm_into_ratatui(event) {
                            data.business_listing_uuid.textarea.input(event);
                        }
                    }
                },
            },
        };
    }

    Ok(())
}
