use bgi::{
    Color, GraphResult, bar, closegraph, fillellipse, fillpoly, graphresult, initgraph, pieslice,
    sector, setcolor, setfillstyle,
};

#[test]
fn test_fillellipse_basic() {
    // Contract: fillellipse should draw filled ellipse at specified coordinates
    let mut gd = 0i32;
    let mut gm = 4i32; // VGA mode

    initgraph(&mut gd, &mut gm, "");
    assert_eq!(graphresult(), GraphResult::Ok);

    // Test basic filled ellipses
    setcolor(Color::RED);
    fillellipse(100, 100, 50, 30); // Wide ellipse
    fillellipse(200, 200, 25, 25); // Circle (equal radii)
    fillellipse(300, 150, 40, 60); // Tall ellipse

    // Test edge cases
    fillellipse(0, 0, 10, 10); // At origin
    fillellipse(500, 400, 1, 1); // Minimal size
    fillellipse(250, 250, 0, 0); // Zero size (should handle gracefully)

    closegraph();
}

#[test]
fn test_sector_drawing() {
    // Contract: sector should draw filled elliptical sector (pie wedge)
    let mut gd = 0i32;
    let mut gm = 4i32; // VGA mode

    initgraph(&mut gd, &mut gm, "");
    assert_eq!(graphresult(), GraphResult::Ok);

    setcolor(Color::BLUE);

    // Test various sector angles
    sector(150, 150, 0, 90, 40, 40); // Quarter circle (0-90°)
    sector(250, 150, 90, 180, 35, 35); // Second quarter (90-180°)
    sector(350, 150, 180, 270, 30, 30); // Third quarter (180-270°)
    sector(450, 150, 270, 360, 25, 25); // Fourth quarter (270-360°)

    // Test full circle
    sector(300, 250, 0, 360, 45, 45); // Full circle

    // Test partial arcs
    sector(200, 300, 45, 135, 50, 30); // 90° elliptical sector
    sector(400, 300, 30, 60, 60, 20); // 30° elliptical sector

    // Test edge cases
    sector(100, 400, 0, 0, 20, 20); // Zero angle
    sector(500, 400, 0, 1, 15, 15); // Minimal angle

    closegraph();
}

#[test]
fn test_pieslice_circular() {
    // Contract: pieslice should draw filled circular sector
    let mut gd = 0i32;
    let mut gm = 4i32; // VGA mode

    initgraph(&mut gd, &mut gm, "");
    assert_eq!(graphresult(), GraphResult::Ok);

    setcolor(Color::GREEN);

    // Test pie slices at different angles
    pieslice(150, 150, 0, 60, 40); // 60° slice
    pieslice(250, 150, 60, 120, 40); // Another 60° slice
    pieslice(350, 150, 120, 240, 40); // 120° slice
    pieslice(450, 150, 240, 360, 40); // 120° slice to complete circle

    // Test various slice sizes
    pieslice(200, 250, 0, 90, 30); // Quarter pie
    pieslice(300, 250, 0, 180, 35); // Half pie
    pieslice(400, 250, 0, 270, 25); // Three-quarter pie

    // Test edge cases
    pieslice(150, 350, 0, 360, 50); // Full circle
    pieslice(250, 350, 45, 45, 20); // Zero angle
    pieslice(350, 350, 0, 5, 30); // Very small slice

    closegraph();
}

#[test]
fn test_bar_rectangles() {
    // Contract: bar should draw filled rectangle
    let mut gd = 0i32;
    let mut gm = 4i32; // VGA mode

    initgraph(&mut gd, &mut gm, "");
    assert_eq!(graphresult(), GraphResult::Ok);

    setcolor(Color::YELLOW);

    // Test various bar sizes and positions
    bar(50, 50, 150, 100); // Horizontal bar
    bar(200, 75, 250, 175); // Vertical bar
    bar(300, 50, 450, 125); // Wide bar
    bar(500, 60, 550, 140); // Narrow bar

    // Test edge cases
    bar(100, 200, 100, 200); // Zero-size bar
    bar(150, 225, 149, 224); // Inverted coordinates
    bar(0, 0, 50, 50); // At origin
    bar(550, 450, 600, 500); // Near viewport edge

    // Test with fill style
    setfillstyle(1, Color::MAGENTA); // Solid fill
    bar(250, 200, 350, 250);

    setfillstyle(0, Color::CYAN); // Empty fill
    bar(400, 200, 500, 250);

    closegraph();
}

#[test]
fn test_fillpoly_polygons() {
    // Contract: fillpoly should draw filled polygon from point array
    let mut gd = 0i32;
    let mut gm = 4i32; // VGA mode

    initgraph(&mut gd, &mut gm, "");
    assert_eq!(graphresult(), GraphResult::Ok);

    setcolor(Color::CYAN);

    // Test triangle
    let triangle = [(100, 50), (50, 150), (150, 150)];
    fillpoly(&triangle);

    // Test quadrilateral
    let quad = [(200, 75), (275, 50), (300, 125), (225, 150)];
    fillpoly(&quad);

    // Test pentagon
    let pentagon = [(400, 50), (450, 75), (440, 125), (360, 125), (350, 75)];
    fillpoly(&pentagon);

    // Test more complex polygon (hexagon)
    let hexagon = [
        (150, 200),
        (200, 175),
        (250, 200),
        (250, 250),
        (200, 275),
        (150, 250),
    ];
    fillpoly(&hexagon);

    // Test edge cases
    let line = [(300, 300), (400, 300)]; // Two points (line)
    fillpoly(&line);

    let single_point = [(450, 350)]; // Single point
    fillpoly(&single_point);

    let empty: &[(i32, i32)] = &[]; // Empty array
    fillpoly(empty);

    closegraph();
}

