//! Shape drawing functions for BGI graphics.

use crate::{
    graphics::{with_graphics_state, with_graphics_state_mut},
    Color, RgbColor,
};

/// Draw a line from (x1, y1) to (x2, y2).
pub fn line(x1: i32, y1: i32, x2: i32, y2: i32) {
    with_graphics_state_mut(|state| {
        let color = state.drawing_state.get_color();
        let pattern = state.drawing_state.get_line_pattern();
        let active_page = state.window_state.pages.active_page;

        // Line drawing using Bresenham's algorithm with pattern support
        draw_line_to_page(
            &mut state.pages,
            active_page,
            x1,
            y1,
            x2,
            y2,
            color,
            pattern,
        );

        // Present to visual backend if available
        #[cfg(feature = "visual-backend")]
        {
            if let (Some(backend), Some(window_id)) = (&mut state.backend, state.current_window) {
                use crate::backend::DrawCommand;
                let rgb_color = color.to_rgb();
                let commands = vec![DrawCommand::Line {
                    x1,
                    y1,
                    x2,
                    y2,
                    color: rgb_color,
                }];
                if let Err(_) = backend.draw(window_id, &commands) {
                    // Ignore draw errors to maintain BGI compatibility
                }
                // Only present if not in batch mode
                if !state.drawing_state.batch_mode {
                    if let Err(_) = backend.present(window_id) {
                        // Ignore present errors to maintain BGI compatibility
                    }
                }
            }
        }
    });
}

/// Draw a line to the current position.
pub fn lineto(x: i32, y: i32) {
    with_graphics_state_mut(|state| {
        let (current_x, current_y) = state.drawing_state.get_position();
        let color = state.drawing_state.get_color();
        let pattern = state.drawing_state.get_line_pattern();
        let active_page = state.window_state.pages.active_page;

        // Draw line from current position to (x, y)
        draw_line_to_page(
            &mut state.pages,
            active_page,
            current_x,
            current_y,
            x,
            y,
            color,
            pattern,
        );

        // Update current position
        state.drawing_state.move_to(x, y);
    });
}

/// Move to a position without drawing.
pub fn moveto(x: i32, y: i32) {
    with_graphics_state_mut(|state| {
        state.drawing_state.move_to(x, y);
    });
}

/// Draw a circle with center (x, y) and given radius.
pub fn circle(x: i32, y: i32, radius: i32) {
    with_graphics_state_mut(|state| {
        let color = state.drawing_state.get_color();
        let active_page = state.window_state.pages.active_page;

        // Simple circle drawing
        draw_circle_to_page(&mut state.pages, active_page, x, y, radius, color);

        // Present to visual backend if available
        #[cfg(feature = "visual-backend")]
        {
            if let (Some(backend), Some(window_id)) = (&mut state.backend, state.current_window) {
                use crate::backend::DrawCommand;
                let rgb_color = color.to_rgb();
                let commands = vec![DrawCommand::Circle {
                    x,
                    y,
                    radius,
                    color: rgb_color,
                    filled: false,
                }];
                if let Err(_) = backend.draw(window_id, &commands) {
                    // Ignore draw errors to maintain BGI compatibility
                }
                // Only present if not in batch mode
                if !state.drawing_state.batch_mode {
                    if let Err(_) = backend.present(window_id) {
                        // Ignore present errors to maintain BGI compatibility
                    }
                }
            }
        }
    });
}

/// Draw an arc of a circle.
pub fn arc(x: i32, y: i32, start_angle: i32, end_angle: i32, radius: i32) {
    with_graphics_state_mut(|state| {
        let color = state.drawing_state.get_color();
        let active_page = state.window_state.pages.active_page;

        // Simple arc drawing
        draw_arc_to_page(
            &mut state.pages,
            active_page,
            x,
            y,
            start_angle,
            end_angle,
            radius,
            color,
        );
    });
}

