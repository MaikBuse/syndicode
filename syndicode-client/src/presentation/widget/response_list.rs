use crate::{
    domain::response::{DomainResponse, ResponseType},
    presentation::theme::{ACCENT_DARK_PURPLE, CYBER_BG, CYBER_FG, CYBER_RED, CYBER_YELLOW},
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
use std::collections::VecDeque;
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
    pub responses: VecDeque<DomainResponse>,
}

impl ResponseListWidget {
    pub fn new() -> Self {
        Self {
            responses: VecDeque::with_capacity(MAX_RESPONSES / 2),
        }
    }

    pub fn get_response(
        &self,
        index: usize,
        hide_game_tick_notification: bool,
    ) -> Option<&DomainResponse> {
        let mut counter = 0_usize;
        for response in self.responses.iter() {
            if hide_game_tick_notification
                && response.response_type == ResponseType::GameTickeNotification
            {
                continue;
            }

            if index == counter {
                return Some(response);
            }

            counter += 1;
        }

        None
    }

    pub fn push(&mut self, response: DomainResponse) {
        if self.responses.len() >= MAX_RESPONSES {
            let mut index_of_oldest_gtn_to_remove: Option<usize> = None;

            // Iterate from newest to oldest to find the oldest GameTickeNotification.
            // The `enumerate()` gives `i` as index from the front (0 = newest).
            // If multiple GTNs exist, the last one encountered in this loop (largest `i`)
            // will be the oldest one.
            for (i, r) in self.responses.iter().enumerate() {
                if r.response_type == ResponseType::GameTickeNotification {
                    index_of_oldest_gtn_to_remove = Some(i);
                }
            }

            if let Some(idx) = index_of_oldest_gtn_to_remove {
                self.responses.remove(idx);
            } else {
                // No GameTickeNotification found, or none to remove. Remove the oldest item overall.
                self.responses.pop_back(); // .pop_back() removes from the end (oldest item)
            }
        }
        self.responses.push_front(response); // Add new response to the front (it becomes the newest)
    }

    // item_content_width is the width available for the text of this single line
    fn create_list_item(response: &'_ DomainResponse, item_content_width: u16) -> ListItem<'_> {
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
                Span::styled("✓  ", SUCCESS_TEXT_STYLE.patch(ICON_STYLE)),
                SUCCESS_TEXT_STYLE,
            ),
            ResponseType::Error => (
                Span::styled("✗  ", ERROR_TEXT_STYLE.patch(ICON_STYLE)),
                ERROR_TEXT_STYLE,
            ),
            ResponseType::Info => (
                Span::styled("ℹ  ", INFO_TEXT_STYLE.patch(ICON_STYLE)),
                NORMAL_TEXT_STYLE,
            ),
            ResponseType::GameTickeNotification => (
                Span::styled("ℹ  ", INFO_TEXT_STYLE.patch(ICON_STYLE)),
                NORMAL_TEXT_STYLE,
            ),
        };

        let code_span = Span::styled(response.code.clone(), base_message_style);
        let separator_span = Span::styled(": ", base_message_style);

        let message = response
            .message
            .clone()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join(" ");
        let message_styled = Span::styled(message, base_message_style);
        let line_spans = vec![
            timestamp_span,
            icon_span,
            code_span,
            separator_span,
            message_styled,
        ];

        ListItem::new(Line::from(line_spans))
    }
}

impl Default for ResponseListWidget {
    fn default() -> Self {
        Self::new()
    }
}

impl ResponseListWidget {
    pub fn render(
        &self,
        area: Rect,
        buf: &mut Buffer,
        state: &mut ListState,
        hide_game_tick_notification: bool,
    ) {
        let response_count = match hide_game_tick_notification {
            true => self
                .responses
                .iter()
                .filter(|x| x.response_type != ResponseType::GameTickeNotification)
                .count(),
            false => self.responses.len(),
        };
        let title_text = format!("Responses ({response_count})");
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

        let mut items: Vec<ListItem> = Vec::with_capacity(self.responses.len());

        'for_response: for response in self.responses.iter() {
            if hide_game_tick_notification
                && response.response_type == ResponseType::GameTickeNotification
            {
                continue 'for_response;
            }

            let mut item = ResponseListWidget::create_list_item(response, item_content_width);
            item = item.style(Style::default().bg(CYBER_BG));

            items.push(item);
        }

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