#[test]
fn test_filled_shapes_with_colors() {
    // Contract: filled shapes should respect current color and fill settings
    let mut gd = 0i32;
    let mut gm = 4i32; // VGA mode

    initgraph(&mut gd, &mut gm, "");
    assert_eq!(graphresult(), GraphResult::Ok);

    // Test each shape with different colors
    setcolor(Color::RED);
    setfillstyle(1, Color::RED);
    fillellipse(100, 100, 25, 25);

    setcolor(Color::BLUE);
    setfillstyle(1, Color::BLUE);
    pieslice(200, 100, 0, 90, 30);

    setcolor(Color::GREEN);
    setfillstyle(1, Color::GREEN);
    bar(250, 75, 300, 125);

    setcolor(Color::YELLOW);
    setfillstyle(1, Color::YELLOW);
    sector(400, 100, 45, 135, 35, 25);

    setcolor(Color::MAGENTA);
    setfillstyle(1, Color::MAGENTA);
    let diamond = [(500, 75), (525, 100), (500, 125), (475, 100)];
    fillpoly(&diamond);

    closegraph();
}

#[test]
fn test_filled_shapes_boundary_cases() {
    // Contract: filled shapes should handle boundary coordinates
    let mut gd = 0i32;
    let mut gm = 4i32; // VGA mode

    initgraph(&mut gd, &mut gm, "");
    assert_eq!(graphresult(), GraphResult::Ok);

    // Test at viewport boundaries
    fillellipse(0, 0, 50, 50); // Top-left corner
    fillellipse(600, 0, 40, 40); // Top-right corner
    fillellipse(0, 450, 35, 35); // Bottom-left corner
    fillellipse(600, 450, 30, 30); // Bottom-right corner

    // Test with large coordinates
    pieslice(1000, 1000, 0, 90, 100);
    bar(2000, 2000, 2100, 2100);

    // Test with negative coordinates
    fillellipse(-50, -50, 75, 75);
    sector(-100, 200, 0, 180, 60, 40);

    // Test shapes extending beyond viewport
    fillellipse(300, 240, 200, 200); // Large ellipse
    bar(-50, -50, 100, 100); // Bar crossing origin

    closegraph();
}

#[test]
fn test_filled_shapes_without_graphics() {
    // Contract: filled shapes should not crash without graphics initialization

    // Should not crash without initgraph
    setcolor(Color::RED);
    fillellipse(100, 100, 50, 30);

    sector(200, 150, 0, 90, 40, 40);
    pieslice(300, 150, 45, 135, 35);
    bar(400, 125, 500, 175);

    let triangle = [(150, 200), (100, 250), (200, 250)];
    fillpoly(&triangle);

    // Functions should complete without error
}

#[test]
fn test_filled_shapes_parameter_validation() {
    // Contract: filled shapes should handle parameter edge cases
    let mut gd = 0i32;
    let mut gm = 4i32; // VGA mode

    initgraph(&mut gd, &mut gm, "");
    assert_eq!(graphresult(), GraphResult::Ok);

    // Test zero and negative radii
    fillellipse(100, 100, 0, 0); // Zero radii
    fillellipse(150, 100, -10, 20); // Negative X radius
    fillellipse(200, 100, 20, -10); // Negative Y radius
    fillellipse(250, 100, -15, -25); // Both negative

    // Test zero radius circles
    pieslice(300, 150, 0, 90, 0); // Zero radius
    pieslice(350, 150, 0, 90, -20); // Negative radius

    // Test invalid angles (should be handled gracefully)
    sector(100, 200, 400, 500, 30, 30); // Angles > 360
    pieslice(200, 200, -45, -90, 35); // Negative angles
    sector(300, 200, 180, 90, 25, 25); // End < start

    // Test degenerate bars
    bar(400, 200, 400, 200); // Zero area
    bar(450, 200, 400, 150); // Inverted coordinates

    // Test self-intersecting polygon
    let self_intersect = [(500, 180), (550, 220), (500, 220), (550, 180)];
    fillpoly(&self_intersect);

    closegraph();
}

#[test]
fn test_filled_shapes_performance() {
    // Contract: filled shapes should handle multiple operations efficiently
    let mut gd = 0i32;
    let mut gm = 4i32; // VGA mode

    initgraph(&mut gd, &mut gm, "");
    assert_eq!(graphresult(), GraphResult::Ok);

    // Draw many small filled shapes
    for i in 0..20 {
        let x = (i % 10) * 60 + 30;
        let y = (i / 10) * 60 + 30;

        match i % 5 {
            0 => fillellipse(x, y, 15, 15),
            1 => pieslice(x, y, 0, 90, 15),
            2 => bar(x - 10, y - 10, x + 10, y + 10),
            3 => sector(x, y, 45, 135, 15, 10),
            4 => {
                let tri = [(x, y - 15), (x - 15, y + 15), (x + 15, y + 15)];
                fillpoly(&tri);
            }
            _ => {}
        }
    }

    // Should complete without performance issues in TDD phase

    closegraph();
}
