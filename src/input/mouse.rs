//! Mouse input handling for BGI library
//!
//! This module provides mouse event conversion and handling utilities.

use super::MouseButton;

/// Mouse button state tracking.
#[derive(Debug, Clone, Copy, Default)]
pub struct MouseButtonState {
    pub left: bool,
    pub right: bool,
    pub middle: bool,
}

impl MouseButtonState {
    /// Create a new MouseButtonState with all buttons released.
    pub fn new() -> Self {
        Self::default()
    }

    /// Update state when a button is pressed.
    pub fn press(&mut self, button: MouseButton) {
        match button {
            MouseButton::Left => self.left = true,
            MouseButton::Right => self.right = true,
            MouseButton::Middle => self.middle = true,
        }
    }

    /// Update state when a button is released.
    pub fn release(&mut self, button: MouseButton) {
        match button {
            MouseButton::Left => self.left = false,
            MouseButton::Right => self.right = false,
            MouseButton::Middle => self.middle = false,
        }
    }

    /// Check if any button is pressed.
    pub fn any_pressed(&self) -> bool {
        self.left || self.right || self.middle
    }

    /// Convert to BGI-style button mask.
    pub fn to_button_mask(&self) -> i32 {
        let mut mask = 0;
        if self.left {
            mask |= 0x01;
        }
        if self.right {
            mask |= 0x02;
        }
        if self.middle {
            mask |= 0x04;
        }
        mask
    }
}

/// Convert raw button code to MouseButton.
pub fn raw_to_button(raw: u32) -> Option<MouseButton> {
    match raw {
        0 | 1 => Some(MouseButton::Left),
        1 | 2 => Some(MouseButton::Right),
        2 | 4 => Some(MouseButton::Middle),
        _ => None,
    }
}
