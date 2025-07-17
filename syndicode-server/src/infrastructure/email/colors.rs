pub(super) struct EmailColors;

impl EmailColors {
    // Background colors
    pub const BACKGROUND: &'static str = "#1a1a1a"; // oklch(0.15 0.04 260) -> dark navy
    pub const CARD_BACKGROUND: &'static str = "#202020"; // oklch(0.18 0.04 260) -> slightly lighter

    // Text colors
    pub const MUTED_FOREGROUND: &'static str = "#a0a0a0"; // oklch(0.65 0.02 260) -> muted gray

    // Brand colors
    pub const PRIMARY: &'static str = "#c466d1"; // oklch(0.7 0.28 325) -> magenta
    pub const SECONDARY: &'static str = "#6bb6ff"; // oklch(0.75 0.2 195) -> cyan

    // Border and input colors
    pub const BORDER: &'static str = "#3a3a3a"; // oklch(0.75 0.2 195 / 20%) -> subtle cyan border

    // Email specific colors
    pub const CODE_BACKGROUND: &'static str = "#1a1a1a"; // Dark background for code blocks
    pub const CODE_BORDER: &'static str = "#555555"; // Border for code elements
    pub const CODE_TEXT: &'static str = "#6bb6ff"; // Cyan for code text
}

