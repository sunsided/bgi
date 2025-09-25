//! Simple pixel buffer backend for testing and headless rendering.

use crate::backend::{Backend, BackendCapabilities, DrawCommand};
use crate::color::RgbColor;
use crate::error::{BgiError, BgiResult};
use crate::backend::InputEvent; // Using temporary placeholder
use crate::types::{GraphicsMode, Rect};
use crate::window::WindowId;
use std::collections::HashMap;

/// Simple pixel buffer window for testing.
#[derive(Debug, Clone)]
pub struct PixelBufferWindow {
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub pixels: Vec<u32>,
    pub viewport: Rect,
}

impl PixelBufferWindow {
    pub fn new(width: u32, height: u32, title: String) -> Self {
        Self {
            width,
            height,
            title,
            pixels: vec![0; (width * height) as usize], // Black background
            viewport: Rect::new(0, 0, width as i32 - 1, height as i32 - 1),
        }
    }

    pub fn put_pixel(&mut self, x: i32, y: i32, color: RgbColor) -> BgiResult<()> {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return Ok(());  // Clip out-of-bounds pixels silently
        }

        let index = (y as u32 * self.width + x as u32) as usize;
        if index < self.pixels.len() {
            self.pixels[index] = color.to_argb32();
        }
        Ok(())
    }

    pub fn get_pixel(&self, x: i32, y: i32) -> BgiResult<RgbColor> {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return Ok(RgbColor::new(0, 0, 0)); // Return black for out-of-bounds
        }

        let index = (y as u32 * self.width + x as u32) as usize;
        if index < self.pixels.len() {
            Ok(RgbColor::from_argb32(self.pixels[index]))
        } else {
            Ok(RgbColor::new(0, 0, 0))
        }
    }

    pub fn clear(&mut self, color: RgbColor) {
        let color_u32 = color.to_argb32();
        self.pixels.fill(color_u32);
    }

    /// Simple line drawing using Bresenham's algorithm.
    pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: RgbColor) -> BgiResult<()> {
        let mut x1 = x1;
        let mut y1 = y1;
        let x2 = x2;
        let y2 = y2;

        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx - dy;

        loop {
            self.put_pixel(x1, y1, color)?;

            if x1 == x2 && y1 == y2 {
                break;
            }

            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x1 += sx;
            }
            if e2 < dx {
                err += dx;
                y1 += sy;
            }
        }
        Ok(())
    }

    /// Simple rectangle drawing.
    pub fn draw_rectangle(&mut self, left: i32, top: i32, right: i32, bottom: i32, color: RgbColor, filled: bool) -> BgiResult<()> {
        if filled {
            for y in top..=bottom {
                for x in left..=right {
                    self.put_pixel(x, y, color)?;
                }
            }
        } else {
            // Draw outline
            self.draw_line(left, top, right, top, color)?;       // Top
            self.draw_line(right, top, right, bottom, color)?;   // Right
            self.draw_line(right, bottom, left, bottom, color)?; // Bottom
            self.draw_line(left, bottom, left, top, color)?;     // Left
        }
        Ok(())
    }

    /// Simple circle drawing using midpoint circle algorithm.
    pub fn draw_circle(&mut self, cx: i32, cy: i32, radius: i32, color: RgbColor, filled: bool) -> BgiResult<()> {
        if filled {
            // Filled circle - draw horizontal lines
            for y in -radius..=radius {
                let x_span = ((radius * radius - y * y) as f32).sqrt() as i32;
                for x in -x_span..=x_span {
                    self.put_pixel(cx + x, cy + y, color)?;
                }
            }
        } else {
            // Circle outline using midpoint algorithm
            let mut x = 0;
            let mut y = radius;
            let mut d = 1 - radius;

            while x <= y {
                // Draw 8 octants
                self.put_pixel(cx + x, cy + y, color)?;
                self.put_pixel(cx - x, cy + y, color)?;
                self.put_pixel(cx + x, cy - y, color)?;
                self.put_pixel(cx - x, cy - y, color)?;
                self.put_pixel(cx + y, cy + x, color)?;
                self.put_pixel(cx - y, cy + x, color)?;
                self.put_pixel(cx + y, cy - x, color)?;
                self.put_pixel(cx - y, cy - x, color)?;

                if d < 0 {
                    d += 2 * x + 3;
                } else {
                    d += 2 * (x - y) + 5;
                    y -= 1;
                }
                x += 1;
            }
        }
        Ok(())
    }
}

/// Simple pixel buffer backend for testing and headless rendering.
pub struct PixelBufferBackend {
    windows: HashMap<WindowId, PixelBufferWindow>,
    current_window: Option<WindowId>,
    next_window_id: u32,
    initialized: bool,
}

impl PixelBufferBackend {
    pub fn new() -> Self {
        Self {
            windows: HashMap::new(),
            current_window: None,
            next_window_id: 1,
            initialized: false,
        }
    }