/// Draw a rectangle from (left, top) to (right, bottom).
pub fn rectangle(left: i32, top: i32, right: i32, bottom: i32) {
    with_graphics_state_mut(|state| {
        let color = state.drawing_state.get_color();
        let pattern = state.drawing_state.get_line_pattern();
        let active_page = state.window_state.pages.active_page;

        // Draw rectangle using four lines
        draw_line_to_page(
            &mut state.pages,
            active_page,
            left,
            top,
            right,
            top,
            color,
            pattern,
        );
        draw_line_to_page(
            &mut state.pages,
            active_page,
            right,
            top,
            right,
            bottom,
            color,
            pattern,
        );
        draw_line_to_page(
            &mut state.pages,
            active_page,
            right,
            bottom,
            left,
            bottom,
            color,
            pattern,
        );
        draw_line_to_page(
            &mut state.pages,
            active_page,
            left,
            bottom,
            left,
            top,
            color,
            pattern,
        );

        // Present to visual backend if available
        #[cfg(feature = "visual-backend")]
        {
            if let (Some(backend), Some(window_id)) = (&mut state.backend, state.current_window) {
                use crate::backend::DrawCommand;
                let rgb_color = color.to_rgb();
                let commands = vec![DrawCommand::Rectangle {
                    x1: left,
                    y1: top,
                    x2: right,
                    y2: bottom,
                    color: rgb_color,
                    filled: false,
                }];
                if let Err(_) = backend.draw(window_id, &commands) {
                    // Ignore draw errors to maintain BGI compatibility
                }
                // Only present if not in batch mode
                if !state.drawing_state.batch_mode {
                    if let Err(_) = backend.present(window_id) {
                        // Ignore present errors to maintain BGI compatibility
                    }
                }
            }
        }
    });
}

/// Draw an ellipse.
pub fn ellipse(x: i32, y: i32, start_angle: i32, end_angle: i32, x_radius: i32, y_radius: i32) {
    with_graphics_state_mut(|state| {
        let color = state.drawing_state.get_color();
        let active_page = state.window_state.pages.active_page;

        // Simple ellipse drawing
        draw_ellipse_to_page(
            &mut state.pages,
            active_page,
            x,
            y,
            start_angle,
            end_angle,
            x_radius,
            y_radius,
            color,
        );
    });
}

/// Put a pixel at (x, y) with specified color.
pub fn putpixel(x: i32, y: i32, color: Color) {
    with_graphics_state_mut(|state| {
        let active_page = state.window_state.pages.active_page;
        set_pixel_in_page(&mut state.pages, active_page, x, y, color);

        // Present to visual backend if available and not in batch mode
        #[cfg(feature = "visual-backend")]
        {
            if let (Some(backend), Some(window_id)) = (&mut state.backend, state.current_window) {
                use crate::backend::DrawCommand;
                let rgb_color = color.to_rgb();
                let commands = vec![DrawCommand::Pixel {
                    x,
                    y,
                    color: rgb_color,
                }];
                if let Err(_) = backend.draw(window_id, &commands) {
                    // Ignore draw errors to maintain BGI compatibility
                }

                // Only present if not in batch mode
                if !state.drawing_state.batch_mode {
                    if let Err(_) = backend.present(window_id) {
                        // Ignore present errors to maintain BGI compatibility
                    }
                }
            }
        }
    });
}

/// Get the color of pixel at (x, y).
pub fn getpixel(x: i32, y: i32) -> Color {
    with_graphics_state(|state| {
        let active_page = state.window_state.pages.active_page;
        get_pixel_from_page(&state.pages, active_page, x, y)
    })
    .unwrap_or(Color::BLACK)
}

/// Draw and fill a polygon.
pub fn fillpoly(points: &[(i32, i32)]) {
    if points.len() < 3 {
        return;
    }

    with_graphics_state_mut(|state| {
        let fill_color = state.drawing_state.fill_style.color;
        let active_page = state.window_state.pages.active_page;

        // Simple polygon fill (draw outline for now)
        for i in 0..points.len() {
            let next_i = (i + 1) % points.len();
            let (x1, y1) = points[i];
            let (x2, y2) = points[next_i];
            draw_line_to_page(
                &mut state.pages,
                active_page,
                x1,
                y1,
                x2,
                y2,
                fill_color,
                0xFFFF,
            );
        }
    });
}

// Internal helper functions for drawing operations

use std::collections::HashMap;

