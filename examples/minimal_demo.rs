//! Minimal BGI demo - draws 1000 random colored lines
//! Rust port of SDL3_bgi minimal.c demo

use bgi::*;
use rand::Rng;
use std::thread;
use std::time::Duration;

fn main() {
    let mut driver = 9; // DETECT equivalent
    let mut mode = 2;   // VGA mode

    // Initialize graphics
    initgraph(&mut driver, &mut mode, "");

    if graphresult() != GraphResult::Ok {
        eprintln!("Graphics initialization failed!");
        return;
    }

    println!("BGI Minimal Demo - Drawing 1000 random lines");
    println!("Visual output should appear in graphics window");

    // Set background and clear
    setbkcolor(Color::BLACK);
    cleardevice();

    // Draw text
    setcolor(Color::WHITE);
    outtextxy(0, 0, "Drawing 1000 lines...");

    let mut rng = rand::thread_rng();
    let max_x = getmaxx();
    let max_y = getmaxy();

    println!("Screen size: {}x{}", max_x + 1, max_y + 1);
    println!("Drawing random lines...");

    // Draw 1000 random lines with random colors
    for i in 0..1000 {
        // Random color (1-15, avoiding black)
        let color_num = 1 + (rng.gen::<u8>() % 15);
        let color = match color_num {
            1 => Color::BLUE,
            2 => Color::GREEN,
            3 => Color::CYAN,
            4 => Color::RED,
            5 => Color::MAGENTA,
            6 => Color::BROWN,
            7 => Color::LIGHTGRAY,
            8 => Color::DARKGRAY,
            9 => Color::LIGHTBLUE,
            10 => Color::LIGHTGREEN,
            11 => Color::LIGHTCYAN,
            12 => Color::LIGHTRED,
            13 => Color::LIGHTMAGENTA,
            14 => Color::YELLOW,
            15 => Color::WHITE,
            _ => Color::WHITE,
        };
        setcolor(color);

        // Random line coordinates
        let x1 = rng.gen_range(0..=max_x);
        let y1 = rng.gen_range(0..=max_y);
        let x2 = rng.gen_range(0..=max_x);
        let y2 = rng.gen_range(0..=max_y);

        line(x1, y1, x2, y2);

        // Progress indicator
        if i % 100 == 0 {
            println!("Drew {} lines...", i + 1);
        }

        // Small delay to see the animation
        if i % 10 == 0 {
            thread::sleep(Duration::from_millis(1));
        }
    }

    println!("All 1000 lines drawn!");
    println!("Window will stay open for 5 seconds to view the result...");
    
    // Keep window open for viewing
    thread::sleep(Duration::from_secs(5));
    
    closegraph();
    println!("Demo complete!");
}
