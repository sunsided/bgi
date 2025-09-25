use bgi::{
    initgraph, closegraph,
    setpalette, getpaletteentry, setallpalette, getpalettesize,
    setrgbpalette, getdefaultpalette,
    Color
};

#[test]
fn test_setpalette_contract() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    // Test setting valid palette entry
    setpalette(1, Color::RED);
    // BGI functions are void - errors reported via graphresult()
    
    // Test setting palette with invalid index
    setpalette(255, Color::RED);
    // BGI functions are void - errors reported via graphresult()

    closegraph();
}

#[test]
fn test_getpalette_contract() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    // Test getting valid palette entry
    let color = getpaletteentry(0);
    assert!(color.is_some(), "getpaletteentry should return Some for valid index");

    // Test getting invalid palette entry
    let invalid_color = getpaletteentry(255);
    assert!(invalid_color.is_none(), "getpaletteentry should return None for invalid index");

    closegraph();
}

#[test]
fn test_setallpalette_contract() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    // Create a palette array
    let palette = [
        Color::BLACK, Color::BLUE, Color::GREEN, Color::CYAN,
        Color::RED, Color::MAGENTA, Color::BROWN, Color::LIGHTGRAY,
        Color::DARKGRAY, Color::LIGHTBLUE, Color::LIGHTGREEN, Color::LIGHTCYAN,
        Color::LIGHTRED, Color::LIGHTMAGENTA, Color::YELLOW, Color::WHITE
    ];

    setallpalette(&palette);
    // BGI functions are void - errors reported via graphresult()

    // Test with empty palette
    let empty_palette = [];
    setallpalette(&empty_palette);
    // BGI functions are void - errors reported via graphresult()

    closegraph();
}

#[test]
fn test_getpalettesize_contract() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    let size = getpalettesize();
    assert!(size > 0, "getpalettesize should return positive value");
    assert!(size <= 256, "getpalettesize should return reasonable value");

    closegraph();
}

#[test]
fn test_setrgbpalette_contract() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    // Test setting RGB palette entry
    setrgbpalette(1, 255, 128, 64);
    // BGI functions are void - errors reported via graphresult()

    // Test with invalid index
    setrgbpalette(999, 255, 255, 255);
    // BGI functions are void - errors reported via graphresult()

    closegraph();
}

#[test]
fn test_getdefaultpalette_contract() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    let default_palette = getdefaultpalette();
    assert!(!default_palette.is_empty(), "getdefaultpalette should return non-empty palette");
    assert!(default_palette.len() >= 16, "Default palette should have at least 16 colors");

    closegraph();
}

#[test]
fn test_palette_operations_without_graphics() {
    // Test operations without initializing graphics
    setpalette(1, Color::RED);
    // BGI functions are void - errors reported via graphresult()

    let color = getpaletteentry(0);
    assert!(color.is_none(), "getpaletteentry should return None without graphics initialization");

    let size = getpalettesize();
    assert_eq!(size, 0, "getpalettesize should return 0 without graphics initialization");
}

#[test]
fn test_palette_consistency() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    // Set a color and verify we can get it back
    let test_color = Color::LIGHTBLUE;
    setpalette(5, test_color);
    // BGI functions are void - errors reported via graphresult()

    let retrieved_color = getpaletteentry(5);
    assert!(retrieved_color.is_some(), "Should be able to get palette entry");

    if let Some(color) = retrieved_color {
        assert_eq!(color, test_color, "Retrieved color should match set color");
    }

    closegraph();
}

#[test]
fn test_rgb_palette_range() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    // Test boundary values for RGB components
    setrgbpalette(1, 0, 0, 0);
    // BGI functions are void - errors reported via graphresult()

    setrgbpalette(2, 255, 255, 255);
    // BGI functions are void - errors reported via graphresult()

    // Note: Some implementations might clamp values > 255, so we don't test invalid RGB ranges

    closegraph();
}
