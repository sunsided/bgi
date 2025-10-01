//! Input system for BGI library
//!
//! This module provides input event handling for keyboard and mouse events,
//! maintaining compatibility with BGI input functions while supporting modern
//! event-driven input patterns.

pub mod keyboard;
pub mod mouse;

use std::collections::VecDeque;

/// Input event types for BGI compatibility
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputEvent {
    Keyboard {
        key_code: KeyCode,
        ascii_code: u8,
        is_pressed: bool,
        modifiers: KeyModifiers,
    },
    Mouse {
        x: i16,           // BGI logical coordinates
        y: i16,           // BGI logical coordinates
        button: MouseButton,
        is_pressed: bool,
    },
    MouseMove {
        x: i16,           // BGI logical coordinates
        y: i16,           // BGI logical coordinates
    },
    WindowClose,
}

/// Virtual key codes for BGI compatibility
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyCode {
    // ASCII printable keys use ASCII values directly
    Ascii(u8),

    // Special keys
    Enter,
    Backspace,
    Tab,
    Escape,
    Space,

    // Arrow keys
    Up,
    Down,
    Left,
    Right,

    // Function keys
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10,

    // Other special keys
    Insert,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
}

/// Mouse button identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

/// Keyboard modifier state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyModifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
}

/// FIFO queue for managing input events
/// Maintains BGI-compatible ordering and size limits
pub struct InputEventQueue {
    events: VecDeque<InputEvent>,
    max_size: usize,
    mouse_position: (i16, i16), // Cache for mousex()/mousey()
}

impl InputEventQueue {
    /// Create new event queue with specified capacity
    pub fn new(max_size: usize) -> Self {
        Self {
            events: VecDeque::with_capacity(max_size.min(1000)),
            max_size,
            mouse_position: (0, 0),
        }
    }

    /// Add event to queue, dropping oldest if at capacity
    pub fn push_event(&mut self, event: InputEvent) {
        // Update mouse position cache for all mouse events
        match event {
            InputEvent::Mouse { x, y, .. } | InputEvent::MouseMove { x, y } => {
                self.mouse_position = (x, y);
            }
            _ => {}
        }

        // Deduplicate consecutive mouse moves
        if let InputEvent::MouseMove { x, y } = event {
            if let Some(InputEvent::MouseMove { .. }) = self.events.back() {
                // Replace the last mouse move with the new one
                self.events.pop_back();
            }
        }

        // Add event, dropping oldest if at capacity
        if self.events.len() >= self.max_size {
            self.events.pop_front();
        }
        self.events.push_back(event);
    }

    /// Remove and return next event
    pub fn pop_event(&mut self) -> Option<InputEvent> {
        self.events.pop_front()
    }

    /// Check if events are available
    pub fn has_events(&self) -> bool {
        !self.events.is_empty()
    }

    /// Get current mouse position (for mousex/mousey BGI functions)
    pub fn mouse_position(&self) -> (i16, i16) {
        self.mouse_position
    }

    /// Get number of queued events
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Check if queue is empty
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Clear all events
    pub fn clear(&mut self) {
        self.events.clear();
    }
}

impl Default for InputEventQueue {
    fn default() -> Self {
        Self::new(1000) // Default capacity
    }
}

/// Helper functions for BGI key code conversion
impl KeyCode {
    /// Convert to BGI-compatible key code
    pub fn to_bgi_code(self) -> u8 {
        match self {
            KeyCode::Ascii(code) => code,
            KeyCode::Enter => 13,
            KeyCode::Backspace => 8,
            KeyCode::Tab => 9,
            KeyCode::Escape => 27,
            KeyCode::Space => 32,
            KeyCode::Up => 0, // Extended key codes handled separately
            KeyCode::Down => 0,
            KeyCode::Left => 0,
            KeyCode::Right => 0,
            KeyCode::F1 => 0,
            KeyCode::F2 => 0,
            KeyCode::F3 => 0,
            KeyCode::F4 => 0,
            KeyCode::F5 => 0,
            KeyCode::F6 => 0,
            KeyCode::F7 => 0,
            KeyCode::F8 => 0,
            KeyCode::F9 => 0,
            KeyCode::F10 => 0,
            KeyCode::Insert => 0,
            KeyCode::Delete => 0,
            KeyCode::Home => 0,
            KeyCode::End => 0,
            KeyCode::PageUp => 0,
            KeyCode::PageDown => 0,
        }
    }

    /// Check if this is an extended key code (requires special handling in BGI)
    pub fn is_extended(self) -> bool {
        matches!(self,
            KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right |
            KeyCode::F1 | KeyCode::F2 | KeyCode::F3 | KeyCode::F4 | KeyCode::F5 |
            KeyCode::F6 | KeyCode::F7 | KeyCode::F8 | KeyCode::F9 | KeyCode::F10 |
            KeyCode::Insert | KeyCode::Delete | KeyCode::Home | KeyCode::End |
            KeyCode::PageUp | KeyCode::PageDown
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_queue_basic() {
        let mut queue = InputEventQueue::new(5);
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);

        let event = InputEvent::Keyboard {
            key_code: KeyCode::Ascii(b'A'),
            ascii_code: b'A',
            is_pressed: true,
            modifiers: KeyModifiers { shift: false, ctrl: false, alt: false },
        };

        queue.push_event(event);
        assert!(!queue.is_empty());
        assert_eq!(queue.len(), 1);
        assert!(queue.has_events());

        let popped = queue.pop_event();
        assert_eq!(popped, Some(event));
        assert!(queue.is_empty());
    }

    #[test]
    fn test_mouse_position_cache() {
        let mut queue = InputEventQueue::new(10);
        assert_eq!(queue.mouse_position(), (0, 0));

        queue.push_event(InputEvent::MouseMove { x: 100, y: 200 });
        assert_eq!(queue.mouse_position(), (100, 200));

        queue.push_event(InputEvent::Mouse {
            x: 150, y: 250,
            button: MouseButton::Left,
            is_pressed: true,
        });
        assert_eq!(queue.mouse_position(), (150, 250));
    }

    #[test]
    fn test_capacity_limit() {
        let mut queue = InputEventQueue::new(2);

        queue.push_event(InputEvent::Keyboard {
            key_code: KeyCode::Ascii(b'A'),
            ascii_code: b'A',
            is_pressed: true,
            modifiers: KeyModifiers::default(),
        });
        queue.push_event(InputEvent::Keyboard {
            key_code: KeyCode::Ascii(b'B'),
            ascii_code: b'B',
            is_pressed: true,
            modifiers: KeyModifiers::default(),
        });
        queue.push_event(InputEvent::Keyboard {
            key_code: KeyCode::Ascii(b'C'),
            ascii_code: b'C',
            is_pressed: true,
            modifiers: KeyModifiers::default(),
        });

        assert_eq!(queue.len(), 2);

        // First event should be dropped, second and third should remain
        let first = queue.pop_event().unwrap();
        if let InputEvent::Keyboard { ascii_code, .. } = first {
            assert_eq!(ascii_code, b'B');
        } else {
            panic!("Expected keyboard event");
        }
    }
}

impl Default for KeyModifiers {
    fn default() -> Self {
        Self {
            shift: false,
            ctrl: false,
            alt: false,
        }
    }
}
