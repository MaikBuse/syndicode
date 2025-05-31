use crate::presentation::theme::{
    ACCENT_DARK_PURPLE, CYBER_BG, CYBER_FG, CYBER_YELLOW, INPUT_AREA_BG,
};

use super::{
    selected_block::{
        SelectedBlockAcquireBusinessListing, SelectedBlockCreateUser, SelectedBlockDeleteUser,
        SelectedBlockGetUser, SelectedBlockLogin, SelectedBlockQueryBusinessListings,
        SelectedBlockRegister, SelectedBlockResend, SelectedBlockVerify,
    },
    service_list::ServiceAction,
};
use ratatui::{
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, BorderType, Borders},
};
use tui_textarea::TextArea;

const TEXTAREA_STYLE: Style = Style {
    fg: Some(CYBER_FG),
    bg: Some(INPUT_AREA_BG),
    underline_color: Some(Color::Reset),
    add_modifier: Modifier::empty(),
    sub_modifier: Modifier::UNDERLINED,
};
const VISIBLE_CURSOR_STYLE: Style = Style::new().add_modifier(Modifier::REVERSED);
const HIDDEN_CURSOR_STYLE: Style = Style::new().bg(INPUT_AREA_BG);
const SELECTED_INPUT_TITLE_STYLE: Style = Style::new()
    .fg(CYBER_YELLOW)
    .bg(ACCENT_DARK_PURPLE)
    .add_modifier(Modifier::BOLD);
const SELECTED_INPUT_BORDER_STYLE: Style = Style::new().fg(CYBER_YELLOW);
const UNSELECTED_INPUT_BORDER_STYLE: Style = Style::new().fg(CYBER_FG).add_modifier(Modifier::DIM);
const INPUT_BLOCK_TITLE_STYLE: Style = Style::new().fg(CYBER_FG).add_modifier(Modifier::DIM);
const POPUP_BACKGROUND_COLOR: Color = CYBER_BG;

#[derive(Debug)]
pub struct SelectedServiceData<'a, B>
where
    B: PartialEq,
{
    pub textarea: TextArea<'a>,
    pub title: &'static str,
    pub block: B,
}

