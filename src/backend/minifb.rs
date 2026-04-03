//! MiniFB backend implementation for BGI library
//!
//! This backend uses the minifb crate to provide cross-platform windowing
//! and pixel buffer rendering with minimal dependencies.

use crate::backend::{Backend, BackendCapabilities, DrawCommand, InputEvent};
use crate::color::RgbColor;
use crate::error::{BgiError, BgiResult};
use crate::types::{GraphicsMode, Point, Rect};
use crate::window::WindowId;
use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use std::collections::HashMap;

/// MiniFB backend that provides windowing via the minifb crate
pub struct MiniFbBackend {
    windows: HashMap<WindowId, minifb::Window>,
    buffers: HashMap<WindowId, Vec<u32>>,
    buffer_dimensions: HashMap<WindowId, (usize, usize)>, // (width, height)
    current_window: Option<WindowId>,
    next_window_id: WindowId,
    initialized: bool,
    /// Track previous key states to detect key press events
    previous_key_states: HashMap<WindowId, HashMap<Key, bool>>,
    /// Track previous mouse button states to detect click events
    previous_mouse_states: HashMap<WindowId, (bool, bool, bool)>, // (left, right, middle)
}

impl MiniFbBackend {
    /// Create a new MiniFB backend
    pub fn new() -> Self {
        Self {
            windows: HashMap::new(),
            buffers: HashMap::new(),
            buffer_dimensions: HashMap::new(),
            current_window: None,
            next_window_id: WindowId(1),
            initialized: false,
            previous_key_states: HashMap::new(),
            previous_mouse_states: HashMap::new(),
        }
    }

    fn next_id(&mut self) -> WindowId {
        let id = self.next_window_id;
        self.next_window_id = WindowId(self.next_window_id.0 + 1);
        id
    }

    fn rgb_to_u32(color: RgbColor) -> u32 {
        ((color.r as u32) << 16) | ((color.g as u32) << 8) | (color.b as u32)
    }

    fn u32_to_rgb(value: u32) -> RgbColor {
        RgbColor {
            r: ((value >> 16) & 0xFF) as u8,
            g: ((value >> 8) & 0xFF) as u8,
            b: (value & 0xFF) as u8,
            a: 255,
        }
    }

    fn clear_buffer(&mut self, window_id: WindowId, color: RgbColor) -> BgiResult<()> {
        if let Some(buffer) = self.buffers.get_mut(&window_id) {
            let color_value = Self::rgb_to_u32(color);
            buffer.fill(color_value);
            Ok(())
        } else {
            Err(BgiError::InvalidWindow)
        }
    }

    fn set_pixel_in_buffer(
        &mut self,
        window_id: WindowId,
        x: i32,
        y: i32,
        color: RgbColor,
    ) -> BgiResult<()> {
        if let (Some(window), Some(buffer)) = (
            self.windows.get(&window_id),
            self.buffers.get_mut(&window_id),
        ) {
            let (width, height) = window.get_size();
            if x >= 0 && y >= 0 && (x as usize) < width && (y as usize) < height {
                let index = (y as usize) * width + (x as usize);
                if index < buffer.len() {
                    buffer[index] = Self::rgb_to_u32(color);
                }
            }
            Ok(())
        } else {
            Err(BgiError::InvalidWindow)
        }
    }

    fn draw_line_in_buffer(
        &mut self,
        window_id: WindowId,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        color: RgbColor,
    ) -> BgiResult<()> {
        // Simple Bresenham line algorithm
        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx - dy;
        let mut x = x1;
        let mut y = y1;

        loop {
            self.set_pixel_in_buffer(window_id, x, y, color)?;

            if x == x2 && y == y2 {
                break;
            }

            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
        Ok(())
    }

    fn draw_circle_in_buffer(
        &mut self,
        window_id: WindowId,
        cx: i32,
        cy: i32,
        radius: i32,
        color: RgbColor,
        filled: bool,
    ) -> BgiResult<()> {
        if filled {
            // Fill the circle
            for y in -radius..=radius {
                for x in -radius..=radius {
                    if x * x + y * y <= radius * radius {
                        self.set_pixel_in_buffer(window_id, cx + x, cy + y, color)?;
                    }
                }
            }
        } else {
            // Draw circle outline using midpoint circle algorithm
            let mut x = 0;
            let mut y = radius;
            let mut d = 1 - radius;

            while x <= y {
                // Draw 8 octants
                self.set_pixel_in_buffer(window_id, cx + x, cy + y, color)?;
                self.set_pixel_in_buffer(window_id, cx - x, cy + y, color)?;
                self.set_pixel_in_buffer(window_id, cx + x, cy - y, color)?;
                self.set_pixel_in_buffer(window_id, cx - x, cy - y, color)?;
                self.set_pixel_in_buffer(window_id, cx + y, cy + x, color)?;
                self.set_pixel_in_buffer(window_id, cx - y, cy + x, color)?;
                self.set_pixel_in_buffer(window_id, cx + y, cy - x, color)?;
                self.set_pixel_in_buffer(window_id, cx - y, cy - x, color)?;

                x += 1;
                if d < 0 {
                    d += 2 * x + 1;
                } else {
                    y -= 1;
                    d += 2 * (x - y) + 1;
                }
            }
        }
        Ok(())
    }
}

impl Backend for MiniFbBackend {
    fn capabilities(&self) -> BackendCapabilities {
        BackendCapabilities {
            multi_window: true,
            hardware_acceleration: false,
            alpha_blending: false,
            fullscreen: true,
            resizable: false,
        }
    }

