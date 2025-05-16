use ratatui::style::Color;

pub(super) const CYBER_BG: Color = Color::Rgb(0, 10, 25); // A dark, desaturated blue
pub(super) const CYBER_FG: Color = Color::Rgb(0, 255, 150); // Bright cyan/mint
pub(super) const CYBER_PINK: Color = Color::Rgb(255, 0, 190);
pub(super) const CYBER_YELLOW: Color = Color::Rgb(255, 135, 0);
pub(super) const CYBER_RED: Color = Color::Rgb(255, 20, 50); // For errors, more distinct than pink
pub(super) const ACCENT_DARK_PURPLE: Color = Color::Rgb(64, 0, 128);
pub(super) const INPUT_AREA_BG: Color = Color::Rgb(5, 15, 30); // Slightly different from CYBER_BG for depth
