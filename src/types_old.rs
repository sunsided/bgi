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
    /// Hercules Monochrome.
    HercMono = 7,
    /// AT&T 400-line display.
    Att400 = 8,
    /// VGA (Video Graphics Array).
    Vga = 9,
    /// PC 3270 display.
    Pc3270 = 10,
}

/// Graphics result codes (BGI error codes).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphResult {
    /// No error.
    Ok,
    /// Graphics not initialized.
    NotInitialized,
    /// Invalid driver.
    InvalidDriver,
    /// Invalid mode.
    InvalidMode,
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

/// Rectangle defined by top-left and bottom-right corners.
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

/// Graphics mode information (stores device and mode combination).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GraphicsMode {
    /// Graphics device/driver.
    pub driver: GraphicsDriver,
    /// Mode number (context-dependent on driver).
    pub mode: i32,
}

impl GraphicsMode {
    /// Create a new graphics mode.
    pub const fn new(driver: GraphicsDriver, mode: i32) -> Self {
        Self { driver, mode }
    }
    
    /// Auto-detect mode.
    pub const DETECT: Self = Self::new(GraphicsDriver::Detect, -1);
    
    /// VGA modes.
    pub const VGA_LO: Self = Self::new(GraphicsDriver::Vga, 1);    // 640x200, 16 colors
    pub const VGA_HI: Self = Self::new(GraphicsDriver::Vga, 2);    // 640x480, 16 colors  
    pub const VGA_MED: Self = Self::new(GraphicsDriver::Vga, 3);   // 640x350, 16 colors
    
    /// CGA modes.
    pub const CGA_HI: Self = Self::new(GraphicsDriver::Cga, 1);    // 640x200, 2 colors
    pub const CGA_MED: Self = Self::new(GraphicsDriver::Cga, 2);   // 320x200, 4 colors
    pub const CGA_LO: Self = Self::new(GraphicsDriver::Cga, 3);    // 320x200, 2 colors
    
    /// EGA modes.
    pub const EGA_HI: Self = Self::new(GraphicsDriver::Ega, 3);    // 640x350, 16 colors
    pub const EGA_64_LO: Self = Self::new(GraphicsDriver::Ega64, 4); // 640x200, 16 colors (EGA64K only)
    pub const EGA_MED: Self = Self::new(GraphicsDriver::Ega, 5);   // 640x350, 4 colors
    
    /// MCGA/VGA 256-color mode.
    pub const MCGA_HI: Self = Self::new(GraphicsDriver::Mcga, 4);  // 320x200, 256 colors
}

impl GraphicsMode {
    /// Get the resolution for this graphics mode.
    pub fn resolution(self) -> Option<(u32, u32)> {
        match (self.driver, self.mode) {
            // CGA modes
            (GraphicsDriver::Cga, 1) => Some((640, 200)),  // CGA_HI
            (GraphicsDriver::Cga, 2) => Some((320, 200)),  // CGA_MED
            (GraphicsDriver::Cga, 3) => Some((320, 200)),  // CGA_LO
            
            // EGA modes
            (GraphicsDriver::Ega, 3) => Some((640, 350)),  // EGA_HI
            (GraphicsDriver::Ega64, 4) => Some((640, 200)), // EGA_64_LO
            (GraphicsDriver::Ega, 5) => Some((640, 350)),  // EGA_MED
            
            // VGA modes
            (GraphicsDriver::Vga, 1) => Some((640, 200)),  // VGA_LO
            (GraphicsDriver::Vga, 2) => Some((640, 480)),  // VGA_HI
            (GraphicsDriver::Vga, 3) => Some((640, 350)),  // VGA_MED
            
            // MCGA modes
            (GraphicsDriver::Mcga, 4) => Some((320, 200)), // MCGA_HI
            
            // Default for unknown combinations
            _ => Some((640, 480)), // Default VGA resolution
        }
    }
    
    /// Get the color depth for this graphics mode.
    pub fn color_depth(self) -> u32 {
        match (self.driver, self.mode) {
            // CGA modes
            (GraphicsDriver::Cga, 1) => 2,   // CGA_HI: 2 colors
            (GraphicsDriver::Cga, 2) => 4,   // CGA_MED: 4 colors
            (GraphicsDriver::Cga, 3) => 2,   // CGA_LO: 2 colors
            
            // EGA/VGA 16-color modes
            (GraphicsDriver::Ega, _) => 16,
            (GraphicsDriver::Ega64, _) => 16,
            (GraphicsDriver::Vga, 1..=3) => 16,
            
            // MCGA 256-color mode
            (GraphicsDriver::Mcga, 4) => 256,
            
            // Default
            _ => 16,
        }
    }
}

/// Line styles for drawing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum LineStyle {
    /// Solid line.
    Solid = 0,
    /// Dotted line.
    Dotted = 1,
    /// Center line.
    Center = 2,
    /// Dashed line.
    Dashed = 3,
    /// User-defined bit pattern.
    UserBit = 4,
}

/// Line thickness.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum LineThickness {
    /// Normal width (1 pixel).
    Normal = 1,
    /// Thick width (3 pixels).
    Thick = 3,
}

/// Write modes for drawing operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum WriteMode {
    /// Copy mode (replace).
    Copy = 0,
    /// XOR mode.
    Xor = 1,
    /// OR mode.
    Or = 2,
    /// AND mode.
    And = 3,
    /// NOT mode (invert).
    Not = 4,
}

/// Line settings structure (compatible with BGI linesettingstype).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineSettings {
    /// Line style.
    pub style: LineStyle,
    /// User pattern (for UserBit style).
    pub pattern: u32,
    /// Line thickness.
    pub thickness: LineThickness,
}

impl Default for LineSettings {
    fn default() -> Self {
        Self {
            style: LineStyle::Solid,
            pattern: 0,
            thickness: LineThickness::Normal,
        }
    }
}

/// Text direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum TextDirection {
    /// Horizontal text.
    Horizontal = 0,
    /// Vertical text.
    Vertical = 1,
}

/// Date structure (for compatibility).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Date {
    /// Year.
    pub year: i32,
    /// Day.
    pub day: i32,
    /// Month.
    pub month: i32,
}

/// Standard BGI colors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum Color {
    /// Black color (0).
    Black = 0,
    /// Blue color (1).
    Blue = 1,
    /// Green color (2).
    Green = 2,
    /// Cyan color (3).
    Cyan = 3,
    /// Red color (4).
    Red = 4,
    /// Magenta color (5).
    Magenta = 5,
    /// Brown color (6).
    Brown = 6,
    /// Light gray color (7).
    LightGray = 7,
    /// Dark gray color (8).
    DarkGray = 8,
    /// Light blue color (9).
    LightBlue = 9,
    /// Light green color (10).
    LightGreen = 10,
    /// Light cyan color (11).
    LightCyan = 11,
    /// Light red color (12).
    LightRed = 12,
    /// Light magenta color (13).
    LightMagenta = 13,
    /// Yellow color (14).
    Yellow = 14,
    /// White color (15).
    White = 15,
}

/// Mouse state structure for getmouse() function.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MouseState {
    /// Mouse X coordinate
    pub x: i32,
    /// Mouse Y coordinate  
    pub y: i32,
    /// Left mouse button pressed
    pub left: bool,
    /// Right mouse button pressed
    pub right: bool,
    /// Middle mouse button pressed
    pub middle: bool,
}
