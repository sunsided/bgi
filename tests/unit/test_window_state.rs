//! Unit tests for window state management.

use bgi::{Color, constants::*, window_state::*};

#[test]
fn test_driver_info_default() {
    let driver_info = DriverInfo::default();

    assert_eq!(driver_info.driver, VGA);
    assert_eq!(driver_info.name, "VGA");
    assert_eq!(driver_info.mode, VGAHI);
    assert_eq!(driver_info.mode_name, "VGA High");
}

#[test]
fn test_screen_mode_default() {
    let screen_mode = ScreenMode::default();

    assert_eq!(screen_mode.width, 640);
    assert_eq!(screen_mode.height, 480);
    assert_eq!(screen_mode.color_depth, 4);
    assert_eq!(screen_mode.max_colors, 16);
    assert_eq!(screen_mode.mode, VGAHI);
}

#[test]
fn test_window_properties_default() {
    let properties = WindowProperties::default();

    assert_eq!(properties.title, "BGI Graphics");
    assert!(!properties.resizable);
    assert!(properties.decorated);
    assert!(!properties.always_on_top);
    assert_eq!(properties.background_color, Color::BLACK);
}

#[test]
fn test_graphics_pages_default() {
    let pages = GraphicsPages::default();

    assert_eq!(pages.active_page, 0);
    assert_eq!(pages.visual_page, 0);
    assert_eq!(pages.total_pages, 1);
}

#[test]
fn test_window_state_default() {
    let state = WindowState::default();

    assert_eq!(state.driver_info.driver, VGA);
    assert_eq!(state.screen_mode.width, 640);
    assert_eq!(state.screen_mode.height, 480);
    assert_eq!(state.properties.title, "BGI Graphics");
    assert_eq!(state.pages.active_page, 0);
    assert!(!state.initialized);
    assert_eq!(state.error_code, 0);
    assert!(state.driver_path.is_empty());
}

#[test]
fn test_window_state_new() {
    let state = WindowState::new();
    let default_state = WindowState::default();

    assert_eq!(state.driver_info.driver, default_state.driver_info.driver);
    assert_eq!(state.screen_mode.width, default_state.screen_mode.width);
    assert_eq!(state.initialized, default_state.initialized);
}

#[test]
fn test_init_graphics_vga_modes() {
    let mut state = WindowState::new();

    // Test VGA High mode
    let result = state.init_graphics(VGA, VGAHI, "/path/to/drivers");
    assert!(result.is_ok());
    assert!(state.is_initialized());
    assert_eq!(state.get_error_code(), 0);
    assert_eq!(state.driver_info.driver, VGA);
    assert_eq!(state.driver_info.mode, VGAHI);
    assert_eq!(state.screen_mode.width, 640);
    assert_eq!(state.screen_mode.height, 480);
    assert_eq!(state.driver_path, "/path/to/drivers");

    // Test VGA Medium mode
    state = WindowState::new();
    let result = state.init_graphics(VGA, VGAMED, "");
    assert!(result.is_ok());
    assert_eq!(state.screen_mode.width, 640);
    assert_eq!(state.screen_mode.height, 350);

    // Test VGA Low mode
    state = WindowState::new();
    let result = state.init_graphics(VGA, VGALO, "");
    assert!(result.is_ok());
    assert_eq!(state.screen_mode.width, 640);
    assert_eq!(state.screen_mode.height, 200);
}

#[test]
fn test_init_graphics_ega_modes() {
    let mut state = WindowState::new();

    // Test EGA High mode
    let result = state.init_graphics(EGA, EGAHI, "");
    assert!(result.is_ok());
    assert_eq!(state.screen_mode.width, 640);
    assert_eq!(state.screen_mode.height, 350);
    assert_eq!(state.screen_mode.max_colors, 16);

    // Test EGA Low mode
    state = WindowState::new();
    let result = state.init_graphics(EGA, EGALO, "");
    assert!(result.is_ok());
    assert_eq!(state.screen_mode.width, 640);
    assert_eq!(state.screen_mode.height, 200);
}

