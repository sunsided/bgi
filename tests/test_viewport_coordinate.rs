use bgi::{
    GraphResult, circle, closegraph, getmaxx, getmaxy, getviewport, getx, gety, graphresult,
    initgraph, line, moverel, moveto, setviewport,
};

#[test]
fn test_setviewport_getviewport_cycle() {
    // Contract: setviewport should set drawing area, getviewport should return it
    let mut gd = 0i32;
    let mut gm = 4i32; // VGA mode

    initgraph(&mut gd, &mut gm, "");
    assert_eq!(graphresult(), GraphResult::Ok);

    // Test default viewport
    let (left, top, right, bottom) = getviewport();
    assert!(right > left);
    assert!(bottom > top);

    // Set new viewport - BGI setviewport is void function
    setviewport(10, 20, 300, 200, true);
    let (new_left, new_top, new_right, new_bottom) = getviewport();
    assert_eq!(new_left, 10);
    assert_eq!(new_top, 20);
    assert_eq!(new_right, 300);
    assert_eq!(new_bottom, 200);

    // Test another viewport
    setviewport(50, 50, 400, 350, true);
    let (final_left, final_top, final_right, final_bottom) = getviewport();
    assert_eq!(final_left, 50);
    assert_eq!(final_top, 50);
    assert_eq!(final_right, 400);
    assert_eq!(final_bottom, 350);

    closegraph();
}

#[test]
fn test_getmaxx_getmaxy_consistency() {
    // Contract: getmaxx/getmaxy should return current viewport boundaries
    let mut gd = 0i32;
    let mut gm = 4i32; // VGA mode

    initgraph(&mut gd, &mut gm, "");
    assert_eq!(graphresult(), GraphResult::Ok);

    // Test default dimensions
    let max_x = getmaxx();
    let max_y = getmaxy();
    assert!(max_x > 0);
    assert!(max_y > 0);

    // Change viewport and verify max coordinates change
    setviewport(0, 0, 800, 600, true);
    let new_max_x = getmaxx();
    let new_max_y = getmaxy();
    assert_eq!(new_max_x, 800);
    assert_eq!(new_max_y, 600);

    // Verify consistency with getviewport
    let (_left, _top, right, bottom) = getviewport();
    assert_eq!(getmaxx(), right);
    assert_eq!(getmaxy(), bottom);

    // Test smaller viewport
    setviewport(100, 100, 200, 300, true);
    assert_eq!(getmaxx(), 200);
    assert_eq!(getmaxy(), 300);

    closegraph();
}

#[test]
fn test_moveto_cursor_positioning() {
    // Contract: moveto should set current cursor position
    let mut gd = 0i32;
    let mut gm = 4i32; // VGA mode

    initgraph(&mut gd, &mut gm, "");
    assert_eq!(graphresult(), GraphResult::Ok);

    // Test initial cursor position
    let init_x = getx();
    let init_y = gety();
    assert!(init_x >= 0);
    assert!(init_y >= 0);

    // Move to specific position
    moveto(100, 150);
    assert_eq!(getx(), 100);
    assert_eq!(gety(), 150);

    // Move to different position
    moveto(250, 75);
    assert_eq!(getx(), 250);
    assert_eq!(gety(), 75);

    // Test boundary positions
    moveto(0, 0);
    assert_eq!(getx(), 0);
    assert_eq!(gety(), 0);

    moveto(getmaxx(), getmaxy());
    assert_eq!(getx(), getmaxx());
    assert_eq!(gety(), getmaxy());

    closegraph();
}

