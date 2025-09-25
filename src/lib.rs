//! # BGI - Borland Graphics Interface for Rust
//!
//! A modern Rust port of the classic Borland Graphics Interface (BGI) with extensible backends.

#![allow(missing_docs, dead_code, unused_imports, unused_variables)]

use std::cell::RefCell;

// Core module declarations - only include working modules for now
pub mod constants;
pub mod types;
pub mod color;
pub mod error;
// pub mod input; // Temporarily disabled due to byte literal issues

// Re-export public API
pub use error::{BgiError, BgiResult};
pub use types::{GraphicsMode, GraphicsDriver, GraphResult, MouseState, BgiTextSettings};
pub use types::colors::*;
pub use color::{Color, RgbColor};
pub use constants::*;

// Include backend module
pub mod backend;
pub mod window;
pub mod line;
pub mod viewport;

// Phase 3.3 entities - make public for unit testing
pub mod drawing_state;
pub mod font_settings;
pub mod input_event;
pub mod window_state;

pub use drawing_state::*;
pub use font_settings::*;
pub use input_event::*;
pub use window_state::*;

// Phase 3.3 API modules
mod graphics;
mod palette;
mod image;
mod shapes;

// Optimizations module for zero-cost abstractions
pub mod optimizations;

pub use graphics::*;
pub use palette::*;
pub use image::*;
pub use shapes::*;
pub use optimizations::{BatchDrawer, DrawingPool, const_optimized};

use backend::{Backend, create_pixel_buffer_backend, DrawCommand};
use window::WindowId;
use line::{LineStyle as LineStyleInternal, draw_thick_line, draw_thick_circle, draw_rectangle_lines, draw_ellipse_arc};
use optimizations::optimized_ctx;

// Graphics context with backend support
pub struct GraphicsContext {
    pub initialized: bool,
    pub mode: GraphicsMode,
    backend: Box<dyn Backend>,
    current_window: Option<WindowId>,
    current_color: Color,
    line_style: LineStyleInternal,
    draw_mode: i32,
}

impl GraphicsContext {
    pub fn new(mode: GraphicsMode) -> Result<Self, BgiError> {
        let mut backend = create_pixel_buffer_backend()?;
        backend.init()?;
        
        let resolution = mode.resolution();
        let window_id = backend.create_window(
            resolution.0 as u32, 
            resolution.1 as u32, 
            Some("BGI Window"), 
            mode
        )?;
        
        Ok(GraphicsContext { 
            initialized: true,
            mode,
            backend,
            current_window: Some(window_id),
            current_color: Color::WHITE,
            line_style: LineStyleInternal::default(),
            draw_mode: COPY_PUT,
        })
    }
    
    /// Create a context for testing - simpler constructor
    pub fn create_test_context() -> Self {
        let mode = GraphicsMode::new(GraphicsDriver::Vga, 2);
        let mut backend = create_pixel_buffer_backend().unwrap();
        backend.init().unwrap();
        
        let window_id = backend.create_window(800, 600, Some("Test"), mode).unwrap();
        
        GraphicsContext { 
            initialized: false, // Start uninitialized for testing
            mode,
            backend,
            current_window: Some(window_id),
            current_color: Color::WHITE,
            line_style: LineStyleInternal::default(),
            draw_mode: COPY_PUT,
        }
    }
    
    /// Initialize for testing
    pub fn initialize(&mut self, width: i32, height: i32, title: &str) {
        self.initialized = true;
    }
    
    pub fn close_graph(&mut self) -> Result<(), BgiError> {
        if let Some(window_id) = self.current_window {
            self.backend.close_window(window_id)?;
        }
        self.backend.shutdown()?;
        self.initialized = false;
        self.current_window = None;
        Ok(())
    }
    
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    pub fn set_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn get_color(&self) -> Color {
        self.current_color
    }

    pub fn set_line_style(&mut self, style: i32, pattern: u16, thickness: i32) {
        use crate::line::LINE_PATTERNS;
        
        let actual_pattern = if style == USERBIT_LINE {
            pattern // Use the provided pattern for user-defined lines
        } else if style >= 0 && (style as usize) < LINE_PATTERNS.len() {
            LINE_PATTERNS[style as usize] // Use predefined pattern
        } else {
            LINE_PATTERNS[SOLID_LINE as usize] // Default to solid line
        };
        
        self.line_style = LineStyleInternal {
            style,
            pattern: actual_pattern,
            thickness,
        };
    }

