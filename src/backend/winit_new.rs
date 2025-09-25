//! Winit-based backend implementation for cross-platform window management.

use std::collections::HashMap;

use pixels::Pixels;
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyEvent, MouseButton as WinitMouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{Key as WinitKey, NamedKey},
    window::{Window, WindowBuilder},
};

use crate::backend::{Backend, DrawCommand};
use crate::color::{Color, RgbColor};
use crate::error::{BgiError, BgiResult};
use crate::input::{InputEvent, Key, MouseButton, MouseEvent};
use crate::types::{GraphicsMode, Point, Rect};
use crate::window::WindowId;

// Internal window state
struct WindowState {
    pixels: Pixels,
    width: u32,
    height: u32,
    buffer: Vec<u32>,
    current_color: u32,
    background_color: u32,
    viewport: Rect,
}

impl WindowState {
    fn new(width: u32, height: u32, pixels: Pixels) -> Self {
        let buffer = vec![0; (width * height) as usize];
        let viewport = Rect {
            left: 0,
            top: 0,
            right: width as i32,
            bottom: height as i32,
        };

        Self {
            pixels,
            width,
            height,
            buffer,
            current_color: 0xFFFFFF, // White
            background_color: 0x000000, // Black
            viewport,
        }
    }

    fn put_pixel(&mut self, x: u32, y: u32, color: u32) {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) as usize;
            if index < self.buffer.len() {
                self.buffer[index] = color;
            }
        }
    }

    fn get_pixel(&self, x: u32, y: u32) -> u32 {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) as usize;
            self.buffer.get(index).copied().unwrap_or(0)
        } else {
            0
        }
    }

    fn clear(&mut self) {
        self.buffer.fill(self.background_color);
    }

    fn present(&mut self) -> BgiResult<()> {
        let frame = self.pixels.frame_mut();
        for (dest, &src) in frame.chunks_exact_mut(4).zip(&self.buffer) {
            let r = ((src >> 16) & 0xFF) as u8;
            let g = ((src >> 8) & 0xFF) as u8;
            let b = (src & 0xFF) as u8;
            dest[0] = r;
            dest[1] = g;
            dest[2] = b;
            dest[3] = 255; // Alpha
        }

        self.pixels.render().map_err(|e| BgiError::GraphicsError(format!("Render error: {}", e)))?;
        Ok(())
    }

    fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        // Bresenham's line algorithm
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
            if x1 >= 0 && y1 >= 0 {
                self.put_pixel(x1 as u32, y1 as u32, self.current_color);
            }

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
    }

    fn draw_circle(&mut self, center_x: i32, center_y: i32, radius: i32) {
        // Bresenham's circle algorithm
        let mut x = 0;
        let mut y = radius;
        let mut d = 3 - 2 * radius;

        while x <= y {
            // Draw 8 octants
            let points = [
                (center_x + x, center_y + y),
                (center_x - x, center_y + y),
                (center_x + x, center_y - y),
                (center_x - x, center_y - y),
                (center_x + y, center_y + x),
                (center_x - y, center_y + x),
                (center_x + y, center_y - x),
                (center_x - y, center_y - x),
            ];

            for (px, py) in points {
                if px >= 0 && py >= 0 {
                    self.put_pixel(px as u32, py as u32, self.current_color);
                }
            }

            if d < 0 {
                d += 4 * x + 6;
            } else {
                d += 4 * (x - y) + 10;
                y -= 1;
            }
            x += 1;
        }
    }
}

pub struct WinitBackend {
    windows: HashMap<WindowId, (Window, WindowState)>,
    current_window: Option<WindowId>,
    next_window_id: u32,
    pending_events: Vec<InputEvent>,
}

impl WinitBackend {
    pub fn new() -> BgiResult<Self> {
        Ok(Self {
            windows: HashMap::new(),
            current_window: None,
            next_window_id: 1,
            pending_events: Vec::new(),
        })
    }

    fn color_to_u32(&self, color: Color) -> u32 {
        match color {
            Color::Indexed(idx) => {
                // BGI 16-color palette approximation
                match idx % 16 {
                    0 => 0x000000, // Black
                    1 => 0x000080, // Blue
                    2 => 0x008000, // Green
                    3 => 0x008080, // Cyan
                    4 => 0x800000, // Red
                    5 => 0x800080, // Magenta
                    6 => 0x808000, // Brown
                    7 => 0xC0C0C0, // Light Gray
                    8 => 0x808080, // Dark Gray
                    9 => 0x0000FF, // Light Blue
                    10 => 0x00FF00, // Light Green
                    11 => 0x00FFFF, // Light Cyan
                    12 => 0xFF0000, // Light Red
                    13 => 0xFF00FF, // Light Magenta
                    14 => 0xFFFF00, // Yellow
                    15 => 0xFFFFFF, // White
                    _ => 0xFFFFFF,
                }
            }
            Color::Rgb(rgb) => {
                ((rgb.r as u32) << 16) | ((rgb.g as u32) << 8) | (rgb.b as u32)
            }
        }
    }
}

impl Backend for WinitBackend {
    fn create_window(
        &mut self,
        title: &str,
        width: u32,
        height: u32,
        _position: Option<&Point>,
        _mode: GraphicsMode,
    ) -> BgiResult<WindowId> {
        // For this simple version, we'll return an error since we can't create windows
        // without access to the event loop. This will be handled differently in a real implementation.
        Err(BgiError::GraphicsError("Cannot create windows without event loop access".to_string()))
    }