    /// Get a reference to the current window.
    pub fn current_window_ref(&self) -> BgiResult<&PixelBufferWindow> {
        let window_id = self.current_window.ok_or_else(|| BgiError::General {
            message: "No current window".to_string(),
        })?;

        self.windows.get(&window_id).ok_or_else(|| BgiError::General {
            message: "Invalid window".to_string(),
        })
    }

    /// Get a mutable reference to the current window.
    pub fn current_window_mut(&mut self) -> BgiResult<&mut PixelBufferWindow> {
        let window_id = self.current_window.ok_or_else(|| BgiError::General {
            message: "No current window".to_string(),
        })?;

        self.windows.get_mut(&window_id).ok_or_else(|| BgiError::General {
            message: "Invalid window".to_string(),
        })
    }

    /// Get the pixel buffer for the current window (for testing).
    pub fn get_pixels(&self) -> BgiResult<&[u32]> {
        let window = self.current_window_ref()?;
        Ok(&window.pixels)
    }

    /// Verify that a pixel has the expected color (for testing).
    pub fn verify_pixel(&self, x: i32, y: i32, expected_color: RgbColor) -> BgiResult<bool> {
        let window = self.current_window_ref()?;
        let actual_color = window.get_pixel(x, y)?;
        Ok(actual_color == expected_color)
    }

    /// Get the number of non-black pixels (for testing drawing operations).
    pub fn count_drawn_pixels(&self) -> BgiResult<usize> {
        let window = self.current_window_ref()?;
        let black = RgbColor::new(0, 0, 0).to_argb32();
        Ok(window.pixels.iter().filter(|&&pixel| pixel != black).count())
    }

    /// Verify that a line was drawn between two points (for testing).
    pub fn verify_line_drawn(&self, x1: i32, y1: i32, x2: i32, y2: i32, color: RgbColor) -> BgiResult<bool> {
        let window = self.current_window_ref()?;
        let color_u32 = color.to_argb32();

        // Simple verification: check start and end points
        let start_pixel = window.get_pixel(x1, y1)?;
        let end_pixel = window.get_pixel(x2, y2)?;

        Ok(start_pixel.to_argb32() == color_u32 && end_pixel.to_argb32() == color_u32)
    }

    /// Clear the pixel buffer (for testing).
    pub fn clear_for_testing(&mut self) -> BgiResult<()> {
        let window = self.current_window_mut()?;
        window.clear(RgbColor::new(0, 0, 0));
        Ok(())
    }

    /// Get window dimensions (for testing).
    pub fn get_dimensions(&self) -> BgiResult<(u32, u32)> {
        let window = self.current_window_ref()?;
        Ok((window.width, window.height))
    }
}

impl Default for PixelBufferBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl Backend for PixelBufferBackend {
    fn capabilities(&self) -> BackendCapabilities {
        BackendCapabilities {
            multi_window: true,
            hardware_acceleration: false,
            alpha_blending: false,
            fullscreen: false,
            resizable: false,
        }
    }

    fn init(&mut self) -> BgiResult<()> {
        self.initialized = true;
        Ok(())
    }

    fn shutdown(&mut self) -> BgiResult<()> {
        self.windows.clear();
        self.current_window = None;
        self.initialized = false;
        Ok(())
    }

    fn create_window(
        &mut self,
        width: u32,
        height: u32,
        title: Option<&str>,
        _mode: GraphicsMode,
    ) -> BgiResult<WindowId> {
        if !self.initialized {
            return Err(BgiError::General {
                message: "Backend not initialized".to_string(),
            });
        }

        let window_id = WindowId(self.next_window_id);
        self.next_window_id += 1;

        let window = PixelBufferWindow::new(
            width,
            height,
            title.unwrap_or("BGI Window").to_string(),
        );

        self.windows.insert(window_id, window);

        // Set as current window if it's the first one
        if self.current_window.is_none() {
            self.current_window = Some(window_id);
        }

        Ok(window_id)
    }

    fn close_window(&mut self, window_id: WindowId) -> BgiResult<()> {
        if self.windows.remove(&window_id).is_none() {
            return Err(BgiError::General {
                message: "Window not found".to_string(),
            });
        }

        // Clear current window if it was closed
        if self.current_window == Some(window_id) {
            self.current_window = self.windows.keys().next().copied();
        }

        Ok(())
    }

    fn set_current_window(&mut self, window_id: WindowId) -> BgiResult<()> {
        if !self.windows.contains_key(&window_id) {
            return Err(BgiError::General {
                message: "Window not found".to_string(),
            });
        }
        self.current_window = Some(window_id);
        Ok(())
    }

    fn current_window(&self) -> Option<WindowId> {
        self.current_window
    }

    fn window_size(&self, window_id: WindowId) -> BgiResult<(u32, u32)> {
        let window = self.windows.get(&window_id).ok_or_else(|| BgiError::General {
            message: "Window not found".to_string(),
        })?;
        Ok((window.width, window.height))
    }

