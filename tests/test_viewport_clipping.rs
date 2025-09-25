use bgi::{
    initgraph, closegraph,
    setviewport, clearviewport, getviewport, getviewsettings,
    moveto, moverel, lineto, linerel,
    setcolor, line, circle, rectangle,
    Color
};

#[test]
fn test_viewport_clipping_integration() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    setcolor(Color::WHITE);

    // Set a viewport in the middle of the screen
    let clip = true;
    setviewport(100, 50, 400, 250, clip);

    // Draw shapes that should be clipped to the viewport
    line(0, 0, 500, 200);  // Line extending beyond viewport
    circle(150, 100, 75);  // Circle partially outside viewport
    rectangle(-50, -50, 350, 300); // Rectangle extending beyond viewport

    // Clear viewport and draw again
    clearviewport();

    // Draw inside viewport bounds
    line(10, 10, 290, 10);   // Horizontal line within viewport
    line(10, 10, 10, 190);   // Vertical line within viewport
    circle(150, 100, 50);    // Circle within viewport

    closegraph();
}

#[test]
fn test_viewport_settings_retrieval() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    // Set specific viewport settings
    setviewport(50, 100, 350, 300, true);

    // Retrieve and verify settings
    let (left, top, right, bottom) = getviewport();

    assert_eq!(left, 50, "Left viewport boundary should match");
    assert_eq!(top, 100, "Top viewport boundary should match");
    assert_eq!(right, 350, "Right viewport boundary should match");
    assert_eq!(bottom, 300, "Bottom viewport boundary should match");

    closegraph();
}

#[test]
fn test_viewport_coordinate_system() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    setcolor(Color::WHITE);

    // Test drawing in full screen coordinates first
    line(0, 0, 100, 100);

    // Set viewport and test coordinate system
    setviewport(200, 100, 500, 300, true);

    // These coordinates are now relative to the viewport
    line(0, 0, 100, 100);    // Should appear at (200,100) to (300,200) in screen coords
    circle(50, 50, 25);      // Should appear at (250,150) in screen coords
    rectangle(10, 10, 90, 90); // Should appear at (210,110) to (290,190) in screen coords

    closegraph();
}

#[test]
fn test_viewport_clipping_boundaries() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    setcolor(Color::WHITE);

    // Test clipping enabled
    setviewport(100, 100, 300, 200, true);

    // Draw lines that cross viewport boundaries
    line(-50, 50, 250, 50);   // Horizontal line crossing left and right
    line(100, -50, 100, 150); // Vertical line crossing top and bottom
    line(-50, -50, 250, 150); // Diagonal crossing all boundaries

    // Test circles that extend beyond boundaries
    circle(0, 0, 50);      // Circle at viewport origin
    circle(200, 100, 75);  // Circle extending beyond right and bottom
    circle(-25, -25, 40);  // Circle extending beyond left and top

    closegraph();
}

#[test]
fn test_viewport_no_clipping() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    setcolor(Color::WHITE);

    // Test clipping disabled
    setviewport(150, 150, 350, 250, false);

    // Draw shapes that extend beyond viewport - should not be clipped
    line(-100, 50, 300, 50);  // Long horizontal line
    circle(100, 50, 100);     // Large circle
    rectangle(-50, -50, 250, 150); // Large rectangle

    closegraph();
}

#[test]
fn test_multiple_viewport_changes() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    setcolor(Color::WHITE);

    // Test changing viewport multiple times
    let viewports = [
        (50, 50, 200, 150),
        (250, 50, 400, 150),
        (50, 200, 200, 300),
        (250, 200, 400, 300),
    ];

    for (i, &(left, top, right, bottom)) in viewports.iter().enumerate() {
        setviewport(left, top, right, bottom, true);

        // Draw something in each viewport
        rectangle(5, 5, right - left - 10, bottom - top - 10);
        line(10, 10, right - left - 20, bottom - top - 20);

        // Verify viewport settings
        let (vp_left, vp_top, vp_right, vp_bottom) = getviewport();
        assert_eq!(vp_left, left, "Viewport {} left should match", i);
        assert_eq!(vp_top, top, "Viewport {} top should match", i);
        assert_eq!(vp_right, right, "Viewport {} right should match", i);
        assert_eq!(vp_bottom, bottom, "Viewport {} bottom should match", i);
    }

    closegraph();
}

