use bgi::{
    clearmouseclick, closegraph, getmouse, initgraph, ismouseclick, mouseclick, mousex, mousey,
    setmouse,
};

#[test]
fn test_mousex_mousey_contract() {
    let mut driver = 9; // VGA
    let mut mode = 2; // VGAHI
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
    let mut mode = 2; // VGAHI
    initgraph(&mut driver, &mut mode, "");

    // Querying valid buttons should not panic (return type guarantees a bool).
    let _ = mouseclick(1); // Left button
    let _ = mouseclick(2); // Right button
    let _ = mouseclick(3); // Middle button

    // Test invalid button
    let invalid_button = mouseclick(99);
    assert!(!invalid_button, "Invalid button should return false");

    closegraph();
}

#[test]
fn test_ismouseclick_contract() {
    let mut driver = 9; // VGA
    let mut mode = 2; // VGAHI
    initgraph(&mut driver, &mut mode, "");

    // Checking valid buttons should not panic (return type guarantees a bool).
    let _ = ismouseclick(1);
    let _ = ismouseclick(2);

    // Test invalid button
    let invalid_clicked = ismouseclick(99);
    assert!(!invalid_clicked, "Invalid button should return false");

    closegraph();
}

#[test]
fn test_getmouse_contract() {
    let mut driver = 9; // VGA
    let mut mode = 2; // VGAHI
    initgraph(&mut driver, &mut mode, "");

    // Test getting comprehensive mouse state
    let mouse_state = getmouse();

    // Should return a valid mouse state structure
    assert!(mouse_state.x >= 0, "Mouse state X should be non-negative");
    assert!(mouse_state.y >= 0, "Mouse state Y should be non-negative");

    closegraph();
}

#[test]
fn test_setmouse_contract() {
    let mut driver = 9; // VGA
    let mut mode = 2; // VGAHI
    initgraph(&mut driver, &mut mode, "");

    // Test setting mouse position
    setmouse(100, 200);
    // BGI functions are void - errors reported via graphresult()

    // Verify the position was set (with some tolerance for system constraints)
    let x = mousex();
    let y = mousey();

    // Note: Some systems may not allow programmatic mouse positioning
    // so we just verify the function doesn't crash
    assert!(
        x >= 0 && y >= 0,
        "Mouse coordinates should remain valid after setmouse"
    );

    // Test with negative coordinates
    setmouse(-10, -10);
    // BGI functions are void - errors reported via graphresult()

    closegraph();
}

#[test]
fn test_clearmouseclick_contract() {
    let mut driver = 9; // VGA
    let mut mode = 2; // VGAHI
    initgraph(&mut driver, &mut mode, "");

    // Clear mouse click states
    clearmouseclick(1); // Clear left button
    clearmouseclick(2); // Clear right button
    clearmouseclick(3); // Clear middle button

    // After clearing, querying state should not panic.
    // Note: we cannot assert the value, since the user may click during the test.
    let _ = ismouseclick(1);
    let _ = ismouseclick(2);

    closegraph();
}

#[test]
fn test_mouse_operations_without_graphics() {
    // Test mouse operations without initializing graphics
    let x = mousex();
    let y = mousey();

    // Should return some default values or 0, not crash
    assert!(
        x >= 0,
        "mousex should return non-negative value even without graphics"
    );
    assert!(
        y >= 0,
        "mousey should return non-negative value even without graphics"
    );

    // Should not panic without graphics initialized.
    let _ = mouseclick(1);
    let _ = ismouseclick(1);
}

#[test]
#[ignore = "Reads the live mouse across calls; only consistent with a headless (non-live) backend. mousex()/mousey()/getmouse() each poll the backend independently, so under a real window they can observe the cursor at different instants."]
fn test_mouse_coordinate_consistency() {
    let mut driver = 9; // VGA
    let mut mode = 2; // VGAHI
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

    assert!(
        x_diff <= 5,
        "Mouse X coordinates should be consistent between calls"
    );
    assert!(
        y_diff <= 5,
        "Mouse Y coordinates should be consistent between calls"
    );

    closegraph();
}

#[test]
fn test_mouse_button_range() {
    let mut driver = 9; // VGA
    let mut mode = 2; // VGAHI
    initgraph(&mut driver, &mut mode, "");

    // Test all standard mouse buttons: querying should not panic.
    for button in 1..=3 {
        let _ = mouseclick(button);
        let _ = ismouseclick(button);
    }

    // Test invalid button numbers
    for button in [0, 4, 5, 99] {
        let click_state = mouseclick(button);
        let is_clicked = ismouseclick(button);

        assert!(
            !click_state,
            "Invalid button {} should return false",
            button
        );
        assert!(
            !is_clicked,
            "Invalid button {} ismouseclick should return false",
            button
        );
    }

    closegraph();
}
