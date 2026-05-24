//! Input event handling for keyboard and mouse interactions.

use crate::constants::*;
use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// Keyboard event information.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyEvent {
    /// The key code (ASCII or extended key code)
    pub key_code: i32,
    /// Whether this is an extended key (function keys, arrows, etc.)
    pub extended: bool,
    /// Timestamp when the key was pressed
    pub timestamp: Instant,
}

impl KeyEvent {
    pub fn new(key_code: i32, extended: bool) -> Self {
        Self {
            key_code,
            extended,
            timestamp: Instant::now(),
        }
    }
}

/// Mouse button states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MouseButtons {
    /// Left mouse button pressed
    pub left: bool,
    /// Right mouse button pressed
    pub right: bool,
    /// Middle mouse button pressed
    pub middle: bool,
}

/// Mouse event information.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MouseEvent {
    /// Mouse X coordinate
    pub x: i32,
    /// Mouse Y coordinate
    pub y: i32,
    /// Button states
    pub buttons: MouseButtons,
    /// Timestamp when the event occurred
    pub timestamp: Instant,
}

impl MouseEvent {
    pub fn new(x: i32, y: i32, buttons: MouseButtons) -> Self {
        Self {
            x,
            y,
            buttons,
            timestamp: Instant::now(),
        }
    }
}

/// Mouse click detection for individual buttons.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MouseClick {
    /// Button number (1=left, 2=right, 3=middle)
    pub button: i32,
    /// Click position
    pub x: i32,
    pub y: i32,
    /// Timestamp of the click
    pub timestamp: Instant,
}

impl MouseClick {
    pub fn new(button: i32, x: i32, y: i32) -> Self {
        Self {
            button,
            x,
            y,
            timestamp: Instant::now(),
        }
    }
}

/// Input event manager for BGI.
#[derive(Debug)]
pub struct InputEvent {
    /// Keyboard event queue
    key_queue: VecDeque<KeyEvent>,
    /// Current mouse state
    mouse_state: MouseEvent,
    /// Previous mouse state (for click detection)
    prev_mouse_state: MouseEvent,
    /// Mouse click history for ismouseclick() function
    mouse_clicks: VecDeque<MouseClick>,
    /// Maximum age of events to keep
    max_event_age: Duration,
    /// Current keyboard hit status
    key_available: bool,
}

impl Default for InputEvent {
    fn default() -> Self {
        let now = Instant::now();
        let default_mouse = MouseEvent::new(0, 0, MouseButtons::default());

        Self {
            key_queue: VecDeque::new(),
            mouse_state: default_mouse,
            prev_mouse_state: default_mouse,
            mouse_clicks: VecDeque::new(),
            max_event_age: Duration::from_secs(5), // Keep events for 5 seconds
            key_available: false,
        }
    }
}

impl InputEvent {
    /// Create a new input event manager.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a keyboard event.
    pub fn add_key_event(&mut self, key_code: i32, extended: bool) {
        let event = KeyEvent::new(key_code, extended);
        self.key_queue.push_back(event);
        self.key_available = true;

        // Clean up old events
        self.cleanup_old_events();
    }

    /// Check if a key is available (kbhit equivalent).
    pub fn key_hit(&self) -> bool {
        self.key_available && !self.key_queue.is_empty()
    }

    /// Get the next key from the queue (getch equivalent).
    pub fn get_key(&mut self) -> Option<i32> {
        if let Some(event) = self.key_queue.pop_front() {
            if self.key_queue.is_empty() {
                self.key_available = false;
            }
            Some(event.key_code)
        } else {
            None
        }
    }

    /// Update mouse state.
    pub fn update_mouse(&mut self, x: i32, y: i32, buttons: MouseButtons) {
        self.prev_mouse_state = self.mouse_state;
        self.mouse_state = MouseEvent::new(x, y, buttons);

        // Detect button clicks (transition from up to down)
        self.detect_mouse_clicks();

        // Clean up old events
        self.cleanup_old_events();
    }

    /// Get current mouse X coordinate.
    pub fn mouse_x(&self) -> i32 {
        self.mouse_state.x
    }

    /// Get current mouse Y coordinate.
    pub fn mouse_y(&self) -> i32 {
        self.mouse_state.y
    }

    /// Check if a mouse button is currently pressed.
    pub fn mouse_button_pressed(&self, button: i32) -> bool {
        match button {
            1 => self.mouse_state.buttons.left,
            2 => self.mouse_state.buttons.right,
            3 => self.mouse_state.buttons.middle,
            _ => false,
        }
    }

    /// Check if a mouse button was clicked (and clear the click state).
    pub fn is_mouse_click(&mut self, button: i32) -> bool {
        // Find and remove the first click for this button
        if let Some(pos) = self
            .mouse_clicks
            .iter()
            .position(|click| click.button == button)
        {
            self.mouse_clicks.remove(pos);
            true
        } else {
            false
        }
    }

