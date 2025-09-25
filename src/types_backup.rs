//! Core type definitions for the BGI library.

/// Graphics driver types (BGI compatible device constants).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GraphicsDriver {
    /// Auto-detect driver.
    Detect = 0,
    /// CGA (Color Graphics Adapter).
    Cga = 1,
    /// MCGA (Multi-Color Graphics Array).
    Mcga = 2,
    /// EGA (Enhanced Graphics Adapter).
    Ega = 3,
    /// EGA with 64K memory.
    Ega64 = 4,
    /// EGA Monochrome.
    EgaMono = 5,
    /// IBM 8514 display.
    Ibm8514 = 6,
    /// Hercules display.
    Hercules = 7,
    /// AT&T 400 line.
    Att400 = 8,
    /// VGA (Video Graphics Array).
    Vga = 9,
    /// PC-3270.
    Pc3270 = 10,
}

/// Graphics result codes (BGI error codes).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphResult {
    /// No error.
    Ok,
    /// Graphics not initialized.
    NotInitialized,
    /// Graphics not initialized (alternative name for BGI compatibility).
    GraphicsNotInitialized,
    /// General graphics error.
    GraphicsError,
    /// Invalid driver.
    InvalidDriver,
    /// Invalid mode.
    InvalidMode,
    /// Device driver file not found.
    DeviceDriverFileNotFound,
    /// Invalid device driver file.
    InvalidDeviceDriverFile,
    /// Not enough memory for driver.
    NotEnoughMemoryForDriver,
    /// Out of memory.
    OutOfMemory,
    /// I/O error.
    IOError,
    /// Font file error.
    FontFileError,
}

impl From<GraphResult> for i32 {
    fn from(result: GraphResult) -> Self {
        match result {
            GraphResult::Ok => 0,
            GraphResult::NotInitialized => -2,
            GraphResult::GraphicsNotInitialized => -2,
            GraphResult::GraphicsError => -1,
            GraphResult::InvalidDriver => -3,
            GraphResult::InvalidMode => -10,
            GraphResult::DeviceDriverFileNotFound => -3,
            GraphResult::InvalidDeviceDriverFile => -11,
            GraphResult::NotEnoughMemoryForDriver => -12,
            GraphResult::OutOfMemory => -13,
            GraphResult::IOError => -14,
            GraphResult::FontFileError => -8,
        }
    }
}

/// Graphics mode type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GraphicsMode {
    pub driver: GraphicsDriver,
    pub mode: i32,
}

impl GraphicsMode {
    /// Create a new graphics mode.
    pub const fn new(driver: GraphicsDriver, mode: i32) -> Self {
        Self { driver, mode }
    }

    /// Get resolution for this mode.
    pub fn resolution(&self) -> Option<(i32, i32)> {
        match (self.driver, self.mode) {
            (GraphicsDriver::Vga, 0) => Some((640, 200)),
            (GraphicsDriver::Vga, 1) => Some((640, 350)),
            (GraphicsDriver::Vga, 2) => Some((640, 480)),
            (GraphicsDriver::Vga, 3) => Some((800, 600)),
            _ => Some((640, 480)), // Default
        }
    }
}

impl Default for GraphicsMode {
    fn default() -> Self {
        Self::new(GraphicsDriver::Vga, 2)
    }
}

/// Point in 2D space.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    /// X coordinate.
    pub x: i32,
    /// Y coordinate.
    pub y: i32,
}

impl Point {
    /// Create a new point.
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// Rectangle structure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    /// Left coordinate.
    pub left: i32,
    /// Top coordinate.
    pub top: i32,
    /// Right coordinate.
    pub right: i32,
    /// Bottom coordinate.
    pub bottom: i32,
}

impl Rect {
    /// Create a new rectangle.
    pub const fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    /// Get width of the rectangle.
    pub fn width(&self) -> i32 {
        (self.right - self.left).abs()
    }

    /// Get height of the rectangle.
    pub fn height(&self) -> i32 {
        (self.bottom - self.top).abs()
    }

    /// Check if a point is inside the rectangle.
    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.left
            && point.x <= self.right
            && point.y >= self.top
            && point.y <= self.bottom
    }
}

/// Arc coordinates (compatible with BGI arccoordstype).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ArcCoords {
    /// Center X coordinate.
    pub x: i32,
    /// Center Y coordinate.
    pub y: i32,
    /// Arc start X coordinate.
    pub x_start: i32,
    /// Arc start Y coordinate.
    pub y_start: i32,
    /// Arc end X coordinate.
    pub x_end: i32,
    /// Arc end Y coordinate.
    pub y_end: i32,
}

/// Date structure (compatible with BGI).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateType {
    /// Year.
    pub year: i32,
    /// Day.
    pub day: i32,
    /// Month.
    pub month: i32,
}

/// Mouse state structure for getmouse() function.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MouseState {
    /// X coordinate
    pub x: i32,
    /// Y coordinate
    pub y: i32,
    /// Left mouse button pressed
    pub left: bool,
    /// Right mouse button pressed
    pub right: bool,
    /// Middle mouse button pressed
    pub middle: bool,
}

/// BGI-compatible text settings structure (textsettingstype).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BgiTextSettings {
    /// Font type.
    pub font: i32,
    /// Text direction.
    pub direction: i32,
    /// Character size.
    pub charsize: i32,
    /// Horizontal justification.
    pub horiz: i32,
    /// Vertical justification.
    pub vert: i32,
}