impl<B> SelectedServiceData<'_, B>
where
    B: PartialEq,
{
    fn new(title: &'static str, placeholder: &'static str, block: B, mask_field: bool) -> Self {
        let mut textarea = TextArea::default();
        textarea.set_placeholder_text(placeholder);
        textarea.set_style(TEXTAREA_STYLE);
        textarea.set_cursor_line_style(Style::default());

        if mask_field {
            textarea.set_mask_char('\u{2022}');
        }

        Self {
            textarea,
            title,
            block,
        }
    }

    pub fn update_textarea(&mut self, currently_selected: B) {
        let is_selected = currently_selected == self.block;

        self.textarea.set_cursor_style(if is_selected {
            VISIBLE_CURSOR_STYLE
        } else {
            HIDDEN_CURSOR_STYLE
        });

        self.textarea
            .set_block(self.create_input_block(self.title, is_selected));
    }

    fn create_input_block(&self, title: &'static str, is_selected: bool) -> Block<'static> {
        let title_style = if is_selected {
            SELECTED_INPUT_TITLE_STYLE
        } else {
            INPUT_BLOCK_TITLE_STYLE
        };
        let border_style = if is_selected {
            SELECTED_INPUT_BORDER_STYLE
        } else {
            UNSELECTED_INPUT_BORDER_STYLE
        };

        let title_span = Line::from(ratatui::text::Span::styled(title, title_style));

        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .border_style(border_style)
            .title(title_span)
            .style(Style::default().bg(TEXTAREA_STYLE.bg.unwrap_or(POPUP_BACKGROUND_COLOR)))
    }
}

#[derive(Debug)]
pub enum SelectedService<'a> {
    Register {
        selected: SelectedBlockRegister,
        user_name: SelectedServiceData<'a, SelectedBlockRegister>,
        user_password: SelectedServiceData<'a, SelectedBlockRegister>,
        corporation_name: SelectedServiceData<'a, SelectedBlockRegister>,
        email: SelectedServiceData<'a, SelectedBlockRegister>,
    },
    VerifyRegistration {
        selected: SelectedBlockVerify,
        user_name: SelectedServiceData<'a, SelectedBlockVerify>,
        code: SelectedServiceData<'a, SelectedBlockVerify>,
    },
    ResendVerification {
        selected: SelectedBlockResend,
        user_name: SelectedServiceData<'a, SelectedBlockResend>,
    },
    Login {
        selected: SelectedBlockLogin,
        user_name: SelectedServiceData<'a, SelectedBlockLogin>,
        user_password: SelectedServiceData<'a, SelectedBlockLogin>,
    },
    CreateUser {
        selected: SelectedBlockCreateUser,
        user_name: SelectedServiceData<'a, SelectedBlockCreateUser>,
        user_password: SelectedServiceData<'a, SelectedBlockCreateUser>,
        user_email: SelectedServiceData<'a, SelectedBlockCreateUser>,
        user_role: SelectedServiceData<'a, SelectedBlockCreateUser>,
        corporation_name: SelectedServiceData<'a, SelectedBlockCreateUser>,
    },
    GetUser {
        selected: SelectedBlockGetUser,
        user_uuid: SelectedServiceData<'a, SelectedBlockGetUser>,
    },
    DeleteUser {
        selected: SelectedBlockDeleteUser,
        user_uuid: SelectedServiceData<'a, SelectedBlockDeleteUser>,
    },
    QueryBusinessListings {
        selected: SelectedBlockQueryBusinessListings,
        min_asking_price: SelectedServiceData<'a, SelectedBlockQueryBusinessListings>,
        max_asking_price: SelectedServiceData<'a, SelectedBlockQueryBusinessListings>,
        seller_corporation_uuid: SelectedServiceData<'a, SelectedBlockQueryBusinessListings>,
        market_uuid: SelectedServiceData<'a, SelectedBlockQueryBusinessListings>,
        min_operational_expenses: SelectedServiceData<'a, SelectedBlockQueryBusinessListings>,
        max_operational_expenses: SelectedServiceData<'a, SelectedBlockQueryBusinessListings>,
        sort_by: SelectedServiceData<'a, SelectedBlockQueryBusinessListings>,
        sort_direction: SelectedServiceData<'a, SelectedBlockQueryBusinessListings>,
        limit: SelectedServiceData<'a, SelectedBlockQueryBusinessListings>,
        offset: SelectedServiceData<'a, SelectedBlockQueryBusinessListings>,
    },
    AcquireBusinessListing {
        selected: SelectedBlockAcquireBusinessListing,
        business_listing_uuid: SelectedServiceData<'a, SelectedBlockAcquireBusinessListing>,
    },
}

