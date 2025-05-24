use crate::presentation::theme::{
    ACCENT_DARK_PURPLE, CYBER_BG, CYBER_FG, CYBER_PINK, CYBER_YELLOW,
};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph, Widget}, // Added Paragraph
};

pub struct MainArea {
    pub services: Rect,
    pub responses: Rect,
}

pub struct MainLayoutWidget;

impl MainLayoutWidget {
    // Modified signature to include status and username
    pub fn render_and_get_areas(
        &self,
        area: Rect,
        buf: &mut Buffer,
        maybe_username: Option<String>,
        is_stream_active: bool,
    ) -> MainArea {
        // Styles (existing)
        let outer_block_style = Style::default().bg(CYBER_BG);
        let main_title_style = Style::default().fg(CYBER_PINK).add_modifier(Modifier::BOLD);
        let outer_border_style = Style::default().fg(CYBER_FG);

        let inner_block_content_style = Style::default().bg(CYBER_BG);
        let inner_block_title_style = Style::default()
            .bg(ACCENT_DARK_PURPLE)
            .add_modifier(Modifier::BOLD);
        let inner_border_style = Style::default().fg(CYBER_FG);

        // Outer Block (existing)
        let title = Line::from(" Syndicode gRPC Client ").style(main_title_style);
        let instructions = Line::from(vec![
            " Hide/Show Notifications ".fg(CYBER_FG),
            "<e>".fg(CYBER_YELLOW),
            " | ".fg(CYBER_FG),
            " Confirm ".fg(CYBER_FG),
            "<Enter>".fg(CYBER_YELLOW),
            " | ".fg(CYBER_FG),
            " Exit ".fg(CYBER_FG),
            "<Esc> ".fg(CYBER_YELLOW).bold(),
        ]);

        let outer_block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK)
            .border_style(outer_border_style)
            .style(outer_block_style);

        outer_block.clone().render(area, buf); // Render outer block first
        let inner_area = outer_block.inner(area); // Get area inside outer block

        // This splits the inner_area into a 1-line header and the rest for main content
        let [header_area, main_content_area] = Layout::vertical([
            Constraint::Length(1), // Header area (1 line high)
            Constraint::Min(0),    // Main content area (takes remaining space)
        ])
        .areas(inner_area);

        let (status, user_name) = match maybe_username {
            Some(user_name) => ("Authenticated", user_name.clone()),
            None => ("Unauthenticated", "-".to_string()),
        };

        let stream = match is_stream_active {
            true => "Active",
            false => "Inactive",
        };

        let header_spans = vec![
            " Status: ".fg(CYBER_FG),
            status.fg(CYBER_YELLOW),
            " | ".fg(CYBER_FG),
            "User: ".fg(CYBER_FG),
            user_name.fg(CYBER_PINK).add_modifier(Modifier::BOLD),
            " | ".fg(CYBER_FG),
            "Stream: ".fg(CYBER_FG),
            stream.fg(CYBER_YELLOW),
        ];
        let header_line = Line::from(header_spans).left_aligned();
        Paragraph::new(header_line).render(header_area, buf);

        let services_responses_layout =
            Layout::horizontal([Constraint::Length(35), Constraint::Min(1)]).margin(1);
        let [services_area, response_area] = services_responses_layout.areas(main_content_area);

        // Services Block (rendered in the new services_area)
        let services_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(inner_border_style) // Style is Copy, no clone needed
            .title(
                Line::from("Services")
                    .centered()
                    .style(inner_block_title_style), // Style is Copy
            )
            .style(inner_block_content_style); // Style is Copy
        services_block.clone().render(services_area, buf); // Use clone for block if you use its inner area later

        MainArea {
            services: services_block.inner(services_area),
            responses: response_area,
        }
    }
}