    fn set_window_title(&mut self, window_id: WindowId, title: &str) -> BgiResult<()> {
        let window = self.windows.get_mut(&window_id).ok_or_else(|| BgiError::General {
            message: "Window not found".to_string(),
        })?;
        window.title = title.to_string();
        Ok(())
    }

    fn is_window_valid(&self, window_id: WindowId) -> bool {
        self.windows.contains_key(&window_id)
    }

    fn draw(&mut self, window_id: WindowId, commands: &[DrawCommand]) -> BgiResult<()> {
        let window = self.windows.get_mut(&window_id).ok_or_else(|| BgiError::General {
            message: "Window not found".to_string(),
        })?;

        for command in commands {
            match command {
                DrawCommand::Clear { color } => {
                    window.clear(*color);
                }
                DrawCommand::Pixel { x, y, color } => {
                    window.put_pixel(*x, *y, *color)?;
                }
                DrawCommand::Line { x1, y1, x2, y2, color } => {
                    window.draw_line(*x1, *y1, *x2, *y2, *color)?;
                }
                DrawCommand::Rectangle { x1, y1, x2, y2, color, filled } => {
                    window.draw_rectangle(*x1, *y1, *x2, *y2, *color, *filled)?;
                }
                DrawCommand::Circle { x, y, radius, color, filled } => {
                    window.draw_circle(*x, *y, *radius, *color, *filled)?;
                }
                DrawCommand::Ellipse { .. } => {
                    // TODO: Implement ellipse drawing
                }
                DrawCommand::Arc { .. } => {
                    // TODO: Implement arc drawing
                }
                DrawCommand::Text { .. } => {
                    // TODO: Implement text rendering
                }
                DrawCommand::Image { .. } => {
                    // TODO: Implement image blitting
                }
            }
        }

        Ok(())
    }

    fn present(&mut self, _window_id: WindowId) -> BgiResult<()> {
        // No-op for pixel buffer backend
        Ok(())
    }

    fn get_pixel(&self, window_id: WindowId, x: i32, y: i32) -> BgiResult<RgbColor> {
        let window = self.windows.get(&window_id).ok_or_else(|| BgiError::General {
            message: "Window not found".to_string(),
        })?;
        window.get_pixel(x, y)
    }

    fn set_viewport(&mut self, window_id: WindowId, rect: Rect) -> BgiResult<()> {
        let window = self.windows.get_mut(&window_id).ok_or_else(|| BgiError::General {
            message: "Window not found".to_string(),
        })?;
        window.viewport = rect;
        Ok(())
    }

    fn viewport(&self, window_id: WindowId) -> BgiResult<Rect> {
        let window = self.windows.get(&window_id).ok_or_else(|| BgiError::General {
            message: "Window not found".to_string(),
        })?;
        Ok(window.viewport)
    }

    fn poll_events(&mut self) -> Vec<InputEvent> {
        // No events for pixel buffer backend
        Vec::new()
    }

    fn has_events(&self) -> bool {
        false
    }

    fn set_fullscreen(&mut self, _window_id: WindowId, _fullscreen: bool) -> BgiResult<()> {
        // Not supported for pixel buffer backend
        Ok(())
    }

    fn screen_size(&self) -> BgiResult<(u32, u32)> {
        // Return a default screen size
        Ok((1024, 768))
    }

    fn copy_surface(
        &mut self,
        _window_id: WindowId,
        _src_rect: Rect,
        _dst_x: i32,
        _dst_y: i32,
    ) -> BgiResult<()> {
        // TODO: Implement surface copying
        Ok(())
    }

    fn get_buffer(&self, window_id: WindowId) -> BgiResult<Vec<u32>> {
        let window = self.windows.get(&window_id).ok_or_else(|| BgiError::General {
            message: "Window not found".to_string(),
        })?;
        Ok(window.pixels.clone())
    }

    fn set_buffer(&mut self, window_id: WindowId, buffer: &[u32]) -> BgiResult<()> {
        let window = self.windows.get_mut(&window_id).ok_or_else(|| BgiError::General {
            message: "Window not found".to_string(),
        })?;

        let expected_size = (window.width * window.height) as usize;
        if buffer.len() != expected_size {
            return Err(BgiError::General {
                message: format!("Buffer size mismatch: expected {}, got {}", expected_size, buffer.len()),
            });
        }

        window.pixels.copy_from_slice(buffer);
        Ok(())
    }

    fn load_image(&mut self, _filename: &str) -> BgiResult<(u32, u32, Vec<u32>)> {
        // TODO: Implement image loading
        Err(BgiError::General {
            message: "Image loading not implemented".to_string(),
        })
    }

    fn save_image(&self, _filename: &str, _width: u32, _height: u32, _pixels: &[u32]) -> BgiResult<()> {
        // TODO: Implement image saving
        Err(BgiError::General {
            message: "Image saving not implemented".to_string(),
        })
    }
}
