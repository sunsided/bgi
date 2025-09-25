# BGI Quickstart Guide

## Basic BGI Program Structure

Every BGI program follows this pattern:

```rust
use bgi::{initgraph, closegraph, GraphicsDriver, GraphicsMode, GraphicsContext};

fn main() -> Result<(), bgi::BgiError> {
    // Initialize graphics
    let mut context = initgraph(GraphicsDriver::Detect, GraphicsMode::Default, "")?;
    
    // Your drawing code here
    // ...
    
    // Cleanup graphics
    closegraph(&mut context);
    Ok(())
}
```

## Simple Drawing Example

```rust
use bgi::*;

fn main() -> Result<(), BgiError> {
    let mut ctx = initgraph(GraphicsDriver::Detect, GraphicsMode::Default, "")?;
    
    // Set drawing color to red
    setcolor(&mut ctx, Color::Red)?;
    
    // Draw a rectangle
    rectangle(&mut ctx, 100, 50, 300, 200)?;
    
    // Draw a circle
    circle(&mut ctx, 200, 125, 50)?;
    
    // Display text
    outtextxy(&mut ctx, 150, 250, "Hello BGI!")?;
    
    // Wait for keypress
    getch(&mut ctx)?;
    
    closegraph(&mut ctx);
    Ok(())
}
```

## Color and Fill Example

```rust
use bgi::*;

fn main() -> Result<(), BgiError> {
    let mut ctx = initgraph(GraphicsDriver::Detect, GraphicsMode::Default, "")?;
    
    // Set fill pattern and color
    setfillstyle(&mut ctx, FillPattern::Solid, Color::Blue)?;
    
    // Draw filled rectangle
    rectangle(&mut ctx, 50, 50, 200, 150)?;
    floodfill(&mut ctx, 100, 100, Color::White)?;
    
    // Set different colors
    setcolor(&mut ctx, Color::Yellow)?;
    setfillstyle(&mut ctx, FillPattern::Hatch, Color::Green)?;
    
    // Draw filled circle
    circle(&mut ctx, 300, 200, 75)?;
    floodfill(&mut ctx, 300, 200, Color::Yellow)?;
    
    getch(&mut ctx)?;
    closegraph(&mut ctx);
    Ok(())
}
```

## Interactive Graphics Example

```rust
use bgi::*;

fn main() -> Result<(), BgiError> {
    let mut ctx = initgraph(GraphicsDriver::Detect, GraphicsMode::Default, "")?;
    
    setcolor(&mut ctx, Color::White)?;
    outtextxy(&mut ctx, 10, 10, "Click to draw circles, press 'q' to quit")?;
    
    loop {
        // Check for mouse clicks
        if let Ok(Some((x, y))) = getmouseclick(&mut ctx, MouseButton::Left) {
            circle(&mut ctx, x, y, 20)?;
        }
        
        // Check for keyboard input
        if kbhit(&ctx) {
            let ch = getch(&mut ctx)?;
            if ch == 'q' || ch == 'Q' {
                break;
            }
        }
        
        delay(10); // Small delay to prevent high CPU usage
    }
    
    closegraph(&mut ctx);
    Ok(())
}
```

## Text and Font Example

```rust
use bgi::*;

fn main() -> Result<(), BgiError> {
    let mut ctx = initgraph(GraphicsDriver::Detect, GraphicsMode::Default, "")?;
    
    // Default font, horizontal, size 1
    settextstyle(&mut ctx, Font::Default, TextDirection::Horizontal, 1)?;
    outtextxy(&mut ctx, 50, 50, "Default font, size 1")?;
    
    // Larger text
    settextstyle(&mut ctx, Font::Default, TextDirection::Horizontal, 2)?;
    outtextxy(&mut ctx, 50, 100, "Larger text")?;
    
    // Vertical text
    settextstyle(&mut ctx, Font::Default, TextDirection::Vertical, 1)?;
    outtextxy(&mut ctx, 300, 50, "Vertical text")?;
    
    // Measure text dimensions
    settextstyle(&mut ctx, Font::Default, TextDirection::Horizontal, 1)?;
    let text = "Measured text";
    let width = textwidth(&ctx, text);
    let height = textheight(&ctx, text);
    
    setcolor(&mut ctx, Color::Yellow)?;
    rectangle(&mut ctx, 50, 200, 50 + width as i32, 200 + height as i32)?;
    setcolor(&mut ctx, Color::White)?;
    outtextxy(&mut ctx, 50, 200, text)?;
    
    getch(&mut ctx)?;
    closegraph(&mut ctx);
    Ok(())
}
```

## Viewport and Clipping Example

```rust
use bgi::*;

fn main() -> Result<(), BgiError> {
    let mut ctx = initgraph(GraphicsDriver::Detect, GraphicsMode::Default, "")?;
    
    // Set viewport with clipping
    setviewport(&mut ctx, 100, 100, 400, 300, true)?;
    
    // Clear viewport area
    clearviewport(&mut ctx)?;
    
    // Draw within viewport (coordinates relative to viewport)
    setcolor(&mut ctx, Color::Red)?;
    rectangle(&mut ctx, 10, 10, 290, 190)?;
    
    // This circle will be clipped at viewport edges
    circle(&mut ctx, 150, 100, 150)?;
    
    // Reset to full screen
    setviewport(&mut ctx, 0, 0, getmaxx(&ctx), getmaxy(&ctx), false)?;
    
    // Draw viewport border
    setcolor(&mut ctx, Color::White)?;
    rectangle(&mut ctx, 100, 100, 400, 300)?;
    
    getch(&mut ctx)?;
    closegraph(&mut ctx);
    Ok(())
}
```

## Error Handling Pattern

```rust
use bgi::*;

fn draw_graphics() -> Result<(), BgiError> {
    let mut ctx = initgraph(GraphicsDriver::Detect, GraphicsMode::Default, "")?;
    
    // All BGI functions return Results - handle them appropriately
    setcolor(&mut ctx, Color::Blue).map_err(|e| {
        closegraph(&mut ctx);
        e
    })?;
    
    rectangle(&mut ctx, 50, 50, 200, 150).map_err(|e| {
        closegraph(&mut ctx);
        e
    })?;
    
    getch(&mut ctx).map_err(|e| {
        closegraph(&mut ctx);
        e
    })?;
    
    closegraph(&mut ctx);
    Ok(())
}

fn main() {
    if let Err(e) = draw_graphics() {
        eprintln!("Graphics error: {}", e);
    }
}
```
