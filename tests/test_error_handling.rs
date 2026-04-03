use bgi::{
    BgiError, Color, GraphResult, closegraph, grapherrormsg, graphresult, initgraph, line, setcolor,
};

#[test]
fn test_error_handling_initialization() {
    // Test successful initialization
    let mut driver = 9; // VGA
    let mut mode = 2; // VGAHI
    initgraph(&mut driver, &mut mode, "");

    let result = graphresult();
    assert_eq!(
        result,
        GraphResult::Ok,
        "Graphics should initialize successfully"
    );

    closegraph();
}

#[test]
fn test_error_handling_invalid_driver() {
    // Test initialization with invalid driver
    let mut driver = 999; // Invalid driver
    let mut mode = 2;

    initgraph(&mut driver, &mut mode, "");

    let result = graphresult();
    // Should return an error code, not Ok
    assert_ne!(
        result,
        GraphResult::Ok,
        "Invalid driver should produce error"
    );

    // Try to close gracefully even after error
    closegraph();
}

#[test]
fn test_error_handling_invalid_mode() {
    // Test initialization with invalid mode
    let mut driver = 9; // VGA
    let mut mode = 999; // Invalid mode

    initgraph(&mut driver, &mut mode, "");

    let result = graphresult();
    // Should return an error code
    assert_ne!(result, GraphResult::Ok, "Invalid mode should produce error");

    closegraph();
}

#[test]
fn test_error_messages() {
    // Test error message retrieval
    let mut driver = 999; // Invalid driver
    let mut mode = 2;

    initgraph(&mut driver, &mut mode, "");

    let result = graphresult();
    if result != GraphResult::Ok {
        let error_msg = grapherrormsg(result);
        assert!(!error_msg.is_empty(), "Error message should not be empty");
        assert!(error_msg.len() > 5, "Error message should be descriptive");
    }

    closegraph();
}

#[test]
fn test_error_handling_operations_after_failure() {
    // Test operations after initialization failure
    let mut driver = 999; // Invalid driver
    let mut mode = 2;

    initgraph(&mut driver, &mut mode, "");

    let result = graphresult();
    if result != GraphResult::Ok {
        // Try graphics operations after failure - should handle gracefully
        setcolor(Color::WHITE);
        line(10, 10, 100, 100);

        // Should not crash, but operations may be no-ops or return errors
    }

    closegraph();
}

#[test]
fn test_error_handling_multiple_failures() {
    // Test multiple initialization attempts with different invalid parameters
    let invalid_configs = [
        (999, 2),   // Invalid driver
        (9, 999),   // Invalid mode
        (888, 777), // Both invalid
    ];

    for &(driver, mode) in &invalid_configs {
        let mut d = driver;
        let mut m = mode;

        initgraph(&mut d, &mut m, "");

        let result = graphresult();
        assert_ne!(
            result,
            GraphResult::Ok,
            "Invalid config ({}, {}) should fail",
            driver,
            mode
        );

        let error_msg = grapherrormsg(result);
        assert!(
            !error_msg.is_empty(),
            "Should have error message for config ({}, {})",
            driver,
            mode
        );

        closegraph();
    }
}

#[test]
fn test_error_handling_recovery() {
    // Test recovery after error by reinitializing correctly

    // First, cause an error
    let mut driver = 999;
    let mut mode = 2;
    initgraph(&mut driver, &mut mode, "");

    let first_result = graphresult();
    assert_ne!(
        first_result,
        GraphResult::Ok,
        "First initialization should fail"
    );

    closegraph();

    // Now initialize correctly
    let mut driver2 = 9; // VGA
    let mut mode2 = 2; // VGAHI
    initgraph(&mut driver2, &mut mode2, "");

    let second_result = graphresult();
    assert_eq!(
        second_result,
        GraphResult::Ok,
        "Second initialization should succeed"
    );

    // Test that graphics operations work after recovery
    setcolor(Color::WHITE);
    line(10, 10, 100, 100);

    closegraph();
}

#[test]
fn test_error_handling_result_consistency() {
    // Test that graphresult() returns consistent results
    let mut driver = 9; // VGA
    let mut mode = 2; // VGAHI
    initgraph(&mut driver, &mut mode, "");

    let result1 = graphresult();
    let result2 = graphresult();
    let result3 = graphresult();

    assert_eq!(result1, result2, "graphresult should be consistent");
    assert_eq!(result2, result3, "graphresult should remain consistent");

    closegraph();
}

