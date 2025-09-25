//! BGI compatibility showcase demonstrating classic BGI API usage.

use bgi::*;

fn main() {
    println!("BGI Compatibility Showcase");
    println!("Demonstrating classic BGI API compatibility");

    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI (640x480, 16 colors)

    // Classic BGI initialization
    initgraph(&mut driver, &mut mode, "");

    // Check initialization result
    match graphresult() {
        GraphResult::Ok => {
            println!("Graphics initialized successfully");
            println!("Driver: {}, Mode: {}", driver, mode);
        }
        _ => {
            eprintln!("Graphics initialization failed: {}", grapherrormsg(graphresult() as i32));
            return;
        }
    }

    // Display screen information
    println!("Screen dimensions: {}x{}", getmaxx() + 1, getmaxy() + 1);

    // Test color system
    println!("Testing color system...");
    for i in 0..16 {
        if let Some(color) = Color::from_int(i) {
            setcolor(color);
            line(i * 40, 10, i * 40 + 30, 10);
            outtextxy(i * 40, 20, &format!("{}", i));
        }
    }

    // Test line styles
    println!("Testing line styles...");
    setcolor(Color::WHITE);

    setlinestyle(SOLID_LINE, 0, NORM_WIDTH);
    line(50, 60, 200, 60);
    outtextxy(210, 55, "SOLID_LINE");

    setlinestyle(DOTTED_LINE, 0, NORM_WIDTH);
    line(50, 80, 200, 80);
    outtextxy(210, 75, "DOTTED_LINE");

    setlinestyle(CENTER_LINE, 0, NORM_WIDTH);
    line(50, 100, 200, 100);
    outtextxy(210, 95, "CENTER_LINE");

    setlinestyle(DASHED_LINE, 0, NORM_WIDTH);
    line(50, 120, 200, 120);
    outtextxy(210, 115, "DASHED_LINE");

    // Test shapes
    println!("Testing shape drawing...");
    setcolor(Color::GREEN);
    circle(100, 200, 30);
    outtextxy(80, 240, "Circle");

    setcolor(Color::RED);
    rectangle(200, 170, 280, 230);
    outtextxy(210, 240, "Rectangle");

    setcolor(Color::BLUE);
    ellipse(400, 200, 0, 360, 40, 25);
    outtextxy(370, 240, "Ellipse");

    // Test filled shapes
    println!("Testing filled shapes...");
    setfillstyle(SOLID_FILL, Color::YELLOW);
    setcolor(Color::YELLOW);
    bar(50, 280, 120, 350);
    outtextxy(60, 360, "Filled Bar");

    setfillstyle(SOLID_FILL, Color::MAGENTA);
    setcolor(Color::MAGENTA);
    fillellipse(200, 315, 35, 25);
    outtextxy(170, 360, "Filled Ellipse");

    // Test text rendering
    println!("Testing text rendering...");
    setcolor(Color::WHITE);
    settextstyle(DEFAULT_FONT, HORIZ_DIR, 1);
    outtextxy(50, 400, "Default Font, Size 1");

    settextstyle(DEFAULT_FONT, HORIZ_DIR, 2);
    outtextxy(50, 420, "Default Font, Size 2");

    // Test viewport
    println!("Testing viewport...");
    setcolor(Color::LIGHTBLUE);
    setviewport(450, 50, 600, 200, true);
    rectangle(0, 0, getmaxx(), getmaxy());
    outtextxy(10, 10, "Viewport");
    outtextxy(10, 30, "Test");

    // Reset viewport
    setviewport(0, 0, getmaxx(), getmaxy(), false);

    // Test coordinate system
    setcolor(Color::LIGHTGREEN);
    moveto(300, 300);
    outtextxy(getx(), gety(), "Current Position");
    
    moverel(50, 50);
    outtextxy(getx(), gety(), "Moved Relative");

    // Test palette functions
    println!("Testing palette operations...");
    let palette_size = getpalettesize();
    outtextxy(50, 450, &format!("Palette size: {}", palette_size));

    // Display completion message
    setcolor(Color::WHITE);
    outtextxy(10, getmaxy() - 20, "BGI Compatibility Test Complete");

    println!("All BGI compatibility tests completed successfully!");
    println!("Press any key to exit...");

    // Close graphics
    closegraph();
    println!("Graphics closed.");
}
