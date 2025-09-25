use bgi::{
    initgraph, closegraph, graphresult, GraphResult,
    setcolor, getcolor, setfillstyle, setfillpattern,
    floodfill, bar, fillellipse, line, circle,
    Color
};

#[test]
fn test_color_fill_integration() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    let result = graphresult();
    assert_eq!(result, GraphResult::Ok, "Graphics should initialize successfully");

    // Test color setting and getting
    setcolor(Color::RED);
    let current_color = getcolor();
    assert_eq!(current_color, Color::RED, "Color should be set and retrieved correctly");

    // Test different colors
    let test_colors = [
        Color::BLACK, Color::BLUE, Color::GREEN, Color::CYAN,
        Color::RED, Color::MAGENTA, Color::BROWN, Color::LIGHTGRAY,
        Color::DARKGRAY, Color::LIGHTBLUE, Color::LIGHTGREEN, Color::LIGHTCYAN,
        Color::LIGHTRED, Color::LIGHTMAGENTA, Color::YELLOW, Color::WHITE
    ];

    for (i, &color) in test_colors.iter().enumerate() {
        setcolor(color);
        let retrieved = getcolor();
        assert_eq!(retrieved, color, "Color {} should match", i);
    }

    closegraph();
}

#[test]
fn test_fill_styles_integration() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    setcolor(Color::WHITE);

    // Test different fill styles
    setfillstyle(1, Color::RED);     // Solid fill
    bar(10, 10, 60, 60);

    setfillstyle(2, Color::BLUE);    // Line fill
    bar(70, 10, 120, 60);

    setfillstyle(3, Color::GREEN);   // Light slash fill
    bar(130, 10, 180, 60);

    setfillstyle(4, Color::YELLOW);  // Slash fill
    bar(190, 10, 240, 60);

    setfillstyle(5, Color::CYAN);    // Back slash fill
    bar(250, 10, 300, 60);

    closegraph();
}

#[test]
fn test_pattern_fill_integration() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    setcolor(Color::WHITE);

    // Test custom fill patterns
    let pattern1 = [0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55]; // Checkerboard
    setfillpattern(&pattern1, Color::RED);
    bar(10, 10, 110, 110);

    let pattern2 = [0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00]; // Horizontal lines
    setfillpattern(&pattern2, Color::BLUE);
    bar(120, 10, 220, 110);

    let pattern3 = [0x88, 0x88, 0x88, 0xFF, 0x88, 0x88, 0x88, 0xFF]; // Grid pattern
    setfillpattern(&pattern3, Color::GREEN);
    bar(230, 10, 330, 110);

    closegraph();
}

#[test]
fn test_filled_shapes_integration() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    setcolor(Color::WHITE);

    // Test filled ellipses with different fill styles
    setfillstyle(1, Color::RED);
    fillellipse(100, 100, 50, 30);

    setfillstyle(2, Color::BLUE);
    fillellipse(250, 100, 40, 40);

    setfillstyle(3, Color::GREEN);
    fillellipse(400, 100, 60, 20);

    // Test bars (filled rectangles) with different colors
    setfillstyle(1, Color::YELLOW);
    bar(50, 200, 150, 250);

    setfillstyle(1, Color::MAGENTA);
    bar(200, 200, 300, 250);

    setfillstyle(1, Color::CYAN);
    bar(350, 200, 450, 250);

    closegraph();
}

#[test]
fn test_floodfill_integration() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    setcolor(Color::WHITE);

    // Draw an enclosed area to fill
    // Create a simple rectangle outline
    line(50, 50, 150, 50);   // Top
    line(150, 50, 150, 150); // Right
    line(150, 150, 50, 150); // Bottom
    line(50, 150, 50, 50);   // Left

    // Set fill style and flood fill the enclosed area
    setfillstyle(1, Color::RED);
    floodfill(100, 100, Color::WHITE); // Fill inside the rectangle

    // Create a circle to fill
    setcolor(Color::BLUE);
    circle(300, 100, 40);

    setfillstyle(2, Color::GREEN);
    floodfill(300, 100, Color::BLUE); // Fill inside the circle

    closegraph();
}

#[test]
fn test_color_fill_combinations() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    // Test various combinations of colors and fill styles
    let colors = [Color::RED, Color::GREEN, Color::BLUE, Color::YELLOW];
    let fill_styles = [1, 2, 3, 4]; // Solid, Line, Light slash, Slash

    for (i, (&color, &style)) in colors.iter().zip(fill_styles.iter()).enumerate() {
        setfillstyle(style, color);
        let x = (i as i32) * 80 + 20;
        bar(x, 50, x + 60, 110);

        // Also test filled ellipses
        fillellipse(x + 30, 200, 25, 15);
    }

    closegraph();
}

#[test]
fn test_fill_without_initialization_fails_gracefully() {
    // Test fill operations without graphics initialization
    // Should handle gracefully without crashing

    setcolor(Color::RED);
    setfillstyle(1, Color::BLUE);

    bar(10, 10, 50, 50);
    fillellipse(100, 100, 30, 20);
    floodfill(75, 75, Color::WHITE);

    let color = getcolor();
    // Should return some default or error value, not crash
    assert!(color == Color::BLACK || color == Color::WHITE || color == Color::RED,
            "getcolor should return a valid color even without graphics");
}

#[test]
fn test_fill_operations_sequence() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    // Test a sequence of fill operations to ensure they work together

    // First layer: background bars
    setfillstyle(1, Color::DARKGRAY);
    bar(0, 0, 200, 100);
    bar(200, 100, 400, 200);

    // Second layer: colored shapes
    setfillstyle(1, Color::RED);
    fillellipse(100, 50, 40, 25);

    setfillstyle(1, Color::BLUE);
    fillellipse(300, 150, 50, 30);

    // Third layer: overlapping bars
    setfillstyle(2, Color::YELLOW); // Line fill
    bar(75, 75, 125, 125);
    bar(275, 125, 325, 175);

    closegraph();
}
