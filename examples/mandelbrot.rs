//! Mandelbrot set demonstration using BGI graphics.

use bgi::*;

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

    println!("Mandelbrot Set Demonstration");

    let width = getmaxx() + 1;
    let height = getmaxy() + 1;

    println!("Rendering {}x{} Mandelbrot set...", width, height);

    // Mandelbrot set parameters
    let x_min = -2.5;
    let x_max = 1.0;
    let y_min = -1.25;
    let y_max = 1.25;
    let max_iter = 100;

    // Calculate scale factors
    let x_scale = (x_max - x_min) / width as f64;
    let y_scale = (y_max - y_min) / height as f64;

    // Generate the Mandelbrot set
    for py in 0..height {
        for px in 0..width {
            let x = x_min + px as f64 * x_scale;
            let y = y_min + py as f64 * y_scale;

            let iter = mandelbrot(x, y, max_iter);
            let color = iter_to_color(iter, max_iter);

            setcolor(color);
            putpixel(px, py, color);
        }

        // Print progress every 50 lines
        if py % 50 == 0 {
            println!("Progress: {}%", (py * 100) / height);
        }
    }

    // Add title
    setcolor(Color::WHITE);
    outtextxy(10, 10, "Mandelbrot Set");
    
    println!("Mandelbrot set rendered. Press any key to exit...");
    
    // In a real application, you might wait for user input here
    // For this example, we'll just close immediately

    // Close graphics
    closegraph();
    println!("Graphics closed.");
}
