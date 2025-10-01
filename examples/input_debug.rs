//! Test for debugging BGI input system.

use bgi::*;

fn main() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI (640x480, 16 colors)

    // Initialize graphics
    initgraph(&mut driver, &mut mode, "");

    // Check if initialization succeeded
    if graphresult() != GraphResult::Ok {
        eprintln!("Graphics initialization failed!");
        return;
    }

    println!("Input Debug Test");

    // Set background and clear
    setbkcolor(Color::BLUE);
    cleardevice();

    // Add instructions
    setcolor(Color::WHITE);
    outtextxy(10, 10, "Input Debug Test");
    outtextxy(10, 30, "Testing key input...");
    outtextxy(10, 50, "Focus window and press keys");

    println!("Window created. Testing input for 10 seconds...");

    // Test input for a limited time
    let start_time = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs(10);

    while start_time.elapsed() < timeout {
        // Test kbhit
        println!("Checking kbhit()...");
        let has_key = kbhit();
        println!("kbhit() returned: {}", has_key);

        if has_key {
            println!("Key detected! Getting character...");
            if let Some(ch) = getch() {
                println!("Got character: '{}'", ch);
                outtextxy(10, 100, &format!("Last key: {}", ch));
                // Exit after first key for easier testing
                break;
            } else {
                println!("getch() returned None");
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(100)); // Check more frequently
    }

    println!("Test completed.");
    
    // Close graphics
    closegraph();
    println!("Graphics closed.");
}
