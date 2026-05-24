//! Code optimizations for BGI library
//!
//! This module contains zero-cost abstractions and optimizations
//! to reduce code duplication and improve performance.

use crate::{Color, GraphResult, GraphicsContext};

/// Macro for context validation to eliminate code duplication
#[macro_export]
macro_rules! validate_context {
    ($context:expr) => {
        if !$context.is_initialized() {
            return GraphResult::NotInitialized;
        }
    };
}

/// Helper trait for zero-cost abstractions over graphics operations
pub trait GraphicsOperation<T> {
    fn execute(self, context: &mut GraphicsContext) -> GraphResult;
}

/// Zero-cost abstraction for drawing operations that don't return values
pub struct DrawingOp<F>
where
    F: FnOnce(&mut GraphicsContext) -> Result<(), crate::error::BgiError>,
{
    operation: F,
}

impl<F> DrawingOp<F>
where
    F: FnOnce(&mut GraphicsContext) -> Result<(), crate::error::BgiError>,
{
    pub fn new(operation: F) -> Self {
        Self { operation }
    }
}

impl<F> GraphicsOperation<()> for DrawingOp<F>
where
    F: FnOnce(&mut GraphicsContext) -> Result<(), crate::error::BgiError>,
{
    fn execute(self, context: &mut GraphicsContext) -> GraphResult {
        validate_context!(context);

        match (self.operation)(context) {
            Ok(()) => GraphResult::Ok,
            Err(_) => GraphResult::InvalidDriver,
        }
    }
}

/// Zero-cost abstraction for query operations that return values
pub struct QueryOp<F, T>
where
    F: FnOnce(&GraphicsContext) -> Result<T, crate::error::BgiError>,
{
    operation: F,
    _phantom: std::marker::PhantomData<T>,
}

