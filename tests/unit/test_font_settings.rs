//! Unit tests for font settings and text rendering configuration.

use bgi::{Color, constants::*, font_settings::*};

#[test]
fn test_font_info_default() {
    let font_info = FontInfo::default();

    assert_eq!(font_info.font, DEFAULT_FONT);
    assert_eq!(font_info.name, "Default");
    assert!(font_info.filename.is_none());
}

#[test]
fn test_text_style_default() {
    let style = TextStyle::default();

    assert_eq!(style.font, DEFAULT_FONT);
    assert_eq!(style.direction, HORIZ_DIR);
    assert_eq!(style.char_size, 1);
}

#[test]
fn test_text_alignment_default() {
    let alignment = TextAlignment::default();

    assert_eq!(alignment.horizontal, LEFT_TEXT);
    assert_eq!(alignment.vertical, TOP_TEXT);
}

#[test]
fn test_font_settings_default() {
    let settings = FontSettings::default();

    assert_eq!(settings.style.font, DEFAULT_FONT);
    assert_eq!(settings.style.direction, HORIZ_DIR);
    assert_eq!(settings.style.char_size, 1);
    assert_eq!(settings.alignment.horizontal, LEFT_TEXT);
    assert_eq!(settings.alignment.vertical, TOP_TEXT);
    assert!(settings.user_char_size.is_none());

    // Test that default fonts are loaded
    assert!(!settings.fonts.is_empty());
    assert!(settings.fonts.iter().any(|f| f.font == DEFAULT_FONT));
    assert!(settings.fonts.iter().any(|f| f.font == TRIPLEX_FONT));
    assert!(settings.fonts.iter().any(|f| f.font == SMALL_FONT));
    assert!(settings.fonts.iter().any(|f| f.font == SANS_SERIF_FONT));
    assert!(settings.fonts.iter().any(|f| f.font == GOTHIC_FONT));
}

#[test]
fn test_font_settings_new() {
    let settings = FontSettings::new();
    let default_settings = FontSettings::default();

    assert_eq!(settings.style.font, default_settings.style.font);
    assert_eq!(
        settings.alignment.horizontal,
        default_settings.alignment.horizontal
    );
    assert_eq!(settings.fonts.len(), default_settings.fonts.len());
}

#[test]
fn test_text_style_operations() {
    let mut settings = FontSettings::new();

    settings.set_text_style(TRIPLEX_FONT, VERT_DIR, 3);
    let (font, direction, char_size) = settings.get_text_style();

    assert_eq!(font, TRIPLEX_FONT);
    assert_eq!(direction, VERT_DIR);
    assert_eq!(char_size, 3);
}

#[test]
fn test_text_alignment_operations() {
    let mut settings = FontSettings::new();

    settings.set_text_justify(CENTER_TEXT, BOTTOM_TEXT);
    let (horizontal, vertical) = settings.get_text_justify();

    assert_eq!(horizontal, CENTER_TEXT);
    assert_eq!(vertical, BOTTOM_TEXT);
}

#[test]
fn test_user_char_size_operations() {
    let mut settings = FontSettings::new();

    // Initially no user char size
    assert!(settings.user_char_size.is_none());

    // Set user char size
    settings.set_user_char_size(16, 24);
    assert_eq!(settings.user_char_size, Some((16, 24)));

    // Clear user char size
    settings.clear_user_char_size();
    assert!(settings.user_char_size.is_none());
}

#[test]
fn test_char_size_calculation() {
    let mut settings = FontSettings::new();

    // Test default font sizes
    settings.set_text_style(DEFAULT_FONT, HORIZ_DIR, 1);
    assert_eq!(settings.get_char_size(), (8, 8));

    settings.set_text_style(TRIPLEX_FONT, HORIZ_DIR, 1);
    assert_eq!(settings.get_char_size(), (13, 16));

    settings.set_text_style(SMALL_FONT, HORIZ_DIR, 1);
    assert_eq!(settings.get_char_size(), (6, 8));

    settings.set_text_style(SANS_SERIF_FONT, HORIZ_DIR, 1);
    assert_eq!(settings.get_char_size(), (11, 16));

    settings.set_text_style(GOTHIC_FONT, HORIZ_DIR, 1);
    assert_eq!(settings.get_char_size(), (14, 16));

    // Test size multiplier
    settings.set_text_style(DEFAULT_FONT, HORIZ_DIR, 3);
    assert_eq!(settings.get_char_size(), (24, 24)); // 8 * 3

    // Test user-defined size overrides
    settings.set_user_char_size(20, 30);
    assert_eq!(settings.get_char_size(), (20, 30)); // User size overrides font size

    // Test unknown font defaults to DEFAULT_FONT
    settings.clear_user_char_size();
    settings.set_text_style(999, HORIZ_DIR, 2); // Unknown font
    assert_eq!(settings.get_char_size(), (16, 16)); // Default font * 2
}

