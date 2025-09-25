//! Unit tests for drawing state management.

use bgi::{
    drawing_state::*,
    Color,
    constants::*,
};

#[test]
fn test_line_style_default() {
    let line_style = LineStyle::default();
    assert_eq!(line_style.style, SOLID_LINE);
    assert_eq!(line_style.pattern, 0xFFFF);
    assert_eq!(line_style.thickness, NORM_WIDTH);
}

#[test]
fn test_fill_style_default() {
    let fill_style = FillStyle::default();
    assert_eq!(fill_style.pattern, SOLID_FILL);
    assert_eq!(fill_style.color, Color::WHITE);
    assert!(fill_style.custom_pattern.is_none());
}

#[test]
fn test_text_justification_default() {
    let justification = TextJustification::default();
    assert_eq!(justification.horizontal, LEFT_TEXT);
    assert_eq!(justification.vertical, TOP_TEXT);
}

#[test]
fn test_position_default() {
    let position = Position::default();
    assert_eq!(position.x, 0);
    assert_eq!(position.y, 0);
}

#[test]
fn test_viewport_default() {
    let viewport = Viewport::default();
    assert_eq!(viewport.left, 0);
    assert_eq!(viewport.top, 0);
    assert_eq!(viewport.right, 639);
    assert_eq!(viewport.bottom, 479);
    assert!(viewport.clip);
}

#[test]
fn test_drawing_state_default() {
    let state = DrawingState::default();
    assert_eq!(state.color, Color::WHITE);
    assert_eq!(state.background_color, Color::BLACK);
    assert_eq!(state.line_style.style, SOLID_LINE);
    assert_eq!(state.fill_style.pattern, SOLID_FILL);
    assert_eq!(state.text_justification.horizontal, LEFT_TEXT);
    assert_eq!(state.position.x, 0);
    assert_eq!(state.position.y, 0);
    assert_eq!(state.write_mode, COPY_PUT);
}

#[test]
fn test_drawing_state_new() {
    let state = DrawingState::new();
    let default_state = DrawingState::default();

    assert_eq!(state.color, default_state.color);
    assert_eq!(state.background_color, default_state.background_color);
    assert_eq!(state.write_mode, default_state.write_mode);
}

#[test]
fn test_color_operations() {
    let mut state = DrawingState::new();

    // Test setting and getting color
    state.set_color(Color::RED);
    assert_eq!(state.get_color(), Color::RED);

    // Test setting and getting background color
    state.set_background_color(Color::BLUE);
    assert_eq!(state.get_background_color(), Color::BLUE);
}

#[test]
fn test_line_style_operations() {
    let mut state = DrawingState::new();

    // Test setting line style
    state.set_line_style(DOTTED_LINE, 0xAAAA, THICK_WIDTH);
    let (style, pattern, thickness) = state.get_line_style();

    assert_eq!(style, DOTTED_LINE);
    assert_eq!(pattern, 0xAAAA);
    assert_eq!(thickness, THICK_WIDTH);
}

#[test]
fn test_fill_style_operations() {
    let mut state = DrawingState::new();

    // Test setting predefined fill style
    state.set_fill_style(HATCH_FILL, Color::GREEN);
    let (pattern, color) = state.get_fill_style();

    assert_eq!(pattern, HATCH_FILL);
    assert_eq!(color, Color::GREEN);
    assert!(state.get_fill_pattern().is_none());

    // Test setting custom fill pattern
    let custom_pattern = [0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00];
    state.set_fill_pattern(&custom_pattern, Color::YELLOW);

    let (pattern, color) = state.get_fill_style();
    assert_eq!(pattern, USER_FILL);
    assert_eq!(color, Color::YELLOW);
    assert_eq!(state.get_fill_pattern(), Some(custom_pattern));
}

#[test]
fn test_text_justification_operations() {
    let mut state = DrawingState::new();

    state.set_text_justify(CENTER_TEXT, BOTTOM_TEXT);
    let (horizontal, vertical) = state.get_text_justify();

    assert_eq!(horizontal, CENTER_TEXT);
    assert_eq!(vertical, BOTTOM_TEXT);
}

#[test]
fn test_position_operations() {
    let mut state = DrawingState::new();

    // Test absolute move
    state.move_to(100, 200);
    let (x, y) = state.get_position();
    assert_eq!(x, 100);
    assert_eq!(y, 200);

    // Test relative move
    state.move_rel(50, -30);
    let (x, y) = state.get_position();
    assert_eq!(x, 150);
    assert_eq!(y, 170);

    // Test relative move with saturation
    state.move_to(i32::MAX - 10, i32::MAX - 10);
    state.move_rel(20, 20); // Should saturate
    let (x, y) = state.get_position();
    assert_eq!(x, i32::MAX);
    assert_eq!(y, i32::MAX);
}

