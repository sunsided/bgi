//! Mandelbrot set demonstration using BGI batch mode for maximum performance.

use bgi::*;
use std::time::Instant;

fn mandelbrot(c_re: f64, c_im: f64, max_iter: i32) -> i32 {
    let mut z_re = 0.0;
    let mut z_im = 0.0;
    let mut iter = 0;

    while z_re * z_re + z_im * z_im <= 4.0 && iter < max_iter {
        let temp = z_re * z_re - z_im * z_im + c_re;
        z_im = 2.0 * z_re * z_im + c_im;
        z_re = temp;
        iter += 1;
    }
    iter
}

fn iter_to_color(iter: i32, max_iter: i32) -> Color {
    if iter == max_iter {
        Color::BLACK
    } else {
        match iter % 15 {
            0 => Color::BLACK,
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
            _ => Color::WHITE,
        }
    }
}

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

    println!("🚀 Mandelbrot Set with Batch Mode Optimization");
    
    // Enable batch mode for maximum performance
    set_batch_mode(true);
    println!("✅ Batch mode enabled for bulk pixel operations");
    
    let width = getmaxx() + 1;
    let height = getmaxy() + 1;
    
    println!("Rendering {}x{} Mandelbrot set...", width, height);

    // Mandelbrot set parameters  
    let x_min = -2.5;
    let x_max = 1.0;
    let y_min = -1.25;
    let y_max = 1.25;
    let max_iter = 100; // High quality

    // Calculate scale factors
    let x_scale = (x_max - x_min) / width as f64;
    let y_scale = (y_max - y_min) / height as f64;

    let start_time = Instant::now();

    // Draw using pixel-by-pixel with batch mode for performance
    for py in 0..height {
        for px in 0..width {
            let x = x_min + px as f64 * x_scale;
            let y = y_min + py as f64 * y_scale;

            let iter = mandelbrot(x, y, max_iter);
            let color = iter_to_color(iter, max_iter);

            putpixel(px, py, color);
        }

        // Update every 20 rows for visual progress
        if py % 20 == 0 {
            refresh(); // Present accumulated changes
            let elapsed = start_time.elapsed();
            let progress = (py * 100) / height;
            println!("Progress: {}% - {:.2}s elapsed", progress, elapsed.as_secs_f64());

            // Check for early exit
            if kbhit() {
                println!("Rendering interrupted by user.");
                break;
            }
        }
    }

    // Final refresh to show complete image
    refresh();
    
    let render_time = start_time.elapsed();
    
    // Disable batch mode
    set_batch_mode(false);
    
    println!("🎯 Performance Results:");
    println!("   Rendering time: {:.3}s", render_time.as_secs_f64());
    println!("   Pixels drawn: {}", width * height);
    println!("   Pixels/second: {:.0}", (width * height) as f64 / render_time.as_secs_f64());

    // Add title
    setcolor(Color::WHITE);
    outtextxy(10, 10, "Mandelbrot Set (Batch Mode)");
    
    if is_headless() {
        println!("Mandelbrot set rendered in headless mode. Exiting...");
    } else {
        outtextxy(10, 30, "Press any key to exit...");
        println!("Mandelbrot set rendered. Press any key in the graphics window to exit...");
        
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
