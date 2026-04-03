# Quickstart: Visual Backend with Input Support

**Purpose**: Validate visual backend and input functionality through interactive examples
**Prerequisites**: BGI library with visual backend feature enabled
**Estimated Time**: 10 minutes

## Overview

This quickstart demonstrates the new visual backend capabilities by running interactive BGI programs that display graphics in a window and respond to user input.

## Setup

### 1. Enable Visual Backend Feature

Add to your `Cargo.toml`:

```toml
[dependencies]
bgi = { version = "0.2", features = ["visual-backend"] }
```

### 2. Verify Installation

Run the basic visual test:

```bash
cargo run --example basic_visual --features visual-backend
```

Expected result: A window opens displaying simple graphics.

## Basic Visual Output

### Example 1: Simple Graphics Window

Create `examples/visual_hello.rs`:

```rust
use bgi::*;

fn main() {
    // Initialize graphics with visual backend
    let driver = DETECT;
    let mode = DETECT;
    initgraph(&driver, &mode, "");

    // Draw some basic shapes
    setcolor(WHITE);
    line(10, 10, 100, 100);
    circle(50, 50, 25);
    outtextxy(10, 150, "Hello Visual BGI!");

    // Wait for user input
    getch();

    // Clean up
    closegraph();
}
```

**Run**: `cargo run --example visual_hello --features visual-backend`

**Expected**:
- Window opens with white lines, circle, and text
- Window stays open until key press
- Window closes cleanly after input

## Interactive Input

### Example 2: Keyboard Input

Create `examples/keyboard_test.rs`:

```rust
use bgi::*;

fn main() {
    initgraph(&DETECT, &DETECT, "");

    setcolor(YELLOW);
    outtextxy(10, 10, "Press keys (ESC to exit):");

    let mut y_pos = 30;
    loop {
        if kbhit() {
            let key = getch();

            // Display the key pressed
            let msg = format!("Key pressed: {} (ASCII: {})", key as char, key);
            outtextxy(10, y_pos, &msg);
            y_pos += 15;

            // Exit on ESC
            if key == 27 { // ESC key
                break;
            }

            // Clear screen if getting full
            if y_pos > 400 {
                cleardevice();
                outtextxy(10, 10, "Press keys (ESC to exit):");
                y_pos = 30;
            }
        }
    }

    closegraph();
}
```

**Run**: `cargo run --example keyboard_test --features visual-backend`

**Expected**:
- Window shows typed characters and ASCII codes
- ESC key exits the program
- Screen clears when full

### Example 3: Mouse Input

Create `examples/mouse_test.rs`:

```rust
use bgi::*;

fn main() {
    initgraph(&DETECT, &DETECT, "");

    setcolor(CYAN);
    outtextxy(10, 10, "Move mouse and click (any key to exit)");

    let mut last_x = -1;
    let mut last_y = -1;

    loop {
        // Check for keyboard exit
        if kbhit() {
            break;
        }

        // Get current mouse position
        let x = mousex();
        let y = mousey();

        // Update position display if mouse moved
        if x != last_x || y != last_y {
            // Clear previous position
            setcolor(BLACK);
            let old_msg = format!("Mouse: ({}, {})", last_x, last_y);
            outtextxy(10, 30, &old_msg);

            // Show new position
            setcolor(WHITE);
            let msg = format!("Mouse: ({}, {})", x, y);
            outtextxy(10, 30, &msg);

            last_x = x;
            last_y = y;
        }

        // Small delay to prevent excessive updates
        delay(10);
    }

    closegraph();
}
```

**Run**: `cargo run --example mouse_test --features visual-backend`

**Expected**:
- Window tracks mouse position coordinates
- Position updates as mouse moves
- Any key press exits

## Backend Switching

### Example 4: PGM Output Mode

Create `examples/pgm_output.rs`:

```rust
use bgi::*;

fn main() {
    // Force PGM backend for headless testing
    std::env::set_var("BGI_BACKEND", "pgm");

    initgraph(&DETECT, &DETECT, "");

    // Draw test pattern
    setcolor(WHITE);
    for i in 0..10 {
        line(i * 10, 0, i * 10, 100);
        line(0, i * 10, 100, i * 10);
    }

    circle(50, 50, 30);
    outtextxy(10, 70, "PGM Test");

    closegraph();

    println!("Graphics saved to output.pgm");
}
```

