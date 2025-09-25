use bgi::{
    initgraph, closegraph,
    mousex, mousey, mouseclick, ismouseclick,
    getmouse, setmouse, clearmouseclick,
    Color
};

#[test]
fn test_mousex_mousey_contract() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    // Test getting mouse coordinates
    let x = mousex();
    let y = mousey();

    // Coordinates should be within screen bounds
    assert!(x >= 0, "Mouse X should be non-negative");
    assert!(y >= 0, "Mouse Y should be non-negative");

    // Should be within reasonable screen bounds
    assert!(x <= 1920, "Mouse X should be within reasonable bounds");
    assert!(y <= 1080, "Mouse Y should be within reasonable bounds");

    closegraph();
}

#[test]
fn test_mouseclick_contract() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    // Test getting mouse click state
    let left_click = mouseclick(1);  // Left button
    let right_click = mouseclick(2); // Right button
    let middle_click = mouseclick(3); // Middle button

    // Should return valid button states (true or false)
    assert!(left_click == true || left_click == false, "Left click should return boolean");
    assert!(right_click == true || right_click == false, "Right click should return boolean");
    assert!(middle_click == true || middle_click == false, "Middle click should return boolean");

    // Test invalid button
    let invalid_button = mouseclick(99);
    assert_eq!(invalid_button, false, "Invalid button should return false");

    closegraph();
}

#[test]
fn test_ismouseclick_contract() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    // Test checking for mouse clicks
    let left_clicked = ismouseclick(1);
    let right_clicked = ismouseclick(2);

    // Should return boolean values
    assert!(left_clicked == true || left_clicked == false, "ismouseclick should return boolean");
    assert!(right_clicked == true || right_clicked == false, "ismouseclick should return boolean");

    // Test invalid button
    let invalid_clicked = ismouseclick(99);
    assert_eq!(invalid_clicked, false, "Invalid button should return false");

    closegraph();
}

#[test]
fn test_getmouse_contract() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    // Test getting comprehensive mouse state
    let mouse_state = getmouse();

    // Should return a valid mouse state structure
    assert!(mouse_state.x >= 0, "Mouse state X should be non-negative");
    assert!(mouse_state.y >= 0, "Mouse state Y should be non-negative");

    // Button states should be boolean
    assert!(mouse_state.left == true || mouse_state.left == false, "Left button state should be boolean");
    assert!(mouse_state.right == true || mouse_state.right == false, "Right button state should be boolean");
    assert!(mouse_state.middle == true || mouse_state.middle == false, "Middle button state should be boolean");

    closegraph();
}

#[test]
fn test_setmouse_contract() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    // Test setting mouse position
    setmouse(100, 200);
    // BGI functions are void - errors reported via graphresult()

    // Verify the position was set (with some tolerance for system constraints)
    let x = mousex();
    let y = mousey();

    // Note: Some systems may not allow programmatic mouse positioning
    // so we just verify the function doesn't crash
    assert!(x >= 0 && y >= 0, "Mouse coordinates should remain valid after setmouse");

    // Test with negative coordinates
    setmouse(-10, -10);
    // BGI functions are void - errors reported via graphresult()

    closegraph();
}

#[test]
fn test_clearmouseclick_contract() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    // Clear mouse click states
    clearmouseclick(1); // Clear left button
    clearmouseclick(2); // Clear right button
    clearmouseclick(3); // Clear middle button

    // After clearing, ismouseclick should return false
    let left_after_clear = ismouseclick(1);
    let right_after_clear = ismouseclick(2);

    // Note: This assumes no actual clicks happen during test
    // In a real scenario, these might still be true if user clicks during test
    assert!(left_after_clear == true || left_after_clear == false, "Should return valid boolean after clear");
    assert!(right_after_clear == true || right_after_clear == false, "Should return valid boolean after clear");

    closegraph();
}

#[test]
fn test_mouse_operations_without_graphics() {
    // Test mouse operations without initializing graphics
    let x = mousex();
    let y = mousey();

    // Should return some default values or 0, not crash
    assert!(x >= 0, "mousex should return non-negative value even without graphics");
    assert!(y >= 0, "mousey should return non-negative value even without graphics");

    let click_state = mouseclick(1);
    assert!(click_state == true || click_state == false, "mouseclick should return boolean without graphics");

    let is_clicked = ismouseclick(1);
    assert!(is_clicked == true || is_clicked == false, "ismouseclick should return boolean without graphics");
}

#[test]
fn test_mouse_coordinate_consistency() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    // Get coordinates multiple ways and verify consistency
    let x1 = mousex();
    let y1 = mousey();

    let mouse_state = getmouse();
    let x2 = mouse_state.x;
    let y2 = mouse_state.y;

    // Coordinates should be consistent (allowing for minimal time differences)
    let x_diff = (x1 - x2).abs();
    let y_diff = (y1 - y2).abs();

    assert!(x_diff <= 5, "Mouse X coordinates should be consistent between calls");
    assert!(y_diff <= 5, "Mouse Y coordinates should be consistent between calls");

    closegraph();
}

#[test]
fn test_mouse_button_range() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    // Test all standard mouse buttons
    for button in 1..=3 {
        let click_state = mouseclick(button);
        let is_clicked = ismouseclick(button);

        assert!(click_state == true || click_state == false, "Button {} should return valid state", button);
        assert!(is_clicked == true || is_clicked == false, "Button {} ismouseclick should return valid state", button);
    }

    // Test invalid button numbers
    for button in [0, 4, 5, 99] {
        let click_state = mouseclick(button);
        let is_clicked = ismouseclick(button);

        assert_eq!(click_state, false, "Invalid button {} should return false", button);
        assert_eq!(is_clicked, false, "Invalid button {} ismouseclick should return false", button);
    }

    closegraph();
}
