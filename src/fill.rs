//! Fill patterns and styles for the BGI library.

/// Fill patterns (compatible with BGI fill styles).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum FillPattern {
    /// Empty fill (background color).
    Empty = 0,
    /// Solid fill.
    Solid = 1,
    /// Horizontal line fill.
    Line = 2,
    /// Light slash fill (///).
    LightSlash = 3,
    /// Thick slash fill (///).
    Slash = 4,
    /// Thick backslash fill (\\\).
    BackSlash = 5,
    /// Light backslash fill (\\\).
    LightBackSlash = 6,
    /// Light hatch fill.
    Hatch = 7,
    /// Heavy cross hatch fill.
    CrossHatch = 8,
    /// Interleaving line fill.
    Interleave = 9,
    /// Widely spaced dot fill.
    WideDot = 10,
    /// Closely spaced dot fill.
    CloseDot = 11,
    /// User-defined fill pattern.
    User = 12,
}

impl FillPattern {
    /// Get pattern name.
    pub fn name(self) -> &'static str {
        match self {
            Self::Empty => "EMPTY_FILL",
            Self::Solid => "SOLID_FILL",
            Self::Line => "LINE_FILL",
            Self::LightSlash => "LTSLASH_FILL",
            Self::Slash => "SLASH_FILL",
            Self::BackSlash => "BKSLASH_FILL",
            Self::LightBackSlash => "LTBKSLASH_FILL",
            Self::Hatch => "HATCH_FILL",
            Self::CrossHatch => "XHATCH_FILL",
            Self::Interleave => "INTERLEAVE_FILL",
            Self::WideDot => "WIDE_DOT_FILL",
            Self::CloseDot => "CLOSE_DOT_FILL",
            Self::User => "USER_FILL",
        }
    }

    /// Create fill pattern from integer.
    pub fn from_int(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Empty),
            1 => Some(Self::Solid),
            2 => Some(Self::Line),
            3 => Some(Self::LightSlash),
            4 => Some(Self::Slash),
            5 => Some(Self::BackSlash),
            6 => Some(Self::LightBackSlash),
            7 => Some(Self::Hatch),
            8 => Some(Self::CrossHatch),
            9 => Some(Self::Interleave),
            10 => Some(Self::WideDot),
            11 => Some(Self::CloseDot),
            12 => Some(Self::User),
            _ => None,
        }
    }

    /// Generate the fill pattern bitmap (8x8 pixels).
    pub fn pattern_bitmap(self) -> [u8; 8] {
        match self {
            Self::Empty => [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
            Self::Solid => [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF],
            Self::Line => [0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00],
            Self::LightSlash => [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80],
            Self::Slash => [0x03, 0x06, 0x0C, 0x18, 0x30, 0x60, 0xC0, 0x81],
            Self::BackSlash => [0x80, 0x40, 0x20, 0x10, 0x08, 0x04, 0x02, 0x01],
            Self::LightBackSlash => [0x80, 0x40, 0x20, 0x10, 0x08, 0x04, 0x02, 0x01],
            Self::Hatch => [0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00],
            Self::CrossHatch => [0xFF, 0x88, 0x88, 0x88, 0xFF, 0x88, 0x88, 0x88],
            Self::Interleave => [0xCC, 0x33, 0xCC, 0x33, 0xCC, 0x33, 0xCC, 0x33],
            Self::WideDot => [0x80, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00],
            Self::CloseDot => [0x88, 0x00, 0x22, 0x00, 0x88, 0x00, 0x22, 0x00],
            Self::User => [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // User-defined
        }
    }
}

/// Fill style combining pattern and color (compatible with BGI fillsettingstype).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FillStyle {
    /// Fill pattern.
    pub pattern: FillPattern,
    /// Fill color.
    pub color: i32,
}

impl FillStyle {
    /// Create new fill style.
    pub const fn new(pattern: FillPattern, color: i32) -> Self {
        Self { pattern, color }
    }
}

impl Default for FillStyle {
    fn default() -> Self {
        Self::new(FillPattern::Solid, 1) // Default: solid white fill
    }
}

/// User-defined fill pattern (8x8 bitmap).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UserFillPattern {
    /// Pattern bitmap (8 bytes for 8x8 pattern).
    pub pattern: [u8; 8],
}

impl UserFillPattern {
    /// Create new user fill pattern.
    pub const fn new(pattern: [u8; 8]) -> Self {
        Self { pattern }
    }

    /// Create from pattern string (for convenience).
    pub fn from_string(pattern_str: &str) -> Option<Self> {
        if pattern_str.len() != 8 {
            return None;
        }

        let mut pattern = [0u8; 8];
        for (i, byte_str) in pattern_str.chars().enumerate() {
            if let Some(digit) = byte_str.to_digit(16) {
                pattern[i] = digit as u8;
            } else {
                return None;
            }
        }

        Some(Self::new(pattern))
    }
}

impl Default for UserFillPattern {
    fn default() -> Self {
        Self::new([0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF])
    }
}
