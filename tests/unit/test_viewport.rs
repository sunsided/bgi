//! Unit tests for viewport management.

use bgi::{types::Rect, viewport::Viewport};

#[test]
fn test_viewport_creation() {
    let viewport = Viewport::new(10, 20, 100, 200, true);

    assert_eq!(viewport.left, 10);
    assert_eq!(viewport.top, 20);
    assert_eq!(viewport.right, 100);
    assert_eq!(viewport.bottom, 200);
    assert!(viewport.clip);
}

#[test]
fn test_viewport_from_rect() {
    let rect = Rect::new(5, 15, 95, 185);
    let viewport = Viewport::from_rect(rect, false);

    assert_eq!(viewport.left, 5);
    assert_eq!(viewport.top, 15);
    assert_eq!(viewport.right, 95);
    assert_eq!(viewport.bottom, 185);
    assert!(!viewport.clip);
}

#[test]
fn test_viewport_to_rect() {
    let viewport = Viewport::new(25, 35, 125, 235, true);
    let rect = viewport.to_rect();

    assert_eq!(rect.left, 25);
    assert_eq!(rect.top, 35);
    assert_eq!(rect.right, 125);
    assert_eq!(rect.bottom, 235);
}

#[test]
fn test_viewport_default() {
    let viewport = Viewport::default();

    assert_eq!(viewport.left, 0);
    assert_eq!(viewport.top, 0);
    assert_eq!(viewport.right, 639);
    assert_eq!(viewport.bottom, 479);
    assert!(viewport.clip);
}

#[test]
fn test_viewport_dimensions() {
    let viewport = Viewport::new(10, 20, 110, 120, true);

    assert_eq!(viewport.width(), 100);
    assert_eq!(viewport.height(), 100);

    // Test with negative dimensions (should return absolute value)
    let viewport_negative = Viewport::new(110, 120, 10, 20, true);
    assert_eq!(viewport_negative.width(), 100);
    assert_eq!(viewport_negative.height(), 100);
}

#[test]
fn test_viewport_contains_point() {
    let viewport = Viewport::new(10, 20, 100, 200, true);

    // Points inside
    assert!(viewport.contains_point(50, 100));
    assert!(viewport.contains_point(10, 20)); // Boundary (inclusive)
    assert!(viewport.contains_point(100, 200)); // Boundary (inclusive)

    // Points outside
    assert!(!viewport.contains_point(5, 100));
    assert!(!viewport.contains_point(105, 100));
    assert!(!viewport.contains_point(50, 15));
    assert!(!viewport.contains_point(50, 205));
}

#[test]
fn test_viewport_clip_coordinates() {
    let viewport_clipping = Viewport::new(10, 20, 100, 200, true);
    let viewport_no_clipping = Viewport::new(10, 20, 100, 200, false);

    // Test with clipping enabled
    assert_eq!(viewport_clipping.clip_coordinates(50, 100), Some((50, 100))); // Inside
    assert_eq!(viewport_clipping.clip_coordinates(5, 100), None); // Outside
    assert_eq!(viewport_clipping.clip_coordinates(10, 20), Some((10, 20))); // Boundary

    // Test with clipping disabled
    assert_eq!(
        viewport_no_clipping.clip_coordinates(50, 100),
        Some((50, 100))
    ); // Inside
    assert_eq!(
        viewport_no_clipping.clip_coordinates(5, 100),
        Some((5, 100))
    ); // Outside but allowed
    assert_eq!(
        viewport_no_clipping.clip_coordinates(-10, -20),
        Some((-10, -20))
    ); // Way outside but allowed
}

#[test]
fn test_coordinate_transformations() {
    let viewport = Viewport::new(50, 75, 200, 300, true);

    // Screen to viewport transformation
    let (vx, vy) = viewport.screen_to_viewport(100, 150);
    assert_eq!(vx, 50); // 100 - 50
    assert_eq!(vy, 75); // 150 - 75

    // Viewport to screen transformation
    let (sx, sy) = viewport.viewport_to_screen(25, 40);
    assert_eq!(sx, 75); // 25 + 50
    assert_eq!(sy, 115); // 40 + 75

    // Round trip test
    let original_x = 123;
    let original_y = 456;
    let (vx, vy) = viewport.screen_to_viewport(original_x, original_y);
    let (sx, sy) = viewport.viewport_to_screen(vx, vy);
    assert_eq!(sx, original_x);
    assert_eq!(sy, original_y);
}