#[test]
fn test_moverel_relative_movement() {
    // Contract: moverel should move cursor relative to current position
    let mut gd = 0i32;
    let mut gm = 4i32; // VGA mode

    initgraph(&mut gd, &mut gm, "");
    assert_eq!(graphresult(), GraphResult::Ok);

    // Start at known position
    moveto(100, 100);
    assert_eq!(getx(), 100);
    assert_eq!(gety(), 100);

    // Move relatively
    moverel(50, 25);
    assert_eq!(getx(), 150);
    assert_eq!(gety(), 125);

    // Move in different direction
    moverel(-30, 40);
    assert_eq!(getx(), 120);
    assert_eq!(gety(), 165);

    // Move back to origin relatively
    moverel(-120, -165);
    assert_eq!(getx(), 0);
    assert_eq!(gety(), 0);

    // Test large relative moves
    moverel(1000, 500);
    assert_eq!(getx(), 1000);
    assert_eq!(gety(), 500);

    closegraph();
}

#[test]
fn test_viewport_drawing_boundaries() {
    // Contract: viewport should affect drawing operations (clipping)
    let mut gd = 0i32;
    let mut gm = 4i32; // VGA mode

    initgraph(&mut gd, &mut gm, "");
    assert_eq!(graphresult(), GraphResult::Ok);

    // Set a specific viewport
    setviewport(50, 50, 200, 150, true);

    // Drawing operations should work within viewport
    moveto(60, 60);
    line(60, 60, 190, 140); // Within viewport
    circle(125, 100, 25); // Within viewport

    // Operations outside viewport should still work (clipped by implementation)
    line(0, 0, 300, 300); // Crosses viewport boundaries
    circle(300, 300, 50); // Outside viewport

    // Verify viewport hasn't changed
    let (left, top, right, bottom) = getviewport();
    assert_eq!(left, 50);
    assert_eq!(top, 50);
    assert_eq!(right, 200);
    assert_eq!(bottom, 150);

    closegraph();
}

#[test]
fn test_viewport_edge_cases() {
    // Contract: viewport should handle edge cases gracefully
    let mut gd = 0i32;
    let mut gm = 4i32; // VGA mode

    initgraph(&mut gd, &mut gm, "");
    assert_eq!(graphresult(), GraphResult::Ok);

    // Test zero-size viewport
    setviewport(100, 100, 100, 100, true);
    let (left, top, right, bottom) = getviewport();
    assert_eq!(left, 100);
    assert_eq!(top, 100);
    assert_eq!(right, 100);
    assert_eq!(bottom, 100);

    // Test inverted viewport (right < left, bottom < top)
    setviewport(200, 200, 50, 50, true);
    let (inv_left, inv_top, inv_right, inv_bottom) = getviewport();
    assert_eq!(inv_left, 200);
    assert_eq!(inv_top, 200);
    assert_eq!(inv_right, 50);
    assert_eq!(inv_bottom, 50);

    // Test negative coordinates
    setviewport(-50, -25, 100, 200, true);
    let (neg_left, neg_top, neg_right, neg_bottom) = getviewport();
    assert_eq!(neg_left, -50);
    assert_eq!(neg_top, -25);
    assert_eq!(neg_right, 100);
    assert_eq!(neg_bottom, 200);

    // Test very large coordinates
    setviewport(0, 0, 10000, 8000, true);
    assert_eq!(getmaxx(), 10000);
    assert_eq!(getmaxy(), 8000);

    closegraph();
}

#[test]
fn test_cursor_boundary_movement() {
    // Contract: cursor movement should handle boundary conditions
    let mut gd = 0i32;
    let mut gm = 4i32; // VGA mode

    initgraph(&mut gd, &mut gm, "");
    assert_eq!(graphresult(), GraphResult::Ok);

    // Move to extreme coordinates
    moveto(i32::MAX, i32::MAX);
    assert_eq!(getx(), i32::MAX);
    assert_eq!(gety(), i32::MAX);

    moveto(i32::MIN, i32::MIN);
    assert_eq!(getx(), i32::MIN);
    assert_eq!(gety(), i32::MIN);

    // Test relative movement from extremes
    moveto(i32::MAX - 10, i32::MAX - 10);
    moverel(5, 5);
    assert_eq!(getx(), i32::MAX - 5);
    assert_eq!(gety(), i32::MAX - 5);

    // Test overflow behavior with relative movement
    moveto(i32::MAX, i32::MAX);
    moverel(1, 1); // Should handle overflow gracefully
    // Values might wrap or clamp depending on implementation

    // Reset to safe values
    moveto(0, 0);
    assert_eq!(getx(), 0);
    assert_eq!(gety(), 0);

    closegraph();
}

