//! Color palette management functions for BGI graphics.

use crate::{Color, graphics::{with_graphics_state, with_graphics_state_mut}};

/// Set a single palette entry.
pub fn setpalette(color_num: i32, color: Color) {
    with_graphics_state_mut(|state| {
        if color_num >= 0 && (color_num as usize) < state.current_palette.len() {
            state.current_palette[color_num as usize] = color;
        }
    });
}

/// Get a single palette entry.
pub fn getpaletteentry(color_num: i32) -> Option<Color> {
    with_graphics_state(|state| {
        if color_num >= 0 && (color_num as usize) < state.current_palette.len() {
            Some(state.current_palette[color_num as usize])
        } else {
            None
        }
    }).flatten()
}

/// Set entire palette from array.
pub fn setallpalette(palette: &[Color]) {
    with_graphics_state_mut(|state| {
        let copy_len = palette.len().min(state.current_palette.len());
        state.current_palette[..copy_len].copy_from_slice(&palette[..copy_len]);
    });
}

/// Set palette entry using RGB values.
pub fn setrgbpalette(color_num: i32, red: u8, green: u8, blue: u8) {
    let color = Color::Rgb(crate::RgbColor { r: red, g: green, b: blue, a: 255 });
    setpalette(color_num, color);
}

/// Get default palette for current graphics mode.
pub fn getdefaultpalette() -> Vec<Color> {
    vec![
        Color::BLACK,        // 0
        Color::BLUE,         // 1
        Color::GREEN,        // 2
        Color::CYAN,         // 3
        Color::RED,          // 4
        Color::MAGENTA,      // 5
        Color::BROWN,        // 6
        Color::LIGHTGRAY,    // 7
        Color::DARKGRAY,     // 8
        Color::LIGHTBLUE,    // 9
        Color::LIGHTGREEN,   // 10
        Color::LIGHTCYAN,    // 11
        Color::LIGHTRED,     // 12
        Color::LIGHTMAGENTA, // 13
        Color::YELLOW,       // 14
        Color::WHITE,        // 15
    ]
}

/// Get current palette size.
pub fn getpalettesize() -> i32 {
    with_graphics_state(|state| state.current_palette.len() as i32)
        .unwrap_or(0)
}

/// Get maximum color index.
pub fn getmaxcolor() -> i32 {
    getpalettesize() - 1
}

/// Get entire current palette.
pub fn getpalette() -> Vec<Color> {
    with_graphics_state(|state| state.current_palette.clone())
        .unwrap_or_else(getdefaultpalette)
}
