//! Minimal window test - just opens a window with a simple message

use bgi::*;
use std::thread;
use std::time::Duration;

fn main() {
    println!("🔧 Testing BGI window creation...");

    let mut driver = 9;
    let mut mode = 2;

    initgraph(&mut driver, &mut mode, "");

    if graphresult() != GraphResult::Ok {
        eprintln!("❌ FAILED: Cannot initialize graphics");
        eprintln!("This might mean:");
        eprintln!("  - No display server running (X11/Wayland)");
        eprintln!("  - Running in headless environment");
        eprintln!("  - Graphics drivers not available");
        return;
    }

    println!("✅ SUCCESS: Graphics initialized!");
    println!("📏 Screen size: {}x{}", getmaxx() + 1, getmaxy() + 1);

    // Fill screen with bright color so it's unmistakable
    setbkcolor(Color::BLUE);
    cleardevice();

    setcolor(Color::YELLOW);
    outtextxy(50, 50, "*** BGI TEST WINDOW ***");
    setcolor(Color::WHITE);
    outtextxy(50, 80, "If you can see this, BGI is working!");
    setcolor(Color::LIGHTGREEN);
    outtextxy(50, 110, "Window will close in 10 seconds...");

    println!("🎯 **BRIGHT BLUE WINDOW SHOULD BE VISIBLE NOW**");
    println!("📍 Look for a 640x480 blue window with yellow text");

    for i in (1..=10).rev() {
        println!("⏰ Closing in {} seconds...", i);
        thread::sleep(Duration::from_secs(1));
    }

    closegraph();
    println!("✅ Test complete!");
}
