//! Keyboard input handling for BGI library
//!
//! This module provides keyboard event conversion and handling utilities.

use super::KeyCode;

/// Convert a raw key code to BGI KeyCode.
pub fn raw_to_keycode(raw: u32) -> Option<KeyCode> {
    // Placeholder conversion - will be implemented with backend integration
    match raw {
        0x1B => Some(KeyCode::Escape),
        0x0D => Some(KeyCode::Enter),
        0x08 => Some(KeyCode::Backspace),
        0x09 => Some(KeyCode::Tab),
        0x20 => Some(KeyCode::Space),
        // Arrow keys (common raw codes)
        0x48 => Some(KeyCode::Up),
        0x50 => Some(KeyCode::Down),
        0x4B => Some(KeyCode::Left),
        0x4D => Some(KeyCode::Right),
        // ASCII printable range
        c @ 0x20..=0x7E => Some(KeyCode::Ascii(c as u8)),
        _ => None,
    }
}

/// Convert BGI KeyCode to ASCII character (if possible).
pub fn keycode_to_ascii(key: KeyCode) -> Option<u8> {
    match key {
        KeyCode::Ascii(c) => Some(c),
        KeyCode::Enter => Some(0x0D),
        KeyCode::Tab => Some(0x09),
        KeyCode::Space => Some(0x20),
        KeyCode::Backspace => Some(0x08),
        KeyCode::Escape => Some(0x1B),
        _ => None,
    }
}

/// Check if a KeyCode represents a printable ASCII character.
pub fn is_printable(key: KeyCode) -> bool {
    matches!(key, KeyCode::Ascii(c) if (0x20..=0x7E).contains(&c))
}
