//! Unit tests for input event handling.

use bgi::input_event::*;
use std::time::Instant;

#[test]
fn test_key_event_creation() {
    let event = KeyEvent::new(65, false); // 'A' key

    assert_eq!(event.key_code, 65);
    assert!(!event.extended);
    assert!(event.timestamp <= Instant::now());
}

#[test]
fn test_mouse_buttons_default() {
    let buttons = MouseButtons::default();

    assert!(!buttons.left);
    assert!(!buttons.right);
    assert!(!buttons.middle);
}

#[test]
fn test_mouse_event_creation() {
    let buttons = MouseButtons {
        left: true,
        right: false,
        middle: false,
    };
    let event = MouseEvent::new(100, 200, buttons);

    assert_eq!(event.x, 100);
    assert_eq!(event.y, 200);
    assert!(event.buttons.left);
    assert!(!event.buttons.right);
    assert!(!event.buttons.middle);
    assert!(event.timestamp <= Instant::now());
}

#[test]
fn test_mouse_click_creation() {
    let click = MouseClick::new(1, 150, 250);

    assert_eq!(click.button, 1);
    assert_eq!(click.x, 150);
    assert_eq!(click.y, 250);
    assert!(click.timestamp <= Instant::now());
}

#[test]
fn test_input_event_default() {
    let input = InputEvent::default();

    assert!(!input.key_hit());
    assert_eq!(input.mouse_x(), 0);
    assert_eq!(input.mouse_y(), 0);
    assert!(!input.mouse_button_pressed(1));
    assert!(!input.mouse_button_pressed(2));
    assert!(!input.mouse_button_pressed(3));
}

#[test]
fn test_input_event_new() {
    let input = InputEvent::new();
    let default_input = InputEvent::default();

    assert_eq!(input.mouse_x(), default_input.mouse_x());
    assert_eq!(input.mouse_y(), default_input.mouse_y());
    assert_eq!(input.key_hit(), default_input.key_hit());
}

#[test]
fn test_keyboard_events() {
    let mut input = InputEvent::new();

    // Initially no keys
    assert!(!input.key_hit());
    assert!(input.get_key().is_none());

    // Add a key event
    input.add_key_event(65, false); // 'A'
    assert!(input.key_hit());

    // Get the key
    let key = input.get_key();
    assert_eq!(key, Some(65));

    // Should be empty now
    assert!(!input.key_hit());
    assert!(input.get_key().is_none());

    // Add multiple keys
    input.add_key_event(66, false); // 'B'
    input.add_key_event(67, false); // 'C'

    assert!(input.key_hit());
    assert_eq!(input.get_key(), Some(66)); // FIFO order
    assert!(input.key_hit());
    assert_eq!(input.get_key(), Some(67));
    assert!(!input.key_hit());
}

#[test]
fn test_extended_keys() {
    let mut input = InputEvent::new();

    // Add extended key (function key)
    input.add_key_event(256, true); // Extended key

    let event = input.get_next_key();
    assert!(event.is_some());
    let event = event.unwrap();
    assert_eq!(event.key_code, 256);
    assert!(event.extended);
}

#[test]
fn test_mouse_position() {
    let mut input = InputEvent::new();

    // Test initial position
    assert_eq!(input.mouse_x(), 0);
    assert_eq!(input.mouse_y(), 0);
    assert_eq!(input.get_mouse_position(), (0, 0));

    // Update mouse position
    let buttons = MouseButtons::default();
    input.update_mouse(150, 250, buttons);

    assert_eq!(input.mouse_x(), 150);
    assert_eq!(input.mouse_y(), 250);
    assert_eq!(input.get_mouse_position(), (150, 250));

    // Test manual position setting
    input.set_mouse_position(300, 400);
    assert_eq!(input.mouse_x(), 300);
    assert_eq!(input.mouse_y(), 400);
}

#[test]
fn test_mouse_button_states() {
    let mut input = InputEvent::new();

    // Test all buttons pressed
    let buttons = MouseButtons {
        left: true,
        right: true,
        middle: true,
    };
    input.update_mouse(100, 200, buttons);

    assert!(input.mouse_button_pressed(1)); // Left
    assert!(input.mouse_button_pressed(2)); // Right
    assert!(input.mouse_button_pressed(3)); // Middle
    assert!(!input.mouse_button_pressed(4)); // Invalid button

    let (left, right, middle) = input.get_mouse_buttons();
    assert!(left);
    assert!(right);
    assert!(middle);

    let (x, y, l, r, m) = input.get_mouse_state();
    assert_eq!(x, 100);
    assert_eq!(y, 200);
    assert!(l);
    assert!(r);
    assert!(m);
}