#[test]
fn test_init_graphics_cga_mode() {
    let mut state = WindowState::new();

    let result = state.init_graphics(CGA, CGAHI, "");
    assert!(result.is_ok());
    assert_eq!(state.screen_mode.width, 640);
    assert_eq!(state.screen_mode.height, 200);
    assert_eq!(state.screen_mode.color_depth, 2);
    assert_eq!(state.screen_mode.max_colors, 4);
}

#[test]
fn test_init_graphics_invalid_driver() {
    let mut state = WindowState::new();

    let result = state.init_graphics(999, VGAHI, ""); // Invalid driver
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), -2); // grInvalidDriver
    assert!(!state.is_initialized());
    assert_eq!(state.get_error_code(), -2);
}

#[test]
fn test_init_graphics_invalid_mode() {
    let mut state = WindowState::new();

    let result = state.init_graphics(VGA, 999, ""); // Invalid mode for VGA
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), -10); // grInvalidMode
    assert!(!state.is_initialized());
    assert_eq!(state.get_error_code(), -10);
}

#[test]
fn test_close_graphics() {
    let mut state = WindowState::new();

    // Initialize first
    state.init_graphics(VGA, VGAHI, "").unwrap();
    assert!(state.is_initialized());

    // Close graphics
    state.close_graphics();
    assert!(!state.is_initialized());
    assert_eq!(state.get_error_code(), 0);
    assert_eq!(state.pages.active_page, 0);
    assert_eq!(state.pages.visual_page, 0);
}

#[test]
fn test_screen_dimensions() {
    let mut state = WindowState::new();
    state.init_graphics(VGA, VGAHI, "").unwrap();

    let (width, height) = state.get_screen_size();
    assert_eq!(width, 640);
    assert_eq!(height, 480);

    assert_eq!(state.get_max_x(), 639); // width - 1
    assert_eq!(state.get_max_y(), 479); // height - 1
    assert_eq!(state.get_max_color(), 15); // max_colors - 1
}

#[test]
fn test_page_management() {
    let mut state = WindowState::new();
    state.init_graphics(VGA, VGAHI, "").unwrap();

    // Initially both pages are 0
    assert_eq!(state.get_active_page(), 0);
    assert_eq!(state.get_visual_page(), 0);

    // Test setting active page (within bounds)
    let result = state.set_active_page(0);
    assert!(result.is_ok());
    assert_eq!(state.get_active_page(), 0);

    // Test setting visual page (within bounds)
    let result = state.set_visual_page(0);
    assert!(result.is_ok());
    assert_eq!(state.get_visual_page(), 0);

    // Test setting page out of bounds
    let result = state.set_active_page(5);
    assert!(result.is_err());
    assert_eq!(state.get_active_page(), 0); // Should remain unchanged

    let result = state.set_visual_page(-1);
    assert!(result.is_err());
    assert_eq!(state.get_visual_page(), 0); // Should remain unchanged
}

#[test]
fn test_window_title() {
    let mut state = WindowState::new();

    // Test default title
    assert_eq!(state.get_window_title(), "BGI Graphics");

    // Set new title
    state.set_window_title("My Application".to_string());
    assert_eq!(state.get_window_title(), "My Application");

    // Set empty title
    state.set_window_title("".to_string());
    assert_eq!(state.get_window_title(), "");
}

#[test]
fn test_error_code_management() {
    let mut state = WindowState::new();

    // Initially no error
    assert_eq!(state.get_error_code(), 0);

    // Set error code
    state.set_error_code(-5);
    assert_eq!(state.get_error_code(), -5);

    // Error code should persist through operations
    state.set_window_title("Test".to_string());
    assert_eq!(state.get_error_code(), -5);

    // Successful init should clear error
    state.init_graphics(VGA, VGAHI, "").unwrap();
    assert_eq!(state.get_error_code(), 0);
}

