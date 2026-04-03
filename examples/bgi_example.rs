// Example of proper BGI usage with error handling
use bgi::*;

fn main() {
    let mut gd = 0i32; // graphics driver
    let mut gm = 4i32; // graphics mode (VGA)

    // Initialize graphics
    initgraph(&mut gd, &mut gm, "");

    // Check if initialization succeeded
    if graphresult() != GraphResult::Ok {
        eprintln!(
            "Graphics initialization failed: {}",
            grapherrormsg(graphresult() as i32)
        );
        return;
    }

    println!("Graphics initialized successfully!");
    println!("Driver: {}, Mode: {}", gd, gm);

    // Draw some basic shapes
    line(100, 100, 200, 200);
    circle(150, 150, 50);
    rectangle(50, 50, 250, 250);

    // Check for errors
    if graphresult() != GraphResult::Ok {
        eprintln!("Drawing error: {}", grapherrormsg(graphresult() as i32));
    }

    // Close graphics
    closegraph();

    println!("Graphics closed.");
}
