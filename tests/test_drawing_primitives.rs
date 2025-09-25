// Contract tests for BGI Drawing Primitives API
// These tests verify the API contracts defined in contracts/drawing_primitives.md
// They should FAIL initially until the implementation is complete

use bgi::{
    initgraph, closegraph, line, circle, rectangle, arc, putpixel, getpixel, ellipse, graphresult,
    GraphResult, Color
};

#[test]
fn test_line_contract() {
    let mut gd = 0i32;
    let mut gm = 4i32; // VGA mode

    initgraph(&mut gd, &mut gm, "");
    assert_eq!(graphresult(), GraphResult::Ok);

    // Test successful line drawing - BGI line is void function
    line(10, 10, 50, 50);

    closegraph();
}

#[test]
fn test_line_uninitialized() {
    // Test error with uninitialized context - BGI would set error state
    line(10, 10, 50, 50);
    // Note: In BGI, uninitialized operations typically don't crash but may not work
    // Error checking would be done via graphresult() if needed
}

#[test]
fn test_circle_contract() {
    let mut gd = 0i32;
    let mut gm = 4i32; // VGA mode

    initgraph(&mut gd, &mut gm, "");
    assert_eq!(graphresult(), GraphResult::Ok);

    // Test successful circle drawing - BGI circle is void function
    circle(100, 100, 25);

    closegraph();
}

#[test]
fn test_rectangle_contract() {
    let mut gd = 0i32;
    let mut gm = 4i32; // VGA mode

    initgraph(&mut gd, &mut gm, "");
    assert_eq!(graphresult(), GraphResult::Ok);

    // Test successful rectangle drawing - BGI rectangle is void function
    rectangle(20, 20, 80, 60);

    closegraph();
}

#[test]
fn test_arc_contract() {
    let mut gd = 0i32;
    let mut gm = 4i32; // VGA mode

    initgraph(&mut gd, &mut gm, "");
    assert_eq!(graphresult(), GraphResult::Ok);

    // Test successful arc drawing - BGI arc is void function
    arc(150, 150, 0, 90, 30);

    closegraph();
}

#[test]
fn test_putpixel_contract() {
    let mut gd = 0i32;
    let mut gm = 4i32; // VGA mode

    initgraph(&mut gd, &mut gm, "");
    assert_eq!(graphresult(), GraphResult::Ok);

    // Test successful pixel setting - BGI putpixel is void function
    putpixel(5, 5, Color::RED);

    closegraph();
}

#[test]
fn test_getpixel_contract() {
    let mut gd = 0i32;
    let mut gm = 4i32; // VGA mode

    initgraph(&mut gd, &mut gm, "");
    assert_eq!(graphresult(), GraphResult::Ok);

    // Test successful pixel getting - BGI getpixel returns Color directly
    let color = getpixel(5, 5);
    // In BGI, getpixel always returns a color value (could be background color)

    closegraph();
}

#[test]
fn test_getpixel_uninitialized() {
    // Test getpixel with uninitialized context - BGI may return default color
    let color = getpixel(5, 5);
    // Note: In BGI, uninitialized operations typically return default values
    // rather than throwing errors
}

#[test]
fn test_ellipse_contract() {
    let mut gd = 0i32;
    let mut gm = 4i32; // VGA mode

    initgraph(&mut gd, &mut gm, "");
    assert_eq!(graphresult(), GraphResult::Ok);

    // Test successful ellipse drawing - BGI ellipse is void function
    ellipse(200, 200, 0, 180, 40, 20);

    closegraph();
}

#[test]
fn test_drawing_primitives_sequence() {
    let mut gd = 0i32;
    let mut gm = 4i32; // VGA mode

    initgraph(&mut gd, &mut gm, "");
    assert_eq!(graphresult(), GraphResult::Ok);

    // Test drawing multiple primitives in sequence - all BGI functions are void
    line(0, 0, 10, 10);
    circle(50, 50, 15);
    rectangle(100, 100, 150, 140);
    arc(200, 200, 45, 135, 25);
    putpixel(250, 250, Color::BLUE);

    let pixel_color = getpixel(250, 250);
    // In BGI, getpixel returns the actual color at that pixel

    closegraph();
}
