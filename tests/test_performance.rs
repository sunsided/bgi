//! Performance validation test for BGI library
//! Validates that typical drawing operations can achieve 30 FPS target

use bgi::*;
use std::time::{Duration, Instant};

#[test]
fn test_30_fps_performance_target() {
    // Initialize graphics
    let mut driver = 0;
    let mut mode = 0;
    initgraph(&mut driver, &mut mode, "");

    // Check if graphics was initialized successfully
    if graphresult() != GraphResult::Ok {
        eprintln!("Warning: Graphics initialization failed in performance test");
        return; // Skip test if graphics can't initialize
    }

    // Test parameters
    const TARGET_FPS: f64 = 30.0;
    const FRAME_DURATION: Duration = Duration::from_nanos((1_000_000_000.0 / TARGET_FPS) as u64);
    const TEST_FRAMES: usize = 100; // Test 100 frames

    println!("Performance Test: Targeting {} FPS ({:.2}ms per frame)",
             TARGET_FPS, FRAME_DURATION.as_secs_f64() * 1000.0);

    let mut frame_times = Vec::with_capacity(TEST_FRAMES);
    let mut successful_frames = 0;

    for frame in 0..TEST_FRAMES {
        let frame_start = Instant::now();

        // Perform typical drawing operations
        perform_typical_drawing_operations(frame);

        let frame_duration = frame_start.elapsed();
        frame_times.push(frame_duration);

        if frame_duration <= FRAME_DURATION {
            successful_frames += 1;
        }

        // Small delay to avoid overwhelming the system
        if frame_duration < FRAME_DURATION {
            let sleep_time = FRAME_DURATION - frame_duration;
            if sleep_time > Duration::from_millis(1) {
                std::thread::sleep(sleep_time);
            }
        }
    }

    // Calculate statistics
    let total_time: Duration = frame_times.iter().sum();
    let average_frame_time = total_time / TEST_FRAMES as u32;
    let average_fps = 1.0 / average_frame_time.as_secs_f64();

    let min_frame_time = *frame_times.iter().min().unwrap();
    let max_frame_time = *frame_times.iter().max().unwrap();
    let max_fps = 1.0 / min_frame_time.as_secs_f64();
    let min_fps = 1.0 / max_frame_time.as_secs_f64();

    let success_rate = (successful_frames as f64 / TEST_FRAMES as f64) * 100.0;

    // Print detailed results
    println!("Performance Results:");
    println!("  Frames tested: {}", TEST_FRAMES);
    println!("  Average FPS: {:.2}", average_fps);
    println!("  Min FPS: {:.2}", min_fps);
    println!("  Max FPS: {:.2}", max_fps);
    println!("  Average frame time: {:.2}ms", average_frame_time.as_secs_f64() * 1000.0);
    println!("  Min frame time: {:.2}ms", min_frame_time.as_secs_f64() * 1000.0);
    println!("  Max frame time: {:.2}ms", max_frame_time.as_secs_f64() * 1000.0);
    println!("  Frames meeting 30 FPS target: {}/{} ({:.1}%)",
             successful_frames, TEST_FRAMES, success_rate);

    closegraph();

    // Performance requirements:
    // - Average FPS should be >= 30
    // - At least 80% of frames should meet the 30 FPS target
    assert!(average_fps >= TARGET_FPS,
        "Average FPS ({:.2}) is below target ({:.2})", average_fps, TARGET_FPS);

    assert!(success_rate >= 80.0,
        "Only {:.1}% of frames met 30 FPS target (need >= 80%)", success_rate);

    println!("✓ Performance validation passed!");
}