#[test]
fn test_coordinate_functions_after_init() {
    // Contract (Constitution: Proper Initialization Contract): coordinate
    // functions operate on the unified GRAPHICS_STATE, which must be set up
    // via initgraph() first. There is no parallel pre-init fallback state.
    let mut gd = 0i32;
    let mut gm = 4i32; // VGA mode

    initgraph(&mut gd, &mut gm, "");
    assert_eq!(graphresult(), GraphResult::Ok);

    setviewport(10, 10, 300, 200, true);
    let (left, top, right, bottom) = getviewport();
    assert_eq!(left, 10);
    assert_eq!(top, 10);
    assert_eq!(right, 300);
    assert_eq!(bottom, 200);

    // Max coordinates should work
    let max_x = getmaxx();
    let max_y = getmaxy();
    assert!(max_x >= 0);
    assert!(max_y >= 0);

    // Cursor operations should work
    moveto(50, 75);
    assert_eq!(getx(), 50);
    assert_eq!(gety(), 75);

    moverel(25, -25);
    assert_eq!(getx(), 75);
    assert_eq!(gety(), 50);

    closegraph();
}

#[test]
fn test_viewport_coordinate_consistency() {
    // Contract: viewport and coordinate functions should be consistent
    let mut gd = 0i32;
    let mut gm = 4i32; // VGA mode

    initgraph(&mut gd, &mut gm, "");
    assert_eq!(graphresult(), GraphResult::Ok);

    // Set multiple viewports and verify consistency
    let test_viewports = [
        (0, 0, 640, 480),
        (100, 50, 500, 400),
        (200, 200, 800, 600),
        (0, 100, 1024, 768),
    ];

    for (left, top, right, bottom) in test_viewports.iter() {
        setviewport(*left, *top, *right, *bottom, true);

        // Verify getviewport returns what we set
        let retrieved = getviewport();
        assert_eq!(retrieved.0, *left);
        assert_eq!(retrieved.1, *top);
        assert_eq!(retrieved.2, *right);
        assert_eq!(retrieved.3, *bottom);

        // Verify max coordinates match viewport
        assert_eq!(getmaxx(), *right);
        assert_eq!(getmaxy(), *bottom);

        // Test cursor movement within viewport
        moveto((*left + *right) / 2, (*top + *bottom) / 2);
        let center_x = getx();
        let center_y = gety();
        assert_eq!(center_x, (*left + *right) / 2);
        assert_eq!(center_y, (*top + *bottom) / 2);
    }

    closegraph();
}

#[test]
fn test_cursor_drawing_interaction() {
    // Contract: cursor position should interact with drawing operations
    let mut gd = 0i32;
    let mut gm = 4i32; // VGA mode

    initgraph(&mut gd, &mut gm, "");
    assert_eq!(graphresult(), GraphResult::Ok);

    // Move cursor and perform drawing operations
    moveto(100, 100);

    // Some BGI functions use current position (lineto, etc.)
    // For now, just verify cursor position is maintained
    let start_x = getx();
    let start_y = gety();

    // Drawing operations should not affect cursor (in basic BGI)
    line(start_x, start_y, start_x + 50, start_y + 50);
    circle(start_x + 25, start_y + 25, 10);

    // Cursor position should remain unchanged
    assert_eq!(getx(), start_x);
    assert_eq!(gety(), start_y);

    // Relative movement should work correctly
    moverel(25, 25);
    assert_eq!(getx(), start_x + 25);
    assert_eq!(gety(), start_y + 25);

    closegraph();
}
