use super::utils::centered_rect;
use crate::presentation::{
    theme::{ACCENT_DARK_PURPLE, CYBER_BG, CYBER_FG, CYBER_PINK, CYBER_YELLOW, INPUT_AREA_BG},
    SelectedService,
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Clear, Widget},
};
use tui_textarea::TextArea;

// --- Styling Constants ---
const POPUP_BACKGROUND_COLOR: Color = CYBER_BG;
const POPUP_BORDER_COLOR: Color = CYBER_PINK;
const POPUP_TITLE_STYLE: Style = Style::new()
    .fg(CYBER_YELLOW)
    .bg(ACCENT_DARK_PURPLE)
    .add_modifier(Modifier::BOLD);

const INPUT_BLOCK_TITLE_STYLE: Style = Style::new().fg(CYBER_FG).add_modifier(Modifier::DIM);
const SELECTED_INPUT_BORDER_STYLE: Style = Style::new().fg(CYBER_YELLOW);
const SELECTED_INPUT_TITLE_STYLE: Style = Style::new()
    .fg(CYBER_YELLOW)
    .bg(ACCENT_DARK_PURPLE)
    .add_modifier(Modifier::BOLD);
const UNSELECTED_INPUT_BORDER_STYLE: Style = Style::new().fg(CYBER_FG).add_modifier(Modifier::DIM);
const TEXTAREA_STYLE: Style = Style::new().fg(CYBER_FG).bg(INPUT_AREA_BG);

const VISIBLE_CURSOR_STYLE: Style = Style::new().add_modifier(Modifier::REVERSED);
const HIDDEN_CURSOR_STYLE: Style = Style::new().bg(INPUT_AREA_BG); // Use the textarea's background

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SelectedBlockRegister {
    #[default]
    UserName,
    UserPassword,
    Email,
    CorporationName,
}

impl SelectedBlockRegister {
    pub fn advance(&mut self) {
        *self = match self {
            SelectedBlockRegister::UserName => Self::UserPassword,
            SelectedBlockRegister::UserPassword => Self::Email,
            SelectedBlockRegister::Email => Self::CorporationName,
            SelectedBlockRegister::CorporationName => Self::UserName,
        };
    }

