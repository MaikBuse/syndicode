use crate::presentation::theme::{ACCENT_DARK_PURPLE, CYBER_BG, CYBER_PINK, CYBER_YELLOW};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Clear, Widget},
};
use tui_textarea::TextArea;

// For the main popup frame of ResponseDetailWidget
const POPUP_TITLE_TEXT: &str = "Response Details";
const POPUP_BACKGROUND_COLOR: Color = CYBER_BG;
const POPUP_BORDER_COLOR: Color = CYBER_PINK;
const POPUP_TITLE_STYLE: Style = Style::new()
    .fg(CYBER_YELLOW)
    .bg(ACCENT_DARK_PURPLE)
    .add_modifier(Modifier::BOLD);

#[derive(Debug)]
pub struct ResponseDetailWidget;

impl ResponseDetailWidget {
    pub fn render(
        &self,
        area: Rect,
        buf: &mut ratatui::prelude::Buffer,
        response_detail_textarea: &TextArea, // Remains &TextArea, styling applied by caller
    ) where
        Self: Sized,
    {
        // Existing popup_area calculation
        let popup_area = {
            let percent_x = Constraint::Percentage(80);
            let percent_y = Constraint::Percentage(90);
            let popup_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage(10),
                        percent_x,
                        Constraint::Percentage(10),
                    ]
                    .as_ref(),
                )
                .split(area);

            Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(5),
                        percent_y,
                        Constraint::Percentage(5),
                    ]
                    .as_ref(),
                )
                .split(popup_layout[1])[1]
        };

        Clear.render(popup_area, buf);

        // Create and render the main popup block (outer frame)
        let outer_popup_block = Block::default()
            .title(Line::styled(POPUP_TITLE_TEXT, POPUP_TITLE_STYLE))
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::default().fg(POPUP_BORDER_COLOR))
            .style(Style::default().bg(POPUP_BACKGROUND_COLOR));

        // Render the outer popup block frame first
        outer_popup_block.clone().render(popup_area, buf);

        // Get the inner area for the content (the TextArea)
        let content_area = outer_popup_block.inner(popup_area);

        response_detail_textarea.render(content_area, buf);
    }
}
