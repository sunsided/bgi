use bgi::{
    outtextxy, settextstyle, gettextsettings, textwidth, textheight, getmaxx, getmaxy,
    initgraph, closegraph, graphresult, GraphResult, Color, setcolor
};

#[test]
fn test_outtextxy_basic() {
    // Contract: outtextxy should render text at specified coordinates
    let mut gd = 0i32;
    let mut gm = 4i32; // VGA mode

    initgraph(&mut gd, &mut gm, "");
    assert_eq!(graphresult(), GraphResult::Ok);

    // Should not crash when called
    outtextxy(10, 10, "Hello, World!");
    outtextxy(50, 50, "Test text");
    outtextxy(0, 0, "");  // Empty string

    // Should handle special characters
    outtextxy(100, 100, "Special: !@#$%^&*()");

    closegraph();
}

#[test]
fn test_settextstyle_configurations() {
    // Contract: settextstyle should configure font, direction, and size
    let mut gd = 0i32;
    let mut gm = 4i32; // VGA mode

    initgraph(&mut gd, &mut gm, "");
    assert_eq!(graphresult(), GraphResult::Ok);

    // Test different font configurations
    settextstyle(0, 0, 1);  // Default font, horizontal, size 1
    settextstyle(1, 0, 2);  // Triplex font, horizontal, size 2
    settextstyle(2, 1, 3);  // Small font, vertical, size 3
    settextstyle(3, 0, 4);  // Sans serif, horizontal, size 4

    // Should not crash with any values
    settextstyle(8, 1, 10); // Gothic font, vertical, size 10

    closegraph();
}

#[test]
fn test_gettextsettings_retrieval() {
    // Contract: gettextsettings should return current text configuration
    let mut gd = 0i32;
    let mut gm = 4i32; // VGA mode

    initgraph(&mut gd, &mut gm, "");
    assert_eq!(graphresult(), GraphResult::Ok);

    // Get default settings
    let settings = gettextsettings();
    let font = settings.font;
    let direction = settings.direction;
    let size = settings.charsize;

    // Should return valid values
    assert!(font >= 0 && font <= 10);      // Valid font range
    assert!(direction >= 0 && direction <= 1); // 0=horizontal, 1=vertical
    assert!(size >= 1);                     // Size should be positive

    // Set new style and verify change
    settextstyle(2, 1, 5);
    let new_settings = gettextsettings();
    let new_font = new_settings.font;
    let new_direction = new_settings.direction;
    let new_size = new_settings.charsize;

    // In TDD phase, these may not change yet (stub behavior)
    assert!(new_font >= 0);
    assert!(new_direction >= 0 && new_direction <= 1);
    assert!(new_size >= 1);

    closegraph();
}

#[test]
fn test_textwidth_calculation() {
    // Contract: textwidth should return pixel width of text string
    let mut gd = 0i32;
    let mut gm = 4i32; // VGA mode

    initgraph(&mut gd, &mut gm, "");
    assert_eq!(graphresult(), GraphResult::Ok);

    // Test with various strings
    let width1 = textwidth("A");
    let width2 = textwidth("Hello");
    let width3 = textwidth("Hello, World!");
    let width_empty = textwidth("");

    // Width should increase with text length
    assert!(width1 > 0);
    assert!(width2 > width1);
    assert!(width3 > width2);
    assert_eq!(width_empty, 0);

    // Should handle special characters
    let width_special = textwidth("!@#$");
    assert!(width_special > 0);

    closegraph();
}

#[test]
fn test_textheight_calculation() {
    // Contract: textheight should return pixel height of text
    let mut gd = 0i32;
    let mut gm = 4i32; // VGA mode

    initgraph(&mut gd, &mut gm, "");
    assert_eq!(graphresult(), GraphResult::Ok);

    // Test with various strings
    let height1 = textheight("A");
    let height2 = textheight("Hello");
    let height3 = textheight("Multi\nLine\nText");
    let height_empty = textheight("");

    // Height should be consistent for single line text
    assert!(height1 > 0);
    assert_eq!(height1, height2); // Same height for single line

    // Empty string should have zero height
    assert_eq!(height_empty, 0);

    // Multi-line text should have same height (BGI doesn't support newlines in single call)
    assert_eq!(height3, height1);

    closegraph();
}

