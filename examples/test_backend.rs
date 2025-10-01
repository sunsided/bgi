//! Test the backend directly to see if windows appear

use bgi::{GraphicsContext, GraphicsMode, GraphicsDriver};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing direct backend usage...");
    
    // Create a graphics context that should use the visual backend
    let mode = GraphicsMode::new(GraphicsDriver::Vga, 2); // VGA 640x480
    let mut context = GraphicsContext::new(mode)?;

    println!("Graphics context created successfully!");
    println!("Mode: {:?}", context.mode);

    // Try to draw something using the context
    context.draw_line(10, 10, 100, 100)?;
    context.draw_circle(200, 200, 50)?;
    context.draw_rectangle(300, 150, 400, 250)?;

    // Present the content
    context.present()?;
    println!("Content presented to window! Window should be visible.");

    // Keep the window open for a few seconds
    println!("Waiting 5 seconds to see the window...");
    std::thread::sleep(std::time::Duration::from_secs(5));
    
    println!("Test completed.");
    Ok(())
}