impl From<ServiceAction> for SelectedService<'_> {
    fn from(value: ServiceAction) -> Self {
        match value {
            ServiceAction::Register => Self::Register {
                selected: SelectedBlockRegister::default(),
                user_name: SelectedServiceData::new(
                    "Username",
                    "Mc_Lovin",
                    SelectedBlockRegister::UserName,
                    false,
                ),
                user_password: SelectedServiceData::new(
                    "Password",
                    "my-secret-password",
                    SelectedBlockRegister::UserPassword,
                    true,
                ),
                corporation_name: SelectedServiceData::new(
                    "Corporation Name",
                    "Lima Hammersmith Inc.",
                    SelectedBlockRegister::CorporationName,
                    false,
                ),
                email: SelectedServiceData::new(
                    "Email",
                    "name@domain.com",
                    SelectedBlockRegister::Email,
                    false,
                ),
            },
            ServiceAction::VerifyRegistration => Self::VerifyRegistration {
                selected: SelectedBlockVerify::default(),
                user_name: SelectedServiceData::new(
                    "Username",
                    "Mc_Lovin",
                    SelectedBlockVerify::UserName,
                    false,
                ),
                code: SelectedServiceData::new(
                    "Code",
                    "8FA3FLI91",
                    SelectedBlockVerify::Code,
                    false,
                ),
            },
            ServiceAction::ResendVerification => Self::ResendVerification {
                selected: SelectedBlockResend::default(),
                user_name: SelectedServiceData::new(
                    "Username",
                    "Mc_Lovin",
                    SelectedBlockResend::UserName,
                    false,
                ),
            },
            ServiceAction::Login => Self::Login {
                selected: SelectedBlockLogin::default(),
                user_name: SelectedServiceData::new(
                    "Username",
                    "Mc_Lovin",
                    SelectedBlockLogin::UserName,
                    false,
                ),
                user_password: SelectedServiceData::new(
                    "Password",
                    "my-secret-password",
                    SelectedBlockLogin::UserPassword,
                    true,
                ),
            },
            ServiceAction::CreateUser => Self::CreateUser {
                selected: SelectedBlockCreateUser::default(),
                user_name: SelectedServiceData::new(
                    "Username",
                    "Mc_Lovin",
                    SelectedBlockCreateUser::UserName,
                    false,
                ),
                user_password: SelectedServiceData::new(
                    "Password",
                    "my-secret-password",
                    SelectedBlockCreateUser::UserPassword,
                    true,
                ),
                user_email: SelectedServiceData::new(
                    "Email",
                    "name@domain.com",
                    SelectedBlockCreateUser::UserEmail,
                    false,
                ),
                user_role: SelectedServiceData::new(
                    "Role",
                    "1 (Admin) or 2 (Player)",
                    SelectedBlockCreateUser::UserRole,
                    false,
                ),
                corporation_name: SelectedServiceData::new(
                    "Corporation Name",
                    "Lima Hammersmith Inc.",
                    SelectedBlockCreateUser::CorporationName,
                    false,
                ),
            },
            ServiceAction::DeleteUser => Self::DeleteUser {
                selected: SelectedBlockDeleteUser::default(),
                user_uuid: SelectedServiceData::new(
                    "User UUID",
                    "7a520b51-ad88-446c-84d6-80de0ed99230",
                    SelectedBlockDeleteUser::UserUuid,
                    false,
                ),
            },
            ServiceAction::QueryBusinessListings => Self::QueryBusinessListings {
                selected: SelectedBlockQueryBusinessListings::default(),
                min_asking_price: SelectedServiceData::new(
                    "Min. Asking Price",
                    "1000",
                    SelectedBlockQueryBusinessListings::MinAskingPrice,
                    false,
                ),
                max_asking_price: SelectedServiceData::new(
                    "Max. Asking Price",
                    "2000",
                    SelectedBlockQueryBusinessListings::MaxAskingPrice,
                    false,
                ),
                seller_corporation_uuid: SelectedServiceData::new(
                    "Seller Corporation UUID",
                    "0196e20b-c252-7520-ae13-935b5d5f0029",
                    SelectedBlockQueryBusinessListings::SellerCorporationUuid,
                    false,
                ),
                market_uuid: SelectedServiceData::new(
                    "Market UUID",
                    "0196e24f-eda1-7145-a177-8d2f8c38f7c4",
                    SelectedBlockQueryBusinessListings::MarketUuid,
                    false,
                ),
                min_operational_expenses: SelectedServiceData::new(
                    "Min. Operational Expenses",
                    "10",
                    SelectedBlockQueryBusinessListings::MinOperationalExpenses,
                    false,
                ),
                max_operational_expenses: SelectedServiceData::new(
                    "Max. Operational Expenses",
                    "30",
                    SelectedBlockQueryBusinessListings::MaxOperationalExpenses,
                    false,
                ),
                sort_by: SelectedServiceData::new(
                    "Sort By",
                    "price, name, op_expenses, market_volume",
                    SelectedBlockQueryBusinessListings::SortBy,
                    false,
                ),
                sort_direction: SelectedServiceData::new(
                    "Sort Direction",
                    "0 or 1",
                    SelectedBlockQueryBusinessListings::SortDirection,
                    false,
                ),
                limit: SelectedServiceData::new(
                    "Limit",
                    "20",
                    SelectedBlockQueryBusinessListings::Limit,
                    false,
                ),
                offset: SelectedServiceData::new(
                    "Offset",
                    "50",
                    SelectedBlockQueryBusinessListings::Offset,
                    false,
                ),
            },
            ServiceAction::GetUser => Self::GetUser {
                selected: SelectedBlockGetUser::default(),
                user_uuid: SelectedServiceData::new(
                    "User UUID",
                    "7a520b51-ad88-446c-84d6-80de0ed99230",
                    SelectedBlockGetUser::UserUuid,
                    false,
                ),
            },
            _ => Self::AcquireBusinessListing {
                selected: SelectedBlockAcquireBusinessListing::default(),
                business_listing_uuid: SelectedServiceData::new(
                    "Business listing UUID",
                    "01970208-57a6-712a-a9c1-497e4e71f764",
                    SelectedBlockAcquireBusinessListing::BusinessListingUuid,
                    false,
                ),
            },
        }
    }
}
