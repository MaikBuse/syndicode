use crate::{
    domain::response::{Response, ResponseType},
    presentation::theme::{
        ACCENT_DARK_PURPLE, CYBER_BG, CYBER_FG, CYBER_PINK, CYBER_RED, CYBER_YELLOW,
    },
};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{
        Block, BorderType, Borders, HighlightSpacing, List, ListDirection, ListItem, ListState,
        Padding, StatefulWidget, Widget,
    },
};
// std::borrow::Cow is not strictly needed here anymore if we only take the first line
// use std::borrow::Cow;
use std::collections::VecDeque;
// textwrap might still be useful for truncating the single line if prefix + first line of message is too long
use textwrap;
use time::macros::format_description;

const MAX_RESPONSES: usize = 100;

// --- Styles --- (Keep your styles as they are)
const NORMAL_TEXT_STYLE: Style = Style::new().fg(CYBER_FG).bg(CYBER_BG);
const ERROR_TEXT_STYLE: Style = Style::new().fg(CYBER_RED).bg(CYBER_BG);
const SUCCESS_TEXT_STYLE: Style = Style::new().fg(CYBER_FG).bg(CYBER_BG);
const INFO_TEXT_STYLE: Style = Style::new().fg(CYBER_YELLOW).bg(CYBER_BG);
const TIMESTAMP_STYLE: Style = Style::new()
    .fg(CYBER_FG)
    .add_modifier(Modifier::DIM)
    .bg(CYBER_BG);
const SELECTED_STYLE: Style = Style::new()
    .fg(CYBER_YELLOW)
    .bg(ACCENT_DARK_PURPLE)
    .add_modifier(Modifier::BOLD);
const PLACEHOLDER_STYLE: Style = Style::new()
    .fg(CYBER_FG)
    .add_modifier(Modifier::DIM)
    .bg(CYBER_BG);
const BORDER_STYLE: Style = Style::new().fg(CYBER_FG);
const TITLE_STYLE: Style = Style::new()
    .fg(CYBER_PINK)
    .bg(ACCENT_DARK_PURPLE)
    .add_modifier(Modifier::BOLD);
const ICON_STYLE: Style = Style::new().add_modifier(Modifier::BOLD);

