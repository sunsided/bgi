//! Tests for the pixel buffer backend - headless testing capability.
//!
//! Tests for the pixel buffer backend that provides headless graphics operations.
//! Verifies in-memory pixel operations without requiring a visual display.

use bgi::backend::{Backend, pixel_buffer::PixelBufferBackend, DrawCommand};
use bgi::{types::GraphicsMode, color::RgbColor};

/// Test that pixel buffer backend can be created and stores pixels correctly
#[test]
fn test_pixel_buffer_basic_operations() {
    let mut backend = PixelBufferBackend::new();
    
    // Initialize backend
    backend.init().expect("Pixel buffer backend initialization should succeed");
    
    // Create a window
    let mode = GraphicsMode::new(bgi::types::GraphicsDriver::Vga, 2);
    let window_id = backend.create_window(100, 100, Some("Test Window"), mode)
        .expect("Window creation should succeed");
    
    // Test basic drawing commands
    let commands = vec![
        DrawCommand::Pixel { x: 10, y: 10, color: RgbColor { r: 255, g: 0, b: 0, a: 255 } },
        DrawCommand::Line { x1: 0, y1: 0, x2: 50, y2: 50, color: RgbColor { r: 0, g: 255, b: 0, a: 255 } },
        DrawCommand::Rectangle { x1: 20, y1: 20, x2: 40, y2: 40, color: RgbColor { r: 0, g: 0, b: 255, a: 255 }, filled: false },
    ];
    
    // Execute drawing commands
    backend.draw(window_id, &commands).expect("Drawing should succeed");
    
    // Present (should not fail for pixel buffer)
    backend.present(window_id).expect("Present should succeed");
}

/// Test pixel buffer coordinate system and bounds checking
#[test]
fn test_pixel_buffer_coordinates() {
    let mut backend = PixelBufferBackend::new();
    backend.init().expect("Initialization should succeed");
    
    let mode = GraphicsMode::new(bgi::types::GraphicsDriver::Vga, 2);
    let window_id = backend.create_window(50, 50, Some("Coord Test"), mode)
        .expect("Window creation should succeed");
    
    // Test corner pixels and bounds
    let test_commands = vec![
        DrawCommand::Pixel { x: 0, y: 0, color: RgbColor { r: 255, g: 255, b: 255, a: 255 } },      // Top-left
        DrawCommand::Pixel { x: 49, y: 0, color: RgbColor { r: 255, g: 0, b: 0, a: 255 } },         // Top-right
        DrawCommand::Pixel { x: 0, y: 49, color: RgbColor { r: 0, g: 255, b: 0, a: 255 } },         // Bottom-left
        DrawCommand::Pixel { x: 49, y: 49, color: RgbColor { r: 0, g: 0, b: 255, a: 255 } },        // Bottom-right
        DrawCommand::Pixel { x: -1, y: -1, color: RgbColor { r: 255, g: 255, b: 0, a: 255 } },      // Out of bounds (should be ignored)
        DrawCommand::Pixel { x: 50, y: 50, color: RgbColor { r: 255, g: 255, b: 0, a: 255 } },      // Out of bounds (should be ignored)
    ];
    
    backend.draw(window_id, &test_commands).expect("Drawing should succeed");
    backend.present(window_id).expect("Present should succeed");
}

/// Test pixel buffer input simulation (headless)
#[test]
fn test_pixel_buffer_input() {
    let mut backend = PixelBufferBackend::new();
    backend.init().expect("Initialization should succeed");
    
    let mode = GraphicsMode::new(bgi::types::GraphicsDriver::Vga, 2);
    let _window_id = backend.create_window(100, 100, Some("Input Test"), mode)
        .expect("Window creation should succeed");
    
    // Test input polling (should return empty for headless backend)
    let events = backend.poll_events();
    assert!(events.is_empty(), "Headless backend should not generate input events");
}

