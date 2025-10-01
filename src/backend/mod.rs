//! Backend abstraction for the BGI library.

use crate::color::RgbColor;
use crate::error::{BgiError, BgiResult};
// use crate::input::InputEvent; // Temporarily disabled

// Temporary placeholder for InputEvent
#[derive(Debug, Clone)]
pub enum InputEvent {
    Key {
        window_id: WindowId,
        key_code: i32,
        extended: bool,
    },
    Mouse {
        window_id: WindowId,
        x: i32,
        y: i32,
        buttons: u32,
    },
    Placeholder,
}
use crate::types::{GraphicsMode, Point, Rect};
use crate::window::WindowId;

/// Capabilities supported by a graphics backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackendCapabilities {
    /// Supports multiple windows.
    pub multi_window: bool,
    /// Supports hardware acceleration.
    pub hardware_acceleration: bool,
    /// Supports alpha blending.
    pub alpha_blending: bool,
    /// Supports fullscreen mode.
    pub fullscreen: bool,
    /// Supports window resizing.
    pub resizable: bool,
}

/// Drawing command for backend rendering.
#[derive(Debug, Clone)]
pub enum DrawCommand {
    /// Clear the entire surface with a color.
    Clear { color: RgbColor },
    /// Draw a pixel at the specified position.
    Pixel { x: i32, y: i32, color: RgbColor },
    /// Draw a line between two points.
    Line {
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        color: RgbColor,
    },
    /// Draw a rectangle.
    Rectangle {
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        color: RgbColor,
        filled: bool,
    },
    /// Draw a circle.
    Circle {
        x: i32,
        y: i32,
        radius: i32,
        color: RgbColor,
        filled: bool,
    },
    /// Draw an ellipse.
    Ellipse {
        x: i32,
        y: i32,
        rx: i32,
        ry: i32,
        color: RgbColor,
        filled: bool,
    },
    /// Draw an arc.
    Arc {
        x: i32,
        y: i32,
        start_angle: i32,
        end_angle: i32,
        radius: i32,
        color: RgbColor,
    },
    /// Draw text.
    Text {
        x: i32,
        y: i32,
        text: String,
        color: RgbColor,
    },
    /// Copy image data to the surface.
    Image {
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        pixels: Vec<u32>,
    },
}

/// Graphics backend trait that abstracts rendering operations.
pub trait Backend {
    /// Get backend capabilities.
    fn capabilities(&self) -> BackendCapabilities;

    /// Initialize the graphics system.
    fn init(&mut self) -> BgiResult<()>;

    /// Shutdown the graphics system.
    fn shutdown(&mut self) -> BgiResult<()>;

    /// Create a new window.
    fn create_window(
        &mut self,
        width: u32,
        height: u32,
        title: Option<&str>,
        mode: GraphicsMode,
    ) -> BgiResult<WindowId>;

    /// Close a window.
    fn close_window(&mut self, window_id: WindowId) -> BgiResult<()>;

    /// Set the current active window.
    fn set_current_window(&mut self, window_id: WindowId) -> BgiResult<()>;

    /// Get the current active window.
    fn current_window(&self) -> Option<WindowId>;

    /// Get window dimensions.
    fn window_size(&self, window_id: WindowId) -> BgiResult<(u32, u32)>;

    /// Set window title.
    fn set_window_title(&mut self, window_id: WindowId, title: &str) -> BgiResult<()>;

    /// Check if a window exists and is valid.
    fn is_window_valid(&self, window_id: WindowId) -> bool;

    /// Execute drawing commands.
    fn draw(&mut self, window_id: WindowId, commands: &[DrawCommand]) -> BgiResult<()>;

    /// Present the rendered content to the screen.
    fn present(&mut self, window_id: WindowId) -> BgiResult<()>;

    /// Get pixel color at position.
    fn get_pixel(&self, window_id: WindowId, x: i32, y: i32) -> BgiResult<RgbColor>;

    /// Set the viewport (clipping region).
    fn set_viewport(&mut self, window_id: WindowId, rect: Rect) -> BgiResult<()>;

    /// Get the current viewport.
    fn viewport(&self, window_id: WindowId) -> BgiResult<Rect>;

    /// Poll for input events.
    fn poll_events(&mut self) -> Vec<InputEvent>;

    /// Check if there are pending events.
    fn has_events(&self) -> bool;

    /// Set fullscreen mode.
    fn set_fullscreen(&mut self, window_id: WindowId, fullscreen: bool) -> BgiResult<()>;

    /// Get screen dimensions (for fullscreen mode).
    fn screen_size(&self) -> BgiResult<(u32, u32)>;

    /// Copy surface data to/from buffer.
    fn copy_surface(
        &mut self,
        window_id: WindowId,
        src_rect: Rect,
        dst_x: i32,
        dst_y: i32,
    ) -> BgiResult<()>;

    /// Get buffer data for direct pixel manipulation.
    fn get_buffer(&self, window_id: WindowId) -> BgiResult<Vec<u32>>;

    /// Set buffer data for direct pixel manipulation.
    fn set_buffer(&mut self, window_id: WindowId, buffer: &[u32]) -> BgiResult<()>;

    /// Load image from file.
    fn load_image(&mut self, filename: &str) -> BgiResult<(u32, u32, Vec<u32>)>;

    /// Save image to file.
    fn save_image(&self, filename: &str, width: u32, height: u32, pixels: &[u32]) -> BgiResult<()>;
}

#[cfg(feature = "winit-backend")]
pub mod winit;

#[cfg(feature = "visual-backend")]
pub mod minifb;

pub mod pixel_buffer;

/// Create default backend based on available features.
pub fn create_default_backend() -> BgiResult<Box<dyn Backend>> {
    #[cfg(feature = "visual-backend")]
    {
        Ok(Box::new(minifb::MiniFbBackend::new()))
    }
    #[cfg(all(feature = "winit-backend", not(feature = "visual-backend")))]
    {
        Ok(Box::new(winit::WinitBackend::new()?))
    }
    #[cfg(not(any(feature = "winit-backend", feature = "visual-backend")))]
    {
        // Use pixel buffer backend as fallback
        Ok(Box::new(pixel_buffer::PixelBufferBackend::new()))
    }
}

/// Create a pixel buffer backend for testing.
pub fn create_pixel_buffer_backend() -> BgiResult<Box<dyn Backend>> {
    Ok(Box::new(pixel_buffer::PixelBufferBackend::new()))
}
