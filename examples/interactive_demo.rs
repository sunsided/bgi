//! Interactive BGI demo showing visual output with minifb backend
//! Press ESC to exit, other keys to draw different shapes

use bgi::*;
use std::thread;
use std::time::Duration;

fn main() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI (640x480, 16 colors)

    // Initialize graphics with visual backend
    initgraph(&mut driver, &mut mode, "");

    if graphresult() != GraphResult::Ok {
        eprintln!("Graphics initialization failed!");
        return;
    }

    println!("Interactive BGI Demo with minifb visual backend");
    println!("Screen resolution: {}x{}", getmaxx() + 1, getmaxy() + 1);
    println!("Watch the graphics window for visual output!");
    println!("This demo will automatically draw shapes for 10 seconds...");

    // Clear screen and set initial color
    cleardevice();
    setcolor(Color::WHITE);

    // Draw initial frame
    outtextxy(10, 10, "BGI with minifb Visual Backend");
    setcolor(Color::YELLOW);
    outtextxy(10, 30, "Animated graphics demo - 10 seconds");

    // Create an animated demo
    for frame in 0..100 {
        // Clear previous frame (keeping text)
        setcolor(Color::BLACK);
        bar(50, 100, 590, 450);

        // Calculate animation values
        let t = frame as f32 * 0.1;
        let center_x = 320;
        let center_y = 275;

        // Draw animated circles
        for i in 0..5 {
            let radius = 20 + i * 15;
            let x = center_x + (t * (i + 1) as f32).cos() as i32 * (30 + i * 10);
            let y = center_y + (t * (i + 1) as f32).sin() as i32 * (20 + i * 5);

            match i {
                0 => setcolor(Color::RED),
                1 => setcolor(Color::GREEN),
                2 => setcolor(Color::BLUE),
                3 => setcolor(Color::CYAN),
                4 => setcolor(Color::MAGENTA),
                _ => setcolor(Color::WHITE),
            }

            circle(x, y, radius);
        }

        // Draw connecting lines
        setcolor(Color::WHITE);
        for i in 0..4 {
            let x1 = center_x + (t * (i + 1) as f32).cos() as i32 * (30 + i * 10);
            let y1 = center_y + (t * (i + 1) as f32).sin() as i32 * (20 + i * 5);
            let x2 = center_x + (t * (i + 2) as f32).cos() as i32 * (30 + (i + 1) * 10);
            let y2 = center_y + (t * (i + 2) as f32).sin() as i32 * (20 + (i + 1) * 5);
            line(x1, y1, x2, y2);
        }

        // Draw frame counter
        setcolor(Color::YELLOW);
        let frame_text = format!("Frame: {}/100", frame + 1);
        outtextxy(10, 50, &frame_text);

        // Small delay to make animation visible
        thread::sleep(Duration::from_millis(100));
    }

    // Final message
    setcolor(Color::GREEN);
    outtextxy(10, 70, "Animation complete! Visual backend working!");
    
    println!("Animation complete!");
    println!("Check the graphics window - you should see animated circles!");
    println!("The window will close automatically in 3 seconds...");
    
    // Keep window open for a bit longer
    thread::sleep(Duration::from_secs(3));

    closegraph();
    println!("Graphics closed. Visual backend demo complete!");
}
