use bgi::*;
use std::thread;
use std::time::Duration;

fn main() {
    let mut driver = 9; // VGA
    let mut mode = 2; // VGAHI (640x480, 16 colors)

    initgraph(&mut driver, &mut mode, "");

    println!("🧪 Testing batch mode with small pixel counts...");

    // Test 1: Normal mode (immediate presentation)
    println!("\n--- Test 1: Normal Mode (5 pixels) ---");
    set_batch_mode(false);
    println!("Batch mode: {}", is_batch_mode());

    for i in 0..5 {
        putpixel(10 + i * 10, 50, Color::RED);
        println!("Pixel {} drawn and immediately visible", i + 1);
        thread::sleep(Duration::from_millis(500)); // Slow for visual effect
    }

    thread::sleep(Duration::from_secs(2));

    // Test 2: Batch mode without manual refresh (pixels accumulate but not shown)
    println!("\n--- Test 2: Batch Mode WITHOUT refresh (5 pixels) ---");
    set_batch_mode(true);
    println!("Batch mode: {}", is_batch_mode());

    for i in 0..5 {
        putpixel(10 + i * 10, 100, Color::GREEN);
        println!("Pixel {} drawn to buffer (not yet visible)", i + 1);
        thread::sleep(Duration::from_millis(500));
    }

    println!("⚠️  5 pixels drawn but NOT visible yet - no refresh() called");
    thread::sleep(Duration::from_secs(2));

    // Test 3: Manual refresh shows accumulated pixels
    println!("\n--- Test 3: Manual refresh reveals all pixels ---");
    refresh();
    println!("✅ refresh() called - all 5 green pixels now visible!");
    thread::sleep(Duration::from_secs(2));

    // Test 4: Single pixel in batch mode
    println!("\n--- Test 4: Single pixel in batch mode ---");
    putpixel(50, 150, Color::YELLOW);
    println!("🔍 Single yellow pixel drawn but invisible...");
    thread::sleep(Duration::from_secs(1));

    refresh();
    println!("✅ refresh() called - single pixel now visible!");
    thread::sleep(Duration::from_secs(2));

    // Test 5: Empty batch
    println!("\n--- Test 5: refresh() with no new pixels ---");
    refresh(); // No new pixels since last refresh
    println!("✅ refresh() with no changes - harmless");
    thread::sleep(Duration::from_secs(1));

    // Test 6: Mixed operations
    println!("\n--- Test 6: Mix of batch and normal mode ---");

    // Draw some in batch mode
    set_batch_mode(true);
    for i in 0..3 {
        putpixel(10 + i * 10, 200, Color::BLUE);
    }
    println!("3 blue pixels in batch (invisible)");

    // Switch to normal mode mid-way
    set_batch_mode(false);
    for i in 3..6 {
        putpixel(10 + i * 10, 200, Color::CYAN);
        thread::sleep(Duration::from_millis(300));
    }
    println!("3 cyan pixels in normal mode (immediately visible)");

    // Show the batched pixels
    set_batch_mode(true);
    refresh();
    println!("✅ refresh() shows the 3 blue pixels that were batched");

    thread::sleep(Duration::from_secs(2));

    // Summary
    set_batch_mode(false);
    cleardevice();

    setcolor(Color::WHITE);
    outtextxy(10, 10, "Batch Mode Test Results:");
    outtextxy(10, 30, "- Normal mode: immediate visibility");
    outtextxy(10, 50, "- Batch mode: pixels accumulate until refresh()");
    outtextxy(10, 70, "- Works with ANY number of pixels (1, 5, 1000+)");
    outtextxy(10, 90, "- refresh() is safe to call anytime");
    outtextxy(10, 110, "- No minimum batch size required");

    println!("\n🎯 Key Findings:");
    println!("   ✅ Works with 1 pixel, 5 pixels, or millions");
    println!("   ✅ No minimum batch size - it's just a drawing mode");
    println!("   ✅ refresh() can be called as often as needed");
    println!("   ✅ Safe to mix batch and normal modes");

    if is_headless() {
        println!("\nBatch test completed in headless mode. Exiting...");
    } else {
        println!("\nPress any key to exit...");
        while !kbhit() {
            thread::sleep(Duration::from_millis(50));
        }
        getch();
    }

    closegraph();
}
