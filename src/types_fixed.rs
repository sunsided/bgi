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
    /// EGA64 (EGA with 64K memory).
    Ega64 = 4,
    /// EGAMono (EGA monochrome).
    EgaMono = 5,
    /// IBM8514 (IBM 8514 graphics).
    Ibm8514 = 6,
    /// HercMono (Hercules monochrome).
    HercMono = 7,
    /// ATT400 (AT&T 400 line graphics).
    Att400 = 8,
    /// VGA (Video Graphics Array).
    Vga = 9,
    /// PC3270 (PC 3270 graphics).
    Pc3270 = 10,
}

/// Graphics modes supported by the BGI library.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GraphicsMode {
    /// CGA Color mode 0 (320x200, 4 colors).
    CgaC0 = 0,
    /// CGA Color mode 1 (320x200, 4 colors).
    CgaC1 = 1,
    /// CGA Color mode 2 (320x200, 4 colors).
    CgaC2 = 2,
    /// CGA Color mode 3 (320x200, 4 colors).
    CgaC3 = 3,
    /// CGA High resolution (640x200, 2 colors).
    CgaHi = 4,
    /// MCGA Color mode 0 (320x200, 256 colors).
    McgaC0 = 0,
    /// MCGA Color mode 1 (320x200, 256 colors).
    McgaC1 = 1,
    /// MCGA Color mode 2 (640x200, 2 colors).
    McgaC2 = 2,
    /// MCGA Color mode 3 (640x480, 2 colors).
    McgaC3 = 3,
    /// MCGA Medium resolution (640x350, 2 colors).
    McgaMed = 4,
    /// MCGA High resolution (640x480, 2 colors).
    McgaHi = 5,
    /// EGA Low resolution (640x200, 16 colors).
    EgaLo = 0,
    /// EGA High resolution (640x350, 16 colors).
    EgaHi = 1,
    /// VGA Low resolution (640x200, 16 colors).
    VgaLo = 0,
    /// VGA Medium resolution (640x350, 16 colors).
    VgaMed = 1,
    /// VGA High resolution (640x480, 16 colors).
    VgaHi = 2,
    /// Hercules monochrome (720x348, 2 colors).
    HercMono = 0,
    /// ATT400 Color mode 0 (320x200, 4 colors).
    Att400C0 = 0,
    /// ATT400 Color mode 1 (320x200, 4 colors).
    Att400C1 = 1,
    /// ATT400 Color mode 2 (320x200, 4 colors).
    Att400C2 = 2,
    /// ATT400 Color mode 3 (320x200, 4 colors).
    Att400C3 = 3,
    /// ATT400 Medium resolution (640x200, 2 colors).
    Att400Med = 4,
    /// ATT400 High resolution (640x400, 2 colors).
    Att400Hi = 5,
    /// PC3270 monochrome (720x350, 2 colors).
    Pc3270Hi = 0,
    /// IBM8514 Low resolution (640x480, 256 colors).
    Ibm8514Lo = 0,
    /// IBM8514 High resolution (1024x768, 256 colors).
    Ibm8514Hi = 1,
}

impl GraphicsMode {
    /// Get the resolution (width, height) for this graphics mode.
    pub fn resolution(self) -> (i32, i32) {
        match self {
            GraphicsMode::CgaC0 | GraphicsMode::CgaC1 | GraphicsMode::CgaC2 | GraphicsMode::CgaC3 => (320, 200),
            GraphicsMode::CgaHi => (640, 200),
            GraphicsMode::McgaC0 | GraphicsMode::McgaC1 => (320, 200),
            GraphicsMode::McgaC2 => (640, 200),
            GraphicsMode::McgaC3 | GraphicsMode::McgaHi => (640, 480),
            GraphicsMode::McgaMed => (640, 350),
            GraphicsMode::EgaLo => (640, 200),
            GraphicsMode::EgaHi => (640, 350),
            GraphicsMode::VgaLo => (640, 200),
            GraphicsMode::VgaMed => (640, 350),
            GraphicsMode::VgaHi => (640, 480),
            GraphicsMode::HercMono => (720, 348),
            GraphicsMode::Att400C0 | GraphicsMode::Att400C1 | GraphicsMode::Att400C2 | GraphicsMode::Att400C3 => (320, 200),
            GraphicsMode::Att400Med => (640, 200),
            GraphicsMode::Att400Hi => (640, 400),
            GraphicsMode::Pc3270Hi => (720, 350),
            GraphicsMode::Ibm8514Lo => (640, 480),
            GraphicsMode::Ibm8514Hi => (1024, 768),
        }
    }

