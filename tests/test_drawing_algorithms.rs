use bgi::*;

fn is_white_pixel(color: Color) -> bool {
    let rgb = color.to_rgb();
    rgb.r == 255 && rgb.g == 255 && rgb.b == 255
}

#[test]
fn test_all_drawing_functions_comprehensive() {
    let mut driver = 9; // VGA
    let mut mode = 2; // VGAHI
    initgraph(&mut driver, &mut mode, "");

    setcolor(Color::WHITE);

    // Test lines
    line(50, 50, 150, 50);
    line(50, 50, 50, 150);

    // Test circles
    circle(100, 100, 25);

    // Test rectangles
    rectangle(200, 50, 250, 100);

    // Test ellipses
    ellipse(300, 100, 0, 360, 20, 15);

    // Verify some pixels were drawn by checking a few key positions
    let mut pixels_found = 0;
    let y = 50;

    // Check horizontal line pixels
    for x in 50..=150 {
        let pixel = getpixel(x, y);
        if is_white_pixel(pixel) {
            pixels_found += 1;
        }
    }

    assert!(
        pixels_found > 0,
        "Expected to find drawn pixels, but found none"
    );

    closegraph();
}

#[test]
fn test_line_comprehensive() {
    let mut driver = 9; // VGA
    let mut mode = 2; // VGAHI
    initgraph(&mut driver, &mut mode, "");

    setcolor(Color::WHITE);

    // Test various line orientations
    line(25, 25, 75, 25); // Horizontal
    line(25, 35, 25, 85); // Vertical
    line(35, 25, 85, 75); // Diagonal
    line(35, 75, 85, 25); // Reverse diagonal

    // Count pixels to verify lines were drawn
    let mut pixels_found = 0;

    // Check a region that should contain drawn pixels
    for y in 25..90 {
        for x in 25..90 {
            let pixel = getpixel(x, y);
            if is_white_pixel(pixel) {
                pixels_found += 1;
            }
        }
    }

    assert!(pixels_found > 0, "Expected to find drawn pixels from lines");

    closegraph();
}

#[test]
fn test_circle_comprehensive() {
    let mut driver = 9; // VGA
    let mut mode = 2; // VGAHI
    initgraph(&mut driver, &mut mode, "");

    setcolor(Color::WHITE);

    // Draw circles of different sizes
    circle(100, 100, 20);
    circle(200, 100, 30);
    circle(300, 100, 40);

    // Count pixels to verify circles were drawn
    let mut pixels_found = 0;

    // Check regions around circle centers
    for y in 70..130 {
        for x in 70..330 {
            let pixel = getpixel(x, y);
            if is_white_pixel(pixel) {
                pixels_found += 1;
            }
        }
    }

    assert!(
        pixels_found > 0,
        "Expected to find drawn pixels from circles"
    );

    closegraph();
}

#[test]
fn test_advanced_pixel_detection() {
    let mut driver = 9; // VGA
    let mut mode = 2; // VGAHI
    initgraph(&mut driver, &mut mode, "");

    setcolor(Color::WHITE);

    // Draw a simple line to test pixel detection
    line(100, 100, 200, 100);

    let mut pixels_found = 0;

    // Count pixels in the line area
    for x in 100..=200 {
        let pixel = getpixel(x, 100);
        if is_white_pixel(pixel) {
            pixels_found += 1;
        }
    }

    // The line should have pixels, but exact count depends on the algorithm
    assert!(
        pixels_found > 50,
        "Expected substantial number of pixels in line, found {}",
        pixels_found
    );

    closegraph();
}

#[test]
fn test_multiple_primitives_together() {
    let mut driver = 9; // VGA
    let mut mode = 2; // VGAHI
    initgraph(&mut driver, &mut mode, "");

    setcolor(Color::WHITE);

    // Draw multiple primitives in different areas
    line(50, 50, 150, 50); // Top horizontal line
    circle(100, 125, 25); // Center circle
    rectangle(200, 50, 300, 100); // Right rectangle
    ellipse(250, 150, 0, 360, 30, 20); // Bottom ellipse

    // Count total pixels across the canvas
    let mut pixels_found = 0;

    for y in 25..175 {
        for x in 25..325 {
            let pixel = getpixel(x, y);
            // Count non-black pixels (assuming black background)
            if is_white_pixel(pixel) {
                pixels_found += 1;
            }
        }
    }

    // We should have found some non-black pixels from our drawing
    assert!(
        pixels_found > 0,
        "Expected to find drawn pixels, but found none"
    );

    closegraph();
}

#[test]
fn test_line_pattern_differences() {
    let mut driver = 9; // VGA
    let mut mode = 2; // VGAHI
    initgraph(&mut driver, &mut mode, "");

    setcolor(Color::WHITE);

    // Draw the same line with different patterns
    setlinestyle(SOLID_LINE, 0, NORM_WIDTH);
    line(10, 10, 100, 10);

    setlinestyle(DOTTED_LINE, 0, NORM_WIDTH);
    line(10, 20, 100, 20);

    setlinestyle(DASHED_LINE, 0, NORM_WIDTH);
    line(10, 30, 100, 30);

    // Count pixels for each line to verify patterns create different pixel counts
    let mut solid_pixels = 0;
    let mut dotted_pixels = 0;
    let mut dashed_pixels = 0;

    // Count pixels in each line - check for white pixels (non-black)
    for x in 10..=100 {
        if is_white_pixel(getpixel(x, 10)) {
            solid_pixels += 1;
        }
        if is_white_pixel(getpixel(x, 20)) {
            dotted_pixels += 1;
        }
        if is_white_pixel(getpixel(x, 30)) {
            dashed_pixels += 1;
        }
    }

    // Solid line should have the most pixels
    // Dotted and dashed should have fewer (due to gaps in the pattern)
    assert!(solid_pixels > 0, "Solid line should have pixels");
    assert!(dotted_pixels > 0, "Dotted line should have some pixels");
    assert!(dashed_pixels > 0, "Dashed line should have some pixels");

    // The exact relationships depend on the pattern implementation,
    // but they should be different
    assert_ne!(
        solid_pixels, dotted_pixels,
        "Solid and dotted should differ"
    );
    assert_ne!(
        solid_pixels, dashed_pixels,
        "Solid and dashed should differ"
    );

    closegraph();
}
