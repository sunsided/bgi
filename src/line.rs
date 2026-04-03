//! Line drawing algorithms and styles for BGI.

use crate::backend::{Backend, DrawCommand};
use crate::color::RgbColor;
use crate::constants::*;
use crate::window::WindowId;

/// Line patterns for different line styles.
/// Based on the original BGI line patterns.
pub const LINE_PATTERNS: [u16; 5] = [
    0xFFFF, // SOLID_LINE  = 1111111111111111
    0xCCCC, // DOTTED_LINE = 1100110011001100
    0xF1F8, // CENTER_LINE = 1111000111111000
    0xF8F8, // DASHED_LINE = 1111100011111000
    0xFFFF, // USERBIT_LINE (default to solid, can be overridden)
];

/// Line style settings.
#[derive(Debug, Clone, Copy)]
pub struct LineStyle {
    pub style: i32,
    pub pattern: u16,
    pub thickness: i32,
}

impl Default for LineStyle {
    fn default() -> Self {
        Self {
            style: SOLID_LINE,
            pattern: LINE_PATTERNS[SOLID_LINE as usize],
            thickness: NORM_WIDTH,
        }
    }
}

/// Draw a line using the Bresenham algorithm with line patterns.
/// This is the core line drawing function that implements the same algorithm
/// as the original BGI line_copy() function.
pub fn draw_line_bresenham(
    backend: &mut dyn Backend,
    window_id: WindowId,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    color: RgbColor,
    line_style: LineStyle,
    draw_mode: i32,
) -> Result<(), crate::error::BgiError> {
    let mut counter = 0u16; // Counter for pattern pixel
    #[allow(unused)]
    let mut pixels_plotted = 0; // Debug counter (only checked in tests)
    let dx = (x2 - x1).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let dy = (y2 - y1).abs();
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = if dx > dy { dx } else { -dy } / 2;
    let mut x = x1;
    let mut y = y1;

    loop {
        // Check if we should plot this pixel based on the line pattern
        let should_plot = if line_style.style == SOLID_LINE {
            true
        } else {
            let pattern = line_style.pattern; // Use the pattern directly from line_style
                                              // Check the bit at position (counter % 16) in the pattern
            let bit_position = 15 - (counter % 16); // BGI patterns are read left-to-right
            (pattern >> bit_position) & 1 != 0
        };

        if should_plot {
            // Plot the pixel - for now we only support COPY_PUT mode
            // Other modes (XOR_PUT, OR_PUT, etc.) would require different implementations
            let commands = vec![DrawCommand::Pixel { x, y, color }];
            backend.draw(window_id, &commands)?;
            pixels_plotted += 1; // Only checked in test assertions
        }

        counter += 1;

        // Check if we've reached the end point
        if x == x2 && y == y2 {
            break;
        }

        // Bresenham algorithm step
        let e2 = err;
        if e2 > -dx {
            err -= dy;
            x += sx;
        }
        if e2 < dy {
            err += dx;
            y += sy;
        }
    }

    Ok(())
}

/// Draw a thick line by drawing multiple parallel lines.
/// This mimics the thick line implementation in the original BGI.
pub fn draw_thick_line(
    backend: &mut dyn Backend,
    window_id: WindowId,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    color: RgbColor,
    line_style: LineStyle,
    draw_mode: i32,
) -> Result<(), crate::error::BgiError> {
    // Draw the main line
    draw_line_bresenham(
        backend, window_id, x1, y1, x2, y2, color, line_style, draw_mode,
    )?;

    if line_style.thickness == THICK_WIDTH {
        // Determine line octant to decide how to draw thick lines
        let octant = get_octant(x2 - x1, y1 - y2);

        match octant {
            1 | 4 | 5 | 8 => {
                // Draw additional lines above and below
                draw_line_bresenham(
                    backend,
                    window_id,
                    x1,
                    y1 - 1,
                    x2,
                    y2 - 1,
                    color,
                    line_style,
                    draw_mode,
                )?;
                draw_line_bresenham(
                    backend,
                    window_id,
                    x1,
                    y1 + 1,
                    x2,
                    y2 + 1,
                    color,
                    line_style,
                    draw_mode,
                )?;
            }
            _ => {
                // Draw additional lines to the left and right
                draw_line_bresenham(
                    backend,
                    window_id,
                    x1 - 1,
                    y1,
                    x2 - 1,
                    y2,
                    color,
                    line_style,
                    draw_mode,
                )?;
                draw_line_bresenham(
                    backend,
                    window_id,
                    x1 + 1,
                    y1,
                    x2 + 1,
                    y2,
                    color,
                    line_style,
                    draw_mode,
                )?;
            }
        }
    }

    Ok(())
}