#[test]
fn test_text_width_calculation() {
    let mut settings = FontSettings::new();

    // Test empty string
    assert_eq!(settings.text_width(""), 0);

    // Test horizontal text
    settings.set_text_style(DEFAULT_FONT, HORIZ_DIR, 1);
    assert_eq!(settings.text_width("Hello"), 5 * 8); // 5 chars * 8 pixels wide

    settings.set_text_style(DEFAULT_FONT, HORIZ_DIR, 2);
    assert_eq!(settings.text_width("Hi"), 2 * 16); // 2 chars * 16 pixels wide (2x scale)

    // Test vertical text (fixed width)
    settings.set_text_style(DEFAULT_FONT, VERT_DIR, 1);
    assert_eq!(settings.text_width("Hello"), 8); // Vertical text has fixed width
    assert_eq!(settings.text_width("Hi"), 8); // Same width regardless of length

    // Test with user-defined char size
    settings.set_user_char_size(12, 18);
    settings.set_text_style(DEFAULT_FONT, HORIZ_DIR, 1);
    assert_eq!(settings.text_width("ABC"), 3 * 12); // 3 chars * 12 pixels wide (user size)
}

#[test]
fn test_text_height_calculation() {
    let mut settings = FontSettings::new();

    // Test empty string
    assert_eq!(settings.text_height(""), 0);

    // Test horizontal text (fixed height)
    settings.set_text_style(DEFAULT_FONT, HORIZ_DIR, 1);
    assert_eq!(settings.text_height("Hello"), 8); // Fixed height
    assert_eq!(settings.text_height("Hi"), 8); // Same height regardless of length

    settings.set_text_style(DEFAULT_FONT, HORIZ_DIR, 3);
    assert_eq!(settings.text_height("Hello"), 24); // 8 * 3 scale factor

    // Test vertical text (height depends on character count)
    settings.set_text_style(DEFAULT_FONT, VERT_DIR, 1);
    assert_eq!(settings.text_height("Hello"), 5 * 8); // 5 chars * 8 pixels high
    assert_eq!(settings.text_height("Hi"), 2 * 8); // 2 chars * 8 pixels high

    // Test with user-defined char size
    settings.set_user_char_size(12, 18);
    settings.set_text_style(DEFAULT_FONT, VERT_DIR, 1);
    assert_eq!(settings.text_height("ABC"), 3 * 18); // 3 chars * 18 pixels high (user size)
}

#[test]
fn test_font_management() {
    let mut settings = FontSettings::new();

    // Test getting existing font info
    let default_font = settings.get_font_info(DEFAULT_FONT);
    assert!(default_font.is_some());
    assert_eq!(default_font.unwrap().name, "Default");

    // Test adding new font
    settings.add_font(
        100,
        "Custom Font".to_string(),
        Some("custom.ttf".to_string()),
    );
    let custom_font = settings.get_font_info(100);
    assert!(custom_font.is_some());
    assert_eq!(custom_font.unwrap().name, "Custom Font");
    assert_eq!(
        custom_font.unwrap().filename,
        Some("custom.ttf".to_string())
    );

    // Test replacing existing font
    let original_count = settings.fonts.len();
    settings.add_font(DEFAULT_FONT, "Modified Default".to_string(), None);
    assert_eq!(settings.fonts.len(), original_count); // Should not increase count

    let modified_font = settings.get_font_info(DEFAULT_FONT);
    assert!(modified_font.is_some());
    assert_eq!(modified_font.unwrap().name, "Modified Default");

    // Test font validation
    assert!(settings.is_valid_font(DEFAULT_FONT));
    assert!(settings.is_valid_font(100)); // Custom font we added
    assert!(!settings.is_valid_font(999)); // Non-existent font
}

#[test]
fn test_font_type_checks() {
    let mut settings = FontSettings::new();

    // Test bitmap font
    settings.set_text_style(DEFAULT_FONT, HORIZ_DIR, 1);
    assert!(!settings.is_vector_font());

    // Test vector fonts
    settings.set_text_style(TRIPLEX_FONT, HORIZ_DIR, 1);
    assert!(settings.is_vector_font());

    settings.set_text_style(SANS_SERIF_FONT, HORIZ_DIR, 1);
    assert!(settings.is_vector_font());
}

