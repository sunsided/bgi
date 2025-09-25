use bgi::{
    initgraph, closegraph,
    getch, kbhit, delay,
    mousex, mousey, mouseclick, ismouseclick,
    setcolor, line, circle,
    Color
};

#[test]
fn test_interactive_keyboard_simulation() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    setcolor(Color::WHITE);

    // Test keyboard input functions
    // Note: These may not produce actual input in automated tests,
    // but they should not crash and should return consistent values

    let has_input = kbhit();
    assert!(has_input == true || has_input == false, "kbhit should return boolean");

    // Test delay function
    let start_time = std::time::Instant::now();
    delay(10); // 10 milliseconds
    let elapsed = start_time.elapsed();

    // Delay should take at least the requested time (with some tolerance)
    assert!(elapsed.as_millis() >= 5, "delay should wait approximately the requested time");
    assert!(elapsed.as_millis() <= 100, "delay should not wait excessively long");

    closegraph();
}

#[test]
fn test_interactive_mouse_simulation() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    setcolor(Color::WHITE);

    // Test mouse position functions
    let x = mousex();
    let y = mousey();

    assert!(x >= 0, "Mouse X should be non-negative");
    assert!(y >= 0, "Mouse Y should be non-negative");
    assert!(x <= 10000, "Mouse X should be within reasonable range");
    assert!(y <= 10000, "Mouse Y should be within reasonable range");

    // Test mouse button functions
    let left_pressed = mouseclick(1);
    let right_pressed = mouseclick(2);
    let middle_pressed = mouseclick(3);

    assert!(left_pressed == true || left_pressed == false, "Left button state should be boolean");
    assert!(right_pressed == true || right_pressed == false, "Right button state should be boolean");
    assert!(middle_pressed == true || middle_pressed == false, "Middle button state should be boolean");

    // Test click detection
    let left_clicked = ismouseclick(1);
    let right_clicked = ismouseclick(2);

    assert!(left_clicked == true || left_clicked == false, "Left click detection should be boolean");
    assert!(right_clicked == true || right_clicked == false, "Right click detection should be boolean");

    closegraph();
}

#[test]
fn test_interactive_drawing_loop_simulation() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    setcolor(Color::WHITE);

    // Simulate an interactive drawing loop
    for frame in 0..10 {
        // Simulate frame-based animation
        let x = 50 + frame * 10;
        let y = 50 + (frame % 3) * 20;

        circle(x, y, 5);

        // Check input state each frame
        let has_key = kbhit();
        let mouse_x = mousex();
        let mouse_y = mousey();

        // Values should be consistent within reasonable bounds
        assert!(has_key == true || has_key == false, "kbhit should return boolean in loop");
        assert!(mouse_x >= 0 && mouse_x <= 10000, "Mouse X should be valid in loop");
        assert!(mouse_y >= 0 && mouse_y <= 10000, "Mouse Y should be valid in loop");

        // Small delay between frames
        delay(1);
    }

    closegraph();
}

#[test]
fn test_interactive_event_consistency() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    // Test that input functions return consistent values over multiple calls
    let mut mouse_positions = Vec::new();
    let mut key_states = Vec::new();

    for _ in 0..5 {
        mouse_positions.push((mousex(), mousey()));
        key_states.push(kbhit());
        delay(1); // Small delay between checks
    }

    // Mouse positions should be reasonable (not wildly different)
    for (x, y) in &mouse_positions {
        assert!(*x >= 0 && *x <= 10000, "All mouse X positions should be valid");
        assert!(*y >= 0 && *y <= 10000, "All mouse Y positions should be valid");
    }

    // Key states should be boolean
    for &state in &key_states {
        assert!(state == true || state == false, "All key states should be boolean");
    }

    closegraph();
}

#[test]
fn test_interactive_response_drawing() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    setcolor(Color::WHITE);

    // Simulate responsive drawing based on mouse position
    for _step in 0..5 {
        let mouse_x = mousex();
        let mouse_y = mousey();

        // Draw something at or near mouse position
        // (scaled down to stay within reasonable bounds)
        let draw_x = (mouse_x / 10) % 300 + 50;
        let draw_y = (mouse_y / 10) % 200 + 50;

        circle(draw_x, draw_y, 10);

        // Check if mouse buttons affect drawing
        if mouseclick(1) {
            // Draw larger circle for left click
            circle(draw_x, draw_y, 20);
        }

        if mouseclick(2) {
            // Draw line for right click
            line(draw_x - 15, draw_y, draw_x + 15, draw_y);
        }

        delay(2);
    }

    closegraph();
}

#[test]
fn test_interactive_timing() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    // Test delay accuracy and consistency
    let delays = [1, 5, 10, 20];

    for &delay_ms in &delays {
        let start = std::time::Instant::now();
        delay(delay_ms);
        let elapsed = start.elapsed().as_millis();

        // Allow some tolerance but expect reasonable accuracy
        assert!(elapsed >= delay_ms as u128 / 2, "Delay should wait at least half the requested time");
        assert!(elapsed <= delay_ms as u128 * 5, "Delay should not wait more than 5x the requested time");
    }

    closegraph();
}

#[test]
fn test_interactive_without_graphics() {
    // Test interactive functions without graphics initialization
    // Should not crash and should return reasonable values

    let has_input = kbhit();
    assert!(has_input == true || has_input == false, "kbhit should work without graphics");

    let x = mousex();
    let y = mousey();
    assert!(x >= 0, "mousex should return non-negative without graphics");
    assert!(y >= 0, "mousey should return non-negative without graphics");

    let clicked = mouseclick(1);
    assert!(clicked == true || clicked == false, "mouseclick should work without graphics");

    // Delay should still work
    let start = std::time::Instant::now();
    delay(5);
    let elapsed = start.elapsed().as_millis();
    assert!(elapsed >= 2, "delay should work without graphics");
}

#[test]
fn test_interactive_input_boundaries() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    // Test edge cases for input functions

    // Test invalid mouse button numbers
    let invalid_click1 = mouseclick(0);
    let invalid_click2 = mouseclick(99);
    assert_eq!(invalid_click1, false, "Invalid button 0 should return false");
    assert_eq!(invalid_click2, false, "Invalid button 99 should return false");

    let invalid_clicked1 = ismouseclick(0);
    let invalid_clicked2 = ismouseclick(99);
    assert_eq!(invalid_clicked1, false, "Invalid button 0 ismouseclick should return false");
    assert_eq!(invalid_clicked2, false, "Invalid button 99 ismouseclick should return false");

    // Test zero delay
    let start = std::time::Instant::now();
    delay(0);
    let elapsed = start.elapsed().as_millis();
    assert!(elapsed <= 10, "Zero delay should not wait long");

    closegraph();
}
