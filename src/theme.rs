use ratatui::style::Color;

/// Bluloco theme colors
pub struct BlulocoTheme;

impl BlulocoTheme {
    // Background colors
    pub const SURFACE: Color = Color::Rgb(40, 40, 40);           // Slightly lighter surface
    
    // Text colors
    pub const TEXT_PRIMARY: Color = Color::Rgb(248, 248, 242);   // Primary text (white-ish)
    pub const TEXT_SECONDARY: Color = Color::Rgb(189, 189, 189); // Secondary text (gray)
    pub const TEXT_MUTED: Color = Color::Rgb(117, 113, 94);      // Muted text (brown-gray)
    
    // Accent colors
    pub const ACCENT_BLUE: Color = Color::Rgb(102, 153, 204);    // Blue accent
    
    // Status colors
    pub const WARNING: Color = Color::Rgb(230, 219, 116);        // Yellow for warning
    pub const ERROR: Color = Color::Rgb(255, 85, 85);            // Red for error
    
    // Selection and focus
    pub const FOCUS: Color = Color::Rgb(102, 153, 204);          // Blue for focus
}