    pub fn previous(&mut self) {
        *self = match self {
            SelectedBlockRegister::UserName => Self::CorporationName,
            SelectedBlockRegister::UserPassword => Self::UserName,
            SelectedBlockRegister::Email => Self::UserPassword,
            SelectedBlockRegister::CorporationName => Self::Email,
        };
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SelectedBlockVerify {
    #[default]
    UserName,
    Code,
}

impl SelectedBlockVerify {
    pub fn advance(&mut self) {
        *self = match self {
            SelectedBlockVerify::UserName => Self::Code,
            SelectedBlockVerify::Code => Self::UserName,
        };
    }

    pub fn previous(&mut self) {
        *self = match self {
            SelectedBlockVerify::UserName => Self::Code,
            SelectedBlockVerify::Code => Self::UserName,
        };
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SelectedBlockResend {
    #[default]
    UserName,
}

impl SelectedBlockResend {
    pub fn advance(&mut self) {
        *self = match self {
            SelectedBlockResend::UserName => Self::UserName,
        };
    }

    pub fn previous(&mut self) {
        *self = match self {
            SelectedBlockResend::UserName => Self::UserName,
        };
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SelectedBlockLogin {
    #[default]
    UserName,
    UserPassword,
}

impl SelectedBlockLogin {
    pub fn advance(&mut self) {
        *self = match self {
            SelectedBlockLogin::UserName => Self::UserPassword,
            SelectedBlockLogin::UserPassword => Self::UserName,
        };
    }

    pub fn previous(&mut self) {
        *self = match self {
            SelectedBlockLogin::UserName => Self::UserPassword,
            SelectedBlockLogin::UserPassword => Self::UserName,
        };
    }
}

#[derive(Debug)]
pub struct EditingPopupWidget;

fn setup_textarea(textarea: &mut TextArea<'_>, title: &'static str, is_selected: bool) {
    textarea.set_style(TEXTAREA_STYLE);
    textarea.set_cursor_style(if is_selected {
        VISIBLE_CURSOR_STYLE
    } else {
        HIDDEN_CURSOR_STYLE
    });
    textarea.set_block(create_input_block(title, is_selected));
}

impl EditingPopupWidget {
    pub fn render(
        &self,
        frame_area: Rect,
        buf: &mut ratatui::prelude::Buffer,
        selected_service: &mut SelectedService,
    ) {
        let popup_area = centered_rect(25, 60, frame_area);

        Clear.render(popup_area, buf);

        let popup_title_text = match selected_service {
            SelectedService::Register { .. } => "Register New User",
            SelectedService::VerifyRegistration { .. } => "Verify a user's registration",
            SelectedService::ResendVerification { .. } => "Resend a user's verification code",
            _ => "Edit Request",
        };

        let popup_instructions = Line::from(vec![
            " Send ".fg(CYBER_FG),
            "<Enter> ".fg(CYBER_YELLOW).bold(),
        ]);

        let popup_block = Block::default()
            .title(Line::styled(popup_title_text, POPUP_TITLE_STYLE))
            .title_bottom(popup_instructions.centered())
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::default().fg(POPUP_BORDER_COLOR))
            .style(Style::default().bg(POPUP_BACKGROUND_COLOR));

        popup_block.clone().render(popup_area, buf);
        let inner_popup_area = popup_block.inner(popup_area);

        match selected_service {
            SelectedService::VerifyRegistration {
                selected,
                user_name_textarea,
                code_textarea,
            } => {
                let input_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([Constraint::Length(3), Constraint::Length(3)])
                    .split(inner_popup_area);

                setup_textarea(
                    user_name_textarea,
                    "Username",
                    *selected == SelectedBlockVerify::UserName,
                );
                setup_textarea(
                    code_textarea,
                    "Code",
                    *selected == SelectedBlockVerify::Code,
                );

                user_name_textarea.render(input_chunks[0], buf);
                code_textarea.render(input_chunks[1], buf);
            }
            SelectedService::Register {
                selected,
                user_name_textarea,
                user_password_textarea,
                email_textarea,
                corporation_name_textarea,
            } => {
                let input_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                    ])
                    .split(inner_popup_area);

                setup_textarea(
                    user_name_textarea,
                    "Username",
                    *selected == SelectedBlockRegister::UserName,
                );
                setup_textarea(
                    user_password_textarea,
                    "Password",
                    *selected == SelectedBlockRegister::UserPassword,
                );
                setup_textarea(
                    email_textarea,
                    "Email",
                    *selected == SelectedBlockRegister::Email,
                );
                setup_textarea(
                    corporation_name_textarea,
                    "Corporation",
                    *selected == SelectedBlockRegister::CorporationName,
                );

                user_name_textarea.render(input_chunks[0], buf);
                user_password_textarea.render(input_chunks[1], buf);
                email_textarea.render(input_chunks[2], buf);
                corporation_name_textarea.render(input_chunks[3], buf);
            }
            SelectedService::ResendVerification {
                selected,
                user_name_textarea,
            } => {
                let input_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Length(3)])
                    .split(inner_popup_area);

                setup_textarea(
                    user_name_textarea,
                    "Username",
                    *selected == SelectedBlockResend::UserName,
                );

                user_name_textarea.render(input_chunks[0], buf);
            }
            SelectedService::Login {
                selected,
                user_name_textarea,
                user_password_textarea,
            } => {
                let input_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Length(3), Constraint::Length(3)])
                    .split(inner_popup_area);

                setup_textarea(
                    user_name_textarea,
                    "Username",
                    *selected == SelectedBlockLogin::UserName,
                );
                setup_textarea(
                    user_password_textarea,
                    "Password",
                    *selected == SelectedBlockLogin::UserPassword,
                );

                user_name_textarea.render(input_chunks[0], buf);
                user_password_textarea.render(input_chunks[1], buf);
            }
        }
    }
}

fn create_input_block(title: &'static str, is_selected: bool) -> Block<'static> {
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
