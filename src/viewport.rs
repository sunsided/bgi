//! Viewport and coordinate system management.

use crate::types::Rect;

/// Viewport settings (compatible with BGI viewporttype).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Viewport {
    /// Left boundary.
    pub left: i32,
    /// Top boundary.
    pub top: i32,
    /// Right boundary.
    pub right: i32,
    /// Bottom boundary.
    pub bottom: i32,
    /// Clipping enabled.
    pub clip: bool,
}

impl Viewport {
    /// Create new viewport.
    pub const fn new(left: i32, top: i32, right: i32, bottom: i32, clip: bool) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
            clip,
        }
    }

    /// Create viewport from rectangle.
    pub const fn from_rect(rect: Rect, clip: bool) -> Self {
        Self::new(rect.left, rect.top, rect.right, rect.bottom, clip)
    }

    /// Convert viewport to rectangle.
    pub const fn to_rect(self) -> Rect {
        Rect::new(self.left, self.top, self.right, self.bottom)
    }

    /// Get viewport width.
    pub fn width(self) -> i32 {
        self.right.saturating_sub(self.left).abs()
    }

    /// Get viewport height.
    pub fn height(self) -> i32 {
        self.bottom.saturating_sub(self.top).abs()
    }

    /// Check if viewport contains a point.
    pub fn contains_point(self, x: i32, y: i32) -> bool {
        x >= self.left && x <= self.right && y >= self.top && y <= self.bottom
    }

    /// Clip coordinates to viewport if clipping is enabled.
    pub fn clip_coordinates(self, x: i32, y: i32) -> Option<(i32, i32)> {
        if self.clip && !self.contains_point(x, y) {
            None
        } else {
            Some((x, y))
        }
    }

    /// Transform screen coordinates to viewport coordinates.
    pub fn screen_to_viewport(self, x: i32, y: i32) -> (i32, i32) {
        (x - self.left, y - self.top)
    }

    /// Transform viewport coordinates to screen coordinates.
    pub fn viewport_to_screen(self, x: i32, y: i32) -> (i32, i32) {
        (x + self.left, y + self.top)
    }

    /// Check if viewport is valid.
    pub fn is_valid(self) -> bool {
        self.left <= self.right && self.top <= self.bottom
    }

    /// Normalize viewport (ensure left <= right, top <= bottom).
    pub fn normalize(mut self) -> Self {
        if self.left > self.right {
            std::mem::swap(&mut self.left, &mut self.right);
        }
        if self.top > self.bottom {
            std::mem::swap(&mut self.top, &mut self.bottom);
        }
        self
    }
}

impl Default for Viewport {
    fn default() -> Self {
        Self::new(0, 0, 639, 479, true) // Default VGA resolution
    }
}
