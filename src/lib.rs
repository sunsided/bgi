//! # BGI - Borland Graphics Interface for Rust
//!
//! A modern Rust port of the classic Borland Graphics Interface (BGI) with extensible backends.

#![allow(missing_docs, dead_code, unused_imports, unused_variables)]

// Core module declarations - only include working modules for now
pub mod color;
pub mod constants;
pub mod error;
pub mod input;
pub mod types;

// Re-export public API
pub use color::{Color, RgbColor};
pub use constants::*;
pub use error::{BgiError, BgiResult};
pub use types::colors::*;
pub use types::{
    BgiFillSettings, BgiLineSettings, BgiTextSettings, BgiViewportSettings, GraphResult,
    GraphicsDriver, GraphicsMode, MouseState, Point,
};

// Include backend module
pub mod backend;
pub mod line;
pub mod viewport;
pub mod window;

// Phase 3.3 entities - make public for unit testing
pub mod drawing_state;
pub mod font_settings;
pub mod input_event;
pub mod window_state;

pub use drawing_state::*;
pub use font_settings::*;
pub use input_event::*;
pub use window_state::*;

// Phase 3.3 API modules
mod graphics;
mod image;
mod palette;
mod shapes;

// Optimizations module for zero-cost abstractions
pub mod optimizations;

// The unified graphics state and all BGI free functions live in `graphics`;
// drawing primitives live in `shapes`. Both operate on the single global
// GRAPHICS_STATE (see `graphics::GRAPHICS_STATE`). These re-exports surface
// the canonical implementations as the public BGI API.
pub use graphics::*;
pub use image::*;
pub use optimizations::{BatchDrawer, DrawingPool, const_optimized};
pub use palette::*;
pub use shapes::*;

// Graphics initialization functions

/// Get resolution and color information for a device/mode combination
pub fn getmodeinfo(driver: i32, mode: i32) -> Option<(u32, u32, u32)> {
    let graphics_driver = match driver {
        0 => GraphicsDriver::Detect,
        1 => GraphicsDriver::Cga,
        2 => GraphicsDriver::Mcga,
        3 => GraphicsDriver::Ega,
        4 => GraphicsDriver::Ega64,
        5 => GraphicsDriver::EgaMono,
        6 => GraphicsDriver::Ibm8514,
        7 => GraphicsDriver::HercMono,
        8 => GraphicsDriver::Att400,
        9 => GraphicsDriver::Vga,
        10 => GraphicsDriver::Pc3270,
        _ => return None,
    };

    let graphics_mode = GraphicsMode::new(graphics_driver, mode);
    let resolution = graphics_mode.resolution();
    let colors = graphics_mode.color_depth();

    Some((resolution.0 as u32, resolution.1 as u32, colors as u32))
}

// Drawing primitives (`line`, `circle`, `rectangle`, `arc`, `ellipse`,
// `putpixel`, `getpixel`, ...) and most state/query functions (`setviewport`,
// `getviewport`, `getmaxx`, `getmaxy`, `moverel`, `getx`, `gety`, `getch`,
// `kbhit`, `getmouse`, `ismouseclick`, ...) are provided directly by the
// `shapes::*` and `graphics::*` re-exports above. They all operate on the
// single GRAPHICS_STATE, so no wrapper functions are defined here.

/// Move the current position to (x, y).
///
/// Both `graphics::moveto` and `shapes::moveto` define this name and write the
/// same `GRAPHICS_STATE.drawing_state`; this local definition disambiguates the
/// two glob re-exports while keeping a single state store.
pub fn moveto(x: i32, y: i32) {
    crate::graphics::moveto(x, y);
}

/// Get current cursor position as a Point.
/// This is a convenience function not in the original BGI API.
pub fn getposition() -> crate::types::Point {
    crate::types::Point {
        x: crate::graphics::getx(),
        y: crate::graphics::gety(),
    }
}

// Filled shapes functions (TDD stubs - drawing logic pending)
pub fn fillellipse(x: i32, y: i32, x_radius: i32, y_radius: i32) {
    // TDD stub - will implement with actual backend
}

pub fn sector(x: i32, y: i32, start_angle: i32, end_angle: i32, x_radius: i32, y_radius: i32) {
    // TDD stub - will implement with actual backend
}

pub fn pieslice(x: i32, y: i32, start_angle: i32, end_angle: i32, radius: i32) {
    // TDD stub - will implement with actual backend
}

pub fn bar(left: i32, top: i32, right: i32, bottom: i32) {
    // TDD stub - will implement with actual backend
}

// NOTE: kept as a stub that shadows `shapes::fillpoly` to preserve prior
// behavior; reconciling this with the real implementation is out of scope here.
pub fn fillpoly(points: &[(i32, i32)]) {
    // TDD stub - will implement with actual backend
}
