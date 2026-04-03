//! Constants used throughout the BGI library.

// ============================================================================
// BGI Graphics Result Constants (C-style, for API compatibility)
// ============================================================================

// These constants use non-standard Rust naming to match the original BGI API.
#![allow(non_upper_case_globals)]

/// Success - graphics operation completed successfully.
pub const grOk: i32 = 0;
/// Graphics not initialized - initgraph() not called.
pub const grNoInitGraph: i32 = -1;
/// Graphics hardware not detected.
pub const grNotDetected: i32 = -2;
/// Device driver file not found.
pub const grFileNotFound: i32 = -3;
/// Invalid device driver file.
pub const grInvalidDriver: i32 = -4;
/// Not enough memory to load driver.
pub const grNoLoadMem: i32 = -5;
/// Out of memory in scan fill.
pub const grNoScanMem: i32 = -6;
/// Out of memory in flood fill.
pub const grNoFloodMem: i32 = -7;
/// Font file not found.
pub const grFontNotFound: i32 = -8;
/// Not enough memory to load font.
pub const grNoFontMem: i32 = -9;
/// Invalid graphics mode for selected driver.
pub const grInvalidMode: i32 = -10;
/// Graphics error.
pub const grError: i32 = -11;
/// Graphics I/O error.
pub const grIOerror: i32 = -12;
/// Invalid font file.
pub const grInvalidFont: i32 = -13;
/// Invalid font number.
pub const grInvalidFontNum: i32 = -14;
/// Invalid device driver version.
pub const grInvalidVersion: i32 = -15;

/// Maximum number of colors in the default palette.
pub const MAX_COLORS: usize = 15;

/// Number of visual pages available.
pub const VISUAL_PAGES: usize = 4;

/// Maximum number of concurrent windows.
pub const MAX_WINDOWS: usize = 16;

/// Special window ID for all windows.
pub const ALL_WINDOWS: i32 = -1;

/// Character size for user-defined character sizing.
pub const USER_CHAR_SIZE: i32 = 0;

/// Boolean constants for compatibility.
pub const NOPE: i32 = 0;
pub const YEAH: i32 = 1;

/// Window title length limit.
pub const WIN_TITLE_LEN: usize = 512;

// ============================================================================
// BGI Graphics Device Constants
// ============================================================================

/// Auto-detect graphics driver.
pub const DETECT: i32 = 0;
/// CGA (Color Graphics Adapter) driver.
pub const CGA: i32 = 1;
/// MCGA (Multi-Color Graphics Array) driver.
pub const MCGA: i32 = 2;
/// EGA (Enhanced Graphics Adapter) driver.
pub const EGA: i32 = 3;
/// EGA with 64K memory driver.
pub const EGA64: i32 = 4;
/// EGA Monochrome driver.
pub const EGAMONO: i32 = 5;
/// IBM 8514 display driver.
pub const IBM8514: i32 = 6;
/// Hercules Monochrome driver.
pub const HERCMONO: i32 = 7;
/// AT&T 400-line display driver.
pub const ATT400: i32 = 8;
/// VGA (Video Graphics Array) driver.
pub const VGA: i32 = 9;
/// PC 3270 display driver.
pub const PC3270: i32 = 10;

// ============================================================================
// BGI Graphics Mode Constants (device-specific)
// ============================================================================

// CGA Modes
/// CGA High resolution mode (640x200, 2 colors).
pub const CGAHI: i32 = 1;
/// CGA Medium resolution mode (320x200, 4 colors).
pub const CGAMED: i32 = 2;
/// CGA Low resolution mode (320x200, 2 colors).
pub const CGALO: i32 = 3;

// EGA/VGA Modes
/// EGA Low resolution mode (640x200, 16 colors).
pub const EGALO: i32 = 2;
/// EGA High resolution mode (640x350, 16 colors).
pub const EGAHI: i32 = 3;
/// EGA 64K Low resolution mode (640x200, 16 colors, EGA64K only).
pub const EGA64LO: i32 = 4;
/// EGA Medium resolution mode (640x350, 4 colors).
pub const EGAMED: i32 = 5;

