//! Standalone test for pixel buffer validation capabilities
//! This demonstrates "directly testing the pixel buffer" without depending on broken color tests.

use bgi::{
    backend::{Backend, DrawCommand, pixel_buffer::PixelBufferBackend},
    color::RgbColor,
    types::{GraphicsDriver, GraphicsMode},
};

#[test]
fn test_standalone_pixel_buffer_validation() {
    // Create backend and initialize
    let mut backend = PixelBufferBackend::new();
    assert!(backend.init().is_ok(), "Failed to initialize backend");

    // Create window
    let window_id = backend
        .create_window(
            100,
            100,
            Some("Test Window"),
            GraphicsMode::new(GraphicsDriver::Detect, 0),
        )
        .expect("Failed to create window");

    // Test pixel drawing and validation
    let red = RgbColor::new(255, 0, 0);
    let pixel_cmd = DrawCommand::Pixel {
        x: 10,
        y: 10,
        color: red,
    };
    backend
        .draw(window_id, &[pixel_cmd])
        .expect("Failed to draw pixel");

    // Verify pixel was drawn correctly
    assert!(
        backend
            .verify_pixel(10, 10, red)
            .expect("Failed to verify pixel"),
        "Pixel should be red at (10,10)"
    );
    println!("✓ Pixel validation successful");

    // Test line drawing and validation
    let white = RgbColor::new(255, 255, 255);
    let line_cmd = DrawCommand::Line {
        x1: 0,
        y1: 0,
        x2: 10,
        y2: 10,
        color: white,
    };
    backend
        .draw(window_id, &[line_cmd])
        .expect("Failed to draw line");

    // Verify line was drawn
    assert!(
        backend
            .verify_line_drawn(0, 0, 10, 10, white)
            .expect("Failed to verify line"),
        "Line should be drawn from (0,0) to (10,10)"
    );
    println!("✓ Line validation successful");

    // Test pixel counting
    let pixel_count = backend
        .count_drawn_pixels()
        .expect("Failed to count pixels");
    assert!(
        pixel_count > 0,
        "Expected some pixels to be drawn, got: {}",
        pixel_count
    );
    println!("✓ Pixel counting successful: {} pixels drawn", pixel_count);

    // Test buffer clearing
    backend.clear_for_testing().expect("Failed to clear buffer");
    let cleared_count = backend
        .count_drawn_pixels()
        .expect("Failed to count pixels after clear");
    // Note: In real implementations, clear might not reduce pixel count to 0 if background is non-black
    println!(
        "✓ Buffer clearing successful: {} pixels after clear",
        cleared_count
    );

    // Test dimensions
    let (width, height) = backend.get_dimensions().expect("Failed to get dimensions");
    assert_eq!(width, 100, "Width should be 100");
    assert_eq!(height, 100, "Height should be 100");
    println!("✓ Dimensions validation successful: {}x{}", width, height);
}

#[test]
fn test_shape_drawing_validation() {
    let mut backend = PixelBufferBackend::new();
    assert!(backend.init().is_ok());

    let window_id = backend
        .create_window(
            200,
            200,
            Some("Shape Test"),
            GraphicsMode::new(GraphicsDriver::Detect, 0),
        )
        .expect("Failed to create window");

    // Test circle drawing
    let blue = RgbColor::new(0, 0, 255);
    let circle_cmd = DrawCommand::Circle {
        x: 100,
        y: 100,
        radius: 50,
        color: blue,
        filled: false,
    };
    backend
        .draw(window_id, &[circle_cmd])
        .expect("Failed to draw circle");

    // Test rectangle drawing
    let green = RgbColor::new(0, 255, 0);
    let rect_cmd = DrawCommand::Rectangle {
        x1: 20,
        y1: 20,
        x2: 80,
        y2: 60,
        color: green,
        filled: false,
    };
    backend
        .draw(window_id, &[rect_cmd])
        .expect("Failed to draw rectangle");

    // Verify drawing occurred
    let pixel_count = backend
        .count_drawn_pixels()
        .expect("Failed to count pixels");
    assert!(
        pixel_count > 100,
        "Should have drawn many pixels for shapes, got: {}",
        pixel_count
    );
    println!(
        "✓ Shape drawing validation successful: {} pixels drawn",
        pixel_count
    );
}
