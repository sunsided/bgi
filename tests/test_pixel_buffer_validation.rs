//! Test pixel buffer validation capabilities
//! This demonstrates "directly testing the pixel buffer" as requested.

use bgi::{
    backend::{Backend, DrawCommand, pixel_buffer::PixelBufferBackend},
    color::RgbColor,
    types::{GraphicsDriver, GraphicsMode},
};

/// Test that we can validate actual drawing operations in the pixel buffer.
#[test]
fn test_pixel_buffer_drawing_validation() {
    // Create a pixel buffer backend for testing
    let mut backend = PixelBufferBackend::new();
    assert!(backend.init().is_ok(), "Failed to initialize backend");

    // Create a test window
    let window_id = backend
        .create_window(
            100,
            100,
            Some("Test Window"),
            GraphicsMode::new(GraphicsDriver::Detect, 0),
        )
        .expect("Failed to create window");

    // Test 1: Draw a pixel and verify it exists
    let red = RgbColor::new(255, 0, 0);
    let pixel_cmd = DrawCommand::Pixel {
        x: 10,
        y: 10,
        color: red,
    };
    backend
        .draw(window_id, &[pixel_cmd])
        .expect("Failed to draw pixel");

    // Verify the pixel was drawn correctly
    assert!(
        backend
            .verify_pixel(10, 10, red)
            .expect("Failed to verify pixel")
    );
    println!("✓ Pixel validation successful");

    // Test 2: Draw a line and verify it
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

    // Verify line was drawn (check endpoints)
    assert!(
        backend
            .verify_line_drawn(0, 0, 10, 10, white)
            .expect("Failed to verify line")
    );
    println!("✓ Line validation successful");

    // Test 3: Count drawn pixels
    let pixel_count = backend
        .count_drawn_pixels()
        .expect("Failed to count pixels");
    assert!(
        pixel_count > 0,
        "Expected some pixels to be drawn, got: {}",
        pixel_count
    );
    println!("✓ Pixel counting successful: {} pixels drawn", pixel_count);

    // Test 4: Clear and verify buffer is empty
    backend.clear_for_testing().expect("Failed to clear buffer");
    let pixel_count = backend
        .count_drawn_pixels()
        .expect("Failed to count pixels after clear");
    assert_eq!(
        pixel_count, 0,
        "Expected 0 pixels after clear, got: {}",
        pixel_count
    );
    println!("✓ Buffer clearing successful");

    println!("All pixel buffer validation tests passed!");
}

/// Test drawing shapes with the pixel buffer backend.
#[test]
fn test_shape_drawing_validation() {
    let mut backend = PixelBufferBackend::new();
    assert!(backend.init().is_ok(), "Failed to initialize backend");

    let window_id = backend
        .create_window(
            200,
            200,
            Some("Drawing Test"),
            GraphicsMode::new(GraphicsDriver::Detect, 0),
        )
        .expect("Failed to create window");

    // Test circle drawing
    let blue = RgbColor::new(0, 0, 255);
    let circle_cmd = DrawCommand::Circle {
        x: 50,
        y: 50,
        radius: 20,
        color: blue,
        filled: false,
    };
    backend
        .draw(window_id, &[circle_cmd])
        .expect("Failed to draw circle");

    let pixel_count = backend
        .count_drawn_pixels()
        .expect("Failed to count pixels");
    assert!(
        pixel_count > 50,
        "Circle should draw many pixels, got: {}",
        pixel_count
    );
    println!("✓ Circle drawing validation: {} pixels", pixel_count);

    // Test rectangle drawing
    let green = RgbColor::new(0, 255, 0);
    let rect_cmd = DrawCommand::Rectangle {
        x1: 10,
        y1: 10,
        x2: 90,
        y2: 90,
        color: green,
        filled: false,
    };
    backend
        .draw(window_id, &[rect_cmd])
        .expect("Failed to draw rectangle");

    let pixel_count = backend
        .count_drawn_pixels()
        .expect("Failed to count pixels");
    assert!(
        pixel_count > 100,
        "Rectangle should add more pixels, got: {}",
        pixel_count
    );
    println!(
        "✓ Rectangle drawing validation: {} pixels total",
        pixel_count
    );

    // Test that we can get specific pixel colors
    // The rectangle should have green pixels at the corners
    assert!(
        backend
            .verify_pixel(10, 10, green)
            .expect("Failed to verify rectangle corner")
    );
    println!("✓ Rectangle corner validation successful");

    // Test dimensions
    let (width, height) = backend.get_dimensions().expect("Failed to get dimensions");
    assert_eq!(width, 200);
    assert_eq!(height, 200);
    println!("✓ Window dimensions correct: {}x{}", width, height);

    println!("Shape drawing validation successful!");
}
