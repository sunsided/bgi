use bgi::{
    BgiTextSettings, Color, closegraph, gettextsettings, initgraph, outtextxy, setcolor,
    settextjustify, settextstyle, textheight, textwidth,
};

#[test]
fn test_text_font_integration() {
    let mut driver = 9; // VGA
    let mut mode = 2; // VGAHI
    initgraph(&mut driver, &mut mode, "");

    setcolor(Color::WHITE);

    // Test basic text output
    outtextxy(50, 50, "Hello, BGI!");
    outtextxy(50, 70, "This is a test string");
    outtextxy(50, 90, "Special chars: !@#$%^&*()");

    // Test empty string
    outtextxy(50, 110, "");

    // Test single character
    outtextxy(50, 130, "A");

    closegraph();
}

#[test]
fn test_text_styles_integration() {
    let mut driver = 9; // VGA
    let mut mode = 2; // VGAHI
    initgraph(&mut driver, &mut mode, "");

    setcolor(Color::WHITE);

    // Test different text styles
    settextstyle(0, 0, 1); // Default font, horizontal, size 1
    outtextxy(50, 50, "Size 1 text");

    settextstyle(0, 0, 2); // Size 2
    outtextxy(50, 80, "Size 2 text");

    settextstyle(0, 0, 3); // Size 3
    outtextxy(50, 120, "Size 3");

    settextstyle(0, 0, 4); // Size 4
    outtextxy(50, 170, "Big");

    // Test vertical direction
    settextstyle(0, 1, 2); // Vertical text
    outtextxy(300, 50, "Vertical");

    closegraph();
}

#[test]
fn test_text_measurement_integration() {
    let mut driver = 9; // VGA
    let mut mode = 2; // VGAHI
    initgraph(&mut driver, &mut mode, "");

    setcolor(Color::WHITE);

    // Test text measurement functions
    settextstyle(0, 0, 2);

    let width = textwidth("Hello");
    let height = textheight("Hello");

    assert!(width > 0, "Text width should be positive");
    assert!(height > 0, "Text height should be positive");

    // Test measurements for different strings
    let short_width = textwidth("Hi");
    let long_width = textwidth("This is a longer string");

    assert!(
        long_width > short_width,
        "Longer text should have greater width"
    );

    // Test empty string measurement
    let empty_width = textwidth("");
    assert_eq!(empty_width, 0, "Empty string should have zero width");

    // Test single character
    let char_width = textwidth("A");
    assert!(
        char_width > 0,
        "Single character should have positive width"
    );

    closegraph();
}

#[test]
fn test_text_justification_integration() {
    let mut driver = 9; // VGA
    let mut mode = 2; // VGAHI
    initgraph(&mut driver, &mut mode, "");

    setcolor(Color::WHITE);
    settextstyle(0, 0, 2);

    // Test different text justifications
    settextjustify(0, 0); // Left, top
    outtextxy(200, 50, "Left-Top");

    settextjustify(1, 0); // Center, top
    outtextxy(200, 80, "Center-Top");

    settextjustify(2, 0); // Right, top
    outtextxy(200, 110, "Right-Top");

    settextjustify(0, 1); // Left, center
    outtextxy(200, 140, "Left-Center");

    settextjustify(1, 1); // Center, center
    outtextxy(200, 170, "Center-Center");

    settextjustify(2, 1); // Right, center
    outtextxy(200, 200, "Right-Center");

    settextjustify(0, 2); // Left, bottom
    outtextxy(200, 230, "Left-Bottom");

    settextjustify(1, 2); // Center, bottom
    outtextxy(200, 260, "Center-Bottom");

    settextjustify(2, 2); // Right, bottom
    outtextxy(200, 290, "Right-Bottom");

    closegraph();
}

#[test]
fn test_text_settings_retrieval() {
    let mut driver = 9; // VGA
    let mut mode = 2; // VGAHI
    initgraph(&mut driver, &mut mode, "");

    // Set specific text settings
    settextstyle(1, 0, 3);
    settextjustify(2, 1);

    // Retrieve and verify settings
    let settings = gettextsettings();

    assert_eq!(settings.font, 1, "Font should match set value");
    assert_eq!(settings.direction, 0, "Direction should match set value");
    assert_eq!(settings.charsize, 3, "Char size should match set value");
    assert_eq!(settings.horiz, 2, "Horizontal justification should match");
    assert_eq!(settings.vert, 1, "Vertical justification should match");

    closegraph();
}

