//! Unit tests for color system.

use bgi::color::{Color, RgbColor, Palette, RgbPalette};

#[test]
fn test_color_constants() {
    assert_eq!(Color::BLACK, Color::Indexed(0));
    assert_eq!(Color::BLUE, Color::Indexed(1));
    assert_eq!(Color::GREEN, Color::Indexed(2));
    assert_eq!(Color::CYAN, Color::Indexed(3));
    assert_eq!(Color::RED, Color::Indexed(4));
    assert_eq!(Color::MAGENTA, Color::Indexed(5));
    assert_eq!(Color::BROWN, Color::Indexed(6));
    assert_eq!(Color::LIGHTGRAY, Color::Indexed(7));
    assert_eq!(Color::DARKGRAY, Color::Indexed(8));
    assert_eq!(Color::LIGHTBLUE, Color::Indexed(9));
    assert_eq!(Color::LIGHTGREEN, Color::Indexed(10));
    assert_eq!(Color::LIGHTCYAN, Color::Indexed(11));
    assert_eq!(Color::LIGHTRED, Color::Indexed(12));
    assert_eq!(Color::LIGHTMAGENTA, Color::Indexed(13));
    assert_eq!(Color::YELLOW, Color::Indexed(14));
    assert_eq!(Color::WHITE, Color::Indexed(15));
}

#[test]
fn test_color_to_rgb() {
    // Test indexed colors
    assert_eq!(Color::BLACK.to_rgb(), RgbColor::new(0, 0, 0));
    assert_eq!(Color::RED.to_rgb(), RgbColor::new(170, 0, 0));
    assert_eq!(Color::WHITE.to_rgb(), RgbColor::new(255, 255, 255));

    // Test RGB color passthrough
    let rgb = RgbColor::new(128, 64, 192);
    assert_eq!(Color::Rgb(rgb).to_rgb(), rgb);
}

#[test]
fn test_color_names() {
    assert_eq!(Color::BLACK.name(), "BLACK");
    assert_eq!(Color::RED.name(), "RED");
    assert_eq!(Color::WHITE.name(), "WHITE");
    assert_eq!(Color::Rgb(RgbColor::new(0, 0, 0)).name(), "RGB");
}

#[test]
fn test_color_from_int() {
    for i in 0..=15 {
        let color = Color::from_int(i);
        assert!(color.is_some());
        assert_eq!(color.unwrap(), Color::Indexed(i as u8));
    }

    // Test invalid values
    assert!(Color::from_int(-1).is_none());
    assert!(Color::from_int(16).is_none());
}

#[test]
fn test_rgb_color_creation() {
    let rgb = RgbColor::new(255, 128, 64);
    assert_eq!(rgb.r, 255);
    assert_eq!(rgb.g, 128);
    assert_eq!(rgb.b, 64);
    assert_eq!(rgb.a, 255); // Default alpha

    let rgba = RgbColor::with_alpha(255, 128, 64, 192);
    assert_eq!(rgba.r, 255);
    assert_eq!(rgba.g, 128);
    assert_eq!(rgba.b, 64);
    assert_eq!(rgba.a, 192);
}

#[test]
fn test_rgb_color_argb_conversion() {
    let rgb = RgbColor::new(255, 128, 64);
    let argb = rgb.to_argb32();
    let back = RgbColor::from_argb32(argb);

    assert_eq!(rgb, back);

    // Test specific ARGB value
    let argb_value = 0xFF804020; // A=255, R=128, G=64, B=32
    let rgb_from_argb = RgbColor::from_argb32(argb_value);
    assert_eq!(rgb_from_argb.a, 255);
    assert_eq!(rgb_from_argb.r, 128);
    assert_eq!(rgb_from_argb.g, 64);
    assert_eq!(rgb_from_argb.b, 32);
}

#[test]
fn test_rgb_color_component_extraction() {
    let argb = 0xFF804020; // A=255, R=128, G=64, B=32

    assert_eq!(RgbColor::alpha(argb), 255);
    assert_eq!(RgbColor::red(argb), 128);
    assert_eq!(RgbColor::green(argb), 64);
    assert_eq!(RgbColor::blue(argb), 32);
}

#[test]
fn test_default_palette() {
    let palette = Palette::default();

    assert_eq!(palette.size, 15); // MAX_COLORS

    // Test first few colors match expectations
    let black_argb = Color::BLACK.to_rgb().to_argb32();
    let red_argb = Color::RED.to_rgb().to_argb32();
    let white_argb = Color::WHITE.to_rgb().to_argb32();

    assert_eq!(palette.colors[0], black_argb);
    assert_eq!(palette.colors[4], red_argb);
    assert_eq!(palette.colors[15], white_argb);
}

#[test]
fn test_rgb_palette() {
    let mut palette = RgbPalette::new(10);
    
    assert_eq!(palette.size, 10);
    assert_eq!(palette.colors.len(), 10);
    
    // Test setting and getting colors
    palette.set_color(5, 0xFF0000FF); // Blue
    assert_eq!(palette.get_color(5), Some(0xFF0000FF));
    assert_eq!(palette.get_color(9), Some(0)); // Should be default (black)
    assert_eq!(palette.get_color(10), None); // Out of bounds

    // Test resizing
    palette.resize(5);
    assert_eq!(palette.size, 5);
    assert_eq!(palette.colors.len(), 5);
    assert_eq!(palette.get_color(5), None); // Now out of bounds
    assert_eq!(palette.get_color(4), Some(0)); // Should still exist
}

#[test]
fn test_rgb_palette_default() {
    let palette = RgbPalette::default();
    assert_eq!(palette.size, 4096);
    assert_eq!(palette.colors.len(), 4096);
}

#[test]
fn test_color_equality() {
    let color1 = Color::RED;
    let color2 = Color::Indexed(4);
    let color3 = Color::Rgb(RgbColor::new(255, 0, 0));

    assert_eq!(color1, color2);
    assert_ne!(color1, color3); // Different variants

    let rgb1 = RgbColor::new(128, 64, 32);
    let rgb2 = RgbColor::new(128, 64, 32);
    let rgb3 = RgbColor::new(128, 64, 33);

    assert_eq!(rgb1, rgb2);
    assert_ne!(rgb1, rgb3);
}

#[test]
fn test_color_indexing_wraparound() {
    // Test that colors wrap around when index > 15
    let color16 = Color::Indexed(16);
    let color0 = Color::Indexed(0);

    assert_eq!(color16.to_rgb(), color0.to_rgb());
    assert_eq!(color16.name(), color0.name());
}

#[test]
fn test_rgb_color_rgb_constructor() {
    let rgb = RgbColor::rgb(100, 150, 200);
    assert_eq!(rgb.r, 100);
    assert_eq!(rgb.g, 150);
    assert_eq!(rgb.b, 200);
    assert_eq!(rgb.a, 255);
}

#[test]
fn test_palette_operations() {
    let mut palette = RgbPalette::new(16);

    // Test setting standard BGI colors
    for i in 0..16 {
        if let Some(color) = Color::from_int(i) {
            let argb = color.to_rgb().to_argb32();
            palette.set_color(i as usize, argb);
        }
    }

    // Verify the colors were set correctly
    assert_eq!(palette.get_color(0), Some(Color::BLACK.to_rgb().to_argb32()));
    assert_eq!(palette.get_color(4), Some(Color::RED.to_rgb().to_argb32()));
    assert_eq!(palette.get_color(15), Some(Color::WHITE.to_rgb().to_argb32()));
}
