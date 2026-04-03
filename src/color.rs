//! Color system for the BGI library.

use crate::constants::MAX_COLORS;

/// BGI color system supporting both indexed and RGB colors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    /// Indexed color using BGI 16-color palette.
    Indexed(u8),
    /// RGB color with 8-bit components.
    Rgb(RgbColor),
}

/// BGI standard color constants.
impl Color {
    /// Black color.
    pub const BLACK: Self = Self::Indexed(0);
    /// Blue color.
    pub const BLUE: Self = Self::Indexed(1);
    /// Green color.
    pub const GREEN: Self = Self::Indexed(2);
    /// Cyan color.
    pub const CYAN: Self = Self::Indexed(3);
    /// Red color.
    pub const RED: Self = Self::Indexed(4);
    /// Magenta color.
    pub const MAGENTA: Self = Self::Indexed(5);
    /// Brown color.
    pub const BROWN: Self = Self::Indexed(6);
    /// Light gray color.
    pub const LIGHTGRAY: Self = Self::Indexed(7);
    /// Dark gray color.
    pub const DARKGRAY: Self = Self::Indexed(8);
    /// Light blue color.
    pub const LIGHTBLUE: Self = Self::Indexed(9);
    /// Light green color.
    pub const LIGHTGREEN: Self = Self::Indexed(10);
    /// Light cyan color.
    pub const LIGHTCYAN: Self = Self::Indexed(11);
    /// Light red color.
    pub const LIGHTRED: Self = Self::Indexed(12);
    /// Light magenta color.
    pub const LIGHTMAGENTA: Self = Self::Indexed(13);
    /// Yellow color.
    pub const YELLOW: Self = Self::Indexed(14);
    /// White color.
    pub const WHITE: Self = Self::Indexed(15);
}

impl Color {
    /// Convert color to RGB value.
    pub fn to_rgb(self) -> RgbColor {
        match self {
            Self::Indexed(idx) => {
                match idx % 16 {
                    0 => RgbColor::new(0, 0, 0),        // Black
                    1 => RgbColor::new(0, 0, 170),      // Blue
                    2 => RgbColor::new(0, 170, 0),      // Green
                    3 => RgbColor::new(0, 170, 170),    // Cyan
                    4 => RgbColor::new(170, 0, 0),      // Red
                    5 => RgbColor::new(170, 0, 170),    // Magenta
                    6 => RgbColor::new(170, 85, 0),     // Brown
                    7 => RgbColor::new(170, 170, 170),  // Light Gray
                    8 => RgbColor::new(85, 85, 85),     // Dark Gray
                    9 => RgbColor::new(85, 85, 255),    // Light Blue
                    10 => RgbColor::new(85, 255, 85),   // Light Green
                    11 => RgbColor::new(85, 255, 255),  // Light Cyan
                    12 => RgbColor::new(255, 85, 85),   // Light Red
                    13 => RgbColor::new(255, 85, 255),  // Light Magenta
                    14 => RgbColor::new(255, 255, 85),  // Yellow
                    15 => RgbColor::new(255, 255, 255), // White
                    _ => RgbColor::new(255, 255, 255),  // Default to white
                }
            }
            Self::Rgb(rgb) => rgb,
        }
    }

    /// Get color name.
    pub fn name(self) -> &'static str {
        match self {
            Self::Indexed(idx) => match idx % 16 {
                0 => "BLACK",
                1 => "BLUE",
                2 => "GREEN",
                3 => "CYAN",
                4 => "RED",
                5 => "MAGENTA",
                6 => "BROWN",
                7 => "LIGHTGRAY",
                8 => "DARKGRAY",
                9 => "LIGHTBLUE",
                10 => "LIGHTGREEN",
                11 => "LIGHTCYAN",
                12 => "LIGHTRED",
                13 => "LIGHTMAGENTA",
                14 => "YELLOW",
                15 => "WHITE",
                _ => "UNKNOWN",
            },
            Self::Rgb(_) => "RGB",
        }
    }

    /// Create color from integer value.
    pub fn from_int(value: i32) -> Option<Self> {
        if value >= 0 && value <= 15 {
            Some(Self::Indexed(value as u8))
        } else {
            None
        }
    }

    /// Convert color to palette index (0-15 for indexed colors, 0 for RGB).
    pub fn to_index(self) -> u8 {
        match self {
            Self::Indexed(idx) => idx,
            Self::Rgb(_) => 0, // RGB colors don't have a palette index
        }
    }
}