#[test]
fn test_driver_validation() {
    let state = WindowState::new();

    // Valid drivers
    assert!(state.is_valid_driver(VGA));
    assert!(state.is_valid_driver(EGA));
    assert!(state.is_valid_driver(CGA));
    assert!(state.is_valid_driver(MCGA));
    assert!(state.is_valid_driver(HERCMONO));
    assert!(state.is_valid_driver(DETECT));

    // Invalid drivers
    assert!(!state.is_valid_driver(999));
    assert!(!state.is_valid_driver(-1));
}

#[test]
fn test_mode_validation() {
    let state = WindowState::new();

    // Valid VGA modes
    assert!(state.is_valid_mode(VGA, VGALO));
    assert!(state.is_valid_mode(VGA, VGAMED));
    assert!(state.is_valid_mode(VGA, VGAHI));
    assert!(state.is_valid_mode(VGA, MCGAHI));

    // Valid EGA modes
    assert!(state.is_valid_mode(EGA, EGALO));
    assert!(state.is_valid_mode(EGA, EGAHI));

    // Valid CGA modes
    assert!(state.is_valid_mode(CGA, CGALO));
    assert!(state.is_valid_mode(CGA, CGAMED));
    assert!(state.is_valid_mode(CGA, CGAHI));

    // Invalid mode for VGA
    assert!(!state.is_valid_mode(VGA, 999)); // Clearly invalid mode value

    // Invalid modes
    assert!(!state.is_valid_mode(VGA, 999));
    assert!(!state.is_valid_mode(999, VGAHI));
}

#[test]
fn test_driver_name_generation() {
    let state = WindowState::new();

    assert_eq!(state.get_driver_name(VGA), "VGA");
    assert_eq!(state.get_driver_name(EGA), "EGA");
    assert_eq!(state.get_driver_name(CGA), "CGA");
    assert_eq!(state.get_driver_name(MCGA), "MCGA");
    assert_eq!(state.get_driver_name(HERCMONO), "Hercules");
    assert_eq!(state.get_driver_name(999), "Driver 999");
}

#[test]
fn test_mode_name_generation() {
    let state = WindowState::new();

    // VGA mode names
    assert_eq!(state.get_mode_name(VGA, VGALO), "VGA Low (640x200)");
    assert_eq!(state.get_mode_name(VGA, VGAMED), "VGA Medium (640x350)");
    assert_eq!(state.get_mode_name(VGA, VGAHI), "VGA High (640x480)");

    // EGA mode names
    assert_eq!(state.get_mode_name(EGA, EGALO), "EGA Low (640x200)");
    assert_eq!(state.get_mode_name(EGA, EGAHI), "EGA High (640x350)");

    // CGA mode names
    assert_eq!(state.get_mode_name(CGA, CGAHI), "CGA High (640x200)");

    // Unknown mode
    assert_eq!(state.get_mode_name(VGA, 999), "Mode 999");
}

#[test]
fn test_screen_mode_generation() {
    let state = WindowState::new();

    // Test VGA High mode
    let mode = state.get_screen_mode(VGA, VGAHI);
    assert!(mode.is_ok());
    let mode = mode.unwrap();
    assert_eq!(mode.width, 640);
    assert_eq!(mode.height, 480);
    assert_eq!(mode.color_depth, 4);
    assert_eq!(mode.max_colors, 16);
    assert_eq!(mode.mode, VGAHI);

    // Test MCGA mode (256 colors)
    let mode = state.get_screen_mode(VGA, MCGAHI);
    assert!(mode.is_ok());
    let mode = mode.unwrap();
    assert_eq!(mode.width, 320);
    assert_eq!(mode.height, 200);
    assert_eq!(mode.color_depth, 8);
    assert_eq!(mode.max_colors, 256);

    // Test invalid mode
    let mode = state.get_screen_mode(999, 999);
    assert!(mode.is_err());
    assert_eq!(mode.err().unwrap(), -10); // grInvalidMode
}