    /// Get the color depth (number of colors) for this graphics mode.
    pub fn color_depth(self) -> i32 {
        match self {
            GraphicsMode::CgaC0 | GraphicsMode::CgaC1 | GraphicsMode::CgaC2 | GraphicsMode::CgaC3 => 4,
            GraphicsMode::CgaHi => 2,
            GraphicsMode::McgaC0 | GraphicsMode::McgaC1 => 256,
            GraphicsMode::McgaC2 | GraphicsMode::McgaC3 | GraphicsMode::McgaMed | GraphicsMode::McgaHi => 2,
            GraphicsMode::EgaLo | GraphicsMode::EgaHi => 16,
            GraphicsMode::VgaLo | GraphicsMode::VgaMed | GraphicsMode::VgaHi => 16,
            GraphicsMode::HercMono => 2,
            GraphicsMode::Att400C0 | GraphicsMode::Att400C1 | GraphicsMode::Att400C2 | GraphicsMode::Att400C3 => 4,
            GraphicsMode::Att400Med | GraphicsMode::Att400Hi => 2,
            GraphicsMode::Pc3270Hi => 2,
            GraphicsMode::Ibm8514Lo | GraphicsMode::Ibm8514Hi => 256,
        }
    }
}

/// BGI-compatible graphics result codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GraphResult {
    /// Success (grOk).
    Ok = 0,
    /// Graphics not initialized (grNoInitGraph).
    GraphicsNotInitialized = -1,
    /// Graphics hardware not detected (grNotDetected).
    NotDetected = -2,
    /// Device driver file not found (grFileNotFound).
    DeviceDriverFileNotFound = -3,
    /// Invalid device driver file (grInvalidDriver).
    InvalidDriver = -4,
    /// Not enough memory to load driver (grNoLoadMem).
    NoLoadMem = -5,
    /// Out of memory in scan fill (grNoScanMem).
    NoScanMem = -6,
    /// Out of memory in flood fill (grNoFloodMem).
    NoFloodMem = -7,
    /// Font file not found (grFontNotFound).
    FontNotFound = -8,
    /// Not enough memory to load font (grNoFontMem).
    NoFontMem = -9,
    /// Invalid graphics mode for selected driver (grInvalidMode).
    InvalidMode = -10,
    /// Graphics error (grError).
    GraphicsError = -11,
    /// Graphics I/O error (grIOerror).
    IoError = -12,
    /// Invalid font file (grInvalidFont).
    InvalidFont = -13,
    /// Invalid font number (grInvalidFontNum).
    InvalidFontNum = -14,
    /// Invalid device driver version (grInvalidVersion).
    InvalidVersion = -15,
}

/// Line styles for setlinestyle function.
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
    /// User-defined line style.
    UserBit = 4,
}

/// Fill patterns for setfillstyle function.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum FillPattern {
    /// Empty fill.
    Empty = 0,
    /// Solid fill.
    Solid = 1,
    /// Line fill.
    Line = 2,
    /// Light slash fill.
    LtSlash = 3,
    /// Slash fill.
    Slash = 4,
    /// Backslash fill.
    BkSlash = 5,
    /// Light backslash fill.
    LtBkSlash = 6,
    /// Hatch fill.
    Hatch = 7,
    /// Cross hatch fill.
    XHatch = 8,
    /// Interleave fill.
    Interleave = 9,
    /// Wide dot fill.
    WideDot = 10,
    /// Close dot fill.
    CloseDot = 11,
    /// User-defined fill pattern.
    User = 12,
}

