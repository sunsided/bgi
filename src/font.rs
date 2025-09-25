//! Font and text handling for the BGI library.

/// BGI font types (compatible with BGI font constants).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum Font {
    /// Default 8x8 bitmap font.
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

impl Font {
    /// Get font name.
    pub fn name(self) -> &'static str {
        match self {
            Self::Default => "DefaultFont",
            Self::Triplex => "TriplexFont",
            Self::Small => "SmallFont",
            Self::SansSerif => "SansSerifFont",
            Self::Gothic => "GothicFont",
            Self::Script => "ScriptFont",
            Self::Simplex => "SimplexFont",
            Self::TriplexScript => "TriplexScriptFont",
            Self::Complex => "ComplexFont",
            Self::European => "EuropeanFont",
            Self::Bold => "BoldFont",
        }
    }

    /// Create font from integer.
    pub fn from_int(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Default),
            1 => Some(Self::Triplex),
            2 => Some(Self::Small),
            3 => Some(Self::SansSerif),
            4 => Some(Self::Gothic),
            5 => Some(Self::Script),
            6 => Some(Self::Simplex),
            7 => Some(Self::TriplexScript),
            8 => Some(Self::Complex),
            9 => Some(Self::European),
            10 => Some(Self::Bold),
            _ => None,
        }
    }

    /// Check if font is a bitmap font.
    pub fn is_bitmap(self) -> bool {
        matches!(self, Self::Default)
    }

    /// Check if font is a vector font.
    pub fn is_vector(self) -> bool {
        !self.is_bitmap()
    }
}

/// Text direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum TextDirection {
    /// Horizontal text (left to right).
    Horizontal = 0,
    /// Vertical text (bottom to top).
    Vertical = 1,
}

impl TextDirection {
    /// Get direction name.
    pub fn name(self) -> &'static str {
        match self {
            Self::Horizontal => "HorizDir",
            Self::Vertical => "VertDir",
        }
    }
}

/// Horizontal text justification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum HorizontalJustification {
    /// Left-aligned text.
    Left = 0,
    /// Center-aligned text.
    Center = 1,
    /// Right-aligned text.
    Right = 2,
}

impl HorizontalJustification {
    /// Get justification name.
    pub fn name(self) -> &'static str {
        match self {
            Self::Left => "LeftText",
            Self::Center => "CenterText",
            Self::Right => "RightText",
        }
    }
}

/// Vertical text justification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum VerticalJustification {
    /// Bottom-aligned text.
    Bottom = 0,
    /// Center-aligned text.
    Center = 1,
    /// Top-aligned text.
    Top = 2,
}

impl VerticalJustification {
    /// Get justification name.
    pub fn name(self) -> &'static str {
        match self {
            Self::Bottom => "BottomText",
            Self::Center => "CenterText",
            Self::Top => "TopText",
        }
    }
}

/// Text justification combining horizontal and vertical alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextJustification {
    /// Horizontal justification.
    pub horizontal: HorizontalJustification,
    /// Vertical justification.
    pub vertical: VerticalJustification,
}

impl TextJustification {
    /// Create new text justification.
    pub const fn new(horizontal: HorizontalJustification, vertical: VerticalJustification) -> Self {
        Self {
            horizontal,
            vertical,
        }
    }

    /// Left-top alignment.
    pub const fn left_top() -> Self {
        Self::new(HorizontalJustification::Left, VerticalJustification::Top)
    }

    /// Center-center alignment.
    pub const fn center() -> Self {
        Self::new(
            HorizontalJustification::Center,
            VerticalJustification::Center,
        )
    }
}

impl Default for TextJustification {
    fn default() -> Self {
        Self::left_top()
    }
}

/// Text settings (compatible with BGI textsettingstype).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextSettings {
    /// Font type.
    pub font: Font,
    /// Text direction.
    pub direction: TextDirection,
    /// Character size (0 for user-defined).
    pub char_size: i32,
    /// Text justification.
    pub justification: TextJustification,
}

impl TextSettings {
    /// Create new text settings.
    pub const fn new(
        font: Font,
        direction: TextDirection,
        char_size: i32,
        justification: TextJustification,
    ) -> Self {
        Self {
            font,
            direction,
            char_size,
            justification,
        }
    }
}

impl Default for TextSettings {
    fn default() -> Self {
        Self::new(
            Font::Default,
            TextDirection::Horizontal,
            1,
            TextJustification::default(),
        )
    }
}

/// User-defined character size settings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UserCharSize {
    /// Multiple in X direction.
    pub mult_x: i32,
    /// Divisor in X direction.
    pub div_x: i32,
    /// Multiple in Y direction.
    pub mult_y: i32,
    /// Divisor in Y direction.
    pub div_y: i32,
}

impl UserCharSize {
    /// Create new user character size.
    pub const fn new(mult_x: i32, div_x: i32, mult_y: i32, div_y: i32) -> Self {
        Self {
            mult_x,
            div_x,
            mult_y,
            div_y,
        }
    }

    /// Get effective scale factor for X.
    pub fn scale_x(self) -> f32 {
        if self.div_x != 0 {
            self.mult_x as f32 / self.div_x as f32
        } else {
            1.0
        }
    }

    /// Get effective scale factor for Y.
    pub fn scale_y(self) -> f32 {
        if self.div_y != 0 {
            self.mult_y as f32 / self.div_y as f32
        } else {
            1.0
        }
    }
}

impl Default for UserCharSize {
    fn default() -> Self {
        Self::new(1, 1, 1, 1) // No scaling
    }
}
