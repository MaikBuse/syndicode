use crate::presentation::theme::{
    ACCENT_DARK_PURPLE, CYBER_BG, CYBER_FG, CYBER_PINK, CYBER_YELLOW,
};
use bon::builder;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::Text,
    widgets::{HighlightSpacing, List, ListItem, ListState, StatefulWidget, Widget}, // Added Widget
};
use std::fmt::Display;

const SELECTED_STYLE: Style = Style::new()
    .fg(CYBER_YELLOW) // Bright yellow for selected item text
    .bg(ACCENT_DARK_PURPLE) // Dark purple for selected item background
    .add_modifier(Modifier::BOLD);

const CATEGORY_HEADER_STYLE: Style = Style::new()
    .fg(CYBER_PINK) // Bright pink for category headers
    .add_modifier(Modifier::BOLD);

const SERVICE_ITEM_STYLE: Style = Style::new().fg(CYBER_FG); // Bright cyan/mint for service items

const INDENTATION: &str = "  "; // Two spaces for indentation

/// Defines the actual operations a service list item can trigger.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ServiceAction {
    // Authorization Category
    Register,
    VerifyRegistration,
    ResendVerification,
    Login,
    GetCurrentUser,
    // Administration Category (Example)
    CreateUser,
    GetUser,
    DeleteUser,
    // Game Category
    PlayStream,
    GetCorporation,
    QueryBusinessListings,
    AcquireListedBusiness,
}

impl Display for ServiceAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceAction::Register => write!(f, "Register User"),
            ServiceAction::VerifyRegistration => write!(f, "Verify User Registration"),
            ServiceAction::ResendVerification => write!(f, "Resend Verification Code"),
            ServiceAction::Login => write!(f, "Login User"),
            ServiceAction::GetCurrentUser => write!(f, "Get Current User"),
            ServiceAction::GetUser => write!(f, "Get User"),
            ServiceAction::CreateUser => write!(f, "Create User"),
            ServiceAction::DeleteUser => write!(f, "Delete User"),
            ServiceAction::PlayStream => write!(f, "Setup the game stream"),
            ServiceAction::GetCorporation => write!(f, "Get your corporation"),
            ServiceAction::QueryBusinessListings => write!(f, "Query business listings"),
            ServiceAction::AcquireListedBusiness => write!(f, "Acquire listed business"),
        }
    }
}

/// Represents a single service item in the list.
#[derive(Debug, Clone)]
pub struct ServiceItem {
    /// The text displayed in the list for this service.
    pub display_name: String,
    /// The action associated with this service.
    pub action: ServiceAction,
}

impl ServiceItem {
    pub fn new(action: ServiceAction) -> Self {
        Self {
            display_name: action.to_string(), // Default display name from action
            action,
        }
    }
    pub fn new_with_name(name: impl Into<String>, action: ServiceAction) -> Self {
        Self {
            display_name: name.into(),
            action,
        }
    }
}

/// Represents a category of services.
#[derive(Debug, Clone)]
pub struct ServiceCategory {
    pub name: String,
    pub items: Vec<ServiceItem>,
}

// --- ServiceList Widget ---

/// Helper enum to distinguish between item types in the flattened list
/// when determining the selected action.
#[derive(Debug, Clone)]
enum FlatListItemType {
    CategoryHeader,
    Service(ServiceAction),
}

#[derive(Debug)]
pub struct ServiceListWidget {
    categories: Vec<ServiceCategory>,
    /// A flattened representation used for mapping ListState index to actions.
    /// This is built internally.
    flat_items_map: Vec<FlatListItemType>,
}

impl ServiceListWidget {
    pub fn new(categories: Vec<ServiceCategory>) -> Self {
        let flat_items_map = Self::build_flat_items_map(&categories);
        Self {
            categories,
            flat_items_map,
        }
    }

    fn build_flat_items_map(categories: &[ServiceCategory]) -> Vec<FlatListItemType> {
        let mut map = Vec::new();
        for category in categories {
            map.push(FlatListItemType::CategoryHeader);
            for item in &category.items {
                map.push(FlatListItemType::Service(item.action.clone()));
            }
        }
        map
    }

    /// Returns the total number of renderable lines (headers + items).
    pub fn total_items(&self) -> usize {
        self.flat_items_map.len()
    }

    /// Gets the action associated with the currently selected item in ListState.
    /// Returns None if a category header is selected or if nothing is selected.
    pub fn get_selected_action(&self, state: &ListState) -> Option<ServiceAction> {
        state.selected().and_then(|selected_index| {
            match self.flat_items_map.get(selected_index) {
                Some(FlatListItemType::Service(action)) => Some(action.clone()),
                _ => None, // Header or out of bounds
            }
        })
    }