#[test]
fn test_direction_checks() {
    let mut settings = FontSettings::new();

    settings.set_text_style(DEFAULT_FONT, HORIZ_DIR, 1);
    assert!(settings.is_horizontal());
    assert!(!settings.is_vertical());

    settings.set_text_style(DEFAULT_FONT, VERT_DIR, 1);
    assert!(!settings.is_horizontal());
    assert!(settings.is_vertical());
}

#[test]
fn test_position_adjustment() {
    let mut settings = FontSettings::new();
    settings.set_text_style(DEFAULT_FONT, HORIZ_DIR, 1);

    let text = "Test"; // 4 chars * 8 pixels = 32 width, 8 height

    // Test left-top alignment (no adjustment)
    settings.set_text_justify(LEFT_TEXT, TOP_TEXT);
    assert_eq!(settings.adjust_position(100, 200, text), (100, 200));

    // Test center-center alignment
    settings.set_text_justify(CENTER_TEXT, CENTER_TEXT);
    assert_eq!(settings.adjust_position(100, 200, text), (84, 196)); // 100-16, 200-4

    // Test right-bottom alignment
    settings.set_text_justify(RIGHT_TEXT, BOTTOM_TEXT);
    assert_eq!(settings.adjust_position(100, 200, text), (68, 192)); // 100-32, 200-8

    // Test with different font size
    settings.set_text_style(DEFAULT_FONT, HORIZ_DIR, 2);
    settings.set_text_justify(CENTER_TEXT, CENTER_TEXT);
    // Text is now 4*16=64 wide, 16 high
    assert_eq!(settings.adjust_position(100, 200, text), (68, 192)); // 100-32, 200-8
}

#[test]
fn test_reset() {
    let mut settings = FontSettings::new();

    // Modify settings
    settings.set_text_style(TRIPLEX_FONT, VERT_DIR, 5);
    settings.set_text_justify(RIGHT_TEXT, BOTTOM_TEXT);
    settings.set_user_char_size(20, 30);

    // Reset
    settings.reset();

    // Verify defaults are restored
    let (font, direction, char_size) = settings.get_text_style();
    assert_eq!(font, DEFAULT_FONT);
    assert_eq!(direction, HORIZ_DIR);
    assert_eq!(char_size, 1);

    let (horizontal, vertical) = settings.get_text_justify();
    assert_eq!(horizontal, LEFT_TEXT);
    assert_eq!(vertical, TOP_TEXT);

    assert!(settings.user_char_size.is_none());
}

#[test]
fn test_structure_equality() {
    let info1 = FontInfo {
        font: DEFAULT_FONT,
        name: "Test".to_string(),
        filename: None,
    };
    let info2 = FontInfo {
        font: DEFAULT_FONT,
        name: "Test".to_string(),
        filename: None,
    };
    let info3 = FontInfo {
        font: TRIPLEX_FONT,
        name: "Test".to_string(),
        filename: None,
    };

    assert_eq!(info1, info2);
    assert_ne!(info1, info3);

    let style1 = TextStyle {
        font: DEFAULT_FONT,
        direction: HORIZ_DIR,
        char_size: 1,
    };
    let style2 = TextStyle {
        font: DEFAULT_FONT,
        direction: HORIZ_DIR,
        char_size: 1,
    };
    let style3 = TextStyle {
        font: DEFAULT_FONT,
        direction: VERT_DIR,
        char_size: 1,
    };

    assert_eq!(style1, style2);
    assert_ne!(style1, style3);

    let align1 = TextAlignment {
        horizontal: LEFT_TEXT,
        vertical: TOP_TEXT,
    };
    let align2 = TextAlignment {
        horizontal: LEFT_TEXT,
        vertical: TOP_TEXT,
    };
    let align3 = TextAlignment {
        horizontal: CENTER_TEXT,
        vertical: TOP_TEXT,
    };

    assert_eq!(align1, align2);
    assert_ne!(align1, align3);
}

#[test]
fn test_edge_cases() {
    let mut settings = FontSettings::new();

    // Test zero character size (should be clamped to 1)
    settings.set_text_style(DEFAULT_FONT, HORIZ_DIR, 0);
    let (width, height) = settings.get_char_size();
    assert_eq!(width, 8); // Should use size 1
    assert_eq!(height, 8);

    // Test negative character size (should be clamped to 1)
    settings.set_text_style(DEFAULT_FONT, HORIZ_DIR, -5);
    let (width, height) = settings.get_char_size();
    assert_eq!(width, 8); // Should use size 1
    assert_eq!(height, 8);
}