    fn close_window(&mut self, window_id: WindowId) -> BgiResult<()> {
        self.windows.remove(&window_id);
        if self.current_window == Some(window_id) {
            self.current_window = self.windows.keys().next().copied();
        }
        Ok(())
    }

    fn set_current_window(&mut self, window_id: WindowId) -> BgiResult<()> {
        if self.windows.contains_key(&window_id) {
            self.current_window = Some(window_id);
            Ok(())
        } else {
            Err(BgiError::InvalidWindow)
        }
    }

    fn current_window(&self) -> Option<WindowId> {
        self.current_window
    }

    fn run_event_loop<F>(self, _callback: F) -> BgiResult<()>
    where
        F: FnMut(&mut dyn Backend, InputEvent) -> bool + 'static,
    {
        // This would be implemented with proper event loop handling
        Err(BgiError::GraphicsError("Event loop not implemented yet".to_string()))
    }

    fn poll_input(&mut self) -> BgiResult<Option<InputEvent>> {
        Ok(self.pending_events.pop())
    }

    fn put_pixel(&mut self, x: i32, y: i32, color: Color) -> BgiResult<()> {
        if let Some(window_id) = self.current_window {
            if let Some((_, state)) = self.windows.get_mut(&window_id) {
                if x >= 0 && y >= 0 {
                    let color_u32 = self.color_to_u32(color);
                    state.current_color = color_u32;
                    state.put_pixel(x as u32, y as u32, color_u32);
                }
            }
        }
        Ok(())
    }

    fn get_pixel(&self, x: i32, y: i32) -> BgiResult<Color> {
        if let Some(window_id) = self.current_window {
            if let Some((_, state)) = self.windows.get(&window_id) {
                if x >= 0 && y >= 0 {
                    let color_u32 = state.get_pixel(x as u32, y as u32);
                    let r = ((color_u32 >> 16) & 0xFF) as u8;
                    let g = ((color_u32 >> 8) & 0xFF) as u8;
                    let b = (color_u32 & 0xFF) as u8;
                    return Ok(Color::Rgb(RgbColor { r, g, b }));
                }
            }
        }
        Ok(Color::Indexed(0)) // Return black as default
    }

    fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) -> BgiResult<()> {
        if let Some(window_id) = self.current_window {
            if let Some((_, state)) = self.windows.get_mut(&window_id) {
                state.draw_line(x1, y1, x2, y2);
            }
        }
        Ok(())
    }

    fn draw_circle(&mut self, center_x: i32, center_y: i32, radius: i32) -> BgiResult<()> {
        if let Some(window_id) = self.current_window {
            if let Some((_, state)) = self.windows.get_mut(&window_id) {
                state.draw_circle(center_x, center_y, radius);
            }
        }
        Ok(())
    }

    fn fill_rectangle(&mut self, rect: Rect, color: Color) -> BgiResult<()> {
        let color_u32 = self.color_to_u32(color);
        if let Some(window_id) = self.current_window {
            if let Some((_, state)) = self.windows.get_mut(&window_id) {
                for y in rect.top..=rect.bottom {
                    for x in rect.left..=rect.right {
                        if x >= 0 && y >= 0 {
                            state.put_pixel(x as u32, y as u32, color_u32);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn clear_screen(&mut self, color: Color) -> BgiResult<()> {
        let color_u32 = self.color_to_u32(color);
        if let Some(window_id) = self.current_window {
            if let Some((_, state)) = self.windows.get_mut(&window_id) {
                state.background_color = color_u32;
                state.clear();
            }
        }
        Ok(())
    }

    fn present(&mut self) -> BgiResult<()> {
        if let Some(window_id) = self.current_window {
            if let Some((_, state)) = self.windows.get_mut(&window_id) {
                state.present()?;
            }
        }
        Ok(())
    }

    fn execute_commands(&mut self, _commands: &[DrawCommand]) -> BgiResult<()> {
        // TODO: Implement batch command execution
        Ok(())
    }

    fn set_color(&mut self, color: Color) -> BgiResult<()> {
        let color_u32 = self.color_to_u32(color);
        if let Some(window_id) = self.current_window {
            if let Some((_, state)) = self.windows.get_mut(&window_id) {
                state.current_color = color_u32;
            }
        }
        Ok(())
    }

    fn set_viewport(&mut self, viewport: Rect) -> BgiResult<()> {
        if let Some(window_id) = self.current_window {
            if let Some((_, state)) = self.windows.get_mut(&window_id) {
                state.viewport = viewport;
            }
        }
        Ok(())
    }

    fn get_viewport(&self) -> BgiResult<Rect> {
        if let Some(window_id) = self.current_window {
            if let Some((_, state)) = self.windows.get(&window_id) {
                return Ok(state.viewport);
            }
        }
        Ok(Rect { left: 0, top: 0, right: 640, bottom: 480 })
    }

    fn get_window_size(&self) -> BgiResult<(u32, u32)> {
        if let Some(window_id) = self.current_window {
            if let Some((_, state)) = self.windows.get(&window_id) {
                return Ok((state.width, state.height));
            }
        }
        Ok((640, 480))
    }

    fn set_text(&mut self, _text: &str, _x: i32, _y: i32) -> BgiResult<()> {
        // TODO: Implement text rendering
        Ok(())
    }
}

impl Default for WinitBackend {
    fn default() -> Self {
        Self::new().expect("Failed to create winit backend")
    }
}