#[test]
fn test_error_handling_all_error_types() {
    // Test all possible GraphResult error types have valid messages
    let error_types = [
        GraphResult::Ok,
        GraphResult::GraphicsNotInitialized,
        GraphResult::GraphicsError,
        GraphResult::DeviceDriverFileNotFound,
        GraphResult::InvalidDeviceDriverFile,
        GraphResult::NotEnoughMemoryForDriver,
        GraphResult::OutOfMemory,
        GraphResult::IOError,
        GraphResult::InvalidMode,
        GraphResult::FontFileError,
    ];

    for &error_type in &error_types {
        let error_msg = grapherrormsg(error_type);

        if error_type == GraphResult::Ok {
            // OK might have empty message or "No error"
            assert!(
                error_msg.is_empty() || error_msg.contains("No error") || error_msg.contains("OK"),
                "OK result should have appropriate message"
            );
        } else {
            // Error types should have non-empty descriptive messages
            assert!(
                !error_msg.is_empty(),
                "Error type {:?} should have message",
                error_type
            );
            assert!(
                error_msg.len() > 3,
                "Error message for {:?} should be descriptive",
                error_type
            );
        }
    }
}

#[test]
fn test_error_handling_bgi_error_conversion() {
    // Test BgiError enum and its conversion from GraphResult
    let mut driver = 999; // Invalid driver
    let mut mode = 2;

    initgraph(&mut driver, &mut mode, "");

    let result = graphresult();
    if result != GraphResult::Ok {
        // Test that we can convert GraphResult to BgiError
        let bgi_error = BgiError::from(result);

        // Test that the error has a proper string representation
        let error_string = format!("{}", bgi_error);
        assert!(
            !error_string.is_empty(),
            "BgiError should have string representation"
        );

        // Test that the error has a proper debug representation
        let debug_string = format!("{:?}", bgi_error);
        assert!(
            !debug_string.is_empty(),
            "BgiError should have debug representation"
        );
    }

    closegraph();
}

#[test]
fn test_error_handling_graceful_degradation() {
    // Test that the system handles errors gracefully without crashing

    // Try various invalid operations
    setcolor(Color::WHITE); // Without graphics init
    line(10, 10, 100, 100); // Without graphics init

    let result = graphresult(); // Check result without init
    // Should return some error, not crash

    let error_msg = grapherrormsg(result);
    // Should return some message, not crash

    // Try to close without init
    closegraph();

    // All of the above should complete without panicking
}

#[test]
fn test_error_handling_resource_cleanup() {
    // Test that resources are properly cleaned up even after errors

    for _iteration in 0..3 {
        // Try to initialize with invalid parameters
        let mut driver = 999;
        let mut mode = 2;
        initgraph(&mut driver, &mut mode, "");

        // Always call closegraph to ensure cleanup
        closegraph();

        // Now try valid initialization
        let mut valid_driver = 9;
        let mut valid_mode = 2;
        initgraph(&mut valid_driver, &mut valid_mode, "");

        let result = graphresult();
        // Should still be able to initialize successfully after previous failures
        assert_eq!(result, GraphResult::Ok, "Should recover after cleanup");

        closegraph();
    }
}

#[test]
fn test_error_handling_concurrent_operations() {
    // Test error handling when multiple operations might fail
    let mut driver = 9;
    let mut mode = 2;
    initgraph(&mut driver, &mut mode, "");

    // Perform multiple operations rapidly
    for i in 0..10 {
        setcolor(Color::WHITE);
        line(i * 10, i * 10, i * 10 + 50, i * 10 + 50);

        let result = graphresult();
        // Should remain OK throughout
        assert_eq!(
            result,
            GraphResult::Ok,
            "Operations should not cause errors"
        );
    }

    closegraph();
}

#[test]
fn test_error_handling_edge_cases() {
    let mut driver = 9;
    let mut mode = 2;
    initgraph(&mut driver, &mut mode, "");

    // Test error handling with extreme coordinates
    setcolor(Color::WHITE);
    line(-10000, -10000, 10000, 10000); // Very large coordinates
    line(0, 0, 0, 0); // Zero-length line

    // Should handle gracefully without errors
    let result = graphresult();
    assert_eq!(
        result,
        GraphResult::Ok,
        "Extreme coordinates should not cause errors"
    );

    closegraph();
}
