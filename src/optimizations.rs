//! Code optimizations for BGI library
//!
//! This module contains zero-cost abstractions and optimizations
//! to reduce code duplication and improve performance.
//!
//! All operations route through the single global `GRAPHICS_STATE`
//! (see [`crate::graphics`]); there is no separate graphics context.

use crate::{Color, GraphResult};

/// Macro for graphics-initialization validation to eliminate code duplication.
///
/// Returns [`GraphResult::NotInitialized`] from the enclosing function when the
/// global graphics state has not been initialized via `initgraph`.
#[macro_export]
macro_rules! validate_graphics {
    () => {
        if !$crate::is_graphics_initialized() {
            return $crate::GraphResult::NotInitialized;
        }
    };
}

/// Optimized drawing operations with a single initialization check.
///
/// These mirror the global BGI drawing functions but return a [`GraphResult`]
/// so callers can detect the uninitialized case. The actual drawing routes
/// through the global `GRAPHICS_STATE` via [`crate::line`] and friends.
pub mod optimized_ctx {
    use super::*;

    /// Optimized line drawing with initialization validation.
    #[inline]
    pub fn line_ctx(x1: i32, y1: i32, x2: i32, y2: i32) -> GraphResult {
        crate::validate_graphics!();
        crate::line(x1, y1, x2, y2);
        GraphResult::Ok
    }

    /// Optimized circle drawing with initialization validation.
    #[inline]
    pub fn circle_ctx(x: i32, y: i32, radius: i32) -> GraphResult {
        crate::validate_graphics!();
        crate::circle(x, y, radius);
        GraphResult::Ok
    }

    /// Optimized rectangle drawing with initialization validation.
    #[inline]
    pub fn rectangle_ctx(left: i32, top: i32, right: i32, bottom: i32) -> GraphResult {
        crate::validate_graphics!();
        crate::rectangle(left, top, right, bottom);
        GraphResult::Ok
    }

    /// Optimized pixel write with initialization validation.
    #[inline]
    pub fn putpixel_ctx(x: i32, y: i32, color: Color) -> GraphResult {
        crate::validate_graphics!();
        crate::putpixel(x, y, color);
        GraphResult::Ok
    }

    /// Optimized pixel query with initialization validation.
    #[inline]
    pub fn getpixel_ctx(x: i32, y: i32) -> (Color, GraphResult) {
        if !crate::is_graphics_initialized() {
            return (Color::BLACK, GraphResult::NotInitialized);
        }
        (crate::getpixel(x, y), GraphResult::Ok)
    }
}

/// A single deferred drawing operation queued in a [`BatchDrawer`].
type BatchOp = Box<dyn FnOnce()>;

/// Batch operation support for improved performance.
///
/// Operations are queued, then executed against the global graphics state in a
/// single pass after one initialization check.
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
            .push(Box::new(move || crate::line(x1, y1, x2, y2)));
    }

    pub fn add_circle(&mut self, x: i32, y: i32, radius: i32) {
        self.operations
            .push(Box::new(move || crate::circle(x, y, radius)));
    }

    pub fn add_rectangle(&mut self, left: i32, top: i32, right: i32, bottom: i32) {
        self.operations
            .push(Box::new(move || crate::rectangle(left, top, right, bottom)));
    }

    pub fn add_pixel(&mut self, x: i32, y: i32, color: Color) {
        self.operations
            .push(Box::new(move || crate::putpixel(x, y, color)));
    }

    /// Execute all batched operations with a single initialization check.
    pub fn execute(self) -> GraphResult {
        crate::validate_graphics!();

        for operation in self.operations {
            operation();
        }

        GraphResult::Ok
    }

    /// Execute operations and return the count of executed operations.
    pub fn execute_with_count(self) -> (usize, GraphResult) {
        if !crate::is_graphics_initialized() {
            return (0, GraphResult::NotInitialized);
        }

        let mut count = 0;
        for operation in self.operations {
            operation();
            count += 1;
        }

        (count, GraphResult::Ok)
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
        pub fn draw(self) -> GraphResult {
            crate::validate_graphics!();

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

                crate::line(prev_x, prev_y, new_x, new_y);

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

    /// Flush all buffered operations to the global graphics state.
    pub fn flush(&mut self) -> GraphResult {
        crate::validate_graphics!();

        for &(x1, y1, x2, y2) in &self.line_buffer {
            crate::line(x1, y1, x2, y2);
        }

        for &(x, y, radius) in &self.circle_buffer {
            crate::circle(x, y, radius);
        }

        for &(x, y, color) in &self.pixel_buffer {
            crate::putpixel(x, y, color);
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