#[test]
fn test_viewport_clear_operations() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    setcolor(Color::WHITE);

    // Set viewport and draw something
    setviewport(100, 100, 300, 200, true);
    line(10, 10, 90, 90);
    circle(50, 50, 30);

    // Clear viewport - should clear only the viewport area
    clearviewport();

    // Draw again after clearing
    rectangle(20, 20, 80, 80);
    line(0, 50, 100, 50);

    closegraph();
}

#[test]
fn test_moveto_lineto_with_viewport() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    setcolor(Color::WHITE);

    // Test moveto/lineto functions with viewport
    setviewport(150, 100, 350, 250, true);

    // Draw connected lines using moveto/lineto
    moveto(50, 50);
    lineto(100, 75);
    lineto(150, 50);
    lineto(100, 25);
    lineto(50, 50);  // Complete a diamond shape

    // Test moverel/linerel
    moveto(25, 100);
    linerel(25, 0);   // Right
    linerel(0, 25);   // Down
    linerel(-25, 0);  // Left
    linerel(0, -25);  // Up - complete a square

    closegraph();
}

#[test]
fn test_viewport_edge_cases() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    setcolor(Color::WHITE);

    // Test minimal viewport (1x1 pixel)
    setviewport(100, 100, 101, 101, true);
    line(0, 0, 10, 10);  // Should be clipped to single pixel

    // Test square viewport
    setviewport(200, 200, 300, 300, true);
    circle(50, 50, 40);  // Circle in square viewport

    // Test wide viewport (much wider than tall)
    setviewport(50, 350, 450, 375, true);
    line(0, 10, 400, 10);  // Horizontal line in wide viewport

    // Test tall viewport (much taller than wide)
    setviewport(500, 50, 525, 300, true);
    line(10, 0, 10, 250);  // Vertical line in tall viewport

    closegraph();
}

#[test]
fn test_viewport_without_graphics() {
    // Test viewport operations without graphics initialization
    // Should handle gracefully

    setviewport(100, 100, 300, 200, true);
    clearviewport();

    let (left, top, right, bottom) = getviewport();

    // Should return some default values
    assert!(left >= 0, "Viewport left should be non-negative without graphics");
    assert!(top >= 0, "Viewport top should be non-negative without graphics");
    assert!(right >= left, "Viewport right should not be less than left");
    assert!(bottom >= top, "Viewport bottom should not be less than top");

    moveto(50, 50);
    lineto(100, 100);
    moverel(10, 10);
    linerel(20, 20);

    // Should not crash
}

#[test]
fn test_viewport_invalid_coordinates() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    setcolor(Color::WHITE);

    // Test invalid viewport coordinates (left > right, top > bottom)
    // Implementation should handle this gracefully
    setviewport(300, 200, 100, 100, true);

    // Try to draw something - should handle gracefully
    line(10, 10, 50, 50);
    circle(25, 25, 15);

    // Test negative coordinates
    setviewport(-50, -50, 100, 100, true);
    line(25, 25, 75, 75);

    closegraph();
}

#[test]
fn test_viewport_coordinate_transformation() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    setcolor(Color::WHITE);

    // Test that viewport coordinates are correctly transformed

    // First, draw reference point in full screen
    setviewport(0, 0, 639, 479, false); // Full screen, no clipping
    line(200, 150, 210, 160); // Reference diagonal

    // Now set viewport and draw the same relative coordinates
    setviewport(200, 150, 400, 300, true);
    line(0, 0, 10, 10); // Should appear at same screen location as reference

    // Test that coordinates are relative to viewport origin
    line(10, 10, 50, 50); // Should appear at (210,160) to (250,200) in screen coords

    closegraph();
}
