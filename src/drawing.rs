//! Drawing primitives for the BGI library.

use crate::color::RgbColor;
use crate::types::{ArcCoords, LineSettings, Point, Rect};

/// Drawing primitive operations.
#[derive(Debug, Clone)]
pub enum DrawingPrimitive {
    /// Clear the entire drawing surface.
    Clear {
        /// Color to clear with.
        color: RgbColor,
    },
    /// Draw a single pixel.
    Pixel {
        /// X coordinate.
        x: i32,
        /// Y coordinate.
        y: i32,
        /// Pixel color.
        color: RgbColor,
    },
    /// Draw a line between two points.
    Line {
        /// Start X coordinate.
        x1: i32,
        /// Start Y coordinate.
        y1: i32,
        /// End X coordinate.
        x2: i32,
        /// End Y coordinate.
        y2: i32,
        /// Line color.
        color: RgbColor,
        /// Line settings.
        settings: LineSettings,
    },
    /// Draw a rectangle.
    Rectangle {
        /// Rectangle bounds.
        rect: Rect,
        /// Line color.
        color: RgbColor,
        /// Line settings.
        settings: LineSettings,
    },
    /// Draw a filled rectangle (bar).
    Bar {
        /// Rectangle bounds.
        rect: Rect,
        /// Fill color.
        color: RgbColor,
    },
    /// Draw a 3D bar.
    Bar3D {
        /// Rectangle bounds.
        rect: Rect,
        /// Bar depth.
        depth: i32,
        /// Top flag (true for raised, false for pressed).
        top_flag: bool,
        /// Fill color.
        color: RgbColor,
    },
    /// Draw a circle.
    Circle {
        /// Center X coordinate.
        x: i32,
        /// Center Y coordinate.
        y: i32,
        /// Radius.
        radius: i32,
        /// Line color.
        color: RgbColor,
        /// Line settings.
        settings: LineSettings,
    },
    /// Draw a filled circle.
    FillCircle {
        /// Center X coordinate.
        x: i32,
        /// Center Y coordinate.
        y: i32,
        /// Radius.
        radius: i32,
        /// Fill color.
        color: RgbColor,
    },
    /// Draw an ellipse.
    Ellipse {
        /// Center X coordinate.
        x: i32,
        /// Center Y coordinate.
        y: i32,
        /// Start angle.
        start_angle: i32,
        /// End angle.
        end_angle: i32,
        /// X radius.
        x_radius: i32,
        /// Y radius.
        y_radius: i32,
        /// Line color.
        color: RgbColor,
        /// Line settings.
        settings: LineSettings,
    },
    /// Draw a filled ellipse.
    FillEllipse {
        /// Center X coordinate.
        x: i32,
        /// Center Y coordinate.
        y: i32,
        /// X radius.
        x_radius: i32,
        /// Y radius.
        y_radius: i32,
        /// Fill color.
        color: RgbColor,
    },
    /// Draw an arc.
    Arc {
        /// Center X coordinate.
        x: i32,
        /// Center Y coordinate.
        y: i32,
        /// Start angle (degrees).
        start_angle: i32,
        /// End angle (degrees).
        end_angle: i32,
        /// Radius.
        radius: i32,
        /// Line color.
        color: RgbColor,
        /// Line settings.
        settings: LineSettings,
    },
    /// Draw a pie slice.
    PieSlice {
        /// Center X coordinate.
        x: i32,
        /// Center Y coordinate.
        y: i32,
        /// Start angle (degrees).
        start_angle: i32,
        /// End angle (degrees).
        end_angle: i32,
        /// Radius.
        radius: i32,
        /// Fill color.
        color: RgbColor,
    },
    /// Draw a sector (arc with radial lines).
    Sector {
        /// Center X coordinate.
        x: i32,
        /// Center Y coordinate.
        y: i32,
        /// Start angle (degrees).
        start_angle: i32,
        /// End angle (degrees).
        end_angle: i32,
        /// X radius.
        x_radius: i32,
        /// Y radius.
        y_radius: i32,
        /// Fill color.
        color: RgbColor,
    },
    /// Draw a polygon.
    Polygon {
        /// Vertex points.
        points: Vec<Point>,
        /// Line color.
        color: RgbColor,
        /// Line settings.
        settings: LineSettings,
    },
    /// Draw a filled polygon.
    FillPolygon {
        /// Vertex points.
        points: Vec<Point>,
        /// Fill color.
        color: RgbColor,
    },
    /// Flood fill operation.
    FloodFill {
        /// Seed X coordinate.
        x: i32,
        /// Seed Y coordinate.
        y: i32,
        /// Boundary color.
        boundary_color: RgbColor,
        /// Fill color.
        fill_color: RgbColor,
    },
    /// Draw text.
    Text {
        /// X coordinate.
        x: i32,
        /// Y coordinate.
        y: i32,
        /// Text string.
        text: String,
        /// Text color.
        color: RgbColor,
    },
    /// Put image data.
    Image {
        /// Destination X coordinate.
        x: i32,
        /// Destination Y coordinate.
        y: i32,
        /// Image width.
        width: u32,
        /// Image height.
        height: u32,
        /// Pixel data (ARGB format).
        pixels: Vec<u32>,
        /// Write mode.
        write_mode: i32,
    },
}

/// Arc coordinate information (for getarccoords compatibility).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ArcInfo {
    /// Last arc coordinates.
    pub coords: ArcCoords,
}

impl ArcInfo {
    /// Create new arc info.
    pub const fn new(coords: ArcCoords) -> Self {
        Self { coords }
    }

    /// Update arc coordinates from arc drawing operation.
    pub fn update_from_arc(
        &mut self,
        x: i32,
        y: i32,
        start_angle: i32,
        end_angle: i32,
        radius: i32,
    ) {
        let start_rad = (start_angle as f64).to_radians();
        let end_rad = (end_angle as f64).to_radians();

        self.coords = ArcCoords {
            x,
            y,
            x_start: x + (start_rad.cos() * radius as f64) as i32,
            y_start: y - (start_rad.sin() * radius as f64) as i32,
            x_end: x + (end_rad.cos() * radius as f64) as i32,
            y_end: y - (end_rad.sin() * radius as f64) as i32,
        };
    }
}