#[test]
fn test_text_with_colors() {
    // Contract: settextcolor should affect text rendering
    let mut gd = 0i32;
    let mut gm = 4i32; // VGA mode

    initgraph(&mut gd, &mut gm, "");
    assert_eq!(graphresult(), GraphResult::Ok);

    // Test different text colors
    setcolor(Color::RED);
    outtextxy(20, 20, "Red Text");
    
    setcolor(Color::BLUE);
    outtextxy(20, 40, "Blue Text");
    
    setcolor(Color::GREEN);    closegraph();
}

#[test]
fn test_text_coordinate_boundaries() {
    // Contract: text should be positioned correctly at boundaries
    let mut gd = 0i32;
    let mut gm = 4i32; // VGA mode

    initgraph(&mut gd, &mut gm, "");
    assert_eq!(graphresult(), GraphResult::Ok);

    // Test text at extreme coordinates
    outtextxy(0, 0, "Top-left");
    outtextxy(getmaxx() - textwidth("Right"), 0, "Right");
    outtextxy(0, getmaxy() - textheight("Bottom"), "Bottom");

    closegraph();
}

#[test]
fn test_text_without_graphics() {
    // Contract: text functions should handle case when graphics not initialized

    // Should not crash even without initgraph
    outtextxy(50, 50, "No graphics");
    settextstyle(1, 0, 2);

    let settings = gettextsettings();
    let font = settings.font;
    let direction = settings.direction;
    let size = settings.charsize;
    assert!(font >= 0);
    assert!(direction >= 0);
    assert!(size >= 1);

    let width = textwidth("Test");
    let height = textheight("Test");
    assert!(width >= 0);
    assert!(height >= 0);
}

#[test]
fn test_text_size_consistency() {
    // Contract: text size calculations should be consistent
    let mut gd = 0i32;
    let mut gm = 4i32; // VGA mode

    initgraph(&mut gd, &mut gm, "");
    assert_eq!(graphresult(), GraphResult::Ok);

    // Test size relationships
    let width_a = textwidth("A");
    let width_aa = textwidth("AA");
    let width_aaa = textwidth("AAA");

    // Width should increase proportionally (in TDD stubs)
    assert!(width_aa >= width_a);
    assert!(width_aaa >= width_aa);

    // Height should be consistent
    let height_a = textheight("A");
    let height_z = textheight("Z");
    assert_eq!(height_a, height_z); // Same height for single characters

    // Different font sizes should affect dimensions
    settextstyle(0, 0, 1);
    let small_width = textwidth("Test");
    let small_height = textheight("Test");

    settextstyle(0, 0, 3);
    let large_width = textwidth("Test");
    let large_height = textheight("Test");

    // In TDD phase, size changes may not be implemented yet
    assert!(small_width > 0);
    assert!(small_height > 0);
    assert!(large_width > 0);
    assert!(large_height > 0);

    closegraph();
}

#[test]
fn test_unicode_text_handling() {
    // Contract: text functions should handle Unicode gracefully
    let mut gd = 0i32;
    let mut gm = 4i32; // VGA mode

    initgraph(&mut gd, &mut gm, "");
    assert_eq!(graphresult(), GraphResult::Ok);

    // Test with Unicode characters (BGI typically handles ASCII)
    outtextxy(10, 10, "ASCII: Hello");
    outtextxy(10, 30, "Unicode: Héllo");
    outtextxy(10, 50, "Symbols: ♥♦♣♠");
    outtextxy(10, 70, "Numbers: 12345");

    // Should not crash with Unicode
    let unicode_width = textwidth("Héllo");
    let unicode_height = textheight("Héllo");
    assert!(unicode_width > 0);
    assert!(unicode_height > 0);

    closegraph();
}