#[test]
fn test_viewport_operations() {
    let mut state = DrawingState::new();

    state.set_viewport(10, 20, 300, 400, false);
    let viewport = state.get_viewport();

    assert_eq!(viewport.left, 10);
    assert_eq!(viewport.top, 20);
    assert_eq!(viewport.right, 300);
    assert_eq!(viewport.bottom, 400);
    assert!(!viewport.clip);
}

#[test]
fn test_write_mode_operations() {
    let mut state = DrawingState::new();

    state.set_write_mode(XOR_PUT);
    assert_eq!(state.get_write_mode(), XOR_PUT);

    state.set_write_mode(OR_PUT);
    assert_eq!(state.get_write_mode(), OR_PUT);
}

#[test]
fn test_reset_state() {
    let mut state = DrawingState::new();
    
    // Modify the state
    state.set_color(Color::RED);
    state.set_background_color(Color::BLUE);
    state.move_to(100, 200);
    state.set_write_mode(XOR_PUT);
    
    // Reset and verify defaults
    state.reset();

    assert_eq!(state.color, Color::WHITE);
    assert_eq!(state.background_color, Color::BLACK);
    assert_eq!(state.position.x, 0);
    assert_eq!(state.position.y, 0);
    assert_eq!(state.write_mode, COPY_PUT);
}

#[test]
fn test_pattern_checks() {
    let mut state = DrawingState::new();

    // Test solid line
    state.set_line_style(SOLID_LINE, 0xFFFF, NORM_WIDTH);
    assert!(!state.is_patterned_line());

    // Test patterned lines
    state.set_line_style(DOTTED_LINE, 0xAAAA, NORM_WIDTH);
    assert!(state.is_patterned_line());

    state.set_line_style(DASHED_LINE, 0xFF00, NORM_WIDTH);
    assert!(state.is_patterned_line());

    state.set_line_style(USERBIT_LINE, 0x5555, NORM_WIDTH);
    assert!(state.is_patterned_line());

    // Test solid fill
    state.set_fill_style(SOLID_FILL, Color::RED);
    assert!(!state.is_patterned_fill());

    state.set_fill_style(EMPTY_FILL, Color::RED);
    assert!(!state.is_patterned_fill());

    // Test patterned fill
    state.set_fill_style(HATCH_FILL, Color::GREEN);
    assert!(state.is_patterned_fill());
}

#[test]
fn test_line_pattern_generation() {
    let mut state = DrawingState::new();

    // Test different line patterns
    state.set_line_style(SOLID_LINE, 0x1234, NORM_WIDTH);
    assert_eq!(state.get_line_pattern(), 0xFFFF);

    state.set_line_style(DOTTED_LINE, 0x1234, NORM_WIDTH);
    assert_eq!(state.get_line_pattern(), 0xAAAA);

    state.set_line_style(CENTER_LINE, 0x1234, NORM_WIDTH);
    assert_eq!(state.get_line_pattern(), 0xF0F0);

    state.set_line_style(DASHED_LINE, 0x1234, NORM_WIDTH);
    assert_eq!(state.get_line_pattern(), 0xFF00);

    state.set_line_style(USERBIT_LINE, 0x5A5A, NORM_WIDTH);
    assert_eq!(state.get_line_pattern(), 0x5A5A);

    // Test unknown pattern defaults to solid
    state.line_style.style = 999; // Invalid style
    assert_eq!(state.get_line_pattern(), 0xFFFF);
}

#[test]
fn test_structures_equality() {
    let line1 = LineStyle { style: SOLID_LINE, pattern: 0xFFFF, thickness: NORM_WIDTH };
    let line2 = LineStyle { style: SOLID_LINE, pattern: 0xFFFF, thickness: NORM_WIDTH };
    let line3 = LineStyle { style: DOTTED_LINE, pattern: 0xFFFF, thickness: NORM_WIDTH };

    assert_eq!(line1, line2);
    assert_ne!(line1, line3);

    let pos1 = Position { x: 10, y: 20 };
    let pos2 = Position { x: 10, y: 20 };
    let pos3 = Position { x: 10, y: 21 };

    assert_eq!(pos1, pos2);
    assert_ne!(pos1, pos3);

    let viewport1 = Viewport { left: 0, top: 0, right: 100, bottom: 100, clip: true };
    let viewport2 = Viewport { left: 0, top: 0, right: 100, bottom: 100, clip: true };
    let viewport3 = Viewport { left: 0, top: 0, right: 100, bottom: 100, clip: false };

    assert_eq!(viewport1, viewport2);
    assert_ne!(viewport1, viewport3);
}

#[test]
fn test_drawing_state_clone() {
    let mut state1 = DrawingState::new();
    state1.set_color(Color::RED);
    state1.move_to(50, 75);

    let state2 = state1.clone();

    assert_eq!(state1.get_color(), state2.get_color());
    assert_eq!(state1.get_position(), state2.get_position());

    // Verify they're independent
    state1.set_color(Color::BLUE);
    assert_eq!(state1.get_color(), Color::BLUE);
    assert_eq!(state2.get_color(), Color::RED);
}