#[test]
fn test_structure_equality() {
    let driver1 = DriverInfo {
        driver: VGA,
        name: "VGA".to_string(),
        mode: VGAHI,
        mode_name: "VGA High".to_string(),
    };
    let driver2 = DriverInfo {
        driver: VGA,
        name: "VGA".to_string(),
        mode: VGAHI,
        mode_name: "VGA High".to_string(),
    };
    let driver3 = DriverInfo {
        driver: EGA,
        name: "EGA".to_string(),
        mode: EGAHI,
        mode_name: "EGA High".to_string(),
    };

    assert_eq!(driver1, driver2);
    assert_ne!(driver1, driver3);

    let mode1 = ScreenMode {
        width: 640,
        height: 480,
        color_depth: 4,
        max_colors: 16,
        mode: VGAHI,
    };
    let mode2 = ScreenMode {
        width: 640,
        height: 480,
        color_depth: 4,
        max_colors: 16,
        mode: VGAHI,
    };
    let mode3 = ScreenMode {
        width: 320,
        height: 200,
        color_depth: 8,
        max_colors: 256,
        mode: MCGAHI,
    };

    assert_eq!(mode1, mode2);
    assert_ne!(mode1, mode3);

    let pages1 = GraphicsPages {
        active_page: 0,
        visual_page: 0,
        total_pages: 1,
    };
    let pages2 = GraphicsPages {
        active_page: 0,
        visual_page: 0,
        total_pages: 1,
    };
    let pages3 = GraphicsPages {
        active_page: 1,
        visual_page: 0,
        total_pages: 2,
    };

    assert_eq!(pages1, pages2);
    assert_ne!(pages1, pages3);
}

#[test]
fn test_window_state_clone() {
    let mut state1 = WindowState::new();
    state1.init_graphics(VGA, VGAHI, "/test/path").unwrap();
    state1.set_window_title("Test Window".to_string());

    let state2 = state1.clone();

    assert_eq!(state1.is_initialized(), state2.is_initialized());
    assert_eq!(state1.get_window_title(), state2.get_window_title());
    assert_eq!(state1.driver_path, state2.driver_path);
    assert_eq!(state1.screen_mode.width, state2.screen_mode.width);

    // Verify they're independent
    state1.set_window_title("Modified".to_string());
    assert_eq!(state1.get_window_title(), "Modified");
    assert_eq!(state2.get_window_title(), "Test Window");
}

#[test]
fn test_edge_cases() {
    let mut state = WindowState::new();

    // Test operations on uninitialized state
    assert_eq!(state.get_screen_size(), (640, 480)); // Should return default
    assert_eq!(state.get_max_x(), 639);
    assert_eq!(state.get_max_y(), 479);

    // Test multiple initializations
    state.init_graphics(VGA, VGAHI, "/path1").unwrap();
    assert_eq!(state.driver_path, "/path1");

    state.init_graphics(EGA, EGAHI, "/path2").unwrap();
    assert_eq!(state.driver_path, "/path2");
    assert_eq!(state.screen_mode.height, 350); // Should be updated
}

#[test]
fn test_multiple_pages_scenario() {
    let mut state = WindowState::new();

    // Simulate a system with multiple pages
    state.pages.total_pages = 4;

    // Test valid page operations
    assert!(state.set_active_page(2).is_ok());
    assert_eq!(state.get_active_page(), 2);

    assert!(state.set_visual_page(3).is_ok());
    assert_eq!(state.get_visual_page(), 3);

    // Test boundary cases
    assert!(state.set_active_page(0).is_ok()); // First page
    assert!(state.set_active_page(3).is_ok()); // Last page

    assert!(state.set_active_page(4).is_err()); // Out of bounds
    assert!(state.set_visual_page(-1).is_err()); // Negative
}