/// Draw a circle using the Bresenham midpoint circle algorithm.
/// This is the same algorithm as the original BGI circle_bresenham() function.
pub fn draw_circle_bresenham(
    backend: &mut dyn Backend,
    window_id: WindowId,
    x: i32,
    y: i32,
    radius: i32,
    color: RgbColor,
) -> Result<(), crate::error::BgiError> {
    if radius <= 0 {
        return Ok(());
    }

    let mut xx = -radius;
    let mut yy = 0;
    let mut err = 2 - 2 * radius;

    loop {
        // Plot the 8 symmetric points
        let commands = vec![
            DrawCommand::Pixel {
                x: x - xx,
                y: y + yy,
                color,
            }, // I quadrant
            DrawCommand::Pixel {
                x: x - yy,
                y: y - xx,
                color,
            }, // II quadrant
            DrawCommand::Pixel {
                x: x + xx,
                y: y - yy,
                color,
            }, // III quadrant
            DrawCommand::Pixel {
                x: x + yy,
                y: y + xx,
                color,
            }, // IV quadrant
        ];
        backend.draw(window_id, &commands)?;

        // Update for next iteration
        let radius_local = err;

        if radius_local <= yy {
            yy += 1;
            err += yy * 2 + 1;
        }

        if radius_local > xx || err > yy {
            xx += 1;
            err += xx * 2 + 1;
        }

        if xx >= 0 {
            break;
        }
    }

    Ok(())
}

/// Draw a thick circle by drawing multiple concentric circles.
pub fn draw_thick_circle(
    backend: &mut dyn Backend,
    window_id: WindowId,
    x: i32,
    y: i32,
    radius: i32,
    color: RgbColor,
    thickness: i32,
) -> Result<(), crate::error::BgiError> {
    if thickness == NORM_WIDTH {
        // Normal thickness - just draw the circle
        draw_circle_bresenham(backend, window_id, x, y, radius, color)
    } else {
        // Thick circle - draw multiple concentric circles
        draw_circle_bresenham(backend, window_id, x, y, radius, color)?;
        if radius > 0 {
            draw_circle_bresenham(backend, window_id, x, y, radius - 1, color)?;
        }
        if radius > 1 {
            draw_circle_bresenham(backend, window_id, x, y, radius + 1, color)?;
        }
        Ok(())
    }
}

/// Draw a rectangle using four line calls, just like the original BGI.
/// This ensures that line patterns and drawing modes are applied consistently
/// to all edges of the rectangle.
pub fn draw_rectangle_lines(
    backend: &mut dyn Backend,
    window_id: WindowId,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    color: RgbColor,
    line_style: LineStyle,
    draw_mode: i32,
) -> Result<(), crate::error::BgiError> {
    // Draw the four sides of the rectangle using line calls
    // Top line: (x1, y1) to (x2, y1)
    draw_thick_line(
        backend, window_id, x1, y1, x2, y1, color, line_style, draw_mode,
    )?;

    // Right line: (x2, y1) to (x2, y2)
    draw_thick_line(
        backend, window_id, x2, y1, x2, y2, color, line_style, draw_mode,
    )?;

    // Bottom line: (x2, y2) to (x1, y2)
    draw_thick_line(
        backend, window_id, x2, y2, x1, y2, color, line_style, draw_mode,
    )?;

    // Left line: (x1, y2) to (x1, y1)
    draw_thick_line(
        backend, window_id, x1, y2, x1, y1, color, line_style, draw_mode,
    )?;

    Ok(())
}

