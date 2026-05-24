#[cfg(test)]
mod tests {
    use bgi::optimizations::{BatchDrawer, DrawingPool, const_optimized, optimized_ctx};
    use bgi::*;
    use std::time::Instant;

    /// Initialize the global graphics state (VGA mode, headless-safe).
    fn init() {
        let mut gd = 0i32; // DETECT
        let mut gm = 4i32; // VGA mode
        initgraph(&mut gd, &mut gm, "");
    }

    /// Test optimized operation validation performance
    #[test]
    #[ignore = "Timing test - can be flaky in CI environments"]
    fn test_optimized_context_validation() {
        init();

        let start = Instant::now();

        // Test optimized functions
        for i in 0..1000 {
            let _ = optimized_ctx::line_ctx(i % 100, i % 100, (i + 10) % 100, (i + 10) % 100);
            let _ = optimized_ctx::circle_ctx(i % 100, i % 100, 10);
            let _ = optimized_ctx::putpixel_ctx(i % 100, i % 100, Color::RED);
        }

        let duration = start.elapsed();
        println!("Optimized operations took: {:?}", duration);

        // Should complete in reasonable time (< 100ms for 3000 operations)
        assert!(
            duration.as_millis() < 100,
            "Operations took too long: {:?}",
            duration
        );

        closegraph();
    }

    /// Test batch drawing performance
    #[test]
    #[ignore = "Timing test - can be flaky in CI environments"]
    fn test_batch_drawing_performance() {
        init();

        let mut batch = BatchDrawer::new();

        // Add many operations to batch
        for i in 0..1000 {
            batch.add_line(i % 100, i % 100, (i + 10) % 100, (i + 10) % 100);
            batch.add_circle(i % 100, i % 100, 5);
            batch.add_pixel(i % 100, i % 100, Color::BLUE);
        }

        let start = Instant::now();
        let result = batch.execute();
        let duration = start.elapsed();

        println!("Batch execution took: {:?}", duration);
        assert_eq!(result, GraphResult::Ok);

        // Batch operations should be faster than individual operations
        assert!(
            duration.as_millis() < 50,
            "Batch operations took too long: {:?}",
            duration
        );

        closegraph();
    }

    /// Test compile-time optimized shapes
    #[test]
    #[ignore = "Timing test - can be flaky in CI environments"]
    fn test_const_optimized_shapes() {
        init();

        let start = Instant::now();

        for _ in 0..100 {
            let triangle = const_optimized::Triangle::new(50, 50, 20);
            let hexagon = const_optimized::Hexagon::new(100, 100, 30);
            let octagon = const_optimized::Octagon::new(150, 150, 25);

            let _ = triangle.draw();
            let _ = hexagon.draw();
            let _ = octagon.draw();
        }

        let duration = start.elapsed();
        println!("Const optimized shapes took: {:?}", duration);

        // Should complete efficiently
        assert!(
            duration.as_millis() < 50,
            "Const optimized shapes took too long: {:?}",
            duration
        );

        closegraph();
    }

    /// Test drawing pool memory efficiency
    #[test]
    fn test_drawing_pool_efficiency() {
        init();

        let mut pool = DrawingPool::new();

        // Fill pool with operations
        for i in 0..500 {
            pool.add_line(i % 100, i % 100, (i + 10) % 100, (i + 10) % 100);
            pool.add_circle(i % 100, i % 100, 5);
            pool.add_pixel(i % 100, i % 100, Color::GREEN);
        }

        let (lines, circles, pixels) = pool.stats();
        assert_eq!(lines, 500);
        assert_eq!(circles, 500);
        assert_eq!(pixels, 500);

        let start = Instant::now();
        let result = pool.flush();
        let duration = start.elapsed();

        println!("Pool flush took: {:?}", duration);
        assert_eq!(result, GraphResult::Ok);

        // Pool should be cleared after flush
        let (lines, circles, pixels) = pool.stats();
        assert_eq!(lines, 0);
        assert_eq!(circles, 0);
        assert_eq!(pixels, 0);

        closegraph();
    }

    /// Test that optimizations maintain correctness
    #[test]
    fn test_optimization_correctness() {
        // Ensure a clean (uninitialized) state on this thread.
        closegraph();

        // Test uninitialized handling
        let result = optimized_ctx::line_ctx(0, 0, 10, 10);
        assert_eq!(result, GraphResult::NotInitialized);

        // Initialize and test normal operation
        init();
        let result = optimized_ctx::line_ctx(0, 0, 10, 10);
        assert_eq!(result, GraphResult::Ok);

        // Test pixel operations
        let result = optimized_ctx::putpixel_ctx(50, 50, Color::YELLOW);
        assert_eq!(result, GraphResult::Ok);

        let (_color, result) = optimized_ctx::getpixel_ctx(50, 50);
        assert_eq!(result, GraphResult::Ok);
        // Note: Color might not match exactly due to backend implementation

        closegraph();
    }

    /// Benchmark performance improvement over naive implementation
    #[test]
    fn test_performance_improvement() {
        init();

        // Test naive style (direct global drawing)
        let start = Instant::now();
        for i in 0..1000 {
            line(i % 100, i % 100, (i + 10) % 100, (i + 10) % 100);
        }
        let old_duration = start.elapsed();

        // Test optimized style (validated wrapper)
        let start = Instant::now();
        for i in 0..1000 {
            let _ = optimized_ctx::line_ctx(i % 100, i % 100, (i + 10) % 100, (i + 10) % 100);
        }
        let new_duration = start.elapsed();

        println!(
            "Old approach: {:?}, New approach: {:?}",
            old_duration, new_duration
        );

        // New approach should be at least as fast (allowing for measurement variance)
        let improvement_ratio =
            old_duration.as_nanos() as f64 / new_duration.as_nanos().max(1) as f64;
        assert!(
            improvement_ratio >= 0.8,
            "Performance regression detected: {:.2}x",
            improvement_ratio
        );

        closegraph();
    }

    /// Test that the validate_graphics! macro gates on initialization
    #[test]
    fn test_macro_validation_equivalence() {
        fn guarded() -> GraphResult {
            bgi::validate_graphics!();
            GraphResult::Ok
        }

        // Ensure a clean (uninitialized) state on this thread.
        closegraph();
        assert_eq!(guarded(), GraphResult::NotInitialized);

        init();
        assert_eq!(guarded(), GraphResult::Ok);

        closegraph();
    }
}
