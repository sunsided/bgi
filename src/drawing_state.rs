//! Drawing state management for BGI graphics operations.

use crate::{Color, constants::*, types::*};

/// Line style settings for drawing operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineStyle {
    /// Line style pattern (SOLID_LINE, DOTTED_LINE, etc.)
    pub style: i32,
    /// User-defined pattern (for USERBIT_LINE)
    pub pattern: u16,
    /// Line thickness (NORM_WIDTH, THICK_WIDTH)
    pub thickness: i32,
}

impl Default for LineStyle {
    fn default() -> Self {
        Self {
            style: SOLID_LINE,
            pattern: 0xFFFF,
            thickness: NORM_WIDTH,
        }
    }
}

/// Fill style settings for filled shapes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FillStyle {
    /// Fill pattern type
    pub pattern: i32,
    /// Fill color
    pub color: Color,
    /// Custom pattern data (8x8 bitmap)
    pub custom_pattern: Option<[u8; 8]>,
}

impl Default for FillStyle {
    fn default() -> Self {
        Self {
            pattern: SOLID_FILL,
            color: Color::WHITE,
            custom_pattern: None,
        }
    }
}

/// Text justification settings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextJustification {
    /// Horizontal justification (LEFT_TEXT, CENTER_TEXT, RIGHT_TEXT)
    pub horizontal: i32,
    /// Vertical justification (BOTTOM_TEXT, CENTER_TEXT, TOP_TEXT)
    pub vertical: i32,
}

impl Default for TextJustification {
    fn default() -> Self {
        Self {
            horizontal: LEFT_TEXT,
            vertical: TOP_TEXT,
        }
    }
}

/// Current drawing position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Default for Position {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

/// Viewport settings for clipping and coordinate system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Viewport {
    /// Left boundary
    pub left: i32,
    /// Top boundary
    pub top: i32,
    /// Right boundary
    pub right: i32,
    /// Bottom boundary
    pub bottom: i32,
    /// Whether clipping is enabled
    pub clip: bool,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            left: 0,
            top: 0,
            right: 639,
            bottom: 479,
            clip: true,
        }
    }
}

/// Complete drawing state for BGI operations.
#[derive(Debug, Clone)]
pub struct DrawingState {
    /// Current drawing color
    pub color: Color,
    /// Background color
    pub background_color: Color,
    /// Line style settings
    pub line_style: LineStyle,
    /// Fill style settings
    pub fill_style: FillStyle,
    /// Text justification
    pub text_justification: TextJustification,
    /// Current drawing position
    pub position: Position,
    /// Viewport settings
    pub viewport: Viewport,
    /// Write mode for drawing operations
    pub write_mode: i32,
    /// Batch mode flag - when true, skip presentation after each draw operation
    pub batch_mode: bool,
}

impl Default for DrawingState {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            background_color: Color::BLACK,
            line_style: LineStyle::default(),
            fill_style: FillStyle::default(),
            text_justification: TextJustification::default(),
            position: Position::default(),
            viewport: Viewport::default(),
            write_mode: COPY_PUT,
            batch_mode: false, // Default to normal mode with presentation
        }
    }
}

impl DrawingState {
    /// Create a new drawing state with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the current drawing color.
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    /// Get the current drawing color.
    pub fn get_color(&self) -> Color {
        self.color
    }

    /// Set the background color.
    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    /// Get the background color.
    pub fn get_background_color(&self) -> Color {
        self.background_color
    }

    /// Set line style parameters.
    pub fn set_line_style(&mut self, line_style: i32, pattern: u16, thickness: i32) {
        self.line_style.style = line_style;
        self.line_style.pattern = pattern;
        self.line_style.thickness = thickness;
    }

    /// Get current line style.
    pub fn get_line_style(&self) -> (i32, u16, i32) {
        (
            self.line_style.style,
            self.line_style.pattern,
            self.line_style.thickness,
        )
    }

    /// Set fill style with predefined pattern.
    pub fn set_fill_style(&mut self, pattern: i32, color: Color) {
        self.fill_style.pattern = pattern;
        self.fill_style.color = color;
        self.fill_style.custom_pattern = None;
    }

    /// Set fill style with custom pattern.
    pub fn set_fill_pattern(&mut self, pattern: &[u8; 8], color: Color) {
        self.fill_style.pattern = USER_FILL;
        self.fill_style.color = color;
        self.fill_style.custom_pattern = Some(*pattern);
    }

    /// Get current fill style.
    pub fn get_fill_style(&self) -> (i32, Color) {
        (self.fill_style.pattern, self.fill_style.color)
    }

    /// Get custom fill pattern if set.
    pub fn get_fill_pattern(&self) -> Option<[u8; 8]> {
        self.fill_style.custom_pattern
    }

    /// Set text justification.
    pub fn set_text_justify(&mut self, horizontal: i32, vertical: i32) {
        self.text_justification.horizontal = horizontal;
        self.text_justification.vertical = vertical;
    }

    /// Get text justification.
    pub fn get_text_justify(&self) -> (i32, i32) {
        (
            self.text_justification.horizontal,
            self.text_justification.vertical,
        )
    }

    /// Move to absolute position (BGI moveto).
    pub fn move_to(&mut self, x: i32, y: i32) {
        self.position.x = x;
        self.position.y = y;
    }

    /// Move relatively from current position (BGI moverel).
    pub fn move_rel(&mut self, dx: i32, dy: i32) {
        self.position.x = self.position.x.saturating_add(dx);
        self.position.y = self.position.y.saturating_add(dy);
    }

    /// Get current position (BGI getx, gety).
    pub fn get_position(&self) -> (i32, i32) {
        (self.position.x, self.position.y)
    }

    /// Set viewport for clipping.
    pub fn set_viewport(&mut self, left: i32, top: i32, right: i32, bottom: i32, clip: bool) {
        self.viewport.left = left;
        self.viewport.top = top;
        self.viewport.right = right;
        self.viewport.bottom = bottom;
        self.viewport.clip = clip;
    }

    /// Get current viewport settings.
    pub fn get_viewport(&self) -> Viewport {
        self.viewport
    }

    /// Set drawing write mode.
    pub fn set_write_mode(&mut self, mode: i32) {
        self.write_mode = mode;
    }

    /// Get drawing write mode.
    pub fn get_write_mode(&self) -> i32 {
        self.write_mode
    }

    /// Reset to default state.
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Check if line style uses a pattern.
    pub fn is_patterned_line(&self) -> bool {
        matches!(
            self.line_style.style,
            DOTTED_LINE | CENTER_LINE | DASHED_LINE | USERBIT_LINE
        )
    }

    /// Check if fill style uses a pattern.
    pub fn is_patterned_fill(&self) -> bool {
        self.fill_style.pattern != SOLID_FILL && self.fill_style.pattern != EMPTY_FILL
    }

    /// Get effective line pattern for drawing.
    pub fn get_line_pattern(&self) -> u16 {
        match self.line_style.style {
            SOLID_LINE => 0xFFFF,
            DOTTED_LINE => 0xAAAA,
            CENTER_LINE => 0xF0F0,
            DASHED_LINE => 0xFF00,
            USERBIT_LINE => self.line_style.pattern,
            _ => 0xFFFF, // Default to solid
        }
    }
}
