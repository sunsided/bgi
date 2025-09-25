# BGI - Borland Graphics Interface for Rust

A modern Rust port of the classic Borland Graphics Interface (BGI) with extensible backends.

## Features

- 🎨 Full BGI API compatibility with modern Rust ergonomics
- 🔧 Extensible backend system (Winit, SDL, custom backends)
- 🖥️ Multi-window support
- 🎮 Mouse and keyboard input handling
- 🎯 Pixel-perfect rendering with classic BGI behavior
- 📦 Zero-cost abstractions with compile-time backend selection
- 🦀 Modern Rust: Edition 2021, MSRV 1.80

## Quick Start

```rust
use bgi::*;

fn main() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI - 640x480, 16 colors
    
    initgraph(&mut driver, &mut mode, "");
    
    setcolor(Color::RED);
    circle(320, 240, 50);
    
    getch(); // Wait for keypress
    closegraph();
}
```

## Examples

Run the examples to see BGI in action:

```bash
cargo run --example simple
cargo run --example mandelbrot
cargo run --example life
```

## BGI Compatibility

This crate provides full compatibility with the classic BGI API including:

- Drawing primitives (line, circle, rectangle, arc, etc.)
- Fill patterns and flood fill
- Text rendering with BGI fonts
- Color palettes (16-color + ARGB extensions)
- Viewport and clipping
- Mouse and keyboard input
- Multi-window support

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