    pub fn get_line_style(&self) -> LineStyleInternal {
        self.line_style
    }

    pub fn set_write_mode(&mut self, mode: i32) {
        self.draw_mode = mode;
    }

    pub fn get_write_mode(&self) -> i32 {
        self.draw_mode
    }

    pub fn draw_ellipse(&mut self, x: i32, y: i32, start_angle: i32, end_angle: i32, x_radius: i32, y_radius: i32) -> Result<(), BgiError> {
        if let Some(window_id) = self.current_window {
            draw_ellipse_arc(
                &mut *self.backend,
                window_id,
                x, y,
                start_angle, end_angle,
                x_radius, y_radius,
                self.current_color.to_rgb(),
                self.line_style,
                self.draw_mode,
            )?;
        }
        Ok(())
    }

    pub fn put_pixel(&mut self, x: i32, y: i32, color: Color) -> Result<(), BgiError> {
        if let Some(window_id) = self.current_window {
            let commands = vec![DrawCommand::Pixel { 
                x, 
                y, 
                color: color.to_rgb() 
            }];
            self.backend.draw(window_id, &commands)?;
        }
        Ok(())
    }

    pub fn get_pixel(&self, x: i32, y: i32) -> Result<Color, BgiError> {
        if let Some(window_id) = self.current_window {
            let rgb_color = self.backend.get_pixel(window_id, x, y)?;
            // Convert back to indexed color if possible, otherwise use RGB
            Ok(Color::Rgb(rgb_color))
        } else {
            Ok(Color::BLACK)
        }
    }

    pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) -> Result<(), BgiError> {
        if let Some(window_id) = self.current_window {
            // Use the Bresenham line algorithm with line patterns
            draw_thick_line(
                &mut *self.backend,
                window_id,
                x1, y1, x2, y2,
                self.current_color.to_rgb(),
                self.line_style,
                self.draw_mode,
            )?;
        }
        Ok(())
    }

    pub fn draw_rectangle(&mut self, left: i32, top: i32, right: i32, bottom: i32) -> Result<(), BgiError> {
        if let Some(window_id) = self.current_window {
            // Use line composition for rectangles, just like the original BGI
            draw_rectangle_lines(
                &mut *self.backend,
                window_id,
                left, top, right, bottom,
                self.current_color.to_rgb(),
                self.line_style,
                self.draw_mode,
            )?;
        }
        Ok(())
    }

    pub fn draw_circle(&mut self, x: i32, y: i32, radius: i32) -> Result<(), BgiError> {
        if let Some(window_id) = self.current_window {
            // Use the Bresenham circle algorithm
            draw_thick_circle(
                &mut *self.backend,
                window_id,
                x, y, radius,
                self.current_color.to_rgb(),
                self.line_style.thickness,
            )?;
        }
        Ok(())
    }

    pub fn clear(&mut self, color: Color) -> Result<(), BgiError> {
        if let Some(window_id) = self.current_window {
            let commands = vec![DrawCommand::Clear { 
                color: color.to_rgb()
            }];
            self.backend.draw(window_id, &commands)?;
        }
        Ok(())
    }

    /// Get access to the backend for testing (e.g., pixel buffer verification).
    pub fn backend(&self) -> &dyn Backend {
        &*self.backend
    }

    /// Get mutable access to the backend for testing.
    pub fn backend_mut(&mut self) -> &mut dyn Backend {
        &mut *self.backend
    }

    /// Get the current window ID.
    pub fn current_window(&self) -> Option<WindowId> {
        self.current_window
    }
}

// Graphics initialization functions





// Note: GraphicsContext cannot be cloned because Backend doesn't implement Clone







/// Get resolution and color information for a device/mode combination
pub fn getmodeinfo(driver: i32, mode: i32) -> Option<(u32, u32, u32)> {
    let graphics_driver = match driver {
        0 => GraphicsDriver::Detect,
        1 => GraphicsDriver::Cga,
        2 => GraphicsDriver::Mcga,
        3 => GraphicsDriver::Ega,
        4 => GraphicsDriver::Ega64,
        5 => GraphicsDriver::EgaMono,
        6 => GraphicsDriver::Ibm8514,
        7 => GraphicsDriver::HercMono,
        8 => GraphicsDriver::Att400,
        9 => GraphicsDriver::Vga,
        10 => GraphicsDriver::Pc3270,
        _ => return None,
    };
    
    let graphics_mode = GraphicsMode::new(graphics_driver, mode);
    let resolution = graphics_mode.resolution();
    let colors = graphics_mode.color_depth();
    
    Some((resolution.0 as u32, resolution.1 as u32, colors as u32))
}