// VGA Modes
/// VGA Low resolution mode (640x200, 16 colors).
pub const VGALO: i32 = 1;
/// VGA High resolution mode (640x480, 16 colors).
pub const VGAHI: i32 = 2;
/// VGA Medium resolution mode (640x350, 16 colors).
pub const VGAMED: i32 = 3;

// MCGA/VGA 256-color Mode
/// MCGA High resolution mode (320x200, 256 colors).
pub const MCGAHI: i32 = 4;

// ============================================================================
// Line Style Constants
// ============================================================================

/// Solid line style.
pub const SOLID_LINE: i32 = 0;
/// Dotted line style.
pub const DOTTED_LINE: i32 = 1;
/// Center line style.
pub const CENTER_LINE: i32 = 2;
/// Dashed line style.
pub const DASHED_LINE: i32 = 3;
/// User-defined line pattern.
pub const USERBIT_LINE: i32 = 4;

// ============================================================================
// Drawing Mode Constants
// ============================================================================

/// Copy pixel to screen (default).
pub const COPY_PUT: i32 = 0;
/// XOR pixel with screen.
pub const XOR_PUT: i32 = 1;
/// OR pixel with screen.
pub const OR_PUT: i32 = 2;
/// AND pixel with screen.
pub const AND_PUT: i32 = 3;
/// NOT pixel (invert).
pub const NOT_PUT: i32 = 4;

// ============================================================================
// Line Thickness Constants
// ============================================================================

/// Normal line thickness.
pub const NORM_WIDTH: i32 = 1;
/// Thick line thickness.
pub const THICK_WIDTH: i32 = 3;

// ============================================================================
// Fill Pattern Constants
// ============================================================================

/// Empty fill pattern (no fill).
pub const EMPTY_FILL: i32 = 0;
/// Solid fill pattern.
pub const SOLID_FILL: i32 = 1;
/// Line fill pattern.
pub const LINE_FILL: i32 = 2;
/// Light slash fill pattern.
pub const LTSLASH_FILL: i32 = 3;
/// Slash fill pattern.
pub const SLASH_FILL: i32 = 4;
/// Backslash fill pattern.
pub const BKSLASH_FILL: i32 = 5;
/// Light backslash fill pattern.
pub const LTBKSLASH_FILL: i32 = 6;
/// Hatch fill pattern.
pub const HATCH_FILL: i32 = 7;
/// Cross hatch fill pattern.
pub const XHATCH_FILL: i32 = 8;
/// Interleaved line fill pattern.
pub const INTERLEAVE_FILL: i32 = 9;
/// Wide dot fill pattern.
pub const WIDE_DOT_FILL: i32 = 10;
/// Close dot fill pattern.
pub const CLOSE_DOT_FILL: i32 = 11;
/// User-defined fill pattern.
pub const USER_FILL: i32 = 12;

// ============================================================================
// Font Constants
// ============================================================================

/// Default bitmap font.
pub const DEFAULT_FONT: i32 = 0;
/// Triplex stroked font.
pub const TRIPLEX_FONT: i32 = 1;
/// Small stroked font.
pub const SMALL_FONT: i32 = 2;
/// Sans serif stroked font.
pub const SANS_SERIF_FONT: i32 = 3;
/// Gothic stroked font.
pub const GOTHIC_FONT: i32 = 4;

// ============================================================================
// Text Direction Constants
// ============================================================================

/// Horizontal text direction.
pub const HORIZ_DIR: i32 = 0;
/// Vertical text direction.
pub const VERT_DIR: i32 = 1;

// ============================================================================
// Text Justification Constants
// ============================================================================

/// Left text justification.
pub const LEFT_TEXT: i32 = 0;
/// Center text justification.
pub const CENTER_TEXT: i32 = 1;
/// Right text justification.
pub const RIGHT_TEXT: i32 = 2;
/// Bottom text justification.
pub const BOTTOM_TEXT: i32 = 0;
/// Top text justification.
pub const TOP_TEXT: i32 = 2;