    /// Get mouse click information without clearing it.
    pub fn peek_mouse_click(&self, button: i32) -> Option<MouseClick> {
        self.mouse_clicks
            .iter()
            .find(|click| click.button == button)
            .copied()
    }

    /// Clear all mouse clicks for a specific button.
    pub fn clear_mouse_clicks(&mut self, button: i32) {
        self.mouse_clicks.retain(|click| click.button != button);
    }

    /// Clear all mouse clicks.
    pub fn clear_all_mouse_clicks(&mut self) {
        self.mouse_clicks.clear();
    }

    /// Get current mouse position.
    pub fn get_mouse_position(&self) -> (i32, i32) {
        (self.mouse_state.x, self.mouse_state.y)
    }

    /// Set mouse position.
    pub fn set_mouse_position(&mut self, x: i32, y: i32) {
        self.mouse_state.x = x;
        self.mouse_state.y = y;
    }

    /// Get mouse button states (left, right, middle).
    pub fn get_mouse_buttons(&self) -> (bool, bool, bool) {
        (
            self.mouse_state.buttons.left,
            self.mouse_state.buttons.right,
            self.mouse_state.buttons.middle,
        )
    }

    /// Check if left mouse button was clicked.
    pub fn has_left_click(&self) -> bool {
        self.mouse_clicks.iter().any(|click| click.button == 1)
    }

    /// Check if right mouse button was clicked.
    pub fn has_right_click(&self) -> bool {
        self.mouse_clicks.iter().any(|click| click.button == 2)
    }

    /// Check if middle mouse button was clicked.
    pub fn has_middle_click(&self) -> bool {
        self.mouse_clicks.iter().any(|click| click.button == 3)
    }

    /// Get last left click information.
    pub fn get_last_left_click(&self) -> Option<MouseClick> {
        self.peek_mouse_click(1)
    }

    /// Get last right click information.
    pub fn get_last_right_click(&self) -> Option<MouseClick> {
        self.peek_mouse_click(2)
    }

    /// Get last middle click information.
    pub fn get_last_middle_click(&self) -> Option<MouseClick> {
        self.peek_mouse_click(4)
    }

    /// Clear left mouse clicks.
    pub fn clear_left_clicks(&mut self) {
        self.clear_mouse_clicks(1);
    }

    /// Clear right mouse clicks.
    pub fn clear_right_clicks(&mut self) {
        self.clear_mouse_clicks(2);
    }

    /// Clear middle mouse clicks.
    pub fn clear_middle_clicks(&mut self) {
        self.clear_mouse_clicks(4);
    }

    /// Check if there are any keyboard events.
    pub fn has_key_event(&self) -> bool {
        !self.key_queue.is_empty()
    }

    /// Get next keyboard event (consuming it).
    pub fn get_next_key(&mut self) -> Option<KeyEvent> {
        self.key_queue.pop_front()
    }

    /// Get complete mouse state (position and all buttons).
    pub fn get_mouse_state(&self) -> (i32, i32, bool, bool, bool) {
        (
            self.mouse_state.x,
            self.mouse_state.y,
            self.mouse_state.buttons.left,
            self.mouse_state.buttons.right,
            self.mouse_state.buttons.middle,
        )
    }

    /// Clear all input events.
    pub fn clear_all(&mut self) {
        self.key_queue.clear();
        self.mouse_clicks.clear();
        self.key_available = false;
    }

    /// Clean up old events to prevent memory leaks.
    fn cleanup_old_events(&mut self) {
        let now = Instant::now();
        let cutoff = now - self.max_event_age;

        // Remove old key events
        self.key_queue.retain(|event| event.timestamp > cutoff);

        // Remove old mouse clicks
        self.mouse_clicks.retain(|click| click.timestamp > cutoff);

        // Update key availability
        if self.key_queue.is_empty() {
            self.key_available = false;
        }
    }

    /// Detect mouse button clicks (transition from up to down).
    fn detect_mouse_clicks(&mut self) {
        let prev = &self.prev_mouse_state.buttons;
        let curr = &self.mouse_state.buttons;

        // Left button click
        if !prev.left && curr.left {
            let click = MouseClick::new(1, self.mouse_state.x, self.mouse_state.y);
            self.mouse_clicks.push_back(click);
        }

        // Right button click
        if !prev.right && curr.right {
            let click = MouseClick::new(2, self.mouse_state.x, self.mouse_state.y);
            self.mouse_clicks.push_back(click);
        }

        // Middle button click
        if !prev.middle && curr.middle {
            let click = MouseClick::new(3, self.mouse_state.x, self.mouse_state.y);
            self.mouse_clicks.push_back(click);
        }
    }

    /// Get statistics about input events.
    pub fn get_stats(&self) -> (usize, usize) {
        (self.key_queue.len(), self.mouse_clicks.len())
    }

    /// Check if any input events are pending.
    pub fn has_pending_events(&self) -> bool {
        !self.key_queue.is_empty() || !self.mouse_clicks.is_empty()
    }
}
