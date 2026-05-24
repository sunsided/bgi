//! Graphics state management tests.
//! These tests verify that graphics state (colors, line styles, fill patterns, viewports)
//! is properly initialized, modified, and persisted across operations.

use bgi::*;

#[test]
fn test_graphics_state_initialization() {
    let mut driver = VGA;
    let mut mode = VGAHI;
    initgraph(&mut driver, &mut mode, "");

    // Test that graphics state is properly initialized
    assert_eq!(
        graphresult(),
        GraphResult::Ok,
        "Graphics should initialize successfully"
    );

    // Test default state values
    assert_eq!(getcolor(), Color::WHITE, "Default color should be WHITE");
    assert_eq!(
        getbkcolor(),
        Color::BLACK,
        "Default background color should be BLACK"
    );

    let line_settings = getlinesettings();
    assert_eq!(
        line_settings.linestyle, SOLID_LINE,
        "Default line style should be SOLID_LINE"
    );
    assert_eq!(
        line_settings.thickness, NORM_WIDTH,
        "Default line thickness should be NORM_WIDTH"
    );

    closegraph();
}

#[test]
fn test_color_state_management() {
    let mut driver = DETECT;
    let mut mode = 0;
    initgraph(&mut driver, &mut mode, "");

    // Test color state changes
    setcolor(Color::RED);
    assert_eq!(
        getcolor(),
        Color::RED,
        "Current color should be RED after setcolor(RED)"
    );

    setcolor(Color::BLUE);
    assert_eq!(
        getcolor(),
        Color::BLUE,
        "Current color should be BLUE after setcolor(BLUE)"
    );

    setbkcolor(Color::YELLOW);
    assert_eq!(
        getbkcolor(),
        Color::YELLOW,
        "Background color should be YELLOW after setbkcolor(YELLOW)"
    );

    closegraph();
}

#[test]
fn test_line_style_state_management() {
    let mut driver = DETECT;
    let mut mode = 0;
    initgraph(&mut driver, &mut mode, "");

    // Test line style state changes
    setlinestyle(DASHED_LINE, 0, NORM_WIDTH);
    let settings1 = getlinesettings();
    assert_eq!(
        settings1.linestyle, DASHED_LINE,
        "Line style should be DASHED_LINE"
    );
    assert_eq!(
        settings1.thickness, NORM_WIDTH,
        "Line thickness should be NORM_WIDTH"
    );

    setlinestyle(DOTTED_LINE, 0, THICK_WIDTH);
    let settings2 = getlinesettings();
    assert_eq!(
        settings2.linestyle, DOTTED_LINE,
        "Line style should be DOTTED_LINE"
    );
    assert_eq!(
        settings2.thickness, THICK_WIDTH,
        "Line thickness should be THICK_WIDTH"
    );

    closegraph();
}

#[test]
fn test_fill_style_state_management() {
    let mut driver = DETECT;
    let mut mode = 0;
    initgraph(&mut driver, &mut mode, "");

    // Test fill style state changes
    setfillstyle(SOLID_FILL, Color::RED);
    let fill_settings1 = getfillsettings();
    assert_eq!(
        fill_settings1.pattern, SOLID_FILL,
        "Fill pattern should be SOLID_FILL"
    );
    assert_eq!(
        fill_settings1.color,
        Color::RED.to_index() as i32,
        "Fill color should be RED"
    );

    setfillstyle(LINE_FILL, Color::BLUE);
    let fill_settings2 = getfillsettings();
    assert_eq!(
        fill_settings2.pattern, LINE_FILL,
        "Fill pattern should be LINE_FILL"
    );
    assert_eq!(
        fill_settings2.color,
        Color::BLUE.to_index() as i32,
        "Fill color should be BLUE"
    );

    closegraph();
}

#[test]
fn test_viewport_state_management() {
    let mut driver = DETECT;
    let mut mode = 0;
    initgraph(&mut driver, &mut mode, "");

    // Test viewport state changes
    setviewport(10, 10, 100, 100, true);
    let viewport1 = getviewsettings();
    assert_eq!(viewport1.left, 10, "Viewport left should be 10");
    assert_eq!(viewport1.top, 10, "Viewport top should be 10");
    assert_eq!(viewport1.right, 100, "Viewport right should be 100");
    assert_eq!(viewport1.bottom, 100, "Viewport bottom should be 100");
    assert!(viewport1.clip, "Viewport clipping should be enabled");

    setviewport(0, 0, 200, 200, false);
    let viewport2 = getviewsettings();
    assert_eq!(viewport2.left, 0, "Viewport left should be 0");
    assert_eq!(viewport2.top, 0, "Viewport top should be 0");
    assert_eq!(viewport2.right, 200, "Viewport right should be 200");
    assert_eq!(viewport2.bottom, 200, "Viewport bottom should be 200");
    assert!(!viewport2.clip, "Viewport clipping should be disabled");

    closegraph();
}

#[test]
fn test_graphics_state_persistence() {
    let mut driver = DETECT;
    let mut mode = 0;
    initgraph(&mut driver, &mut mode, "");

    // Set various state values
    setcolor(Color::CYAN);
    setbkcolor(Color::MAGENTA);
    setlinestyle(DASHED_LINE, 0, THICK_WIDTH);
    setfillstyle(HATCH_FILL, Color::GREEN);
    setviewport(5, 5, 50, 50, true);

    // Perform some graphics operations that might modify state
    line(10, 10, 20, 20);
    circle(25, 25, 10);

    // Verify state is preserved
    assert_eq!(
        getcolor(),
        Color::CYAN,
        "Color should persist after graphics operations"
    );
    assert_eq!(
        getbkcolor(),
        Color::MAGENTA,
        "Background color should persist"
    );

    let line_settings = getlinesettings();
    assert_eq!(
        line_settings.linestyle, DASHED_LINE,
        "Line style should persist"
    );
    assert_eq!(
        line_settings.thickness, THICK_WIDTH,
        "Line thickness should persist"
    );

    let fill_settings = getfillsettings();
    assert_eq!(
        fill_settings.pattern, HATCH_FILL,
        "Fill pattern should persist"
    );
    assert_eq!(
        fill_settings.color,
        Color::GREEN.to_index() as i32,
        "Fill color should persist"
    );

    let viewport = getviewsettings();
    assert_eq!(viewport.left, 5, "Viewport should persist");
    assert!(viewport.clip, "Viewport clipping should persist");

    closegraph();
}