    /// Call this to adjust selection to the next/previous *actual* service item,
    /// skipping over category headers.
    /// `delta` should be 1 for next, -1 for previous.
    pub fn adjust_selection(&self, state: &mut ListState, delta: i32) {
        if self.flat_items_map.is_empty() {
            state.select(None);
            return;
        }

        let len = self.flat_items_map.len();
        let len_i32 = len as i32;

        let current_selected_opt = state.selected();

        // Determine the starting point for the search as an i32.
        let mut search_candidate_signed: i32 = match current_selected_opt {
            Some(current_idx) => current_idx as i32 + delta,
            None => {
                if delta < 0 {
                    len_i32 - 1
                } else {
                    0
                }
            }
        };

        let step = if delta == 0 { 1 } else { delta.signum() };

        for _iteration_count in 0..len {
            if search_candidate_signed >= len_i32 {
                search_candidate_signed = 0;
            } else if search_candidate_signed < 0 {
                search_candidate_signed = len_i32 - 1;
            }

            let candidate_usize = search_candidate_signed as usize;

            if let Some(FlatListItemType::Service(_)) = self.flat_items_map.get(candidate_usize) {
                if current_selected_opt != Some(candidate_usize) || delta != 0 {
                    state.select(Some(candidate_usize));
                }
                return;
            }
            search_candidate_signed += step;
        }
        state.select(None);
    }
}

/// Provides a default set of categorized services.
#[builder]
pub fn default_services(is_logged_in: bool, is_stream_active: bool) -> Vec<ServiceCategory> {
    let mut auth_items = vec![
        ServiceItem::new(ServiceAction::Register),
        ServiceItem::new(ServiceAction::VerifyRegistration),
        ServiceItem::new(ServiceAction::ResendVerification),
        ServiceItem::new(ServiceAction::Login),
    ];

    let mut admin_items: Vec<ServiceItem> = Vec::new();

    let mut game_items: Vec<ServiceItem> = Vec::new();

    if is_logged_in {
        auth_items.push(ServiceItem::new(ServiceAction::GetCurrentUser));

        let mut logged_in_admin_items = vec![
            ServiceItem::new(ServiceAction::CreateUser),
            ServiceItem::new(ServiceAction::DeleteUser),
            ServiceItem::new(ServiceAction::GetUser),
        ];
        admin_items.append(&mut logged_in_admin_items);

        game_items.push(ServiceItem::new(ServiceAction::PlayStream));

        if is_stream_active {
            let mut streaming_game_items = vec![
                ServiceItem::new(ServiceAction::GetCorporation),
                ServiceItem::new(ServiceAction::QueryBusinessListings),
                ServiceItem::new(ServiceAction::AcquireListedBusiness),
            ];

            game_items.append(&mut streaming_game_items);
        }
    }

    let mut service_categories = vec![ServiceCategory {
        name: "🔒 Authorization".to_string(),
        items: auth_items,
    }];

    if is_logged_in {
        let mut logged_in_service_categories = vec![
            ServiceCategory {
                name: "⚙️Administration".to_string(),
                items: admin_items,
            },
            ServiceCategory {
                name: "🕹️ Game".to_string(),
                items: game_items,
            },
        ];

        service_categories.append(&mut logged_in_service_categories);
    }

    service_categories
}

impl StatefulWidget for &ServiceListWidget {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let mut list_items: Vec<ListItem> = Vec::new();

        for category in &self.categories {
            // Category Header
            list_items.push(ListItem::new(Text::styled(
                category.name.clone(),
                CATEGORY_HEADER_STYLE,
            )));

            // Service Items under this category
            for item in &category.items {
                let display_text = format!("{}{}", INDENTATION, item.display_name);
                list_items.push(ListItem::new(Text::styled(
                    display_text,
                    SERVICE_ITEM_STYLE,
                )));
            }
        }

        // Apply general background to all items if not overridden
        // This ensures the CYBER_BG is used for non-styled parts of list items.
        let styled_list_items: Vec<ListItem> = list_items
            .into_iter()
            .map(|item| item.style(Style::default().bg(CYBER_BG))) // Set default background for all items
            .collect();

        if styled_list_items.is_empty() {
            let placeholder = ListItem::new("No services configured.").style(
                Style::default()
                    .fg(CYBER_FG)
                    .add_modifier(Modifier::DIM)
                    .bg(CYBER_BG),
            ); // Dimmed foreground on cyber background
            let list = List::new(vec![placeholder]);

            Widget::render(list.style(Style::default().bg(CYBER_BG)), area, buf);
            return;
        }

        let list = List::new(styled_list_items)
            .style(Style::default().bg(CYBER_BG)) // Ensure overall list background
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">> ") // Cyberpunk-ish highlight symbol
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, state);
    }
}
