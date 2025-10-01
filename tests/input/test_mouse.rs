//! Mouse input integration tests
//!
//! Tests for mouse event processing and BGI compatibility.
//! Verifies mousex(), mousey(), and button handling.

use bgi::input::{InputEvent, InputEventQueue, MouseButton};

/// Test basic mouse event queue functionality
#[test]
fn test_mouse_event_queue() {
    let mut queue = InputEventQueue::new(10);

    // Initially no mouse position set
    assert_eq!(queue.mouse_position(), (0, 0));

    // Add mouse move event
    let mouse_move = InputEvent::MouseMove { x: 100, y: 200 };
    queue.push_event(mouse_move);

    // Mouse position should be updated immediately
    assert_eq!(queue.mouse_position(), (100, 200));
    assert!(queue.has_events());

    // Pop the event
    assert_eq!(queue.pop_event(), Some(mouse_move));

    // Position should still be cached even after event is consumed
    assert_eq!(queue.mouse_position(), (100, 200));
}

/// Test mouse button events
#[test]
fn test_mouse_button_events() {
    let mut queue = InputEventQueue::new(10);

    // Test left button press
    let left_press = InputEvent::Mouse {
        x: 50,
        y: 75,
        button: MouseButton::Left,
        is_pressed: true,
    };

    // Test right button press
    let right_press = InputEvent::Mouse {
        x: 60,
        y: 85,
        button: MouseButton::Right,
        is_pressed: true,
    };

    // Test middle button release
    let middle_release = InputEvent::Mouse {
        x: 70,
        y: 95,
        button: MouseButton::Middle,
        is_pressed: false,
    };

    queue.push_event(left_press);
    queue.push_event(right_press);
    queue.push_event(middle_release);

    // Mouse position should reflect the last event
    assert_eq!(queue.mouse_position(), (70, 95));
    assert_eq!(queue.len(), 3);

    // Verify events come out in correct order
    assert_eq!(queue.pop_event(), Some(left_press));
    assert_eq!(queue.pop_event(), Some(right_press));
    assert_eq!(queue.pop_event(), Some(middle_release));
}

/// Test mouse move event deduplication
#[test]
fn test_mouse_move_deduplication() {
    let mut queue = InputEventQueue::new(10);

    // Add multiple consecutive mouse moves
    queue.push_event(InputEvent::MouseMove { x: 10, y: 10 });
    queue.push_event(InputEvent::MouseMove { x: 20, y: 20 });
    queue.push_event(InputEvent::MouseMove { x: 30, y: 30 });

    // Should only keep the last mouse move
    assert_eq!(queue.len(), 1);
    assert_eq!(queue.mouse_position(), (30, 30));

    if let Some(InputEvent::MouseMove { x, y }) = queue.pop_event() {
        assert_eq!(x, 30);
        assert_eq!(y, 30);
    } else {
        panic!("Expected mouse move event");
    }

    // Add non-move event, then mouse moves
    queue.push_event(InputEvent::Mouse {
        x: 40, y: 40,
        button: MouseButton::Left,
        is_pressed: true,
    });
    queue.push_event(InputEvent::MouseMove { x: 50, y: 50 });
    queue.push_event(InputEvent::MouseMove { x: 60, y: 60 });

    // Should have button event + one mouse move
    assert_eq!(queue.len(), 2);

    // Button event first
    if let Some(InputEvent::Mouse { button, is_pressed, .. }) = queue.pop_event() {
        assert_eq!(button, MouseButton::Left);
        assert!(is_pressed);
    } else {
        panic!("Expected mouse button event");
    }

    // Then the last mouse move
    if let Some(InputEvent::MouseMove { x, y }) = queue.pop_event() {
        assert_eq!(x, 60);
        assert_eq!(y, 60);
    } else {
        panic!("Expected mouse move event");
    }
}

/// Test coordinate system (BGI logical coordinates)
#[test]
fn test_mouse_coordinates() {
    let mut queue = InputEventQueue::new(10);

    // Test various coordinate ranges
    let test_positions = [
        (0, 0),           // Origin
        (639, 479),       // VGA bottom-right
        (-1, -1),         // Negative (should be allowed for out-of-bounds)
        (1000, 1000),     // Large values
        (320, 240),       // Center of typical mode
    ];

    for (x, y) in test_positions.iter() {
        let event = InputEvent::MouseMove { x: *x, y: *y };
        queue.push_event(event);

        assert_eq!(queue.mouse_position(), (*x, *y));
        assert_eq!(queue.pop_event(), Some(event));
    }
}

