use super::selected_service::SelectedService;
use crate::presentation::{
    theme::{ACCENT_DARK_PURPLE, CYBER_BG, CYBER_FG, CYBER_PINK, CYBER_YELLOW},
    widget::utils::centered_rect,
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Clear, Widget},
};

const SINGE_LINE_CONSTRAINT: Constraint = Constraint::Length(3);

// --- Styling Constants ---
const POPUP_BACKGROUND_COLOR: Color = CYBER_BG;
const POPUP_BORDER_COLOR: Color = CYBER_PINK;
const POPUP_TITLE_STYLE: Style = Style::new()
    .fg(CYBER_YELLOW)
    .bg(ACCENT_DARK_PURPLE)
    .add_modifier(Modifier::BOLD);

#[derive(Debug)]
pub struct ServiceDetailWidget;
impl ServiceDetailWidget {
    pub fn render(
        &self,
        frame_area: Rect,
        buf: &mut ratatui::prelude::Buffer,
        selected_service: &mut SelectedService,
    ) {
        let popup_area = centered_rect(25, 80, frame_area);

        Clear.render(popup_area, buf);

        let popup_title_text = match selected_service {
            SelectedService::Register(_) => "Register New User",
            SelectedService::VerifyRegistration(_) => "Verify a user's registration",
            SelectedService::ResendVerification(_) => "Resend a user's verification code",
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
            SelectedService::VerifyRegistration(data) => {
                let input_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([SINGE_LINE_CONSTRAINT, SINGE_LINE_CONSTRAINT])
                    .split(inner_popup_area);

                data.user_name.update_textarea(data.selected);
                data.user_name.textarea.render(input_chunks[0], buf);

                data.code.update_textarea(data.selected);
                data.code.textarea.render(input_chunks[1], buf);
            }
            SelectedService::Register(data) => {
                let input_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        SINGE_LINE_CONSTRAINT,
                        SINGE_LINE_CONSTRAINT,
                        SINGE_LINE_CONSTRAINT,
                        SINGE_LINE_CONSTRAINT,
                    ])
                    .split(inner_popup_area);

                data.user_name.update_textarea(data.selected);
                data.user_name.textarea.render(input_chunks[0], buf);

                data.user_password.update_textarea(data.selected);
                data.user_password.textarea.render(input_chunks[1], buf);

                data.email.update_textarea(data.selected);
                data.email.textarea.render(input_chunks[2], buf);

                data.corporation_name.update_textarea(data.selected);
                data.corporation_name.textarea.render(input_chunks[3], buf);
            }
            SelectedService::ResendVerification(data) => {
                let input_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([SINGE_LINE_CONSTRAINT])
                    .split(inner_popup_area);

                data.user_name.update_textarea(data.selected);
                data.user_name.textarea.render(input_chunks[0], buf);
            }
            SelectedService::Login(data) => {
                let input_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([SINGE_LINE_CONSTRAINT, SINGE_LINE_CONSTRAINT])
                    .split(inner_popup_area);

                data.user_name.update_textarea(data.selected);
                data.user_name.textarea.render(input_chunks[0], buf);

                data.user_password.update_textarea(data.selected);
                data.user_password.textarea.render(input_chunks[1], buf);
            }
            SelectedService::CreateUser(data) => {
                let input_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        SINGE_LINE_CONSTRAINT,
                        SINGE_LINE_CONSTRAINT,
                        SINGE_LINE_CONSTRAINT,
                        SINGE_LINE_CONSTRAINT,
                        SINGE_LINE_CONSTRAINT,
                    ])
                    .split(inner_popup_area);

                data.user_name.update_textarea(data.selected);
                data.user_name.textarea.render(input_chunks[0], buf);

                data.user_password.update_textarea(data.selected);
                data.user_password.textarea.render(input_chunks[1], buf);

                data.user_email.update_textarea(data.selected);
                data.user_email.textarea.render(input_chunks[2], buf);

                data.user_role.update_textarea(data.selected);
                data.user_role.textarea.render(input_chunks[3], buf);

                data.corporation_name.update_textarea(data.selected);
                data.corporation_name.textarea.render(input_chunks[4], buf);
            }
            SelectedService::GetUser(data) => {
                let input_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([SINGE_LINE_CONSTRAINT])
                    .split(inner_popup_area);

                data.user_uuid.update_textarea(data.selected);
                data.user_uuid.textarea.render(input_chunks[0], buf);
            }
            SelectedService::DeleteUser(data) => {
                let input_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([SINGE_LINE_CONSTRAINT])
                    .split(inner_popup_area);

                data.user_uuid.update_textarea(data.selected);
                data.user_uuid.textarea.render(input_chunks[0], buf);
            }
            SelectedService::QueryBusinessListings(data) => {
                let input_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        SINGE_LINE_CONSTRAINT,
                        SINGE_LINE_CONSTRAINT,
                        SINGE_LINE_CONSTRAINT,
                        SINGE_LINE_CONSTRAINT,
                        SINGE_LINE_CONSTRAINT,
                        SINGE_LINE_CONSTRAINT,
                        SINGE_LINE_CONSTRAINT,
                        SINGE_LINE_CONSTRAINT,
                        SINGE_LINE_CONSTRAINT,
                        SINGE_LINE_CONSTRAINT,
                    ])
                    .split(inner_popup_area);

                data.min_asking_price.update_textarea(data.selected);
                data.min_asking_price.textarea.render(input_chunks[0], buf);

                data.max_asking_price.update_textarea(data.selected);
                data.max_asking_price.textarea.render(input_chunks[1], buf);

                data.seller_corporation_uuid.update_textarea(data.selected);
                data.seller_corporation_uuid
                    .textarea
                    .render(input_chunks[2], buf);

                data.market_uuid.update_textarea(data.selected);
                data.market_uuid.textarea.render(input_chunks[3], buf);

                data.min_operational_expenses.update_textarea(data.selected);
                data.min_operational_expenses
                    .textarea
                    .render(input_chunks[4], buf);

                data.max_operational_expenses.update_textarea(data.selected);
                data.max_operational_expenses
                    .textarea
                    .render(input_chunks[5], buf);

                data.sort_by.update_textarea(data.selected);
                data.sort_by.textarea.render(input_chunks[6], buf);

                data.sort_direction.update_textarea(data.selected);
                data.sort_direction.textarea.render(input_chunks[7], buf);

                data.limit.update_textarea(data.selected);
                data.limit.textarea.render(input_chunks[8], buf);

                data.offset.update_textarea(data.selected);
                data.offset.textarea.render(input_chunks[9], buf);
            }
            SelectedService::AcquireBusinessListing(data) => {
                let input_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([SINGE_LINE_CONSTRAINT])
                    .split(inner_popup_area);

                data.business_listing_uuid.update_textarea(data.selected);
                data.business_listing_uuid
                    .textarea
                    .render(input_chunks[0], buf);
            }
        }
    }
}
