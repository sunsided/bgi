//! Window management for the BGI library.

use crate::error::{BgiError, BgiResult};
use crate::types::GraphicsMode;

/// Window identifier type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WindowId(pub u32);

impl WindowId {
    /// Create a new window ID.
    pub const fn new(id: u32) -> Self {
        Self(id)
    }

    /// Get the raw ID value.
    pub fn raw(self) -> u32 {
        self.0
    }
}

/// Window configuration options.
#[derive(Debug, Clone)]
pub struct WindowOptions {
    /// Window title.
    pub title: String,
    /// X position (or -1 for default).
    pub x: i32,
    /// Y position (or -1 for default).
    pub y: i32,
    /// Additional flags.
    pub flags: u32,
}

impl Default for WindowOptions {
    fn default() -> Self {
        Self {
            title: "BGI Window".to_string(),
            x: -1,
            y: -1,
            flags: 0,
        }
    }
}

/// Window state and properties.
#[derive(Debug, Clone)]
pub struct Window {
    /// Window ID.
    pub id: WindowId,
    /// Window width.
    pub width: u32,
    /// Window height.
    pub height: u32,
    /// Window title.
    pub title: String,
    /// Graphics mode.
    pub mode: GraphicsMode,
    /// Window options.
    pub options: WindowOptions,
    /// Whether window is active.
    pub active: bool,
    /// Whether window is visible.
    pub visible: bool,
}

impl Window {
    /// Create a new window.
    pub fn new(id: WindowId, width: u32, height: u32, title: String, mode: GraphicsMode) -> Self {
        Self {
            id,
            width,
            height,
            title: title.clone(),
            mode,
            options: WindowOptions {
                title,
                ..Default::default()
            },
            active: false,
            visible: true,
        }
    }

    /// Set window title.
    pub fn set_title(&mut self, title: String) {
        self.title = title.clone();
        self.options.title = title;
    }

    /// Check if window has valid dimensions.
    pub fn is_valid(&self) -> bool {
        self.width > 0 && self.height > 0
    }

    /// Get aspect ratio.
    pub fn aspect_ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }
}