#[test]
fn test_mouse_click_detection() {
    let mut input = InputEvent::new();

    // Initially no clicks
    assert!(!input.has_left_click());
    assert!(!input.has_right_click());
    assert!(!input.has_middle_click());

    // Start with buttons up
    let buttons_up = MouseButtons::default();
    input.update_mouse(100, 200, buttons_up);

    // Press left button (should generate click)
    let buttons_left = MouseButtons {
        left: true,
        right: false,
        middle: false,
    };
    input.update_mouse(100, 200, buttons_left);

    assert!(input.has_left_click());
    assert!(!input.has_right_click());
    assert!(!input.has_middle_click());

    // Check click information
    let left_click = input.get_last_left_click();
    assert!(left_click.is_some());
    let click = left_click.unwrap();
    assert_eq!(click.button, 1);
    assert_eq!(click.x, 100);
    assert_eq!(click.y, 200);

    // Consuming the click should clear it
    assert!(input.is_mouse_click(1));
    assert!(!input.is_mouse_click(1)); // Should be gone now
}

#[test]
fn test_multiple_button_clicks() {
    let mut input = InputEvent::new();

    // Start with all buttons up
    let buttons_up = MouseButtons::default();
    input.update_mouse(50, 60, buttons_up);

    // Press all buttons simultaneously
    let buttons_all = MouseButtons {
        left: true,
        right: true,
        middle: true,
    };
    input.update_mouse(50, 60, buttons_all);

    // Should generate clicks for all buttons
    assert!(input.has_left_click());
    assert!(input.has_right_click());
    assert!(input.has_middle_click());

    // Consume clicks in order
    assert!(input.is_mouse_click(1)); // Left
    assert!(input.is_mouse_click(2)); // Right
    assert!(input.is_mouse_click(3)); // Middle

    // All should be consumed
    assert!(!input.has_left_click());
    assert!(!input.has_right_click());
    assert!(!input.has_middle_click());
}

#[test]
fn test_click_clear_operations() {
    let mut input = InputEvent::new();

    // Generate some clicks
    let buttons_up = MouseButtons::default();
    input.update_mouse(100, 200, buttons_up);

    let buttons_all = MouseButtons {
        left: true,
        right: true,
        middle: true,
    };
    input.update_mouse(100, 200, buttons_all);

    // Clear individual button clicks
    input.clear_left_clicks();
    assert!(!input.has_left_click());
    assert!(input.has_right_click()); // Others should remain
    assert!(input.has_middle_click());

    input.clear_right_clicks();
    assert!(!input.has_right_click());
    assert!(input.has_middle_click()); // Middle should remain

    // Add more clicks
    input.update_mouse(100, 200, buttons_up);
    input.update_mouse(100, 200, buttons_all);

    // Clear all clicks
    input.clear_all_mouse_clicks();
    assert!(!input.has_left_click());
    assert!(!input.has_right_click());
    assert!(!input.has_middle_click());
}

#[test]
fn test_peek_vs_consume_clicks() {
    let mut input = InputEvent::new();

    // Generate a click
    let buttons_up = MouseButtons::default();
    input.update_mouse(100, 200, buttons_up);

    let buttons_left = MouseButtons {
        left: true,
        right: false,
        middle: false,
    };
    input.update_mouse(120, 180, buttons_left);

    // Peek at the click (shouldn't consume it)
    let click1 = input.peek_mouse_click(1);
    assert!(click1.is_some());
    assert_eq!(click1.unwrap().x, 120);
    assert_eq!(click1.unwrap().y, 180);

    // Should still be there
    let click2 = input.peek_mouse_click(1);
    assert!(click2.is_some());
    assert_eq!(click1.unwrap().x, click2.unwrap().x);

    // Now consume it
    assert!(input.is_mouse_click(1));

    // Should be gone
    assert!(input.peek_mouse_click(1).is_none());
    assert!(!input.is_mouse_click(1));
}