/// Drawing primitives using GRAPHICS_STATE from graphics.rs
/// Note: These maintain the classic BGI API without explicit context parameters

pub fn line(x1: i32, y1: i32, x2: i32, y2: i32) {
    // Use the same implementation as shapes.rs for consistency
    crate::shapes::line(x1, y1, x2, y2);
    set_graph_result(GraphResult::Ok);
}

pub fn circle(x: i32, y: i32, radius: i32) {
    // Use the same implementation as shapes.rs for consistency
    crate::shapes::circle(x, y, radius);
    set_graph_result(GraphResult::Ok);
}

pub fn rectangle(left: i32, top: i32, right: i32, bottom: i32) {
    // Use the same implementation as shapes.rs for consistency
    crate::shapes::rectangle(left, top, right, bottom);
    set_graph_result(GraphResult::Ok);
}

pub fn arc(x: i32, y: i32, start_angle: i32, end_angle: i32, radius: i32) {
    // Use the same implementation as shapes.rs for consistency
    crate::shapes::arc(x, y, start_angle, end_angle, radius);
    set_graph_result(GraphResult::Ok);
}

pub fn putpixel(x: i32, y: i32, color: Color) {
    // Use the same implementation as shapes.rs for consistency
    crate::shapes::putpixel(x, y, color);
    set_graph_result(GraphResult::Ok);
}

pub fn getpixel(x: i32, y: i32) -> Color {
    // Use the same implementation as shapes.rs for consistency
    let color = crate::shapes::getpixel(x, y);
    set_graph_result(GraphResult::Ok);
    color
}

pub fn ellipse(x: i32, y: i32, start_angle: i32, end_angle: i32, x_radius: i32, y_radius: i32) {
    // Use the same implementation as shapes.rs for consistency
    crate::shapes::ellipse(x, y, start_angle, end_angle, x_radius, y_radius);
    set_graph_result(GraphResult::Ok);
}

/// Explicit context API - alternative versions for contract tests that expect explicit context passing
/// Note: These are designed to satisfy the drawing primitives contract tests

pub fn line_ctx(context: &mut GraphicsContext, x1: i32, y1: i32, x2: i32, y2: i32) -> GraphResult {
    optimized_ctx::line_ctx(context, x1, y1, x2, y2)
}

pub fn circle_ctx(context: &mut GraphicsContext, x: i32, y: i32, radius: i32) -> GraphResult {
    optimized_ctx::circle_ctx(context, x, y, radius)
}

pub fn rectangle_ctx(context: &mut GraphicsContext, left: i32, top: i32, right: i32, bottom: i32) -> GraphResult {
    optimized_ctx::rectangle_ctx(context, left, top, right, bottom)
}

pub fn arc_ctx(context: &mut GraphicsContext, _x: i32, _y: i32, _start_angle: i32, _end_angle: i32, _radius: i32) -> GraphResult {
    crate::validate_context!(context);
    // TDD stub - drawing logic will be implemented
    GraphResult::Ok
}

pub fn putpixel_ctx(context: &mut GraphicsContext, x: i32, y: i32, color: Color) -> GraphResult {
    optimized_ctx::putpixel_ctx(context, x, y, color)
}

pub fn getpixel_ctx(context: &GraphicsContext, x: i32, y: i32) -> Result<Color, BgiError> {
    let (color, result) = optimized_ctx::getpixel_ctx(context, x, y);
    match result {
        GraphResult::Ok => Ok(color),
        _ => Err(BgiError::NotInitialized),
    }
}

pub fn ellipse_ctx(context: &mut GraphicsContext, x: i32, y: i32, start_angle: i32, end_angle: i32, x_radius: i32, y_radius: i32) -> GraphResult {
    crate::validate_context!(context);
    
    match context.draw_ellipse(x, y, start_angle, end_angle, x_radius, y_radius) {
        Ok(()) => GraphResult::Ok,
        Err(_) => GraphResult::InvalidDriver,
    }
}

// TDD: Thread-local storage for current color and error state
thread_local! {
    static CURRENT_COLOR: RefCell<Color> = RefCell::new(Color::WHITE);
    static LAST_RESULT: RefCell<GraphResult> = RefCell::new(GraphResult::Ok);
    static CURRENT_MODE: RefCell<i32> = RefCell::new(4); // Default to VGA mode
}