    fn init(&mut self) -> BgiResult<()> {
        self.initialized = true;
        Ok(())
    }

    fn shutdown(&mut self) -> BgiResult<()> {
        self.windows.clear();
        self.buffers.clear();
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
        let window_title = title.unwrap_or("BGI Graphics");

        // Create window with explicit options for better visibility
        let mut options = WindowOptions::default();
        options.resize = false;
        options.scale = minifb::Scale::X1;
        options.topmost = true; // Try to keep window on top

        let mut window = Window::new(window_title, width as usize, height as usize, options)
            .map_err(|e| BgiError::BackendError {
                message: format!("Failed to create window: {}", e),
            })?;

        // Limit update rate for better performance
        window.limit_update_rate(Some(std::time::Duration::from_micros(16600))); // ~60 FPS

        let window_id = self.next_id();
        let buffer_size = (width * height) as usize;
        let buffer = vec![0u32; buffer_size]; // Black background

        self.windows.insert(window_id, window);
        self.buffers.insert(window_id, buffer);
        self.buffer_dimensions
            .insert(window_id, (width as usize, height as usize));

        if self.current_window.is_none() {
            self.current_window = Some(window_id);
        }

        Ok(window_id)
    }

    fn close_window(&mut self, window_id: WindowId) -> BgiResult<()> {
        self.windows.remove(&window_id);
        self.buffers.remove(&window_id);

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

    fn window_size(&self, window_id: WindowId) -> BgiResult<(u32, u32)> {
        if let Some(window) = self.windows.get(&window_id) {
            let (width, height) = window.get_size();
            Ok((width as u32, height as u32))
        } else {
            Err(BgiError::InvalidWindow)
        }
    }

    fn set_window_title(&mut self, window_id: WindowId, title: &str) -> BgiResult<()> {
        if let Some(window) = self.windows.get_mut(&window_id) {
            window.set_title(title);
            Ok(())
        } else {
            Err(BgiError::InvalidWindow)
        }
    }

    fn is_window_valid(&self, window_id: WindowId) -> bool {
        self.windows.contains_key(&window_id)
            && self.windows.get(&window_id).map_or(false, |w| w.is_open())
    }

    fn draw(&mut self, window_id: WindowId, commands: &[DrawCommand]) -> BgiResult<()> {
        for command in commands {
            match command {
                DrawCommand::Clear { color } => {
                    self.clear_buffer(window_id, *color)?;
                }
                DrawCommand::Pixel { x, y, color } => {
                    self.set_pixel_in_buffer(window_id, *x, *y, *color)?;
                }
                DrawCommand::Line {
                    x1,
                    y1,
                    x2,
                    y2,
                    color,
                } => {
                    self.draw_line_in_buffer(window_id, *x1, *y1, *x2, *y2, *color)?;
                }
                DrawCommand::Rectangle {
                    x1,
                    y1,
                    x2,
                    y2,
                    color,
                    filled,
                } => {
                    if *filled {
                        // Fill rectangle
                        for y in *y1..=*y2 {
                            self.draw_line_in_buffer(window_id, *x1, y, *x2, y, *color)?;
                        }
                    } else {
                        // Draw rectangle outline
                        self.draw_line_in_buffer(window_id, *x1, *y1, *x2, *y1, *color)?; // Top
                        self.draw_line_in_buffer(window_id, *x1, *y2, *x2, *y2, *color)?; // Bottom
                        self.draw_line_in_buffer(window_id, *x1, *y1, *x1, *y2, *color)?; // Left
                        self.draw_line_in_buffer(window_id, *x2, *y1, *x2, *y2, *color)?;
                        // Right
                    }
                }
                DrawCommand::Circle {
                    x,
                    y,
                    radius,
                    color,
                    filled,
                } => {
                    self.draw_circle_in_buffer(window_id, *x, *y, *radius, *color, *filled)?;
                }
                DrawCommand::Text { x, y, text, color } => {
                    // Simple text rendering - draw basic letters as pixels
                    // This is a minimal implementation
                    for (i, _ch) in text.chars().enumerate() {
                        let char_x = x + (i as i32 * 8);
                        // Draw a simple 5x7 character placeholder
                        for py in 0..7 {
                            for px in 0..5 {
                                self.set_pixel_in_buffer(window_id, char_x + px, y + py, *color)?;
                            }
                        }
                    }
                }
                _ => {
                    // Ignore unsupported commands for now
                }
            }
        }
        Ok(())
    }

    fn present(&mut self, window_id: WindowId) -> BgiResult<()> {
        if let (Some(window), Some(buffer), Some((width, height))) = (
            self.windows.get_mut(&window_id),
            self.buffers.get(&window_id),
            self.buffer_dimensions.get(&window_id),
        ) {
            // Check if window is still open before updating
            if !window.is_open() {
                return Err(BgiError::InvalidWindow);
            }

            window
                .update_with_buffer(buffer, *width, *height)
                .map_err(|e| BgiError::BackendError {
                    message: format!("Failed to update window: {}", e),
                })?;
        } else {
            return Err(BgiError::InvalidWindow);
        }
        Ok(())
    }

    fn get_pixel(&self, window_id: WindowId, x: i32, y: i32) -> Result<RgbColor, BgiError> {
        if let (Some(buffer), Some((width, height))) = (
            self.buffers.get(&window_id),
            self.buffer_dimensions.get(&window_id),
        ) {
            let width = *width as i32;
            let height = *height as i32;

            if x >= 0 && x < width && y >= 0 && y < height {
                let idx = (y * width + x) as usize;
                let pixel = buffer[idx];
                Ok(RgbColor {
                    r: ((pixel >> 16) & 0xFF) as u8,
                    g: ((pixel >> 8) & 0xFF) as u8,
                    b: (pixel & 0xFF) as u8,
                    a: 255, // Full opacity
                })
            } else {
                Err(BgiError::InvalidCoordinates { x, y })
            }
        } else {
            Err(BgiError::InvalidWindow)
        }
    }

    fn set_viewport(&mut self, _window_id: WindowId, _rect: Rect) -> BgiResult<()> {
        // TODO: Implement viewport clipping
        Ok(())
    }

    fn viewport(&self, _window_id: WindowId) -> BgiResult<Rect> {
        // TODO: Return actual viewport
        Ok(Rect {
            left: 0,
            top: 0,
            right: 640,
            bottom: 480,
        })
    }

    fn poll_events(&mut self) -> Vec<InputEvent> {
        let mut events = Vec::new();

        // Poll events from all windows
        for (window_id, window) in &mut self.windows {
            // First ensure the window is updated to process events
            // MiniFB processes input events during window updates
            if let Some(buffer) = self.buffers.get(window_id) {
                if let Some((width, height)) = self.buffer_dimensions.get(window_id) {
                    if window.is_open() {
                        // Update window to process input events
                        let _ = window.update_with_buffer(buffer, *width, *height);
                    }
                }
            }

            // Get previous key states for this window
            let prev_states = self
                .previous_key_states
                .entry(*window_id)
                .or_insert_with(HashMap::new);

            // Check all keys we care about
            let keys_to_check = [
                Key::A,
                Key::B,
                Key::C,
                Key::D,
                Key::E,
                Key::F,
                Key::G,
                Key::H,
                Key::I,
                Key::J,
                Key::K,
                Key::L,
                Key::M,
                Key::N,
                Key::O,
                Key::P,
                Key::Q,
                Key::R,
                Key::S,
                Key::T,
                Key::U,
                Key::V,
                Key::W,
                Key::X,
                Key::Y,
                Key::Z,
                Key::Key0,
                Key::Key1,
                Key::Key2,
                Key::Key3,
                Key::Key4,
                Key::Key5,
                Key::Key6,
                Key::Key7,
                Key::Key8,
                Key::Key9,
                Key::Space,
                Key::Enter,
                Key::Escape,
                Key::Tab,
                Key::Backspace,
            ];

            for key in &keys_to_check {
                let is_pressed = window.is_key_down(*key);
                let was_pressed = prev_states.get(key).unwrap_or(&false);

                // Detect key press event (was not pressed, now is pressed)
                if is_pressed && !was_pressed {
                    let key_code = match key {
                        Key::A => 'a' as i32,
                        Key::B => 'b' as i32,
                        Key::C => 'c' as i32,
                        Key::D => 'd' as i32,
                        Key::E => 'e' as i32,
                        Key::F => 'f' as i32,
                        Key::G => 'g' as i32,
                        Key::H => 'h' as i32,
                        Key::I => 'i' as i32,
                        Key::J => 'j' as i32,
                        Key::K => 'k' as i32,
                        Key::L => 'l' as i32,
                        Key::M => 'm' as i32,
                        Key::N => 'n' as i32,
                        Key::O => 'o' as i32,
                        Key::P => 'p' as i32,
                        Key::Q => 'q' as i32,
                        Key::R => 'r' as i32,
                        Key::S => 's' as i32,
                        Key::T => 't' as i32,
                        Key::U => 'u' as i32,
                        Key::V => 'v' as i32,
                        Key::W => 'w' as i32,
                        Key::X => 'x' as i32,
                        Key::Y => 'y' as i32,
                        Key::Z => 'z' as i32,
                        Key::Key0 => '0' as i32,
                        Key::Key1 => '1' as i32,
                        Key::Key2 => '2' as i32,
                        Key::Key3 => '3' as i32,
                        Key::Key4 => '4' as i32,
                        Key::Key5 => '5' as i32,
                        Key::Key6 => '6' as i32,
                        Key::Key7 => '7' as i32,
                        Key::Key8 => '8' as i32,
                        Key::Key9 => '9' as i32,
                        Key::Space => ' ' as i32,
                        Key::Enter => 13,    // CR
                        Key::Escape => 27,   // ESC
                        Key::Tab => 9,       // TAB
                        Key::Backspace => 8, // BS
                        _ => 0,
                    };

                    if key_code != 0 {
                        events.push(InputEvent::Key {
                            window_id: *window_id,
                            key_code,
                            extended: false,
                        });
                    }
                }

                // Update the previous state
                prev_states.insert(*key, is_pressed);
            }

            // Poll mouse position and button states
            if let Some((mx, my)) = window.get_mouse_pos(MouseMode::Clamp) {
                let left = window.get_mouse_down(MouseButton::Left);
                let right = window.get_mouse_down(MouseButton::Right);
                let middle = window.get_mouse_down(MouseButton::Middle);

                // Get previous mouse state for this window
                let prev_mouse = self
                    .previous_mouse_states
                    .entry(*window_id)
                    .or_insert((false, false, false));

                // Emit mouse event with current position and button mask
                // buttons: bit 0 = left, bit 1 = right, bit 2 = middle
                let buttons = (if left { 1u32 } else { 0 })
                    | (if right { 2u32 } else { 0 })
                    | (if middle { 4u32 } else { 0 });

                events.push(InputEvent::Mouse {
                    window_id: *window_id,
                    x: mx as i32,
                    y: my as i32,
                    buttons,
                });

                // Update previous mouse state
                *prev_mouse = (left, right, middle);
            }
        }

        events
    }

    fn has_events(&self) -> bool {
        for window in self.windows.values() {
            let keys = window.get_keys();
            if !keys.is_empty() {
                return true;
            }
        }
        false
    }

    fn set_fullscreen(&mut self, _window_id: WindowId, _fullscreen: bool) -> BgiResult<()> {
        // TODO: Implement fullscreen mode
        Ok(())
    }

    fn screen_size(&self) -> BgiResult<(u32, u32)> {
        Ok((1920, 1080)) // Default screen size
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
        if let Some(buffer) = self.buffers.get(&window_id) {
            Ok(buffer.clone())
        } else {
            Err(BgiError::InvalidWindow)
        }
    }

    fn set_buffer(&mut self, window_id: WindowId, buffer: &[u32]) -> BgiResult<()> {
        if let Some(existing_buffer) = self.buffers.get_mut(&window_id) {
            if buffer.len() == existing_buffer.len() {
                existing_buffer.copy_from_slice(buffer);
                Ok(())
            } else {
                Err(BgiError::InvalidParameter(
                    "Buffer size mismatch".to_string(),
                ))
            }
        } else {
            Err(BgiError::InvalidWindow)
        }
    }

    fn load_image(&mut self, _filename: &str) -> BgiResult<(u32, u32, Vec<u32>)> {
        // TODO: Implement image loading
        Err(BgiError::NotSupported(
            "Image loading not implemented".to_string(),
        ))
    }

    fn save_image(
        &self,
        _filename: &str,
        _width: u32,
        _height: u32,
        _pixels: &[u32],
    ) -> BgiResult<()> {
        // TODO: Implement image saving
        Ok(())
    }
}
