//! Image manipulation functions for BGI graphics.

use crate::{Color, graphics::{with_graphics_state, with_graphics_state_mut}};
use std::collections::HashMap;

thread_local! {
    /// Stored images for putimage/getimage operations.
    static IMAGE_STORE: std::cell::RefCell<HashMap<u32, Vec<u8>>> = std::cell::RefCell::new(HashMap::new());
    static NEXT_IMAGE_ID: std::cell::RefCell<u32> = std::cell::RefCell::new(1);
}

/// Image data structure for BGI compatibility.
#[derive(Debug, Clone)]
pub struct ImageData {
    /// Image width
    pub width: i32,
    /// Image height
    pub height: i32,
    /// Pixel data (RGBA format)
    pub data: Vec<u8>,
}

impl ImageData {
    /// Create new image data.
    pub fn new(width: i32, height: i32) -> Self {
        let size = (width * height * 4) as usize; // 4 bytes per pixel (RGBA)
        Self {
            width,
            height,
            data: vec![0; size],
        }
    }
    
    /// Get pixel at coordinates.
    pub fn get_pixel(&self, x: i32, y: i32) -> Option<Color> {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return None;
        }
        
        let index = ((y * self.width + x) * 4) as usize;
        if index + 3 < self.data.len() {
            Some(Color::Rgb(crate::RgbColor {
                r: self.data[index],
                g: self.data[index + 1],
                b: self.data[index + 2],
                a: 255,
            }))
        } else {
            None
        }
    }

    /// Set pixel at coordinates.
    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return;
        }

        let index = ((y * self.width + x) * 4) as usize;
        if index + 3 < self.data.len() {
            let rgb = color.to_rgb();
            self.data[index] = rgb.r;
            self.data[index + 1] = rgb.g;
            self.data[index + 2] = rgb.b;
            self.data[index + 3] = 255; // Alpha
        }
    }
}

/// Get image from screen region.
pub fn getimage(left: i32, top: i32, right: i32, bottom: i32) -> Option<Vec<u8>> {
    // Check if graphics are initialized first
    let result = with_graphics_state(|state| {
        // Check for invalid coordinates or regions
        if left < 0 || top < 0 || right < left || bottom < top {
            return None;
        }

        let width = right - left + 1;
        let height = bottom - top + 1;

        if width <= 0 || height <= 0 {
            return None;
        }

        // Create image data buffer
        let mut image_data = ImageData::new(width, height);

        // Fill with current screen data (simplified - would normally read from actual screen buffer)
        let active_page = state.window_state.pages.active_page;
        if let Some(_page_data) = state.pages.get(&active_page) {
            // For now, just fill with background color
            let bg_color = state.window_state.properties.background_color;
            for y in 0..height {
                for x in 0..width {
                    image_data.set_pixel(x, y, bg_color);
                }
            }
        }

        // Store image and return identifier as bytes
        let image_id = NEXT_IMAGE_ID.with(|id| {
            let current_id = *id.borrow();
            *id.borrow_mut() = current_id + 1;
            current_id
        });

        // Store both the image data and dimensions
        IMAGE_STORE.with(|store| {
            // Store width and height in the first 8 bytes, then the pixel data
            let mut full_data = Vec::new();
            full_data.extend_from_slice(&width.to_le_bytes());
            full_data.extend_from_slice(&height.to_le_bytes());
            full_data.extend_from_slice(&image_data.data);
            store.borrow_mut().insert(image_id, full_data);
        });

        // Return image ID as bytes (BGI compatibility)
        Some(image_id.to_le_bytes().to_vec())
    });

    // Return None if graphics not initialized, otherwise return the result
    result.flatten()
}

/// Put image to screen at specified location.
pub fn putimage(left: i32, top: i32, image_data: &[u8], mode: i32) {
    if image_data.len() < 4 {
        return;
    }

    let image_id = u32::from_le_bytes([
        image_data[0],
        image_data[1],
        image_data[2],
        image_data[3]
    ]);

    IMAGE_STORE.with(|store| {
        if let Some(data) = store.borrow().get(&image_id) {
            // For now, just mark the operation as completed
            // Real implementation would draw to screen buffer
            with_graphics_state_mut(|_state| {
                // Drawing operation would go here
            });
        }
    });
}

/// Load image from file.
pub fn loadimage(filename: &str) -> Option<Vec<u8>> {
    // Stub implementation - would load actual image file
    // For now, return empty image
    None
}

/// Save current screen region to file.
pub fn saveimage(filename: &str, left: i32, top: i32, right: i32, bottom: i32) -> bool {
    // Stub implementation - would save actual image file
    false
}

/// Calculate size needed for image buffer.
pub fn imagesize(left: i32, top: i32, right: i32, bottom: i32) -> i32 {
    // Check if graphics are initialized first
    let result = with_graphics_state(|_state| {
        // Check for invalid coordinates or regions
        if left < 0 || top < 0 || right < left || bottom < top {
            return 0;
        }

        let width = right - left + 1;
        let height = bottom - top + 1;

        if width <= 0 || height <= 0 {
            return 0;
        }

        // BGI format: width, height, then pixel data
        4 + 4 + (width * height * 4) // 4 bytes for width, 4 for height, 4 bytes per pixel
    });

    // Return 0 if graphics not initialized, otherwise return the calculated size
    result.unwrap_or(0)
}

/// Get dimensions of image.
pub fn getimagesize(image_data: &[u8]) -> (i32, i32) {
    if image_data.len() < 4 {
        return (0, 0);
    }

    let image_id = u32::from_le_bytes([
        image_data[0],
        image_data[1],
        image_data[2],
        image_data[3]
    ]);

    IMAGE_STORE.with(|store| {
        if let Some(data) = store.borrow().get(&image_id) {
            // Extract width and height from the first 8 bytes
            if data.len() >= 8 {
                let width = i32::from_le_bytes([data[0], data[1], data[2], data[3]]);
                let height = i32::from_le_bytes([data[4], data[5], data[6], data[7]]);
                (width, height)
            } else {
                (0, 0)
            }
        } else {
            (0, 0)
        }
    })
}