fn draw_line_to_page(
    pages: &mut HashMap<i32, Vec<u8>>,
    page: i32,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    color: Color,
    pattern: u16,
) {
    if pages.get(&page).is_none() {
        return;
    }

    // Bresenham's line algorithm with pattern support
    let mut x0 = x1;
    let mut y0 = y1;
    let x1 = x2;
    let y1 = y2;

    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    let mut pixel_count = 0u16; // Track pixel position for pattern

    loop {
        // Check if this pixel should be drawn based on the pattern
        let bit_position = pixel_count % 16;
        let should_draw = (pattern >> (15 - bit_position)) & 1 == 1;

        if should_draw {
            set_pixel_in_page(pages, page, x0, y0, color);
        }

        pixel_count += 1;

        if x0 == x1 && y0 == y1 {
            break;
        }

        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}

fn draw_circle_to_page(
    pages: &mut HashMap<i32, Vec<u8>>,
    page: i32,
    cx: i32,
    cy: i32,
    radius: i32,
    color: Color,
) {
    if let Some(_page_data) = pages.get_mut(&page) {
        // Midpoint circle algorithm would be implemented here
        // For now, just approximate with 8 points
        let r = radius as f64;
        for i in 0..360 {
            let angle = (i as f64) * std::f64::consts::PI / 180.0;
            let x = cx + (r * angle.cos()) as i32;
            let y = cy + (r * angle.sin()) as i32;
            set_pixel_in_page(pages, page, x, y, color);
        }
    }
}

fn draw_arc_to_page(
    pages: &mut HashMap<i32, Vec<u8>>,
    page: i32,
    cx: i32,
    cy: i32,
    start_angle: i32,
    end_angle: i32,
    radius: i32,
    color: Color,
) {
    if let Some(_page_data) = pages.get_mut(&page) {
        let start_rad = (start_angle as f64) * std::f64::consts::PI / 180.0;
        let end_rad = (end_angle as f64) * std::f64::consts::PI / 180.0;
        let r = radius as f64;

        let x1 = cx + (r * start_rad.cos()) as i32;
        let y1 = cy + (r * start_rad.sin()) as i32;
        let x2 = cx + (r * end_rad.cos()) as i32;
        let y2 = cy + (r * end_rad.sin()) as i32;

        draw_line_to_page(pages, page, cx, cy, x1, y1, color, 0xFFFF);
        draw_line_to_page(pages, page, cx, cy, x2, y2, color, 0xFFFF);
    }
}

fn draw_ellipse_to_page(
    pages: &mut HashMap<i32, Vec<u8>>,
    page: i32,
    cx: i32,
    cy: i32,
    _start_angle: i32,
    _end_angle: i32,
    x_radius: i32,
    y_radius: i32,
    color: Color,
) {
    if let Some(_page_data) = pages.get_mut(&page) {
        // Simple ellipse approximation using parametric form
        for i in 0..360 {
            let angle = (i as f64) * std::f64::consts::PI / 180.0;
            let x = cx + ((x_radius as f64) * angle.cos()) as i32;
            let y = cy + ((y_radius as f64) * angle.sin()) as i32;
            set_pixel_in_page(pages, page, x, y, color);
        }
    }
}

/// Set a pixel color in a specific page.
pub fn set_pixel_in_page(
    pages: &mut HashMap<i32, Vec<u8>>,
    page: i32,
    x: i32,
    y: i32,
    color: Color,
) {
    if let Some(page_data) = pages.get_mut(&page) {
        if x >= 0 && y >= 0 && x < 640 && y < 480 {
            let index = ((y * 640 + x) * 3) as usize;
            if index + 2 < page_data.len() {
                let rgb = color.to_rgb();
                page_data[index] = rgb.r;
                page_data[index + 1] = rgb.g;
                page_data[index + 2] = rgb.b;
            }
        }
    }
}

/// Get pixel color from a specific page.
pub fn get_pixel_from_page(pages: &HashMap<i32, Vec<u8>>, page: i32, x: i32, y: i32) -> Color {
    if let Some(page_data) = pages.get(&page) {
        if x >= 0 && y >= 0 && x < 640 && y < 480 {
            let index = ((y * 640 + x) * 3) as usize;
            if index + 2 < page_data.len() {
                let r = page_data[index];
                let g = page_data[index + 1];
                let b = page_data[index + 2];
                return Color::Rgb(RgbColor::new(r, g, b));
            }
        }
    }
    Color::BLACK
}
