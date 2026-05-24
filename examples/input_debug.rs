//! Test for debugging BGI input system with visual mouse feedback.
//!
//! Colors the window using RGB triangulation based on cursor position:
//! - Red intensity based on distance from top-left
//! - Green intensity based on distance from top-right
//! - Blue intensity based on distance from bottom-center

use bgi::*;

fn main() {
    let mut driver = 9; // VGA
    let mut mode = 2; // VGAHI (640x480, 16 colors)

    // Initialize graphics
    initgraph(&mut driver, &mut mode, "");

    // Check if initialization succeeded
    if graphresult() != GraphResult::Ok {
        eprintln!("Graphics initialization failed!");
        return;
    }

    // Enable batch mode to eliminate flicker - all draw calls will accumulate
    // in the buffer until refresh() is called at the end of each frame.
    set_batch_mode(true);

    let width = getmaxx() as f32;
    let height = getmaxy() as f32;

    // RGB triangle vertices
    let red_x = 0.0_f32;
    let red_y = 0.0_f32;
    let green_x = width;
    let green_y = 0.0_f32;
    let blue_x = width / 2.0;
    let blue_y = height;

    println!("Input Debug Test - RGB Triangle Mode");
    println!("Move mouse to change background color");
    println!("Press 'Q' or ESC to quit");
    println!("Window size: {}x{}", width as i32, height as i32);

    let mut last_mx = -1;
    let mut last_my = -1;
    let mut frame_count = 0u32;

    loop {
        // Get mouse position
        let mx = mousex();
        let my = mousey();

        // Only redraw if mouse moved (optimization)
        if mx != last_mx || my != last_my || frame_count == 0 {
            last_mx = mx;
            last_my = my;

            // Calculate distances to RGB vertices (normalized 0-1)
            let fx = mx as f32;
            let fy = my as f32;

            // Distance to red corner (top-left)
            let dist_red = ((fx - red_x).powi(2) + (fy - red_y).powi(2)).sqrt();
            let max_dist = (width.powi(2) + height.powi(2)).sqrt();
            let red = (255.0 * (1.0 - (dist_red / max_dist).min(1.0))) as u8;

            // Distance to green corner (top-right)
            let dist_green = ((fx - green_x).powi(2) + (fy - green_y).powi(2)).sqrt();
            let green = (255.0 * (1.0 - (dist_green / max_dist).min(1.0))) as u8;

            // Distance to blue point (bottom-center)
            let dist_blue = ((fx - blue_x).powi(2) + (fy - blue_y).powi(2)).sqrt();
            let blue = (255.0 * (1.0 - (dist_blue / max_dist).min(1.0))) as u8;

            // Set background color using RGB
            let bg_color = Color::Rgb(RgbColor::rgb(red, green, blue));
            setbkcolor(bg_color);
            cleardevice();

            // Draw RGB triangle vertices as circles
            setcolor(Color::RED);
            circle(red_x as i32, red_y as i32, 20);
            setcolor(Color::GREEN);
            circle(green_x as i32, green_y as i32, 20);
            setcolor(Color::BLUE);
            circle(blue_x as i32, blue_y as i32, 20);

            // Draw crosshair at mouse position
            setcolor(Color::WHITE);
            line(mx - 10, my, mx + 10, my);
            line(mx, my - 10, mx, my + 10);

            // Calculate text color for contrast
            let luminance = (0.299 * red as f32 + 0.587 * green as f32 + 0.114 * blue as f32) as u8;
            let text_color = if luminance > 128 {
                Color::BLACK
            } else {
                Color::WHITE
            };
            setcolor(text_color);

            // Display info
            outtextxy(10, 10, "Input Debug - RGB Triangle");
            outtextxy(10, 30, &format!("Mouse: ({}, {})", mx, my));
            outtextxy(10, 50, &format!("RGB: ({}, {}, {})", red, green, blue));
            outtextxy(10, 70, "Press Q or ESC to quit");

            // Show mouse button state (1=left, 2=right, 4=middle)
            let left = if ismouseclick(1) { "L" } else { "-" };
            let middle = if ismouseclick(4) { "M" } else { "-" };
            let right = if ismouseclick(2) { "R" } else { "-" };
            outtextxy(10, 90, &format!("Buttons: [{}{}{}]", left, middle, right));

            // Present all accumulated draw commands to the screen at once
            refresh();
        }

        // Check for keyboard input
        if kbhit()
            && let Some(ch) = getch()
        {
            println!("Key pressed: '{}'", ch);
            if ch == 'q' || ch == 'Q' || ch as u8 == 27 {
                // ESC = 27
                println!("Exiting...");
                break;
            }
            // Display last key pressed
            setcolor(Color::YELLOW);
            outtextxy(10, 110, &format!("Last key: '{}'", ch));
            refresh();
        }

        frame_count = frame_count.wrapping_add(1);
        delay(16); // ~60 FPS
    }

    closegraph();
    println!("Graphics closed.");
}
