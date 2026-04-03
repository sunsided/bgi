//! Visual backend (minifb) contract tests
//!
//! Tests specific to the minifb visual backend implementation.
//! These tests verify window management, scaling, and visual rendering.

#[cfg(feature = "visual-backend")]
mod minifb_tests {
    use bgi::backend::{Backend, minifb::MiniFbBackend};
    use bgi::color::RgbColor;
    use bgi::types::GraphicsMode;

    /// Test that minifb backend can be created and initialized
    #[test]
    fn test_minifb_backend_creation() {
        // Test creation
        let mut backend = MiniFbBackend::new();

        // Test initialization
        let init_result = backend.init();
        assert!(
            init_result.is_ok(),
            "MiniFB backend initialization should succeed"
        );

        // Test cleanup
        let shutdown_result = backend.shutdown();
        assert!(
            shutdown_result.is_ok(),
            "MiniFB backend shutdown should succeed"
        );
    }

    /// Test window creation and management
    #[test]
    fn test_window_management() {
        let mut backend = MiniFbBackend::new();
        backend
            .init()
            .expect("Backend initialization should succeed");

        // Create a window with standard VGA mode
        let vga_mode = GraphicsMode::default();
        let window_result = backend.create_window(640, 480, Some("Test Window"), vga_mode);
        assert!(window_result.is_ok(), "Window creation should succeed");

        let window_id = window_result.unwrap();
        assert!(
            backend.is_window_valid(window_id),
            "Created window should be valid"
        );

        // Test window size
        let size_result = backend.window_size(window_id);
        assert!(size_result.is_ok(), "Getting window size should succeed");
        let (width, height) = size_result.unwrap();
        assert_eq!(width, 640, "Window width should be 640");
        assert_eq!(height, 480, "Window height should be 480");

        // Close the window
        let close_result = backend.close_window(window_id);
        assert!(close_result.is_ok(), "Window close should succeed");
        assert!(
            !backend.is_window_valid(window_id),
            "Closed window should not be valid"
        );

        backend.shutdown().ok();
    }

    /// Test pixel operations
    #[test]
    fn test_pixel_operations() {
        let mut backend = MiniFbBackend::new();
        backend
            .init()
            .expect("Backend initialization should succeed");

        let mode = GraphicsMode::default();
        let window_id = backend
            .create_window(640, 480, None, mode)
            .expect("Window creation should succeed");

        // Test get_pixel (should return a color without crashing)
        let pixel_result = backend.get_pixel(window_id, 100, 100);
        assert!(pixel_result.is_ok(), "Getting pixel should succeed");

        backend.close_window(window_id).ok();
        backend.shutdown().ok();
    }

    /// Test multiple windows support
    #[test]
    fn test_multiple_windows() {
        let mut backend = MiniFbBackend::new();
        backend
            .init()
            .expect("Backend initialization should succeed");

        let caps = backend.capabilities();

        if caps.multi_window {
            let mode = GraphicsMode::default();

            // Create multiple windows
            let window1 = backend
                .create_window(320, 240, Some("Window 1"), mode.clone())
                .expect("First window creation should succeed");
            let window2 = backend
                .create_window(320, 240, Some("Window 2"), mode.clone())
                .expect("Second window creation should succeed");

            assert!(
                backend.is_window_valid(window1),
                "First window should be valid"
            );
            assert!(
                backend.is_window_valid(window2),
                "Second window should be valid"
            );

            // Test setting current window
            let set_result = backend.set_current_window(window1);
            assert!(set_result.is_ok(), "Setting current window should succeed");
            assert_eq!(
                backend.current_window(),
                Some(window1),
                "Current window should be window1"
            );

            backend.close_window(window1).ok();
            backend.close_window(window2).ok();
        }

        backend.shutdown().ok();
    }

    /// Test event polling (non-blocking)
    #[test]
    fn test_event_polling() {
        let mut backend = MiniFbBackend::new();
        backend
            .init()
            .expect("Backend initialization should succeed");

        let mode = GraphicsMode::default();
        let _window_id = backend
            .create_window(640, 480, None, mode)
            .expect("Window creation should succeed");

        // Poll events - should not block and should not crash
        let _events = backend.poll_events();

        // Check has_events - should work
        let _has_events = backend.has_events();

        backend.shutdown().ok();
    }

    /// Test backend capabilities
    #[test]
    fn test_backend_capabilities() {
        let backend = MiniFbBackend::new();
        let caps = backend.capabilities();

        // MiniFB backend should have basic capabilities
        // (exact capabilities depend on implementation)
        println!("Backend capabilities: {:?}", caps);

        // These tests just verify the capability fields exist and are valid bools
        let _multi = caps.multi_window;
        let _hw = caps.hardware_acceleration;
        let _alpha = caps.alpha_blending;
        let _fullscreen = caps.fullscreen;
        let _resizable = caps.resizable;
    }
}

/// Placeholder test for when visual-backend feature is not enabled
#[cfg(not(feature = "visual-backend"))]
#[test]
fn test_minifb_backend_requires_feature() {
    // This test just verifies the feature flag system works
    // MiniFB backend tests are only run when visual-backend feature is enabled
}