/// Perform typical drawing operations that represent real-world usage
fn perform_typical_drawing_operations(frame: usize) {
    // Clear screen
    cleardevice();

    // Set various colors
    setcolor(Color::WHITE);
    setbkcolor(Color::BLACK);

    // Draw lines (common operation)
    for i in 0..10 {
        let x1 = (i * 50) % 600;
        let y1 = (i * 30) % 400;
        let x2 = ((i + 1) * 50) % 600;
        let y2 = ((i + 1) * 30) % 400;
        line(x1, y1, x2, y2);
    }

    // Draw rectangles
    for i in 0..5 {
        let x = (i * 100) % 500;
        let y = (i * 60) % 300;
        rectangle(x, y, x + 80, y + 50);
    }

    // Draw circles
    for i in 0..8 {
        let x = 100 + (i * 60) % 400;
        let y = 100 + (i * 40) % 200;
        let radius = 20 + (i * 5);
        circle(x, y, radius);
    }

    // Draw filled shapes (more expensive)
    setcolor(Color::RED);
    setfillstyle(SOLID_FILL, Color::RED);
    bar(10, 10, 50, 50);
    
    setcolor(Color::GREEN);
    setfillstyle(SOLID_FILL, Color::GREEN);
    fillellipse(300, 200, 30, 20);
    
    // Draw text (often used for debugging/UI)
    setcolor(Color::YELLOW);
    outtextxy(10, 400, &format!("Frame: {}", frame));
    outtextxy(10, 420, "Performance Test");

    // Draw pixels (basic operation)
    setcolor(Color::CYAN);
    for i in 0..50 {
        let x = (frame + i) % 640;
        let y = (frame * 2 + i) % 480;
        putpixel(x as i32, y as i32, Color::CYAN);
    }

    // Simulate reading pixels (testing backend roundtrip)
    for i in 0..10 {
        let x = (frame + i * 10) % 640;
        let y = (frame + i * 10) % 480;
        let _color = getpixel(x as i32, y as i32);
    }
}

#[test]
fn test_individual_operation_performance() {
    // Test individual operations to identify bottlenecks
    let mut driver = 0;
    let mut mode = 0;
    initgraph(&mut driver, &mut mode, "");

    if graphresult() != GraphResult::Ok {
        eprintln!("Warning: Graphics initialization failed in individual performance test");
        return;
    }

    const ITERATIONS: usize = 1000;

    // Test line drawing performance
    let start = Instant::now();
    for i in 0..ITERATIONS {
        let x1 = (i % 600) as i32;
        let y1 = (i % 400) as i32;
        let x2 = ((i + 100) % 600) as i32;
        let y2 = ((i + 100) % 400) as i32;
        line(x1, y1, x2, y2);
    }
    let line_time = start.elapsed();
    let lines_per_second = ITERATIONS as f64 / line_time.as_secs_f64();

    // Test circle drawing performance
    let start = Instant::now();
    for i in 0..ITERATIONS {
        let x = ((i % 50) * 10 + 50) as i32;
        let y = ((i % 40) * 10 + 50) as i32;
        let radius = (i % 50 + 10) as i32;
        circle(x, y, radius);
    }
    let circle_time = start.elapsed();
    let circles_per_second = ITERATIONS as f64 / circle_time.as_secs_f64();

    // Test pixel operations
    let start = Instant::now();
    for i in 0..ITERATIONS {
        let x = (i % 640) as i32;
        let y = (i % 480) as i32;
        putpixel(x, y, Color::WHITE);
    }
    let pixel_time = start.elapsed();
    let pixels_per_second = ITERATIONS as f64 / pixel_time.as_secs_f64();

    // Test text operations
    let start = Instant::now();
    for i in 0..100 { // Fewer iterations for text
        let x = (i % 60) * 10;
        let y = (i % 40) * 10;
        outtextxy(x, y, &format!("Text {}", i));
    }
    let text_time = start.elapsed();
    let text_per_second = 100.0 / text_time.as_secs_f64();

    closegraph();

    // Print performance metrics
    println!("Individual Operation Performance:");
    println!("  Lines per second: {:.0}", lines_per_second);
    println!("  Circles per second: {:.0}", circles_per_second);
    println!("  Pixels per second: {:.0}", pixels_per_second);
    println!("  Text operations per second: {:.0}", text_per_second);

    // Minimum performance requirements for 30 FPS with typical frame content:
    // - Lines: At least 300/second (10 lines per frame * 30 FPS)
    // - Circles: At least 240/second (8 circles per frame * 30 FPS)
    // - Pixels: At least 1500/second (50 pixels per frame * 30 FPS)
    // - Text: At least 60/second (2 text operations per frame * 30 FPS)

    assert!(lines_per_second >= 300.0,
        "Line drawing too slow: {:.0}/sec (need >= 300)", lines_per_second);

    assert!(circles_per_second >= 240.0,
        "Circle drawing too slow: {:.0}/sec (need >= 240)", circles_per_second);

    assert!(pixels_per_second >= 1500.0,
        "Pixel operations too slow: {:.0}/sec (need >= 1500)", pixels_per_second);

    assert!(text_per_second >= 60.0,
        "Text operations too slow: {:.0}/sec (need >= 60)", text_per_second);

    println!("✓ Individual operation performance validation passed!");
}