/// RGB color with 8-bit components.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RgbColor {
    /// Red component (0-255).
    pub r: u8,
    /// Green component (0-255).
    pub g: u8,
    /// Blue component (0-255).
    pub b: u8,
    /// Alpha component (0-255).
    pub a: u8,
}

impl RgbColor {
    /// Create new RGB color with full opacity.
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// Create new RGBA color.
    pub const fn with_alpha(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Convert to 32-bit ARGB value.
    pub fn to_argb32(self) -> u32 {
        (u32::from(self.a) << 24)
            | (u32::from(self.r) << 16)
            | (u32::from(self.g) << 8)
            | u32::from(self.b)
    }

    /// Create RGB color from 32-bit ARGB value.
    pub fn from_argb32(argb: u32) -> Self {
        Self {
            a: ((argb >> 24) & 0xFF) as u8,
            r: ((argb >> 16) & 0xFF) as u8,
            g: ((argb >> 8) & 0xFF) as u8,
            b: (argb & 0xFF) as u8,
        }
    }

    /// Create RGB color from separate components.
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new(r, g, b)
    }

    /// Extract red component.
    pub fn red(argb: u32) -> u8 {
        ((argb >> 16) & 0xFF) as u8
    }

    /// Extract green component.
    pub fn green(argb: u32) -> u8 {
        ((argb >> 8) & 0xFF) as u8
    }

    /// Extract blue component.
    pub fn blue(argb: u32) -> u8 {
        (argb & 0xFF) as u8
    }

    /// Extract alpha component.
    pub fn alpha(argb: u32) -> u8 {
        ((argb >> 24) & 0xFF) as u8
    }
}

/// Color palette (compatible with BGI palettetype).
#[derive(Debug, Clone)]
pub struct Palette {
    /// Palette size.
    pub size: u8,
    /// Color values.
    pub colors: [u32; MAX_COLORS + 1],
}

impl Default for Palette {
    fn default() -> Self {
        let mut colors = [0u32; MAX_COLORS + 1];
        for i in 0..=MAX_COLORS {
            if let Some(color) = Color::from_int(i as i32) {
                colors[i] = color.to_rgb().to_argb32();
            }
        }

        Self {
            size: MAX_COLORS as u8,
            colors,
        }
    }
}

/// Extended RGB palette (compatible with BGI rgbpalettetype).
#[derive(Debug, Clone)]
pub struct RgbPalette {
    /// Palette size.
    pub size: u32,
    /// Color values.
    pub colors: Vec<u32>,
}

impl RgbPalette {
    /// Create new RGB palette with specified size.
    pub fn new(size: u32) -> Self {
        Self {
            size,
            colors: vec![0; size as usize],
        }
    }

    /// Set color at index.
    pub fn set_color(&mut self, index: usize, color: u32) {
        if index < self.colors.len() {
            self.colors[index] = color;
        }
    }

    /// Get color at index.
    pub fn get_color(&self, index: usize) -> Option<u32> {
        self.colors.get(index).copied()
    }

    /// Resize palette.
    pub fn resize(&mut self, new_size: u32) {
        self.size = new_size;
        self.colors.resize(new_size as usize, 0);
    }
}

impl Default for RgbPalette {
    fn default() -> Self {
        Self::new(4096) // Default size as per SDL_bgi
    }
}

/// Check if a color value represents a BGI color.
pub fn is_bgi_color(_color: i32) -> bool {
    // In the original BGI, this depends on the current palette mode
    // For simplicity, we'll always return true for standard color range
    true
}

/// Check if a color value represents an RGB color.
pub fn is_rgb_color(_color: i32) -> bool {
    // In the original BGI, this depends on the current palette mode
    // For simplicity, we'll return true for extended colors
    true
}
