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
                user_name,
                code,
            } => {
                let input_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([SINGE_LINE_CONSTRAINT, SINGE_LINE_CONSTRAINT])
                    .split(inner_popup_area);

                user_name.update_textarea(*selected);
                user_name.textarea.render(input_chunks[0], buf);

                code.update_textarea(*selected);
                code.textarea.render(input_chunks[1], buf);
            }
            SelectedService::Register {
                selected,
                user_name,
                user_password,
                email,
                corporation_name,
            } => {
                let input_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        SINGE_LINE_CONSTRAINT,
                        SINGE_LINE_CONSTRAINT,
                        SINGE_LINE_CONSTRAINT,
                        SINGE_LINE_CONSTRAINT,
                    ])
                    .split(inner_popup_area);

                user_name.update_textarea(*selected);
                user_name.textarea.render(input_chunks[0], buf);

                user_password.update_textarea(*selected);
                user_password.textarea.render(input_chunks[1], buf);

                email.update_textarea(*selected);
                email.textarea.render(input_chunks[2], buf);

                corporation_name.update_textarea(*selected);
                corporation_name.textarea.render(input_chunks[3], buf);
            }
            SelectedService::ResendVerification {
                selected,
                user_name,
            } => {
                let input_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([SINGE_LINE_CONSTRAINT])
                    .split(inner_popup_area);

                user_name.update_textarea(*selected);
                user_name.textarea.render(input_chunks[0], buf);
            }
            SelectedService::Login {
                selected,
                user_name,
                user_password,
            } => {
                let input_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([SINGE_LINE_CONSTRAINT, SINGE_LINE_CONSTRAINT])
                    .split(inner_popup_area);

                user_name.update_textarea(*selected);
                user_name.textarea.render(input_chunks[0], buf);

                user_password.update_textarea(*selected);
                user_password.textarea.render(input_chunks[1], buf);
            }
            SelectedService::CreateUser {
                selected,
                user_name,
                user_password,
                user_email,
                user_role,
                corporation_name,
            } => {
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

                user_name.update_textarea(*selected);
                user_name.textarea.render(input_chunks[0], buf);

                user_password.update_textarea(*selected);
                user_password.textarea.render(input_chunks[1], buf);

                user_email.update_textarea(*selected);
                user_email.textarea.render(input_chunks[2], buf);

                user_role.update_textarea(*selected);
                user_role.textarea.render(input_chunks[3], buf);

                corporation_name.update_textarea(*selected);
                corporation_name.textarea.render(input_chunks[4], buf);
            }
            SelectedService::DeleteUser {
                selected,
                user_uuid,
            } => {
                let input_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([SINGE_LINE_CONSTRAINT])
                    .split(inner_popup_area);

                user_uuid.update_textarea(*selected);
                user_uuid.textarea.render(input_chunks[0], buf);
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
            } => {
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

                min_asking_price.update_textarea(*selected);
                min_asking_price.textarea.render(input_chunks[0], buf);

                max_asking_price.update_textarea(*selected);
                max_asking_price.textarea.render(input_chunks[1], buf);

                seller_corporation_uuid.update_textarea(*selected);
                seller_corporation_uuid
                    .textarea
                    .render(input_chunks[2], buf);

                market_uuid.update_textarea(*selected);
                market_uuid.textarea.render(input_chunks[3], buf);

                min_operational_expenses.update_textarea(*selected);
                min_operational_expenses
                    .textarea
                    .render(input_chunks[4], buf);

                max_operational_expenses.update_textarea(*selected);
                max_operational_expenses
                    .textarea
                    .render(input_chunks[5], buf);

                sort_by.update_textarea(*selected);
                sort_by.textarea.render(input_chunks[6], buf);

                sort_direction.update_textarea(*selected);
                sort_direction.textarea.render(input_chunks[7], buf);

                limit.update_textarea(*selected);
                limit.textarea.render(input_chunks[8], buf);

                offset.update_textarea(*selected);
                offset.textarea.render(input_chunks[9], buf);
            }
            SelectedService::AcquireBusinessListing {
                selected,
                business_listing_uuid,
            } => {
                let input_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([SINGE_LINE_CONSTRAINT])
                    .split(inner_popup_area);

                business_listing_uuid.update_textarea(*selected);
                business_listing_uuid.textarea.render(input_chunks[0], buf);
            }
        }
    }
}
