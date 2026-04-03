# BGI - Borland Graphics Interface for Rust

A modern Rust port of the classic Borland Graphics Interface (BGI) with extensible backends.

## Features

- **Full BGI API compatibility** with modern Rust ergonomics
- **Multiple graphics backends**: MiniFB (visual), Pixel Buffer (headless/testing), PGM output
- **Multiple graphics drivers**: CGA, MCGA, EGA, VGA, IBM 8514, Hercules, AT&T 400, PC 3270
- **Double-buffering support** with active/visual page management
- **Keyboard and mouse input** handling
- **16-color EGA/VGA palette** with RGB color support
- **Line styles and fill patterns** with custom bit patterns
- **Viewport clipping** and coordinate management
- **Zero-cost abstractions** with `BatchDrawer` and compile-time optimizations
- **Modern Rust**: Edition 2024, MSRV 1.90

## Quick Start

```rust
use bgi::*;

fn main() {
    let mut driver = DETECT;
    let mut mode = VGAHI;
    
    initgraph(&mut driver, &mut mode, "");
    
    if graphresult() != GraphResult::Ok {
        eprintln!("Graphics initialization failed!");
        return;
    }
    
    setcolor(Color::RED);
    circle(320, 240, 50);
    
    outtextxy(250, 300, "Press any key to exit");
    
    getch();
    closegraph();
}
```

Run with visual output:
```bash
cargo run --example simple --features visual-backend
```

## Examples

```bash
# Basic drawing demo
cargo run --example simple --features visual-backend

# Mandelbrot set visualization
cargo run --example mandelbrot --features visual-backend

# Interactive demonstration with mouse/keyboard
cargo run --example interactive_demo --features visual-backend

# BGI compatibility test
cargo run --example compatibility --features visual-backend

# Batch rendering (optimized)
cargo run --example mandelbrot_batched --features visual-backend
```

## API Overview

### Graphics Initialization
- `initgraph()` / `closegraph()` - Initialize and close graphics mode
- `detectgraph()` - Auto-detect best graphics driver/mode
- `getgraphmode()` / `setgraphmode()` - Get/set current mode
- `graphresult()` / `grapherrormsg()` - Error handling

### Drawing Primitives
- `line()`, `lineto()`, `linerel()` - Line drawing
- `circle()` - Circle with Bresenham algorithm
- `arc()`, `ellipse()` - Arc and ellipse drawing
- `rectangle()` - Rectangle outline
- `putpixel()`, `getpixel()` - Pixel operations
- `fillpoly()` - Polygon fill (outline only)

### Colors & Palettes
- `setcolor()`, `getcolor()` - Drawing color
- `setbkcolor()`, `getbkcolor()` - Background color
- `setpalette()`, `getpalette()` - Palette management
- `setrgbpalette()` - RGB palette entry
- 16 predefined colors: `Color::BLACK`, `Color::WHITE`, `Color::RED`, etc.

### Line Styles & Fill
- `setlinestyle()` - Solid, dashed, dotted, dot-dashed, user-defined patterns
- `getlinesettings()` - Get current line style
- `setfillstyle()` - Fill pattern and color
- `setfillpattern()` - Custom 8x8 fill pattern
- `setwritemode()` - COPY_PUT, XOR_PUT, OR_PUT, AND_PUT, NOT_PUT

### Viewports & Coordinates
- `setviewport()` / `getviewport()` - Clipping region
- `clearviewport()` / `cleardevice()` - Screen clearing
- `getmaxx()`, `getmaxy()` - Screen dimensions
- `moveto()`, `moverel()`, `getx()`, `gety()` - Cursor control

### Text (Partial)
- `outtextxy()` - Display text at position (stub)
- `settextstyle()`, `settextjustify()` - Text configuration
- `textwidth()`, `textheight()` - Text measurement

### Input Handling
- `getch()`, `kbhit()` - Keyboard input
- `getmouse()`, `ismouseclick()` - Mouse state
- `mousex()`, `mousey()` - Mouse position
- `setmouse()`, `clearmouseclick()` - Mouse control

### Pages & Buffering
- `setactivepage()` / `getactivepage()` - Draw to off-screen page
- `setvisualpage()` / `getvisualpage()` - Display page

### Utilities
- `delay()` - Millisecond delay
- `is_headless()` - Check if running without display
- `set_batch_mode()`, `refresh()` - Batch rendering optimization

### Optimizations
- `BatchDrawer` - Batch multiple draw operations
- `DrawingPool` - Pre-allocated buffers for repeated operations
- `PatternShape<N>` - Compile-time optimized regular polygons

## Supported Graphics Modes

| Driver | Mode | Resolution | Colors |
|--------|------|------------|--------|
| CGA | CGAC0-CGAC3 | 320x200 | 4 |
| CGA | CGAHI | 640x200 | 2 |
| MCGA | MCGAC0-MCGAC3 | 320x200 | 256 |
| MCGA | MCGAMED | 640x200 | 256 |
| MCGA | MCGAHI | 640x480 | 256 |
| EGA | EGALO | 640x200 | 16 |
| EGA | EGAHI | 640x350 | 16 |
| VGA | VGALO | 640x200 | 16 |
| VGA | VGAMED | 640x350 | 16 |
| VGA | VGAHI | 640x480 | 16 |

## Backends

### MiniFB Backend (`visual-backend` feature)
Cross-platform windowing using `minifb`. Provides real-time visual output with keyboard and mouse support.

### Pixel Buffer Backend (default)
Headless backend for testing and offline rendering. Stores pixels in memory without display output.

### PGM Backend (`pgm-backend` feature)
Outputs to Portable GrayMap format files for image export.

## Implementation Status

**Complete:**
- Graphics initialization and mode management
- All drawing primitives (line, circle, arc, ellipse, rectangle)
- Color and palette management
- Line styles with custom patterns
- Viewports and coordinate systems
- Keyboard and mouse input
- Page flipping / double buffering

**Partial:**
- Text rendering (outtextxy is a stub)
- Fill patterns (outline only for polygons)
- Image operations (getimage/putimage exist but limited)

**Not Yet Implemented:**
- Flood fill (`floodfill`)
- Filled shapes (`bar`, `fillellipse`, `sector`, `pieslice`)
- Font file loading

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
