//! Test the visual backend through the global BGI API to see if a window appears.

use bgi::*;

fn main() {
    println!("Testing visual backend via the global BGI API...");

    let mut gd = 0i32; // DETECT
    let mut gm = 4i32; // VGA 640x480
    initgraph(&mut gd, &mut gm, "");

    if graphresult() != GraphResult::Ok {
        eprintln!("initgraph failed: {:?}", graphresult());
        return;
    }
    println!("Graphics initialized (mode {})", getgraphmode());

    // Draw something using the global drawing primitives.
    setcolor(Color::WHITE);
    line(10, 10, 100, 100);
    circle(200, 200, 50);
    rectangle(300, 150, 400, 250);

    // Present accumulated drawing to the window.
    refresh();
    println!("Content presented to window! Window should be visible.");

    // Keep the window open for a few seconds.
    println!("Waiting 5 seconds to see the window...");
    std::thread::sleep(std::time::Duration::from_secs(5));

    closegraph();
    println!("Test completed.");
}