/// Text direction constants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum TextDirection {
    /// Horizontal text.
    Horizontal = 0,
    /// Vertical text.
    Vertical = 1,
}

/// Text justification constants for horizontal alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum HorizontalJustification {
    /// Left justified.
    Left = 0,
    /// Center justified.
    Center = 1,
    /// Right justified.
    Right = 2,
}

/// Text justification constants for vertical alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum VerticalJustification {
    /// Bottom justified.
    Bottom = 0,
    /// Center justified.
    Center = 1,
    /// Top justified.
    Top = 2,
}

/// Font types supported by BGI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum Font {
    /// Default font.
    Default = 0,
    /// Triplex font.
    Triplex = 1,
    /// Small font.
    Small = 2,
    /// Sans serif font.
    SansSerif = 3,
    /// Gothic font.
    Gothic = 4,
    /// Script font.
    Script = 5,
    /// Simplex font.
    Simplex = 6,
    /// Triplex script font.
    TriplexScript = 7,
    /// Complex font.
    Complex = 8,
    /// European font.
    European = 9,
    /// Bold font.
    Bold = 10,
}

/// Viewport information structure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ViewportType {
    /// Left x coordinate.
    pub left: i32,
    /// Top y coordinate.
    pub top: i32,
    /// Right x coordinate.
    pub right: i32,
    /// Bottom y coordinate.
    pub bottom: i32,
    /// Clipping flag.
    pub clip: bool,
}

/// Palette information structure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PaletteType {
    /// Size of the palette.
    pub size: i32,
    /// Colors in the palette.
    pub colors: [i32; 16],
}

/// Line settings structure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineSettingsType {
    /// Line style.
    pub linestyle: i32,
    /// Line pattern (for user-defined styles).
    pub upattern: u32,
    /// Line thickness.
    pub thickness: i32,
}

/// Fill settings structure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FillSettingsType {
    /// Fill pattern.
    pub pattern: i32,
    /// Fill color.
    pub color: i32,
}

/// Arc coordinates structure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArcCoordsType {
    /// Center x coordinate.
    pub x: i32,
    /// Center y coordinate.
    pub y: i32,
    /// Start x coordinate.
    pub xstart: i32,
    /// Start y coordinate.
    pub ystart: i32,
    /// End x coordinate.
    pub xend: i32,
    /// End y coordinate.
    pub yend: i32,
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

/// Color constants used in BGI.
pub mod colors {
    /// Black color.
    pub const BLACK: i32 = 0;
    /// Blue color.
    pub const BLUE: i32 = 1;
    /// Green color.
    pub const GREEN: i32 = 2;
    /// Cyan color.
    pub const CYAN: i32 = 3;
    /// Red color.
    pub const RED: i32 = 4;
    /// Magenta color.
    pub const MAGENTA: i32 = 5;
    /// Brown color.
    pub const BROWN: i32 = 6;
    /// Light gray color.
    pub const LIGHTGRAY: i32 = 7;
    /// Dark gray color.
    pub const DARKGRAY: i32 = 8;
    /// Light blue color.
    pub const LIGHTBLUE: i32 = 9;
    /// Light green color.
    pub const LIGHTGREEN: i32 = 10;
    /// Light cyan color.
    pub const LIGHTCYAN: i32 = 11;
    /// Light red color.
    pub const LIGHTRED: i32 = 12;
    /// Light magenta color.
    pub const LIGHTMAGENTA: i32 = 13;
    /// Yellow color.
    pub const YELLOW: i32 = 14;
    /// White color.
    pub const WHITE: i32 = 15;
}
