use super::utils::centered_rect;
use crate::presentation::theme::{ACCENT_DARK_PURPLE, CYBER_BG, CYBER_PINK, CYBER_YELLOW};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Widget, Wrap},
};

const POPUP_BACKGROUND_COLOR: Color = CYBER_BG;
const POPUP_BORDER_COLOR: Color = CYBER_PINK;
const POPUP_TITLE_STYLE: Style = Style::new()
    .fg(CYBER_YELLOW)
    .bg(ACCENT_DARK_PURPLE)
    .add_modifier(Modifier::BOLD);

#[derive(Clone, Copy, Debug)]
pub struct ExitPopupWidget;

impl Widget for ExitPopupWidget {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let popup_area_percentage_height = 15;
        let popup_area_percentage_width = 30;
        let popup_rect = centered_rect(
            popup_area_percentage_width,
            popup_area_percentage_height,
            area,
        );

        // Clear the area before drawing the popup
        Clear.render(popup_rect, buf);

        let popup_title = Line::from(" Exit ").patch_style(POPUP_TITLE_STYLE);

        let popup_block = Block::default()
            .title(popup_title)
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::default().fg(POPUP_BORDER_COLOR))
            .style(Style::default().bg(POPUP_BACKGROUND_COLOR));

        // Render the block first to get its inner area for content placement
        let inner_popup_area = popup_block.inner(popup_rect);
        popup_block.render(popup_rect, buf); // Render the block itself

        // Styled text for the exit prompt
        let exit_text_line = Line::from(vec![
            Span::styled(
                "Would you like to exit the application? ",
                Style::default().fg(CYBER_PINK),
            ),
            Span::styled(
                "(y/n)",
                Style::default()
                    .fg(CYBER_YELLOW)
                    .add_modifier(Modifier::BOLD),
            ),
        ])
        .alignment(Alignment::Center); // Horizontally center the text line

        // --- Vertical Centering using Layout ---
        // We expect the text to take 1 line of height.
        let text_height = 1;

        // Create a layout to center the text vertically within the inner_popup_area
        // It will have three parts: a top flexible spacer, the text area, and a bottom flexible spacer.
        if inner_popup_area.height >= text_height {
            // Ensure there's enough space
            let vertical_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(
                        (100 - (text_height * 100 / inner_popup_area.height)) / 2,
                    ), // Top spacer
                    Constraint::Length(text_height), // Text content
                    Constraint::Percentage(
                        (100 - (text_height * 100 / inner_popup_area.height)) / 2,
                    ), // Bottom spacer
                ])
                .split(inner_popup_area);

            // The text will be rendered in the middle chunk
            let text_area = vertical_chunks[1];

            let exit_paragraph = Paragraph::new(exit_text_line)
                .wrap(Wrap { trim: false })
                .alignment(Alignment::Center);

            // Render the paragraph to the calculated text_area
            exit_paragraph.render(text_area, buf);
        } else {
            // Fallback if inner_popup_area is too small (e.g., just render in the top of inner_popup_area)
            // This case should ideally not be hit with reasonable popup_area_percentage_height
            let exit_paragraph = Paragraph::new(exit_text_line.clone()) // Clone if needed, or re-create
                .wrap(Wrap { trim: false })
                .alignment(Alignment::Center);
            exit_paragraph.render(inner_popup_area, buf);
        }
    }
}