#[test]
fn test_memory_performance() {
    // Test memory-intensive operations for performance
    let mut driver = 0;
    let mut mode = 0;
    initgraph(&mut driver, &mut mode, "");

    if graphresult() != GraphResult::Ok {
        eprintln!("Warning: Graphics initialization failed in memory performance test");
        return;
    }

    // Test large buffer operations
    let start = Instant::now();

    // Fill entire screen multiple times
    for color_idx in 0..5 {
        let color = match color_idx {
            0 => Color::RED,
            1 => Color::GREEN,
            2 => Color::BLUE,
            3 => Color::YELLOW,
            _ => Color::WHITE,
        };

        setfillstyle(SOLID_FILL, color);
        bar(0, 0, 639, 479); // Fill entire screen
        cleardevice(); // Clear for next iteration
    }

    let memory_ops_time = start.elapsed();
    let screen_fills_per_second = 5.0 / memory_ops_time.as_secs_f64();

    closegraph();

    println!("Memory Performance:");
    println!("  Full screen fills per second: {:.1}", screen_fills_per_second);

    // Should be able to fill screen at least 30 times per second for smooth animation
    assert!(screen_fills_per_second >= 30.0,
        "Screen fill too slow: {:.1}/sec (need >= 30)", screen_fills_per_second);

    println!("✓ Memory performance validation passed!");
}

#[test]
fn test_viewport_performance() {
    // Test performance with different viewport settings
    let mut driver = 0;
    let mut mode = 0;
    initgraph(&mut driver, &mut mode, "");

    if graphresult() != GraphResult::Ok {
        eprintln!("Warning: Graphics initialization failed in viewport performance test");
        return;
    }

    const OPERATIONS: usize = 500;

    // Test with clipping enabled (default)
    setviewport(100, 100, 500, 300, true);

    let start = Instant::now();
    for i in 0..OPERATIONS {
        let x1 = (i % 200) as i32;
        let y1 = (i % 150) as i32;
        let x2 = ((i + 50) % 200) as i32;
        let y2 = ((i + 50) % 150) as i32;
        line(x1, y1, x2, y2);
    }
    let clipped_time = start.elapsed();

    // Reset to full screen
    setviewport(0, 0, 639, 479, false);

    let start = Instant::now();
    for i in 0..OPERATIONS {
        let x1 = (i % 600) as i32;
        let y1 = (i % 400) as i32;
        let x2 = ((i + 50) % 600) as i32;
        let y2 = ((i + 50) % 400) as i32;
        line(x1, y1, x2, y2);
    }
    let full_screen_time = start.elapsed();

    closegraph();

    let clipped_ops_per_sec = OPERATIONS as f64 / clipped_time.as_secs_f64();
    let full_screen_ops_per_sec = OPERATIONS as f64 / full_screen_time.as_secs_f64();

    println!("Viewport Performance:");
    println!("  Clipped operations per second: {:.0}", clipped_ops_per_sec);
    println!("  Full screen operations per second: {:.0}", full_screen_ops_per_sec);

    // Both should be fast enough for 30 FPS
    assert!(clipped_ops_per_sec >= 300.0,
        "Clipped drawing too slow: {:.0}/sec", clipped_ops_per_sec);

    assert!(full_screen_ops_per_sec >= 300.0,
        "Full screen drawing too slow: {:.0}/sec", full_screen_ops_per_sec);

    println!("✓ Viewport performance validation passed!");
}
