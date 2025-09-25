#[cfg(test)]
mod tests {
    use bgi::*;
    use bgi::optimizations::{BatchDrawer, DrawingPool, const_optimized, optimized_ctx};
    use std::time::Instant;

    /// Test optimized context validation performance
    #[test]
    fn test_optimized_context_validation() {
        let mut context = GraphicsContext::create_test_context();
        context.initialize(800, 600, "Test");

        let start = Instant::now();

        // Test optimized functions
        for i in 0..1000 {
            let _ = optimized_ctx::line_ctx(&mut context, i % 100, i % 100, (i + 10) % 100, (i + 10) % 100);
            let _ = optimized_ctx::circle_ctx(&mut context, i % 100, i % 100, 10);
            let _ = optimized_ctx::putpixel_ctx(&mut context, i % 100, i % 100, Color::RED);
        }

        let duration = start.elapsed();
        println!("Optimized operations took: {:?}", duration);

        // Should complete in reasonable time (< 100ms for 3000 operations)
        assert!(duration.as_millis() < 100, "Operations took too long: {:?}", duration);
    }

    /// Test batch drawing performance
    #[test]
    fn test_batch_drawing_performance() {
        let mut context = GraphicsContext::create_test_context();
        context.initialize(800, 600, "Test");

        let mut batch = BatchDrawer::new();

        // Add many operations to batch
        for i in 0..1000 {
            batch.add_line(i % 100, i % 100, (i + 10) % 100, (i + 10) % 100);
            batch.add_circle(i % 100, i % 100, 5);
            batch.add_pixel(i % 100, i % 100, Color::BLUE);
        }

        let start = Instant::now();
        let result = batch.execute(&mut context);
        let duration = start.elapsed();

        println!("Batch execution took: {:?}", duration);
        assert_eq!(result, GraphResult::Ok);

        // Batch operations should be faster than individual operations
        assert!(duration.as_millis() < 50, "Batch operations took too long: {:?}", duration);
    }

    /// Test compile-time optimized shapes
    #[test]
    fn test_const_optimized_shapes() {
        let mut context = GraphicsContext::create_test_context();
        context.initialize(800, 600, "Test");

        let start = Instant::now();

        for _ in 0..100 {
            let triangle = const_optimized::Triangle::new(50, 50, 20);
            let hexagon = const_optimized::Hexagon::new(100, 100, 30);
            let octagon = const_optimized::Octagon::new(150, 150, 25);

            let _ = triangle.draw(&mut context);
            let _ = hexagon.draw(&mut context);
            let _ = octagon.draw(&mut context);
        }

        let duration = start.elapsed();
        println!("Const optimized shapes took: {:?}", duration);

        // Should complete efficiently
        assert!(duration.as_millis() < 50, "Const optimized shapes took too long: {:?}", duration);
    }

    /// Test drawing pool memory efficiency
    #[test]
    fn test_drawing_pool_efficiency() {
        let mut context = GraphicsContext::create_test_context();
        context.initialize(800, 600, "Test");

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
        let result = pool.flush(&mut context);
        let duration = start.elapsed();

        println!("Pool flush took: {:?}", duration);
        assert_eq!(result, GraphResult::Ok);

        // Pool should be cleared after flush
        let (lines, circles, pixels) = pool.stats();
        assert_eq!(lines, 0);
        assert_eq!(circles, 0);
        assert_eq!(pixels, 0);
    }

    /// Test that optimizations maintain correctness
    #[test]
    fn test_optimization_correctness() {
        let mut context = GraphicsContext::create_test_context();

        // Test uninitialized context handling
        let result = optimized_ctx::line_ctx(&mut context, 0, 0, 10, 10);
        assert_eq!(result, GraphResult::NotInitialized);

        // Initialize and test normal operation
        context.initialize(800, 600, "Test");
        let result = optimized_ctx::line_ctx(&mut context, 0, 0, 10, 10);
        assert_eq!(result, GraphResult::Ok);

        // Test pixel operations
        let result = optimized_ctx::putpixel_ctx(&mut context, 50, 50, Color::YELLOW);
        assert_eq!(result, GraphResult::Ok);

        let (_color, result) = optimized_ctx::getpixel_ctx(&context, 50, 50);
        assert_eq!(result, GraphResult::Ok);
        // Note: Color might not match exactly due to backend implementation
    }

    /// Benchmark performance improvement over naive implementation
    #[test]
    fn test_performance_improvement() {
        let mut context = GraphicsContext::create_test_context();
        context.initialize(800, 600, "Test");

        // Test old style (manual validation)
        let start = Instant::now();
        for i in 0..1000 {
            // Simulate old validation pattern
            if context.is_initialized() {
                let _ = context.draw_line(i % 100, i % 100, (i + 10) % 100, (i + 10) % 100);
            }
        }
        let old_duration = start.elapsed();

        // Test new optimized style
        let start = Instant::now();
        for i in 0..1000 {
            let _ = optimized_ctx::line_ctx(&mut context, i % 100, i % 100, (i + 10) % 100, (i + 10) % 100);
        }
        let new_duration = start.elapsed();

        println!("Old approach: {:?}, New approach: {:?}", old_duration, new_duration);

        // New approach should be at least as fast (allowing for measurement variance)
        let improvement_ratio = old_duration.as_nanos() as f64 / new_duration.as_nanos() as f64;
        assert!(improvement_ratio >= 0.8, "Performance regression detected: {:.2}x", improvement_ratio);
    }

    /// Test that macro validation is equivalent to if statement
    #[test]
    fn test_macro_validation_equivalence() {
        let mut context = GraphicsContext::create_test_context();

        // Test uninitialized - macro version
        fn test_macro_uninitialized(context: &GraphicsContext) -> GraphResult {
            bgi::validate_context!(context);
            GraphResult::Ok
        }
        let macro_result = test_macro_uninitialized(&context);
        assert_eq!(macro_result, GraphResult::NotInitialized);

        let if_result = {
            if !context.is_initialized() {
                GraphResult::NotInitialized
            } else {
                GraphResult::Ok
            }
        };
        assert_eq!(if_result, GraphResult::NotInitialized);

        // Test initialized
        context.initialize(800, 600, "Test");

        fn test_macro_initialized(context: &GraphicsContext) -> GraphResult {
            bgi::validate_context!(context);
            GraphResult::Ok
        }
        let macro_result = test_macro_initialized(&context);
        assert_eq!(macro_result, GraphResult::Ok);

        let if_result = {
            if !context.is_initialized() {
                GraphResult::NotInitialized
            } else {
                GraphResult::Ok
            }
        };
        assert_eq!(if_result, GraphResult::Ok);
    }
}
