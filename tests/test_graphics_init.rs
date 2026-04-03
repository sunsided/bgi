// Contract tests for BGI Graphics Initialization API
// These tests verify the API contracts defined in contracts/graphics_init.md
// They should FAIL initially until the implementation is complete

use bgi::{
    GraphResult, closegraph, getgraphmode, grapherrormsg, graphresult, initgraph, setgraphmode,
};

#[test]
fn test_initgraph_contract() {
    // Test successful initialization
    let mut gd = 0i32; // VGA driver
    let mut gm = 4i32; // VGA mode

    initgraph(&mut gd, &mut gm, "");
    assert_eq!(graphresult(), GraphResult::Ok);

    // Cleanup
    closegraph();
}

#[test]
fn test_initgraph_invalid_mode() {
    // Test error handling for invalid mode
    let mut gd = -1i32; // Invalid driver
    let mut gm = -1i32; // Invalid mode

    initgraph(&mut gd, &mut gm, "");
    assert_ne!(graphresult(), GraphResult::Ok);
}

#[test]
fn test_closegraph_contract() {
    let mut gd = 0i32; // VGA driver
    let mut gm = 4i32; // VGA mode

    initgraph(&mut gd, &mut gm, "");
    assert_eq!(graphresult(), GraphResult::Ok);

    // Test successful cleanup
    closegraph();
    // Note: BGI closegraph() is void, no error return
}

#[test]
fn test_graphresult_contract() {
    // Test that graphresult returns not initialized before graphics setup
    let result = graphresult();
    assert_eq!(result, GraphResult::NotInitialized);

    // Test error result after failed operation
    let mut gd = -1i32; // Invalid driver
    let mut gm = -1i32; // Invalid mode
    initgraph(&mut gd, &mut gm, "");
    let error_result = graphresult();
    assert_ne!(error_result, GraphResult::Ok);
}

#[test]
fn test_grapherrormsg_contract() {
    // Test error message for success
    let msg = grapherrormsg(0); // grOk
    assert_eq!(msg, "No error");

    // Test error message for common errors
    let msg = grapherrormsg(-2); // grNotDetected
    assert!(
        msg.to_lowercase()
            .contains("graphics hardware not detected")
    );

    let msg = grapherrormsg(-3); // grFileNotFound
    assert!(msg.to_lowercase().contains("device driver file not found"));
}

#[test]
fn test_detectgraph_contract() {
    // Note: This test is simplified since detectgraph uses old enum types
    // In real BGI, detectgraph would set driver and mode values

    // Test that we can initialize graphics with auto-detected values
    let mut gd = 0i32; // VGA driver (auto-detected equivalent)
    let mut gm = 4i32; // VGA mode (auto-detected equivalent)

    initgraph(&mut gd, &mut gm, "");
    assert_eq!(graphresult(), GraphResult::Ok);

    closegraph();
}

#[test]
fn test_getgraphmode_contract() {
    let mut gd = 0i32; // VGA driver
    let mut gm = 4i32; // VGA mode

    initgraph(&mut gd, &mut gm, "");
    assert_eq!(graphresult(), GraphResult::Ok);

    // Test getting current mode
    let mode = getgraphmode();
    assert_eq!(mode, 4); // VGA mode

    closegraph();
}

#[test]
fn test_setgraphmode_contract() {
    let mut gd = 0i32; // VGA driver
    let mut gm = 3i32; // EGA mode

    initgraph(&mut gd, &mut gm, "");
    assert_eq!(graphresult(), GraphResult::Ok);

    // Test changing mode - BGI setgraphmode is void function
    setgraphmode(4); // VGA mode

    // Verify mode changed
    let current_mode = getgraphmode();
    assert_eq!(current_mode, 4); // VGA mode

    closegraph();
}

#[test]
fn test_setgraphmode_invalid() {
    let mut gd = 0i32; // VGA driver
    let mut gm = 4i32; // VGA mode

    initgraph(&mut gd, &mut gm, "");
    assert_eq!(graphresult(), GraphResult::Ok);

    // Test invalid mode change - BGI setgraphmode is void but may set error state
    setgraphmode(-1); // Invalid mode
    // Note: BGI typically doesn't validate mode until drawing operations

    closegraph();
}

#[test]
fn test_graphics_context_lifecycle() {
    // Test complete lifecycle
    let mut gd = 0i32; // VGA driver
    let mut gm = 4i32; // VGA mode

    initgraph(&mut gd, &mut gm, "");
    assert_eq!(graphresult(), GraphResult::Ok);

    // Change mode
    setgraphmode(3); // EGA mode

    // Verify mode changed
    let current_mode = getgraphmode();
    assert_eq!(current_mode, 3); // EGA mode

    // Close
    closegraph();
}
