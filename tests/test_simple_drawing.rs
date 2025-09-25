use bgi::{
    initgraph, closegraph, graphresult, GraphResult,
    line, circle, rectangle,
    setcolor, Color
};

#[test]
fn test_simple_drawing_integration() {
    // Test basic drawing workflow from quickstart
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI

    // Initialize graphics
    initgraph(&mut driver, &mut mode, "");

    // Check that graphics initialized successfully
    let result = graphresult();
    assert_eq!(result, GraphResult::Ok, "Graphics should initialize successfully");

    // Set drawing color
    setcolor(Color::WHITE);

    // Draw basic shapes
    line(10, 10, 100, 10);      // Horizontal line
    line(10, 10, 10, 100);      // Vertical line
    line(10, 10, 100, 100);     // Diagonal line

    circle(200, 50, 30);        // Circle
    rectangle(300, 20, 400, 80); // Rectangle

    // Test that we can draw multiple shapes without issues
    for i in 0..5 {
        circle(50 + i * 20, 150, 10);
    }

    // Test drawing at boundaries
    line(0, 0, 100, 0);         // Top edge
    line(0, 0, 0, 100);         // Left edge

    // Clean up
    closegraph();
}

#[test]
fn test_simple_drawing_coordinates() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    setcolor(Color::WHITE);

    // Test various coordinate combinations
    line(0, 0, 50, 50);         // From origin
    line(100, 100, 150, 100);   // Horizontal at offset
    line(100, 100, 100, 150);   // Vertical at offset
    line(200, 50, 150, 100);    // Diagonal reverse

    // Test circles at different positions
    circle(50, 200, 25);        // Bottom left area
    circle(200, 200, 35);       // Bottom right area
    circle(300, 100, 15);       // Top right area

    // Test rectangles
    rectangle(10, 250, 80, 300);    // Small rectangle
    rectangle(100, 250, 200, 350);  // Larger rectangle

    closegraph();
}

#[test]
fn test_drawing_edge_cases() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    setcolor(Color::WHITE);

    // Test zero-length line (same start and end point)
    line(50, 50, 50, 50);

    // Test zero-radius circle (point)
    circle(100, 100, 0);

    // Test zero-size rectangle (line)
    rectangle(150, 150, 150, 150);
    rectangle(200, 200, 200, 210); // Height but no width
    rectangle(250, 200, 260, 200); // Width but no height

    // Test very small shapes
    circle(300, 300, 1);
    rectangle(350, 350, 351, 351);

    closegraph();
}

#[test]
fn test_drawing_without_initialization_fails_gracefully() {
    // Attempt to draw without initializing graphics
    // This should not crash but may produce warnings or errors

    setcolor(Color::WHITE);
    line(10, 10, 100, 100);
    circle(50, 50, 25);
    rectangle(100, 100, 200, 200);

    // The system should handle this gracefully without crashing
    // Exact behavior depends on implementation
}

#[test]
fn test_multiple_graphics_sessions() {
    // Test initializing and closing graphics multiple times
    for _session in 0..3 {
        let mut driver = 9; // VGA
        let mut mode = 2;   // VGAHI

        initgraph(&mut driver, &mut mode, "");

        let result = graphresult();
        assert_eq!(result, GraphResult::Ok, "Each graphics session should initialize successfully");

        setcolor(Color::WHITE);
        line(10, 10, 100, 100);
        circle(150, 50, 20);

        closegraph();
    }
}

#[test]
fn test_drawing_order_independence() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    setcolor(Color::WHITE);

    // Draw shapes in different orders to test independence

    // First pattern: lines, then circles, then rectangles
    line(10, 10, 50, 10);
    line(10, 20, 50, 20);
    circle(100, 30, 15);
    circle(150, 30, 15);
    rectangle(200, 10, 250, 50);
    rectangle(260, 10, 310, 50);

    // Second pattern: mixed order
    rectangle(10, 100, 50, 140);
    line(100, 110, 150, 110);
    circle(200, 120, 20);
    line(100, 130, 150, 130);
    rectangle(260, 100, 310, 140);
    circle(350, 120, 20);

    closegraph();
}
