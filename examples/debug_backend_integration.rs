//! Debug backend integration

use bgi::*;

fn main() {
    println!("Testing backend integration...");

    let mut driver = 9; // VGA
    let mut mode = 2; // VGAHI (640x480, 16 colors)

    // Initialize graphics
    println!("Calling initgraph...");
    initgraph(&mut driver, &mut mode, "");

    // Check if initialization succeeded
    if graphresult() != GraphResult::Ok {
        eprintln!("Graphics initialization failed!");
        return;
    }

    println!("Graphics initialized successfully!");
    println!("Driver: {}, Mode: {}", driver, mode);
    println!("Screen resolution: {}x{}", getmaxx() + 1, getmaxy() + 1);

    // Draw something
    println!("Drawing line...");
    setcolor(Color::WHITE);
    line(10, 10, 100, 100);

    println!("Drawing circle...");
    setcolor(Color::RED);
    circle(200, 200, 50);

    println!("Drawing rectangle...");
    setcolor(Color::GREEN);
    rectangle(300, 150, 400, 250);

    println!("Drawing completed. Keeping window open for 10 seconds...");
    println!("Look for a graphics window on your desktop!");

    // Keep the window alive by calling a BGI function periodically
    for i in 0..10 {
        println!("  {} seconds remaining...", 10 - i);

        // Draw a small indicator to keep the window active
        setcolor(Color::YELLOW);
        putpixel(10 + i * 5, 10, Color::YELLOW);

        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    // Close graphics
    println!("Closing graphics...");
    closegraph();
    println!("Done.");
}
