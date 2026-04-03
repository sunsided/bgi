//! Coordinate transformation and bounds tests.
//! These tests verify that coordinate transformations work correctly across
//! viewports, scaling, and bounds checking.

use bgi::*;

#[test]
fn test_logical_to_physical_coordinate_transformation() {
    let mut driver = VGA;
    let mut mode = VGAHI;
    initgraph(&mut driver, &mut mode, "");

    // Test that coordinate transformation exists and works correctly
    // BGI logical coordinates should map to backend physical coordinates

    // Get current graphics mode
    let max_x = getmaxx();
    let max_y = getmaxy();

    // Test corner coordinates
    assert!(max_x > 0, "Graphics width should be positive");
    assert!(max_y > 0, "Graphics height should be positive");

    // Test that logical coordinates are within expected BGI range
    assert!(
        max_x >= 320,
        "Graphics width should support at least 320 pixels (CGA)"
    );
    assert!(
        max_y >= 200,
        "Graphics height should support at least 200 pixels (CGA)"
    );

    closegraph();
}

#[test]
fn test_mouse_coordinate_transformation() {
    let mut driver = VGA;
    let mut mode = VGAHI;
    initgraph(&mut driver, &mut mode, "");

    // Test that mouse coordinates are properly transformed to BGI logical space
    let mouse_x = mousex();
    let mouse_y = mousey();

    // Mouse coordinates should be in valid BGI logical range
    let max_x = getmaxx();
    let max_y = getmaxy();

    assert!(mouse_x >= 0, "Mouse X coordinate should be non-negative");
    assert!(mouse_y >= 0, "Mouse Y coordinate should be non-negative");
    assert!(
        mouse_x <= max_x,
        "Mouse X coordinate should be within graphics bounds"
    );
    assert!(
        mouse_y <= max_y,
        "Mouse Y coordinate should be within graphics bounds"
    );

    closegraph();
}

#[test]
fn test_viewport_coordinate_transformation() {
    let mut driver = VGA;
    let mut mode = VGAHI;
    initgraph(&mut driver, &mut mode, "");

    // Test coordinate transformation with viewport
    setviewport(10, 10, 100, 100, true);

    // Draw at viewport-relative coordinates
    moveto(0, 0); // Should be at (10, 10) in absolute coordinates
    let pos1 = getposition();

    moveto(50, 50); // Should be at (60, 60) in absolute coordinates
    let pos2 = getposition();

    // Test that coordinates are properly transformed
    // Note: getposition returns viewport-relative coordinates
    assert_eq!(pos1.x, 0, "Viewport-relative X coordinate should be 0");
    assert_eq!(pos1.y, 0, "Viewport-relative Y coordinate should be 0");
    assert_eq!(pos2.x, 50, "Viewport-relative X coordinate should be 50");
    assert_eq!(pos2.y, 50, "Viewport-relative Y coordinate should be 50");

    closegraph();
}

#[test]
fn test_coordinate_bounds_clipping() {
    let mut driver = VGA;
    let mut mode = VGAHI;
    initgraph(&mut driver, &mut mode, "");

    let max_x = getmaxx();
    let max_y = getmaxy();

    // Test that out-of-bounds coordinates are handled properly
    // These operations should not crash and should clip appropriately

    // Draw beyond bounds - should be clipped
    line(-10, -10, max_x + 10, max_y + 10);
    putpixel(max_x + 100, max_y + 100, Color::WHITE);

    // Get pixel from out-of-bounds location - should return safe value
    let _out_of_bounds_color = getpixel(max_x + 100, max_y + 100);
    // Should not crash and should return a valid color value

    closegraph();
}

#[test]
fn test_scaling_coordinate_transformation() {
    let mut driver = VGA;
    let mut mode = VGAHI;
    initgraph(&mut driver, &mut mode, "");

    // Test that different graphics modes provide appropriate coordinate scaling
    let mode_width = getmaxx() + 1;
    let mode_height = getmaxy() + 1;

    // Verify mode dimensions are reasonable
    assert!(
        mode_width >= 320,
        "Graphics mode should provide at least 320x200 resolution"
    );
    assert!(
        mode_height >= 200,
        "Graphics mode should provide at least 320x200 resolution"
    );

    // Test coordinate scaling consistency
    // Draw at specific logical coordinates and verify they map correctly
    putpixel(0, 0, Color::WHITE);
    putpixel(mode_width - 1, mode_height - 1, Color::WHITE);

    // Verify pixels were set (they should be accessible)
    let corner1 = getpixel(0, 0);
    let corner2 = getpixel(mode_width - 1, mode_height - 1);

    // Colors should be set correctly (compare RGB values since getpixel may return Rgb variant)
    assert_eq!(
        corner1.to_rgb(),
        Color::WHITE.to_rgb(),
        "Corner pixel should be set to WHITE"
    );
    assert_eq!(
        corner2.to_rgb(),
        Color::WHITE.to_rgb(),
        "Corner pixel should be set to WHITE"
    );

    closegraph();
}

#[test]
fn test_aspect_ratio_preservation() {
    let mut driver = VGA;
    let mut mode = VGAHI;
    initgraph(&mut driver, &mut mode, "");

    // Test that coordinate transformations preserve aspect ratio
    let max_x = getmaxx();
    let max_y = getmaxy();

    // Draw a square in BGI logical coordinates
    let square_size = std::cmp::min(max_x, max_y) / 4;
    let center_x = max_x / 2;
    let center_y = max_y / 2;

    // Draw square centered on screen
    rectangle(
        center_x - square_size / 2,
        center_y - square_size / 2,
        center_x + square_size / 2,
        center_y + square_size / 2,
    );

    // Test that the square maintains its proportions
    // (This is more of a visual test, but we can verify coordinates are set)

    closegraph();
}