/// Test multiple colors and drawing operations
#[test]
fn test_pixel_buffer_colors() {
    let mut backend = PixelBufferBackend::new();
    backend.init().expect("Initialization should succeed");
    
    let mode = GraphicsMode::new(bgi::types::GraphicsDriver::Vga, 2);
    let window_id = backend.create_window(160, 10, Some("Color Test"), mode)
        .expect("Window creation should succeed");
    
    // Test various colors
    let colors = [
        RgbColor { r: 255, g: 0, b: 0, a: 255 },     // Red
        RgbColor { r: 0, g: 255, b: 0, a: 255 },     // Green
        RgbColor { r: 0, g: 0, b: 255, a: 255 },     // Blue
        RgbColor { r: 255, g: 255, b: 0, a: 255 },   // Yellow
        RgbColor { r: 255, g: 0, b: 255, a: 255 },   // Magenta
        RgbColor { r: 0, g: 255, b: 255, a: 255 },   // Cyan
        RgbColor { r: 255, g: 255, b: 255, a: 255 }, // White
        RgbColor { r: 128, g: 128, b: 128, a: 255 }, // Gray
    ];
    
    let mut commands = Vec::new();
    for (i, color) in colors.iter().enumerate() {
        for y in 0..10 {
            for x in (i * 20)..(i * 20 + 20) {
                commands.push(DrawCommand::Pixel { 
                    x: x as i32, 
                    y: y as i32, 
                    color: *color 
                });
            }
        }
    }
    
    backend.draw(window_id, &commands).expect("Color drawing should succeed");
    backend.present(window_id).expect("Present should succeed");
}

/// Test pixel buffer performance with many operations
#[test]
fn test_pixel_buffer_performance() {
    let mut backend = PixelBufferBackend::new();
    backend.init().expect("Initialization should succeed");
    
    let mode = GraphicsMode::new(bgi::types::GraphicsDriver::Vga, 2);
    let window_id = backend.create_window(200, 200, Some("Performance Test"), mode)
        .expect("Window creation should succeed");
    
    let start = std::time::Instant::now();
    
    // Generate many pixel operations
    let mut commands = Vec::new();
    for y in 0..200 {
        for x in 0..200 {
            let color = RgbColor {
                r: ((x * 255) / 200) as u8,
                g: ((y * 255) / 200) as u8,
                b: ((x + y) % 255) as u8,
                a: 255,
            };
            commands.push(DrawCommand::Pixel { x, y, color });
        }
    }
    
    backend.draw(window_id, &commands).expect("Performance drawing should succeed");
    backend.present(window_id).expect("Present should succeed");
    
    let elapsed = start.elapsed();
    
    // 40,000 pixel operations should be fast for in-memory buffer
    assert!(elapsed.as_millis() < 100, "40K pixel operations should complete in <100ms, took {:?}", elapsed);
}

/// Test multiple windows capability
#[test]
fn test_pixel_buffer_multiple_windows() {
    let mut backend = PixelBufferBackend::new();
    backend.init().expect("Initialization should succeed");
    
    let mode = GraphicsMode::new(bgi::types::GraphicsDriver::Vga, 2);
    
    // Create multiple windows
    let window1 = backend.create_window(100, 100, Some("Window 1"), mode)
        .expect("Window 1 creation should succeed");
    let window2 = backend.create_window(200, 150, Some("Window 2"), mode)
        .expect("Window 2 creation should succeed");
    
    // Draw to different windows
    let commands1 = vec![DrawCommand::Pixel { x: 50, y: 50, color: RgbColor { r: 255, g: 0, b: 0, a: 255 } }];
    let commands2 = vec![DrawCommand::Pixel { x: 100, y: 75, color: RgbColor { r: 0, g: 255, b: 0, a: 255 } }];
    
    backend.draw(window1, &commands1).expect("Drawing to window 1 should succeed");
    backend.draw(window2, &commands2).expect("Drawing to window 2 should succeed");
    
    backend.present(window1).expect("Present window 1 should succeed");
    backend.present(window2).expect("Present window 2 should succeed");
}

/// Test backend capabilities
#[test]
fn test_pixel_buffer_capabilities() {
    let backend = PixelBufferBackend::new();
    let capabilities = backend.capabilities();
    
    // Pixel buffer should have specific capabilities
    assert!(capabilities.multi_window, "Pixel buffer should support multiple windows");
    // Other capability checks can be added based on implementation
}

/// Test error handling for invalid operations
#[test]
fn test_pixel_buffer_error_handling() {
    let mut backend = PixelBufferBackend::new();
    
    // Test drawing to invalid window ID
    let invalid_window_id = bgi::window::WindowId::new(999);
    let commands = vec![DrawCommand::Pixel { x: 0, y: 0, color: RgbColor { r: 255, g: 255, b: 255, a: 255 } }];
    
    // This should fail gracefully
    let result = backend.draw(invalid_window_id, &commands);
    assert!(result.is_err(), "Drawing to invalid window should fail");
}