#[test]
fn test_viewport_validity() {
    // Valid viewports
    assert!(Viewport::new(0, 0, 100, 100, true).is_valid());
    assert!(Viewport::new(-10, -20, 10, 20, true).is_valid());
    assert!(Viewport::new(50, 50, 50, 50, true).is_valid()); // Single point

    // Invalid viewports
    assert!(!Viewport::new(100, 0, 50, 100, true).is_valid()); // left > right
    assert!(!Viewport::new(0, 100, 100, 50, true).is_valid()); // top > bottom
    assert!(!Viewport::new(100, 100, 50, 50, true).is_valid()); // Both invalid
}

#[test]
fn test_viewport_normalization() {
    // Test normalizing inverted viewport
    let inverted = Viewport::new(100, 200, 50, 100, true);
    let normalized = inverted.normalize();

    assert_eq!(normalized.left, 50);
    assert_eq!(normalized.top, 100);
    assert_eq!(normalized.right, 100);
    assert_eq!(normalized.bottom, 200);
    assert!(normalized.clip);

    // Test normalizing already valid viewport (should be unchanged)
    let valid = Viewport::new(10, 20, 90, 180, false);
    let normalized_valid = valid.normalize();

    assert_eq!(normalized_valid.left, 10);
    assert_eq!(normalized_valid.top, 20);
    assert_eq!(normalized_valid.right, 90);
    assert_eq!(normalized_valid.bottom, 180);
    assert!(!normalized_valid.clip);

    // Test normalizing with only one dimension inverted
    let partially_inverted = Viewport::new(100, 20, 50, 180, true);
    let normalized_partial = partially_inverted.normalize();

    assert_eq!(normalized_partial.left, 50);
    assert_eq!(normalized_partial.top, 20);
    assert_eq!(normalized_partial.right, 100);
    assert_eq!(normalized_partial.bottom, 180);
}

#[test]
fn test_viewport_equality() {
    let viewport1 = Viewport::new(10, 20, 100, 200, true);
    let viewport2 = Viewport::new(10, 20, 100, 200, true);
    let viewport3 = Viewport::new(10, 20, 100, 200, false);
    let viewport4 = Viewport::new(15, 20, 100, 200, true);

    assert_eq!(viewport1, viewport2);
    assert_ne!(viewport1, viewport3); // Different clipping
    assert_ne!(viewport1, viewport4); // Different coordinates
}

#[test]
fn test_viewport_copy_and_clone() {
    let original = Viewport::new(25, 50, 125, 250, false);
    let copied = original; // Copy semantics for simple structs
    let cloned = original;

    assert_eq!(original, copied);
    assert_eq!(original, cloned);

    // Verify they're independent values (not references)
    assert_eq!(copied.left, 25);
    assert_eq!(cloned.left, 25);
}

#[test]
fn test_viewport_edge_cases() {
    // Zero-width viewport
    let zero_width = Viewport::new(50, 0, 50, 100, true);
    assert_eq!(zero_width.width(), 0);
    assert_eq!(zero_width.height(), 100);
    assert!(zero_width.is_valid());

    // Zero-height viewport
    let zero_height = Viewport::new(0, 50, 100, 50, true);
    assert_eq!(zero_height.width(), 100);
    assert_eq!(zero_height.height(), 0);
    assert!(zero_height.is_valid());

    // Single pixel viewport
    let single_pixel = Viewport::new(100, 100, 100, 100, true);
    assert_eq!(single_pixel.width(), 0);
    assert_eq!(single_pixel.height(), 0);
    assert!(single_pixel.is_valid());
    assert!(single_pixel.contains_point(100, 100));
    assert!(!single_pixel.contains_point(99, 100));
    assert!(!single_pixel.contains_point(101, 100));
}

#[test]
fn test_viewport_large_coordinates() {
    let large_viewport = Viewport::new(
        i32::MIN + 1000,
        i32::MIN + 2000,
        i32::MAX - 1000,
        i32::MAX - 2000,
        true,
    );

    assert!(large_viewport.is_valid());

    // Test width and height calculations with large numbers
    let width = large_viewport.width();
    let height = large_viewport.height();

    // These should not overflow
    assert!(width > 0);
    assert!(height > 0);

    // Test coordinate transformations with large numbers
    let (vx, vy) = large_viewport.screen_to_viewport(0, 0);
    let (sx, sy) = large_viewport.viewport_to_screen(vx, vy);

    assert_eq!(sx, 0);
    assert_eq!(sy, 0);
}
