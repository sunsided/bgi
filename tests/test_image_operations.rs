use bgi::{
    initgraph, closegraph,
    getimage, putimage, loadimage, saveimage,
    imagesize, getimagesize,
};

#[test]
fn test_getimage_contract() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    // Test getting image from a region
    let image_data = getimage(10, 10, 50, 50);
    assert!(image_data.is_some(), "getimage should return Some for valid region");

    // Test getting image from invalid region
    let invalid_image = getimage(-10, -10, 0, 0);
    assert!(invalid_image.is_none(), "getimage should return None for invalid region");

    closegraph();
}

#[test]
fn test_putimage_contract() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    // First get an image
    let image_data = getimage(10, 10, 50, 50);
    assert!(image_data.is_some(), "Should be able to get image data");

    if let Some(data) = image_data {
        // Test putting image back
        putimage(100, 100, &data, 0); // COPY_PUT mode
        // BGI functions are void - errors reported via graphresult()

        // Test putting image with invalid mode
        putimage(100, 100, &data, 999);
        // BGI functions are void - errors reported via graphresult()
    }

    closegraph();
}

#[test]
fn test_loadimage_contract() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    // Test loading non-existent file
    let result = loadimage("non_existent_file.bmp");
    assert!(result.is_none(), "loadimage should return None for non-existent file");

    // Test loading with empty filename
    let empty_result = loadimage("");
    assert!(empty_result.is_none(), "loadimage should return None for empty filename");

    closegraph();
}

#[test]
fn test_saveimage_contract() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    // Test saving to invalid path
    let result = saveimage("/invalid/path/test.bmp", 0, 0, 100, 100);
    assert!(!result, "saveimage should return false for invalid path");

    // Test saving with invalid coordinates
    let invalid_coords = saveimage("test.bmp", -10, -10, 0, 0);
    assert!(!invalid_coords, "saveimage should return false for invalid coordinates");

    closegraph();
}

#[test]
fn test_imagesize_contract() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    // Test getting size of valid region
    let size = imagesize(0, 0, 100, 100);
    assert!(size > 0, "imagesize should return positive value for valid region");

    // Test getting size of invalid region
    let invalid_size = imagesize(-10, -10, 0, 0);
    assert_eq!(invalid_size, 0, "imagesize should return 0 for invalid region");

    closegraph();
}

#[test]
fn test_getimagesize_contract() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    // Get an image first
    let image_data = getimage(10, 10, 50, 50);
    assert!(image_data.is_some(), "Should be able to get image data");

    if let Some(data) = image_data {
        let (width, height) = getimagesize(&data);
        assert!(width > 0, "Image width should be positive");
        assert!(height > 0, "Image height should be positive");
        assert_eq!(width, 41, "Width should match expected value (50-10+1)");
        assert_eq!(height, 41, "Height should match expected value (50-10+1)");
    }

    closegraph();
}

#[test]
fn test_image_operations_without_graphics() {
    // Test operations without initializing graphics - should fail gracefully
    let result = getimage(0, 0, 10, 10);
    assert!(result.is_none(), "getimage should fail without graphics initialization");

    let size = imagesize(0, 0, 10, 10);
    assert_eq!(size, 0, "imagesize should return 0 without graphics initialization");
}

#[test]
fn test_image_roundtrip() {
    let mut driver = 9; // VGA
    let mut mode = 2;   // VGAHI
    initgraph(&mut driver, &mut mode, "");

    // Draw something, capture it, put it elsewhere
    // This tests the full image workflow

    // First draw something identifiable
    // Note: This will fail until drawing functions are implemented

    // Get the image
    let image_data = getimage(0, 0, 20, 20);
    assert!(image_data.is_some(), "Should capture image data");

    if let Some(data) = image_data {
        // Put it elsewhere
        putimage(50, 50, &data, 0);
        // BGI functions are void - errors reported via graphresult()

        // Verify the size is correct
        let (width, height) = getimagesize(&data);
        assert_eq!(width, 21, "Image width should be correct");
        assert_eq!(height, 21, "Image height should be correct");
    }

    closegraph();
}