/// Set the last graphics result (for graphresult() function)
fn set_graph_result(result: GraphResult) {
    LAST_RESULT.with(|r| *r.borrow_mut() = result);
}





// Palette functions are implemented in palette.rs and re-exported

// Text rendering functions  
pub fn outtextxy(x: i32, y: i32, text: &str) {
    // TDD stub - will implement with actual backend
}

pub fn settextstyle(font: i32, direction: i32, char_size: i32) {
    // TDD stub - will implement with actual backend  
}

pub fn gettextsettings() -> crate::types::BgiTextSettings {
    // TDD stub - returns default settings
    crate::types::BgiTextSettings {
        font: 0,
        direction: 0,
        charsize: 1,
        horiz: 0,
        vert: 0,
    }
}

pub fn textwidth(text: &str) -> i32 {
    // TDD stub - simple calculation
    text.len() as i32 * 8 // 8 pixels per character
}

pub fn textheight(text: &str) -> i32 {
    // TDD stub - simple calculation  
    if text.is_empty() { 0 } else { 16 } // 16 pixels height
}

// TDD: Thread-local storage for viewport and cursor position
thread_local! {
    static VIEWPORT: RefCell<(i32, i32, i32, i32)> = RefCell::new((0, 0, 639, 479)); // Default 640x480
    static CURSOR_POS: RefCell<(i32, i32)> = RefCell::new((0, 0));
}

// Viewport and coordinate functions

/// Set viewport (drawing area boundaries).
pub fn setviewport(left: i32, top: i32, right: i32, bottom: i32, clip: bool) {
    // Update global viewport state
    VIEWPORT.with(|vp| *vp.borrow_mut() = (left, top, right, bottom));
    
    // Also update graphics state if initialized
    graphics::setviewport(left, top, right, bottom, clip);
}

/// Get current viewport settings.
pub fn getviewport() -> (i32, i32, i32, i32) {
    VIEWPORT.with(|vp| *vp.borrow())
}

/// Get maximum X coordinate (right edge of viewport).
pub fn getmaxx() -> i32 {
    VIEWPORT.with(|vp| vp.borrow().2)
}

/// Get maximum Y coordinate (bottom edge of viewport).
pub fn getmaxy() -> i32 {
    VIEWPORT.with(|vp| vp.borrow().3)
}

/// Move cursor to absolute position.
pub fn moveto(x: i32, y: i32) {
    CURSOR_POS.with(|pos| *pos.borrow_mut() = (x, y));
    
    // Also update graphics state if initialized
    graphics::moveto(x, y);
}

/// Move cursor relative to current position.
pub fn moverel(dx: i32, dy: i32) {
    CURSOR_POS.with(|pos| {
        let mut position = pos.borrow_mut();
        position.0 = position.0.saturating_add(dx);
        position.1 = position.1.saturating_add(dy);
    });
    
    // Also update graphics state if initialized
    graphics::moverel(dx, dy);
}

/// Get current cursor X position.
pub fn getx() -> i32 {
    CURSOR_POS.with(|pos| pos.borrow().0)
}

/// Get current cursor Y position.
pub fn gety() -> i32 {
    CURSOR_POS.with(|pos| pos.borrow().1)
}

// Filled shapes functions
pub fn fillellipse(x: i32, y: i32, x_radius: i32, y_radius: i32) {
    // TDD stub - will implement with actual backend
}

pub fn sector(x: i32, y: i32, start_angle: i32, end_angle: i32, x_radius: i32, y_radius: i32) {
    // TDD stub - will implement with actual backend
}

pub fn pieslice(x: i32, y: i32, start_angle: i32, end_angle: i32, radius: i32) {
    // TDD stub - will implement with actual backend
}

pub fn bar(left: i32, top: i32, right: i32, bottom: i32) {
    // TDD stub - will implement with actual backend
}

pub fn fillpoly(points: &[(i32, i32)]) {
    // TDD stub - will implement with actual backend
}

// Input handling functions
pub fn getch() -> Option<char> {
    // TDD stub - simulate no input available
    None
}

pub fn kbhit() -> bool {
    // TDD stub - simulate no key pressed
    false
}

pub fn getmouse() -> MouseState {
    // TDD stub - returns mouse state with no buttons pressed
    MouseState {
        x: 0,
        y: 0,
        left: false,
        right: false,
        middle: false,
    }
}

pub fn ismouseclick(button: i32) -> bool {
    // TDD stub - simulate no mouse clicks
    false
}