// --- Custom Time Format ---
const TIME_FORMAT_DESC: &[time::format_description::FormatItem<'_>] =
    format_description!("[hour repr:24]:[minute]:[second] ");

// --- Widget constants ---
const HIGHLIGHT_SYMBOL_WIDTH: u16 = 3; // For ">> "

#[derive(Debug)]
pub struct ResponseListWidget {
    pub responses: VecDeque<Response>, // Made public to allow app to get selected response
                                       // Alternatively, add a method like `pub fn get_response(&self, index: usize) -> Option<&Response>`
}

impl ResponseListWidget {
    pub fn new() -> Self {
        Self {
            responses: VecDeque::with_capacity(MAX_RESPONSES / 2),
        }
    }

    pub fn push(&mut self, response: Response) {
        if self.responses.len() >= MAX_RESPONSES {
            self.responses.pop_back();
        }
        self.responses.push_front(response);
    }

    pub fn clear(&mut self) {
        self.responses.clear();
    }

    // item_content_width is the width available for the text of this single line
    fn create_list_item(response: &'_ Response, item_content_width: u16) -> ListItem<'_> {
        if item_content_width == 0 {
            return ListItem::new(Text::raw(""));
        }

        let timestamp_str = response
            .timestamp
            .format(&TIME_FORMAT_DESC)
            .unwrap_or_else(|_| "TimeErr ".to_string());
        let timestamp_span = Span::styled(timestamp_str, TIMESTAMP_STYLE);

        let (icon_span, base_message_style) = match response.response_type {
            ResponseType::Success => (
                Span::styled("✓ ", SUCCESS_TEXT_STYLE.patch(ICON_STYLE)),
                SUCCESS_TEXT_STYLE,
            ),
            ResponseType::Error => (
                Span::styled("✗ ", ERROR_TEXT_STYLE.patch(ICON_STYLE)),
                ERROR_TEXT_STYLE,
            ),
            ResponseType::Info => (
                Span::styled("ℹ ", INFO_TEXT_STYLE.patch(ICON_STYLE)),
                NORMAL_TEXT_STYLE,
            ),
        };

        let code_span = Span::styled(response.code.clone(), base_message_style);
        let separator_span = Span::styled(": ", base_message_style);

        let mut line_spans = vec![timestamp_span, icon_span, code_span, separator_span];
        let prefix_width = line_spans.iter().map(|s| s.width()).sum::<usize>() as u16;

        // Get the first line of the message, or an empty string if no message
        let message_first_line = response.message.lines().next().unwrap_or("").to_string();
        // Alternative: let message_summary = "[Details...]";

        let available_width_for_message_summary = item_content_width.saturating_sub(prefix_width);

        if available_width_for_message_summary > 0 {
            // Truncate the message summary if it's too long for the remaining space
            let options = textwrap::Options::new(available_width_for_message_summary as usize)
                .word_separator(textwrap::WordSeparator::AsciiSpace)
                .break_words(false); // Don't break words, just truncate the line

            // textwrap::wrap returns a Vec, we only care about the first (and only) line
            let wrapped_summary_lines = textwrap::wrap(&message_first_line, &options);
            let summary_to_display = if let Some(first_wrapped_line) = wrapped_summary_lines.get(0)
            {
                if first_wrapped_line.len() < message_first_line.len()
                    && first_wrapped_line.len() > 3
                {
                    // Add ellipsis if truncated and there's space
                    format!(
                        "{}...",
                        first_wrapped_line
                            .chars()
                            .take(first_wrapped_line.len().saturating_sub(3))
                            .collect::<String>()
                    )
                } else {
                    first_wrapped_line.to_string()
                }
            } else {
                "".to_string() // Should not happen if message_first_line is not empty
            };
            line_spans.push(Span::styled(summary_to_display, base_message_style));
        }
        // If available_width_for_message_summary is 0, we just show the prefix (which might also be truncated by ratatui)

        ListItem::new(Line::from(line_spans))
    }
}

impl Default for ResponseListWidget {
    fn default() -> Self {
        Self::new()
    }
}

// The render function remains largely the same, but item_content_width
// calculation is now for a single line summary.
impl StatefulWidget for &ResponseListWidget {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let title_text = format!("Responses ({})", self.responses.len());
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(BORDER_STYLE)
            .title(Span::styled(title_text, TITLE_STYLE))
            .padding(Padding::horizontal(1))
            .style(Style::default().bg(CYBER_BG));

        let list_content_area = block.inner(area);

        if list_content_area.width == 0 || list_content_area.height == 0 {
            block.render(area, buf);
            return;
        }

        // item_content_width is the width available for the single line of text in the list item,
        // after accounting for the highlight symbol.
        let item_content_width = list_content_area
            .width
            .saturating_sub(HIGHLIGHT_SYMBOL_WIDTH);

        if self.responses.is_empty() {
            let placeholder_text = Line::from(vec![
                Span::styled("· ", PLACEHOLDER_STYLE.patch(ICON_STYLE)),
                Span::styled("No responses yet...", PLACEHOLDER_STYLE),
            ]);
            let placeholder_list_item =
                ListItem::new(placeholder_text).style(Style::default().bg(CYBER_BG));
            let placeholder_list = List::new(vec![placeholder_list_item])
                .block(block.clone())
                .style(Style::default().bg(CYBER_BG));
            Widget::render(placeholder_list, area, buf);
            return;
        }

        let items: Vec<ListItem> = self
            .responses
            .iter()
            .map(|response| ResponseListWidget::create_list_item(response, item_content_width))
            .map(|item| item.style(Style::default().bg(CYBER_BG)))
            .collect();

        let list_widget = List::new(items)
            .block(block)
            .style(Style::default().bg(CYBER_BG))
            .direction(ListDirection::BottomToTop)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">> ")
            .highlight_spacing(HighlightSpacing::WhenSelected);

        StatefulWidget::render(list_widget, area, buf, state);
    }
}