#[test]
fn test_input_event_stats() {
    let mut input = InputEvent::new();

    // Initial stats
    let (key_count, click_count) = input.get_stats();
    assert_eq!(key_count, 0);
    assert_eq!(click_count, 0);
    assert!(!input.has_pending_events());

    // Add some events
    input.add_key_event(65, false);
    input.add_key_event(66, false);

    let buttons_up = MouseButtons::default();
    input.update_mouse(100, 200, buttons_up);
    let buttons_left = MouseButtons {
        left: true,
        right: false,
        middle: false,
    };
    input.update_mouse(100, 200, buttons_left);

    let (key_count, click_count) = input.get_stats();
    assert_eq!(key_count, 2);
    assert_eq!(click_count, 1);
    assert!(input.has_pending_events());

    // Consume some events
    input.get_key();
    input.is_mouse_click(1);

    let (key_count, click_count) = input.get_stats();
    assert_eq!(key_count, 1);
    assert_eq!(click_count, 0);
    assert!(input.has_pending_events()); // Still one key

    // Consume remaining
    input.get_key();
    let (key_count, click_count) = input.get_stats();
    assert_eq!(key_count, 0);
    assert_eq!(click_count, 0);
    assert!(!input.has_pending_events());
}

#[test]
fn test_clear_all_events() {
    let mut input = InputEvent::new();

    // Add various events
    input.add_key_event(65, false);
    input.add_key_event(66, false);

    let buttons_up = MouseButtons::default();
    input.update_mouse(100, 200, buttons_up);
    let buttons_all = MouseButtons {
        left: true,
        right: true,
        middle: false,
    };
    input.update_mouse(100, 200, buttons_all);

    // Verify events exist
    assert!(input.has_pending_events());
    assert!(input.key_hit());
    assert!(input.has_left_click());

    // Clear all
    input.clear_all();

    // Verify everything is cleared
    assert!(!input.has_pending_events());
    assert!(!input.key_hit());
    assert!(!input.has_left_click());
    assert!(!input.has_right_click());
    assert!(!input.has_middle_click());

    let (key_count, click_count) = input.get_stats();
    assert_eq!(key_count, 0);
    assert_eq!(click_count, 0);
}

#[test]
fn test_has_key_event() {
    let mut input = InputEvent::new();

    assert!(!input.has_key_event());

    input.add_key_event(65, false);
    assert!(input.has_key_event());

    input.get_next_key();
    assert!(!input.has_key_event());
}

#[test]
fn test_no_duplicate_clicks_on_hold() {
    let mut input = InputEvent::new();

    // Start with button up
    let buttons_up = MouseButtons::default();
    input.update_mouse(100, 200, buttons_up);

    // Press button
    let buttons_down = MouseButtons {
        left: true,
        right: false,
        middle: false,
    };
    input.update_mouse(100, 200, buttons_down);

    assert!(input.has_left_click());

    // Hold button (multiple updates with same state)
    input.update_mouse(105, 205, buttons_down);
    input.update_mouse(110, 210, buttons_down);

    // Should only have one click
    let (_, click_count) = input.get_stats();
    assert_eq!(click_count, 1);

    // Consume the click
    assert!(input.is_mouse_click(1));
    assert!(!input.has_left_click());
}

#[test]
fn test_click_button_numbers() {
    let mut input = InputEvent::new();

    let buttons_up = MouseButtons::default();
    input.update_mouse(100, 200, buttons_up);

    // Test each button individually
    let buttons_left = MouseButtons {
        left: true,
        right: false,
        middle: false,
    };
    input.update_mouse(100, 200, buttons_left);

    let left_click = input.peek_mouse_click(1);
    assert!(left_click.is_some());
    assert_eq!(left_click.unwrap().button, 1);

    input.clear_all_mouse_clicks();

    let buttons_right = MouseButtons {
        left: false,
        right: true,
        middle: false,
    };
    input.update_mouse(100, 200, buttons_right);

    let right_click = input.peek_mouse_click(2);
    assert!(right_click.is_some());
    assert_eq!(right_click.unwrap().button, 2);

    input.clear_all_mouse_clicks();

    let buttons_middle = MouseButtons {
        left: false,
        right: false,
        middle: true,
    };
    input.update_mouse(100, 200, buttons_middle);

    let middle_click = input.peek_mouse_click(3);
    assert!(middle_click.is_some());
    assert_eq!(middle_click.unwrap().button, 3);
}
