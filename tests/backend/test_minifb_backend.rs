//! Visual backend (minifb) contract tests
//!
//! Tests specific to the minifb visual backend implementation.
//! These tests verify window management, scaling, and visual rendering.

#[cfg(feature = "visual-backend")]
use bgi::backend::{Backend, GraphicsMode, Color};

/// Test that minifb backend can be created and initialized
#[cfg(feature = "visual-backend")]
#[test]
fn test_minifb_backend_creation() {
    // This test will fail until MiniFBBackend is implemented

    // Test creation
    let backend_result = bgi::backend::minifb::MiniFBBackend::new();
    assert!(backend_result.is_ok(), "MiniFB backend creation should succeed");

    let mut backend = backend_result.unwrap();

    // Test initialization with standard VGA mode
    let vga_mode = GraphicsMode {
        width: 640,
        height: 480,
        colors: 16,
        mode_id: 1,
    };

    let init_result = backend.init_graphics(vga_mode);
    assert!(init_result.is_ok(), "VGA mode initialization should succeed");
    assert!(backend.is_initialized());
    assert_eq!(backend.get_mode_info(), Some(vga_mode));

    // Clean up
    backend.close_graphics();
    assert!(!backend.is_initialized());
}

/// Test window scaling and coordinate transformation
#[cfg(feature = "visual-backend")]
#[test]
fn test_window_scaling() {
    let mut backend = bgi::backend::minifb::MiniFBBackend::new()
        .expect("Backend creation should succeed");

    // Initialize with small BGI mode that should be scaled up
    let small_mode = GraphicsMode {
        width: 320,
        height: 240,
        colors: 16,
        mode_id: 2,
    };

    backend.init_graphics(small_mode).expect("Initialization should succeed");

    // Test that logical coordinates work correctly
    backend.set_pixel(0, 0, Color::WHITE);
    assert_eq!(backend.get_pixel(0, 0), Color::WHITE);

    backend.set_pixel(319, 239, Color::RED); // Bottom-right in logical space
    assert_eq!(backend.get_pixel(319, 239), Color::RED);

    // Test that out-of-bounds coordinates are handled gracefully
    backend.set_pixel(-1, -1, Color::BLUE);
    backend.set_pixel(320, 240, Color::GREEN);

    // These should not crash and should not affect valid pixels
    assert_eq!(backend.get_pixel(0, 0), Color::WHITE);
    assert_eq!(backend.get_pixel(319, 239), Color::RED);

    backend.close_graphics();
}

/// Test visual backend input event capture
#[cfg(feature = "visual-backend")]
#[test]
fn test_visual_input_capture() {
    let mut backend = bgi::backend::minifb::MiniFBBackend::new()
        .expect("Backend creation should succeed");

    let mode = GraphicsMode { width: 640, height: 480, colors: 16, mode_id: 1 };
    backend.init_graphics(mode).expect("Initialization should succeed");

    // Test input polling (should not block)
    assert!(!backend.has_keyboard_input());
    assert_eq!(backend.poll_keyboard(), None);
    assert_eq!(backend.poll_mouse(), None);

    // Test mouse position tracking
    let (x, y) = backend.get_mouse_position();
    assert!(x >= 0 && y >= 0); // Position should be valid

    // Test window close detection
    assert!(!backend.should_close()); // Window should be open initially

    backend.close_graphics();
}

/// Test multiple graphics modes support
#[cfg(feature = "visual-backend")]
#[test]
fn test_multiple_graphics_modes() {
    let mut backend = bgi::backend::minifb::MiniFBBackend::new()
        .expect("Backend creation should succeed");

    // Test different BGI-compatible modes
    let modes = vec![
        GraphicsMode { width: 320, height: 200, colors: 16, mode_id: 3 }, // CGA
        GraphicsMode { width: 640, height: 200, colors: 16, mode_id: 4 }, // EGA
        GraphicsMode { width: 640, height: 480, colors: 16, mode_id: 1 }, // VGA
        GraphicsMode { width: 800, height: 600, colors: 256, mode_id: 5 }, // SVGA
    ];

    for mode in modes {
        let result = backend.init_graphics(mode);
        assert!(result.is_ok(), "Mode {:?} should be supported", mode);
        assert_eq!(backend.get_mode_info(), Some(mode));

        // Test basic pixel operations in each mode
        backend.set_pixel(10, 10, Color::YELLOW);
        backend.flush(); // Should not crash
        assert_eq!(backend.get_pixel(10, 10), Color::YELLOW);

        backend.close_graphics();
    }
}

/// Test performance requirements for visual backend
#[cfg(feature = "visual-backend")]
#[test]
fn test_visual_backend_performance() {
    let mut backend = bgi::backend::minifb::MiniFBBackend::new()
        .expect("Backend creation should succeed");

    let mode = GraphicsMode { width: 640, height: 480, colors: 16, mode_id: 1 };
    backend.init_graphics(mode).expect("Initialization should succeed");

    let start = std::time::Instant::now();

    // Test pixel operations performance (should be fast)
    for i in 0..1000 {
        backend.set_pixel(i % 640, (i / 640) % 480, Color::WHITE);
    }

    let pixel_time = start.elapsed();
    assert!(pixel_time.as_millis() < 100, "1000 pixel operations should complete in <100ms");

    // Test flush performance
    let flush_start = std::time::Instant::now();
    backend.flush();
    let flush_time = flush_start.elapsed();

    // Flush should complete within 16ms for 60 FPS target (but we allow 33ms for 30 FPS)
    assert!(flush_time.as_millis() < 33, "Flush should complete in <33ms for 30 FPS");

    backend.close_graphics();
}

/// Placeholder test that will fail until implementation is complete
#[test]
#[should_panic(expected = "MiniFB backend not implemented")]
fn test_minifb_backend_placeholder() {
    #[cfg(not(feature = "visual-backend"))]
    panic!("MiniFB backend not implemented");

    #[cfg(feature = "visual-backend")]
    {
        // This will fail until the actual MiniFBBackend type exists
        let _backend = bgi::backend::minifb::MiniFBBackend::new();
        panic!("MiniFB backend not implemented");
    }
}