/// Draw an ellipse using John Kennedy's fast Bresenham-type algorithm.
/// This is the same algorithm as the original BGI _ellipse() function.
/// From "A Fast Bresenham Type Algorithm For Drawing Ellipses" by John Kennedy.
pub fn draw_ellipse_bresenham(
    backend: &mut dyn Backend,
    window_id: WindowId,
    cx: i32,
    cy: i32,
    xradius: i32,
    yradius: i32,
    color: RgbColor,
) -> Result<(), crate::error::BgiError> {
    if xradius == 0 && yradius == 0 {
        return Ok(());
    }

    let two_a_square = 2 * xradius * xradius;
    let two_b_square = 2 * yradius * yradius;
    let mut x = xradius;
    let mut y = 0;
    let mut x_change = yradius * yradius * (1 - 2 * xradius);
    let mut y_change = xradius * xradius;
    let mut ellipse_error = 0;
    let mut stopping_x = two_b_square * xradius;
    let mut stopping_y = 0;

    // First set of points, y' > -1
    while stopping_x >= stopping_y {
        let commands = vec![
            DrawCommand::Pixel {
                x: cx + x,
                y: cy - y,
                color,
            },
            DrawCommand::Pixel {
                x: cx - x,
                y: cy - y,
                color,
            },
            DrawCommand::Pixel {
                x: cx - x,
                y: cy + y,
                color,
            },
            DrawCommand::Pixel {
                x: cx + x,
                y: cy + y,
                color,
            },
        ];
        backend.draw(window_id, &commands)?;

        y += 1;
        stopping_y += two_a_square;
        ellipse_error += y_change;
        y_change += two_a_square;

        if (2 * ellipse_error + x_change) > 0 {
            x -= 1;
            stopping_x -= two_b_square;
            ellipse_error += x_change;
            x_change += two_b_square;
        }
    }

    // 1st point set is done; start the 2nd set of points
    x = 0;
    y = yradius;
    x_change = yradius * yradius;
    y_change = xradius * xradius * (1 - 2 * yradius);
    ellipse_error = 0;
    stopping_x = 0;
    stopping_y = two_a_square * yradius;

    // Second set of points, y' < -1
    while stopping_x <= stopping_y {
        let commands = vec![
            DrawCommand::Pixel {
                x: cx + x,
                y: cy - y,
                color,
            },
            DrawCommand::Pixel {
                x: cx - x,
                y: cy - y,
                color,
            },
            DrawCommand::Pixel {
                x: cx - x,
                y: cy + y,
                color,
            },
            DrawCommand::Pixel {
                x: cx + x,
                y: cy + y,
                color,
            },
        ];
        backend.draw(window_id, &commands)?;

        x += 1;
        stopping_x += two_b_square;
        ellipse_error += x_change;
        x_change += two_b_square;

        if (2 * ellipse_error + y_change) > 0 {
            y -= 1;
            stopping_y -= two_a_square;
            ellipse_error += y_change;
            y_change += two_a_square;
        }
    }

    Ok(())
}

/// Draw an elliptical arc using line approximation.
/// This matches the approach in the original BGI ellipse() function.
pub fn draw_ellipse_arc(
    backend: &mut dyn Backend,
    window_id: WindowId,
    x: i32,
    y: i32,
    start_angle: i32,
    end_angle: i32,
    xradius: i32,
    yradius: i32,
    color: RgbColor,
    line_style: LineStyle,
    draw_mode: i32,
) -> Result<(), crate::error::BgiError> {
    if xradius == 0 && yradius == 0 {
        return Ok(());
    }

    let mut end_angle = end_angle;
    if end_angle < start_angle {
        end_angle += 360;
    }

    // For a complete ellipse, use the optimized Bresenham algorithm
    if start_angle == 0 && end_angle == 360 {
        return draw_ellipse_bresenham(backend, window_id, x, y, xradius, yradius, color);
    }

    // For arcs, use line approximation (like the original BGI)
    const PI_CONV: f64 = std::f64::consts::PI / 180.0;

    for angle in start_angle..end_angle {
        let angle_rad = angle as f64 * PI_CONV;
        let next_angle_rad = (angle + 1) as f64 * PI_CONV;

        let x1 = x + (xradius as f64 * angle_rad.cos()) as i32;
        let y1 = y - (yradius as f64 * angle_rad.sin()) as i32;
        let x2 = x + (xradius as f64 * next_angle_rad.cos()) as i32;
        let y2 = y - (yradius as f64 * next_angle_rad.sin()) as i32;

        draw_thick_line(
            backend, window_id, x1, y1, x2, y2, color, line_style, draw_mode,
        )?;
    }

    Ok(())
}

/// Determine the octant of a line for thick line drawing.
/// This is the same octant function from the original BGI.
fn get_octant(dx: i32, dy: i32) -> i32 {
    if dx >= 0 {
        if dy >= 0 {
            if dx >= dy {
                1 // First octant
            } else {
                2 // Second octant
            }
        } else {
            let neg_dy = -dy;
            if dx >= neg_dy {
                8 // Eighth octant
            } else {
                7 // Seventh octant
            }
        }
    } else {
        let neg_dx = -dx;
        if dy >= 0 {
            if neg_dx >= dy {
                4 // Fourth octant
            } else {
                3 // Third octant
            }
        } else {
            if neg_dx >= -dy {
                5 // Fifth octant
            } else {
                6 // Sixth octant
            }
        }
    }
}
