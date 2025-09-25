//! Font settings and text rendering configuration for BGI.

use crate::{constants::*, types::*};

/// Font information structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FontInfo {
    /// Font ID/type
    pub font: i32,
    /// Font name
    pub name: String,
    /// Font file path (if loaded from file)
    pub filename: Option<String>,
}

impl Default for FontInfo {
    fn default() -> Self {
        Self {
            font: DEFAULT_FONT,
            name: "Default".to_string(),
            filename: None,
        }
    }
}

/// Text style settings for rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextStyle {
    /// Font type/ID
    pub font: i32,
    /// Text direction (HORIZ_DIR or VERT_DIR)
    pub direction: i32,
    /// Character size multiplier
    pub char_size: i32,
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            font: DEFAULT_FONT,
            direction: HORIZ_DIR,
            char_size: 1,
        }
    }
}

/// Text positioning and alignment settings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextAlignment {
    /// Horizontal alignment (LEFT_TEXT, CENTER_TEXT, RIGHT_TEXT)
    pub horizontal: i32,
    /// Vertical alignment (TOP_TEXT, CENTER_TEXT, BOTTOM_TEXT)
    pub vertical: i32,
}

impl Default for TextAlignment {
    fn default() -> Self {
        Self {
            horizontal: LEFT_TEXT,
            vertical: TOP_TEXT,
        }
    }
}

/// Complete font and text settings.
#[derive(Debug, Clone)]
pub struct FontSettings {
    /// Current text style
    pub style: TextStyle,
    /// Text alignment settings
    pub alignment: TextAlignment,
    /// Available fonts
    pub fonts: Vec<FontInfo>,
    /// User-defined character size (for scalable fonts)
    pub user_char_size: Option<(i32, i32)>, // (width, height)
}

impl Default for FontSettings {
    fn default() -> Self {
        let mut fonts = Vec::new();

        // Add default BGI fonts
        fonts.push(FontInfo {
            font: DEFAULT_FONT,
            name: "Default".to_string(),
            filename: None,
        });

        fonts.push(FontInfo {
            font: TRIPLEX_FONT,
            name: "Triplex".to_string(),
            filename: None,
        });

        fonts.push(FontInfo {
            font: SMALL_FONT,
            name: "Small".to_string(),
            filename: None,
        });

        fonts.push(FontInfo {
            font: SANS_SERIF_FONT,
            name: "SansSerif".to_string(),
            filename: None,
        });

        fonts.push(FontInfo {
            font: GOTHIC_FONT,
            name: "Gothic".to_string(),
            filename: None,
        });

        Self {
            style: TextStyle::default(),
            alignment: TextAlignment::default(),
            fonts,
            user_char_size: None,
        }
    }
}

impl FontSettings {
    /// Create new font settings with defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set text style parameters.
    pub fn set_text_style(&mut self, font: i32, direction: i32, char_size: i32) {
        self.style.font = font;
        self.style.direction = direction;
        self.style.char_size = char_size;
    }

    /// Get current text style.
    pub fn get_text_style(&self) -> (i32, i32, i32) {
        (self.style.font, self.style.direction, self.style.char_size)
    }

    /// Set text alignment/justification.
    pub fn set_text_justify(&mut self, horizontal: i32, vertical: i32) {
        self.alignment.horizontal = horizontal;
        self.alignment.vertical = vertical;
    }

    /// Get text alignment.
    pub fn get_text_justify(&self) -> (i32, i32) {
        (self.alignment.horizontal, self.alignment.vertical)
    }

    /// Set user-defined character size.
    pub fn set_user_char_size(&mut self, width: i32, height: i32) {
        self.user_char_size = Some((width, height));
    }

    /// Clear user-defined character size.
    pub fn clear_user_char_size(&mut self) {
        self.user_char_size = None;
    }

    /// Get effective character size for current font.
    pub fn get_char_size(&self) -> (i32, i32) {
        if let Some((width, height)) = self.user_char_size {
            return (width, height);
        }

        // Default character sizes based on font and size multiplier
        let base_size = match self.style.font {
            DEFAULT_FONT => (8, 8),
            TRIPLEX_FONT => (13, 16),
            SMALL_FONT => (6, 8),
            SANS_SERIF_FONT => (11, 16),
            GOTHIC_FONT => (14, 16),
            _ => (8, 8), // Default
        };

        let multiplier = self.style.char_size.max(1);
        (base_size.0 * multiplier, base_size.1 * multiplier)
    }

    /// Calculate text width for a given string.
    pub fn text_width(&self, text: &str) -> i32 {
        if text.is_empty() {
            return 0;
        }

        let (char_width, _) = self.get_char_size();
        let char_count = text.chars().count() as i32;

        match self.style.direction {
            HORIZ_DIR => char_width * char_count,
            VERT_DIR => char_width, // Vertical text has fixed width
            _ => char_width * char_count,
        }
    }

    /// Calculate text height for a given string.
    pub fn text_height(&self, text: &str) -> i32 {
        if text.is_empty() {
            return 0;
        }

        let (_, char_height) = self.get_char_size();

        match self.style.direction {
            HORIZ_DIR => char_height,
            VERT_DIR => {
                // Vertical text height depends on character count
                let char_count = text.chars().count() as i32;
                char_height * char_count
            }
            _ => char_height,
        }
    }

    /// Get font information by ID.
    pub fn get_font_info(&self, font_id: i32) -> Option<&FontInfo> {
        self.fonts.iter().find(|f| f.font == font_id)
    }

    /// Add a new font.
    pub fn add_font(&mut self, font_id: i32, name: String, filename: Option<String>) {
        let font_info = FontInfo {
            font: font_id,
            name,
            filename,
        };

        // Replace existing font with same ID or add new
        if let Some(existing) = self.fonts.iter_mut().find(|f| f.font == font_id) {
            *existing = font_info;
        } else {
            self.fonts.push(font_info);
        }
    }

    /// Check if font ID is valid.
    pub fn is_valid_font(&self, font_id: i32) -> bool {
        self.fonts.iter().any(|f| f.font == font_id)
    }

    /// Check if current font is bitmap or vector.
    pub fn is_vector_font(&self) -> bool {
        // In BGI, fonts >= TRIPLEX_FONT are typically vector fonts
        self.style.font >= TRIPLEX_FONT
    }

    /// Check if text direction is horizontal.
    pub fn is_horizontal(&self) -> bool {
        self.style.direction == HORIZ_DIR
    }

    /// Check if text direction is vertical.
    pub fn is_vertical(&self) -> bool {
        self.style.direction == VERT_DIR
    }

    /// Calculate text position adjustment based on alignment.
    pub fn adjust_position(&self, x: i32, y: i32, text: &str) -> (i32, i32) {
        let text_width = self.text_width(text);
        let text_height = self.text_height(text);

        let adjusted_x = match self.alignment.horizontal {
            LEFT_TEXT => x,
            CENTER_TEXT => x - text_width / 2,
            RIGHT_TEXT => x - text_width,
            _ => x,
        };

        let adjusted_y = match self.alignment.vertical {
            TOP_TEXT => y,
            CENTER_TEXT => y - text_height / 2,
            BOTTOM_TEXT => y - text_height,
            _ => y,
        };

        (adjusted_x, adjusted_y)
    }

    /// Reset to default settings.
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