impl<F, T> QueryOp<F, T>
where
    F: FnOnce(&GraphicsContext) -> Result<T, crate::error::BgiError>,
{
    pub fn new(operation: F) -> Self {
        Self {
            operation,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<F, T> GraphicsOperation<T> for QueryOp<F, T>
where
    F: FnOnce(&GraphicsContext) -> Result<T, crate::error::BgiError>,
    T: Default,
{
    fn execute(self, context: &mut GraphicsContext) -> GraphResult {
        validate_context!(context);

        match (self.operation)(context) {
            Ok(_) => GraphResult::Ok,
            Err(_) => GraphResult::InvalidDriver,
        }
    }
}

/// Optimized context operations that reduce function call overhead
pub mod optimized_ctx {
    use super::*;

    /// Optimized line drawing with context validation
    #[inline]
    pub fn line_ctx(
        context: &mut GraphicsContext,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    ) -> GraphResult {
        DrawingOp::new(|ctx| ctx.draw_line(x1, y1, x2, y2)).execute(context)
    }

    /// Optimized circle drawing with context validation
    #[inline]
    pub fn circle_ctx(context: &mut GraphicsContext, x: i32, y: i32, radius: i32) -> GraphResult {
        DrawingOp::new(|ctx| ctx.draw_circle(x, y, radius)).execute(context)
    }

    /// Optimized rectangle drawing with context validation
    #[inline]
    pub fn rectangle_ctx(
        context: &mut GraphicsContext,
        left: i32,
        top: i32,
        right: i32,
        bottom: i32,
    ) -> GraphResult {
        DrawingOp::new(|ctx| ctx.draw_rectangle(left, top, right, bottom)).execute(context)
    }

    /// Optimized pixel operations with context validation
    #[inline]
    pub fn putpixel_ctx(
        context: &mut GraphicsContext,
        x: i32,
        y: i32,
        color: Color,
    ) -> GraphResult {
        DrawingOp::new(|ctx| ctx.put_pixel(x, y, color)).execute(context)
    }

    /// Optimized pixel query with context validation
    #[inline]
    pub fn getpixel_ctx(context: &GraphicsContext, x: i32, y: i32) -> (Color, GraphResult) {
        if !context.is_initialized() {
            return (Color::BLACK, GraphResult::NotInitialized);
        }

        match context.get_pixel(x, y) {
            Ok(color) => (color, GraphResult::Ok),
            Err(_) => (Color::BLACK, GraphResult::InvalidDriver),
        }
    }
}

/// A single deferred drawing operation queued in a [`BatchDrawer`].
type BatchOp = Box<dyn FnOnce(&mut GraphicsContext) -> Result<(), crate::error::BgiError>>;

/// Batch operation support for improved performance
pub struct BatchDrawer {
    operations: Vec<BatchOp>,
}

impl BatchDrawer {
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }

    pub fn add_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        self.operations
            .push(Box::new(move |ctx| ctx.draw_line(x1, y1, x2, y2)));
    }

    pub fn add_circle(&mut self, x: i32, y: i32, radius: i32) {
        self.operations
            .push(Box::new(move |ctx| ctx.draw_circle(x, y, radius)));
    }

    pub fn add_rectangle(&mut self, left: i32, top: i32, right: i32, bottom: i32) {
        self.operations.push(Box::new(move |ctx| {
            ctx.draw_rectangle(left, top, right, bottom)
        }));
    }

    pub fn add_pixel(&mut self, x: i32, y: i32, color: Color) {
        self.operations
            .push(Box::new(move |ctx| ctx.put_pixel(x, y, color)));
    }

    /// Execute all batched operations with single context validation
    pub fn execute(self, context: &mut GraphicsContext) -> GraphResult {
        crate::validate_context!(context);

        for operation in self.operations {
            if operation(context).is_err() {
                return GraphResult::InvalidDriver;
            }
        }

        GraphResult::Ok
    }

    /// Execute operations and return count of successful operations
    pub fn execute_with_count(self, context: &mut GraphicsContext) -> (usize, GraphResult) {
        if !context.is_initialized() {
            return (0, GraphResult::NotInitialized);
        }

        let mut success_count = 0;

        for operation in self.operations {
            if operation(context).is_ok() {
                success_count += 1;
            } else {
                return (success_count, GraphResult::InvalidDriver);
            }
        }

        (success_count, GraphResult::Ok)
    }
}

impl Default for BatchDrawer {
    fn default() -> Self {
        Self::new()
    }
}

/// Compile-time optimizations using const generics
pub mod const_optimized {
    use super::*;

    /// Compile-time optimized shape drawing for known patterns
    pub struct PatternShape<const SIDES: usize> {
        center_x: i32,
        center_y: i32,
        radius: i32,
    }

    impl<const SIDES: usize> PatternShape<SIDES> {
        pub const fn new(center_x: i32, center_y: i32, radius: i32) -> Self {
            Self {
                center_x,
                center_y,
                radius,
            }
        }

        /// Draw regular polygon with compile-time known number of sides
        pub fn draw(self, context: &mut GraphicsContext) -> GraphResult {
            crate::validate_context!(context);

            if SIDES < 3 {
                return GraphResult::InvalidDriver;
            }

            let angle_step = 360.0 / SIDES as f64;
            let mut prev_x = self.center_x + self.radius;
            let mut prev_y = self.center_y;

            for i in 1..=SIDES {
                let angle = (i as f64 * angle_step).to_radians();
                let new_x = self.center_x + (self.radius as f64 * angle.cos()) as i32;
                let new_y = self.center_y + (self.radius as f64 * angle.sin()) as i32;

                if context.draw_line(prev_x, prev_y, new_x, new_y).is_err() {
                    return GraphResult::InvalidDriver;
                }

                prev_x = new_x;
                prev_y = new_y;
            }

            GraphResult::Ok
        }
    }

    /// Type aliases for common shapes with compile-time optimization
    pub type Triangle = PatternShape<3>;
    pub type Square = PatternShape<4>;
    pub type Pentagon = PatternShape<5>;
    pub type Hexagon = PatternShape<6>;
    pub type Octagon = PatternShape<8>;
}

/// Memory pool for reducing allocations in drawing operations
pub struct DrawingPool {
    line_buffer: Vec<(i32, i32, i32, i32)>,
    circle_buffer: Vec<(i32, i32, i32)>,
    pixel_buffer: Vec<(i32, i32, Color)>,
}

impl DrawingPool {
    pub fn new() -> Self {
        Self {
            line_buffer: Vec::with_capacity(1000), // Pre-allocate for common case
            circle_buffer: Vec::with_capacity(100),
            pixel_buffer: Vec::with_capacity(1000),
        }
    }

    pub fn clear(&mut self) {
        self.line_buffer.clear();
        self.circle_buffer.clear();
        self.pixel_buffer.clear();
    }

    pub fn add_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        self.line_buffer.push((x1, y1, x2, y2));
    }

    pub fn add_circle(&mut self, x: i32, y: i32, radius: i32) {
        self.circle_buffer.push((x, y, radius));
    }

    pub fn add_pixel(&mut self, x: i32, y: i32, color: Color) {
        self.pixel_buffer.push((x, y, color));
    }

    /// Flush all buffered operations to the graphics context
    pub fn flush(&mut self, context: &mut GraphicsContext) -> GraphResult {
        crate::validate_context!(context);

        // Draw all lines
        for &(x1, y1, x2, y2) in &self.line_buffer {
            if context.draw_line(x1, y1, x2, y2).is_err() {
                return GraphResult::InvalidDriver;
            }
        }

        // Draw all circles
        for &(x, y, radius) in &self.circle_buffer {
            if context.draw_circle(x, y, radius).is_err() {
                return GraphResult::InvalidDriver;
            }
        }

        // Draw all pixels
        for &(x, y, color) in &self.pixel_buffer {
            if context.put_pixel(x, y, color).is_err() {
                return GraphResult::InvalidDriver;
            }
        }

        // Clear buffers for reuse
        self.clear();

        GraphResult::Ok
    }

    /// Get statistics about buffer usage
    pub fn stats(&self) -> (usize, usize, usize) {
        (
            self.line_buffer.len(),
            self.circle_buffer.len(),
            self.pixel_buffer.len(),
        )
    }
}

impl Default for DrawingPool {
    fn default() -> Self {
        Self::new()
    }
}
