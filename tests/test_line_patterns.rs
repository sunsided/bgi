use bgi::*;

#[test]
fn test_line_patterns_basic() {
    let mut driver = 9; // VGA
    let mut mode = 2; // VGAHI
    initgraph(&mut driver, &mut mode, "");

    // Test that setlinestyle doesn't crash
    setlinestyle(SOLID_LINE, 0, NORM_WIDTH);
    setlinestyle(DOTTED_LINE, 0, NORM_WIDTH);
    setlinestyle(DASHED_LINE, 0, NORM_WIDTH);

    // Draw lines with different patterns - they should not error
    line(10, 10, 100, 10);

    setlinestyle(DOTTED_LINE, 0, NORM_WIDTH);
    line(10, 20, 100, 20);

    setlinestyle(DASHED_LINE, 0, NORM_WIDTH);
    line(10, 30, 100, 30);

    // Test thick lines
    setlinestyle(SOLID_LINE, 0, THICK_WIDTH);
    line(10, 50, 100, 50);

    closegraph();
}

#[test]
fn test_rectangle_with_line_patterns() {
    let mut driver = 9; // VGA
    let mut mode = 2; // VGAHI
    initgraph(&mut driver, &mut mode, "");

    // Test that rectangles use line patterns
    setlinestyle(DOTTED_LINE, 0, NORM_WIDTH);
    rectangle(50, 50, 150, 100);

    setlinestyle(DASHED_LINE, 0, THICK_WIDTH);
    rectangle(200, 50, 300, 100);

    closegraph();
}

#[test]
fn test_write_mode_functions() {
    let mut driver = 9; // VGA
    let mut mode = 2; // VGAHI
    initgraph(&mut driver, &mut mode, "");

    // Test that setwritemode doesn't crash
    setwritemode(COPY_PUT);
    setwritemode(XOR_PUT);
    setwritemode(OR_PUT);

    // Draw something with different modes
    setwritemode(COPY_PUT);
    line(10, 10, 100, 10);

    closegraph();
}