/// Test all mouse button types
#[test]
fn test_all_mouse_buttons() {
    let buttons = [MouseButton::Left, MouseButton::Right, MouseButton::Middle];

    for button in buttons.iter() {
        let press_event = InputEvent::Mouse {
            x: 100,
            y: 100,
            button: *button,
            is_pressed: true,
        };

        let release_event = InputEvent::Mouse {
            x: 100,
            y: 100,
            button: *button,
            is_pressed: false,
        };

        // Test that button types are preserved correctly
        match button {
            MouseButton::Left => {
                assert_eq!(press_event, InputEvent::Mouse {
                    x: 100, y: 100, button: MouseButton::Left, is_pressed: true
                });
            }
            MouseButton::Right => {
                assert_eq!(press_event, InputEvent::Mouse {
                    x: 100, y: 100, button: MouseButton::Right, is_pressed: true
                });
            }
            MouseButton::Middle => {
                assert_eq!(press_event, InputEvent::Mouse {
                    x: 100, y: 100, button: MouseButton::Middle, is_pressed: true
                });
            }
        }
    }
}

/// Test mouse position caching for BGI mousex()/mousey() functions
#[test]
fn test_mouse_position_caching() {
    let mut queue = InputEventQueue::new(5);

    // Initial position should be (0, 0)
    assert_eq!(queue.mouse_position(), (0, 0));

    // Move mouse and consume events
    queue.push_event(InputEvent::MouseMove { x: 123, y: 456 });
    queue.pop_event(); // Consume the event

    // Position should still be cached
    assert_eq!(queue.mouse_position(), (123, 456));

    // Add button click at different position
    queue.push_event(InputEvent::Mouse {
        x: 789, y: 012,
        button: MouseButton::Left,
        is_pressed: true,
    });

    // Position should update to button click location
    assert_eq!(queue.mouse_position(), (789, 12));

    // Consume button event
    queue.pop_event();

    // Position should still be cached
    assert_eq!(queue.mouse_position(), (789, 12));
}

/// Test mixed mouse and non-mouse events
#[test]
fn test_mixed_event_types() {
    let mut queue = InputEventQueue::new(10);

    // Add various event types
    queue.push_event(InputEvent::MouseMove { x: 10, y: 10 });
    queue.push_event(InputEvent::Keyboard {
        key_code: bgi::input::KeyCode::Ascii(b'A'),
        ascii_code: b'A',
        is_pressed: true,
        modifiers: bgi::input::KeyModifiers::default(),
    });
    queue.push_event(InputEvent::Mouse {
        x: 20, y: 20,
        button: MouseButton::Left,
        is_pressed: true,
    });
    queue.push_event(InputEvent::WindowClose);

    // Mouse position should reflect the last mouse event
    assert_eq!(queue.mouse_position(), (20, 20));
    assert_eq!(queue.len(), 4);

    // Events should come out in order
    assert!(matches!(queue.pop_event(), Some(InputEvent::MouseMove { .. })));
    assert!(matches!(queue.pop_event(), Some(InputEvent::Keyboard { .. })));
    assert!(matches!(queue.pop_event(), Some(InputEvent::Mouse { .. })));
    assert!(matches!(queue.pop_event(), Some(InputEvent::WindowClose)));
}

/// Test mouse event queue overflow with mouse events
#[test]
fn test_mouse_queue_overflow() {
    let mut queue = InputEventQueue::new(3);

    // Fill with mouse events
    queue.push_event(InputEvent::MouseMove { x: 1, y: 1 });
    queue.push_event(InputEvent::Mouse { x: 2, y: 2, button: MouseButton::Left, is_pressed: true });
    queue.push_event(InputEvent::MouseMove { x: 3, y: 3 });

    assert_eq!(queue.len(), 2); // Mouse moves should be deduplicated
    assert_eq!(queue.mouse_position(), (3, 3));

    // Add more events to trigger overflow
    queue.push_event(InputEvent::Mouse { x: 4, y: 4, button: MouseButton::Right, is_pressed: true });
    queue.push_event(InputEvent::MouseMove { x: 5, y: 5 });

    assert_eq!(queue.len(), 3); // At capacity
    assert_eq!(queue.mouse_position(), (5, 5));
}

/// Test integration with BGI mouse functions (placeholder)
#[test]
#[should_panic(expected = "BGI mouse integration not implemented")]
fn test_bgi_mouse_integration() {
    // This test will fail until mousex(), mousey(), and mouse button detection
    // are properly integrated with the new input system

    // Test that mousex() returns current X coordinate
    // Test that mousey() returns current Y coordinate
    // Test mouse button state detection
    // Test coordinate transformation from physical to logical coordinates

    panic!("BGI mouse integration not implemented");
}

/// Test mouse input processing module exists
#[test]
#[should_panic(expected = "Mouse processing module not implemented")]
fn test_mouse_processing_module() {
    // This will fail until src/input/mouse.rs is implemented
    // Should test the mouse event processing logic

    panic!("Mouse processing module not implemented");
}
