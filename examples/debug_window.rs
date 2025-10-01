//! Debug version to understand window behavior

use bgi::*;
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== BGI Window Debug Test ===");

    let mut driver = 9;
    let mut mode = 2;

    println!("1. Attempting to initialize graphics...");
    initgraph(&mut driver, &mut mode, "");

    let result = graphresult();
    println!("2. Graphics result: {:?}", result);

    if result != GraphResult::Ok {
        eprintln!("❌ Graphics initialization failed!");
        println!("   This usually means:");
        println!("   - No display server (X11/Wayland) available");
        println!("   - Running in headless environment (no GUI)");
        println!("   - Display not accessible from current session");
        return;
    }

    println!("3. ✅ Graphics initialized successfully!");

    let width = getmaxx() + 1;
    let height = getmaxy() + 1;
    println!("4. Screen dimensions: {}x{}", width, height);

    // Test basic drawing immediately after initialization
    println!("5. Setting background to bright red...");
    setbkcolor(Color::RED);
    cleardevice();

    println!("6. Drawing test pattern...");
    setcolor(Color::YELLOW);
    
    // Draw a very obvious test pattern
    for i in 0..10 {
        let y = 50 + i * 20;
        line(50, y, 550, y);
        outtextxy(560, y - 5, &format!("Line {}", i + 1));
    }
    
    // Draw a big obvious circle
    setcolor(Color::WHITE);
    circle(320, 240, 100);
    outtextxy(250, 200, "BIG TEST CIRCLE");

    // Draw corner markers
    setcolor(Color::CYAN);
    putpixel(0, 0, Color::CYAN);
    putpixel(width-1, 0, Color::CYAN);
    putpixel(0, height-1, Color::CYAN);
    putpixel(width-1, height-1, Color::CYAN);

    println!("7. 🔴 RED BACKGROUND + YELLOW LINES + WHITE CIRCLE should be visible");
    println!("   Window should be VERY obvious with bright red background");
    println!("");

    // Extended wait with detailed countdown
    for i in (1..=15).rev() {
        println!("⏰ Window open for {} more seconds... LOOK FOR BRIGHT RED WINDOW!", i);
        thread::sleep(Duration::from_secs(1));
    }

    println!("8. Closing graphics...");
    closegraph();
    println!("9. ✅ Complete!");

    println!("");
    println!("SUMMARY:");
    println!("- Graphics initialization: SUCCESS");
    println!("- Window should have been: 640x480 with bright red background");
    println!("- If you didn't see it, you may be in a headless environment");
    println!("- Try running this on a desktop with GUI, not SSH/headless server");
}
