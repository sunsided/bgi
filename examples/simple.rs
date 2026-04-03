//! Simple BGI example demonstrating basic drawing operations.

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

    println!("Simple BGI Example");
    println!("Screen resolution: {}x{}", getmaxx() + 1, getmaxy() + 1);

    // Set drawing color to white
    setcolor(Color::WHITE);

    // Draw some basic shapes
    line(10, 10, 100, 10);        // Horizontal line
    line(10, 10, 10, 100);        // Vertical line
    circle(200, 50, 30);          // Circle
    rectangle(300, 20, 400, 80);  // Rectangle

    // Draw with different colors
    setcolor(Color::RED);
    line(50, 150, 150, 250);      // Red diagonal line

    setcolor(Color::GREEN);
    circle(300, 200, 40);         // Green circle

    setcolor(Color::BLUE);
    rectangle(450, 150, 550, 250); // Blue rectangle

    // Add some text
    setcolor(Color::YELLOW);
    outtextxy(10, 300, "Simple BGI Graphics Demo");
    
    if is_headless() {
        println!("Drawing completed in headless mode. Exiting...");
    } else {
        outtextxy(10, 320, "Press any key to exit...");
        println!("Drawing completed. Press any key in the graphics window to exit...");
        
        // Wait for key press from graphics window
        loop {
            if kbhit() {
                let ch = getch();
                println!("Key pressed: {:?}", ch);
                break;
            }
            delay(10); // Small delay to avoid busy waiting
        }
    }
    
    // Close graphics
    closegraph();
    println!("Graphics closed.");
}