#[test]
fn test_text_with_different_colors() {
    let mut driver = 9; // VGA
    let mut mode = 2; // VGAHI
    initgraph(&mut driver, &mut mode, "");

    settextstyle(0, 0, 2);

    // Test text with different colors
    let colors = [
        Color::WHITE,
        Color::RED,
        Color::GREEN,
        Color::BLUE,
        Color::YELLOW,
        Color::CYAN,
        Color::MAGENTA,
    ];

    for (i, &color) in colors.iter().enumerate() {
        setcolor(color);
        let y = 50 + (i as i32) * 25;
        outtextxy(50, y, &format!("Color {}: {:?}", i, color));
    }

    closegraph();
}

#[test]
fn test_text_positioning_precision() {
    let mut driver = 9; // VGA
    let mut mode = 2; // VGAHI
    initgraph(&mut driver, &mut mode, "");

    setcolor(Color::WHITE);
    settextstyle(0, 0, 1);

    // Test precise positioning
    for i in 0..10 {
        let x = 50 + i * 30;
        let y = 50 + i * 15;
        outtextxy(x, y, &format!("{}", i));
    }

    // Test boundary positions
    outtextxy(0, 0, "Origin");
    outtextxy(1, 1, "Near origin");

    // Test larger coordinates
    outtextxy(500, 300, "Far position");

    closegraph();
}

#[test]
fn test_text_measurement_consistency() {
    let mut driver = 9; // VGA
    let mut mode = 2; // VGAHI
    initgraph(&mut driver, &mut mode, "");

    setcolor(Color::WHITE);

    // Test that measurements are consistent across different sizes
    for size in 1..=4 {
        settextstyle(0, 0, size);

        let width_a = textwidth("A");
        let width_aa = textwidth("AA");
        let width_aaa = textwidth("AAA");

        // Width should generally increase with more characters
        assert!(
            width_aa >= width_a,
            "Two characters should be at least as wide as one at size {}",
            size
        );
        assert!(
            width_aaa >= width_aa,
            "Three characters should be at least as wide as two at size {}",
            size
        );

        let height = textheight("Test");
        assert!(
            height > 0,
            "Text height should be positive at size {}",
            size
        );
    }

    closegraph();
}

#[test]
fn test_text_without_graphics() {
    // Test text operations without graphics initialization
    // Should handle gracefully without crashing

    settextstyle(0, 0, 2);
    settextjustify(1, 1);

    outtextxy(50, 50, "This should not crash");

    let width = textwidth("Test");
    let height = textheight("Test");

    // Should return some reasonable default values or 0
    assert!(
        width >= 0,
        "textwidth should return non-negative value without graphics"
    );
    assert!(
        height >= 0,
        "textheight should return non-negative value without graphics"
    );
}

#[test]
fn test_text_special_characters() {
    let mut driver = 9; // VGA
    let mut mode = 2; // VGAHI
    initgraph(&mut driver, &mut mode, "");

    setcolor(Color::WHITE);
    settextstyle(0, 0, 2);

    // Test various special characters and strings
    outtextxy(50, 50, "Numbers: 0123456789");
    outtextxy(50, 80, "Symbols: !@#$%^&*()");
    outtextxy(50, 110, "Brackets: []{}()<>");
    outtextxy(50, 140, "Punctuation: .,;:\"'?/");
    outtextxy(50, 170, "Math: +-*/=");
    outtextxy(50, 200, "Mixed: Hello123!@#");

    // Test string with spaces
    outtextxy(50, 230, "Spaces between words");
    outtextxy(50, 260, "  Leading spaces");
    outtextxy(50, 290, "Trailing spaces  ");

    closegraph();
}

#[test]
fn test_text_font_size_scaling() {
    let mut driver = 9; // VGA
    let mut mode = 2; // VGAHI
    initgraph(&mut driver, &mut mode, "");

    setcolor(Color::WHITE);

    // Test that font sizes scale appropriately
    let test_string = "Scale";
    let mut previous_width = 0;
    let mut previous_height = 0;

    for size in 1..=4 {
        settextstyle(0, 0, size);

        let width = textwidth(test_string);
        let height = textheight(test_string);

        if size > 1 {
            assert!(
                width >= previous_width,
                "Width should not decrease with larger size"
            );
            assert!(
                height >= previous_height,
                "Height should not decrease with larger size"
            );
        }

        // Output the text to visually verify scaling
        let y = 50 + (size - 1) * 40;
        outtextxy(50, y, &format!("Size {}: {}", size, test_string));

        previous_width = width;
        previous_height = height;
    }

    closegraph();
}