**Run**: `cargo run --example pgm_output`

**Expected**:
- No window opens
- File `output.pgm` created with graphics
- Can view PGM file with image viewer

## Validation Tests

### Test 1: Window Resizing

1. Run `visual_hello` example
2. Resize the window using window controls
3. Verify graphics scale proportionally
4. Confirm coordinates remain in BGI logical space

### Test 2: Input Event Ordering

1. Run `keyboard_test` example
2. Type quickly: "hello123"
3. Verify all characters appear in correct order
4. Confirm no events are lost

### Test 3: Backend Fallback

1. Run any visual example in headless environment:
   ```bash
   DISPLAY= cargo run --example visual_hello --features visual-backend
   ```
2. Verify automatic fallback to PGM output
3. Check that PGM file is created

### Test 4: Multiple Graphics Modes

Create test for different BGI modes:

```rust
use bgi::*;

fn test_mode(driver: i16, mode: i16, name: &str) {
    println!("Testing mode: {}", name);
    initgraph(&driver, &mode, "");

    if graphresult() == grOk {
        // Draw test pattern
        setcolor(WHITE);
        line(0, 0, getmaxx(), getmaxy());
        outtextxy(10, 10, name);
        getch();
        closegraph();
        println!("✓ {} works", name);
    } else {
        println!("✗ {} failed", name);
    }
}

fn main() {
    test_mode(VGA, VGAHI, "VGA High Resolution");
    test_mode(EGA, EGAHI, "EGA High Resolution");
    test_mode(CGA, CGAHI, "CGA High Resolution");
}
```

## Performance Validation

### Frame Rate Test

```rust
use bgi::*;
use std::time::Instant;

fn main() {
    initgraph(&DETECT, &DETECT, "");

    let start = Instant::now();
    let mut frames = 0;

    while start.elapsed().as_secs() < 5 {
        cleardevice();

        // Draw moving pattern
        let t = frames as f32 * 0.1;
        let x = (50.0 + 30.0 * t.cos()) as i16;
        let y = (50.0 + 30.0 * t.sin()) as i16;

        setcolor(WHITE);
        circle(x, y, 10);

        frames += 1;
    }

    let fps = frames as f32 / 5.0;
    println!("Average FPS: {:.1}", fps);

    closegraph();

    // Validate performance
    assert!(fps >= 30.0, "Frame rate too low: {:.1} FPS", fps);
    println!("✓ Performance test passed");
}
```

## Troubleshooting

### Common Issues

**Window doesn't appear**:
- Check if visual backend feature is enabled
- Verify display environment (Linux: DISPLAY variable)
- Try PGM backend as fallback

**Input not working**:
- Ensure window has focus
- Check if running in terminal vs IDE
- Verify keyboard/mouse permissions

**Performance issues**:
- Check if vsync is enabled
- Verify graphics drivers are working
- Test with simpler graphics operations

**Compilation errors**:
- Verify feature flags are correct
- Check Rust version (minimum 1.80)
- Ensure minifb dependency is available

### Debug Commands

Enable debug logging:
```bash
RUST_LOG=bgi=debug cargo run --example visual_hello --features visual-backend
```

Force specific backend:
```bash
BGI_BACKEND=pgm cargo run --example test_program
BGI_BACKEND=visual cargo run --example test_program
```

## Success Criteria

After completing this quickstart, you should have:

- ✅ Visual window opening and displaying graphics
- ✅ Keyboard input captured and processed correctly
- ✅ Mouse position tracking working
- ✅ PGM output functioning for headless scenarios
- ✅ Backend switching working automatically
- ✅ Performance meeting 30+ FPS target
- ✅ All test examples running without errors

## Next Steps

1. **Advanced Examples**: Explore complex graphics and input combinations
2. **Integration**: Add visual backend to existing BGI programs
3. **Testing**: Use PGM backend for automated visual testing
4. **Performance**: Profile and optimize graphics-intensive applications
