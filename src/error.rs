//! Error types for the BGI library.

use thiserror::Error;

/// Result type used throughout the BGI library.
pub type BgiResult<T> = Result<T, BgiError>;

/// Errors that can occur in BGI operations.
#[derive(Error, Debug, Clone)]
pub enum BgiError {
    /// Graphics system not initialized.
    #[error("Graphics system not initialized")]
    NotInitialized,

    /// Graphics mode not supported.
    #[error("Graphics mode not supported: {mode}")]
    InvalidMode { mode: i32 },

    /// Invalid driver specified.
    #[error("Invalid graphics driver")]
    InvalidDriver,

    /// Not enough memory for operation.
    #[error("Not enough memory for {operation}")]
    OutOfMemory { operation: String },

    /// Font not found or invalid.
    #[error("Font error: {message}")]
    FontError { message: String },

    /// Invalid coordinates or dimensions.
    #[error("Invalid coordinates: ({x}, {y})")]
    InvalidCoordinates { x: i32, y: i32 },

    /// Color value out of range.
    #[error("Invalid color value: {color}")]
    InvalidColor { color: i32 },

    /// Window operation failed.
    #[error("Window error: {message}")]
    WindowError { message: String },

    /// Backend-specific error.
    #[error("Backend error: {message}")]
    BackendError { message: String },

    /// I/O operation failed.
    #[error("I/O error: {message}")]
    IoError { message: String },

    /// General BGI error.
    #[error("BGI error: {message}")]
    General { message: String },

    /// Graphics operation error.
    #[error("Graphics error: {0}")]
    GraphicsError(String),

    /// Invalid window reference.
    #[error("Invalid window")]
    InvalidWindow,

    /// Initialization error.
    #[error("Initialization error: {0}")]
    InitializationError(String),

    /// Invalid parameter.
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    /// Operation not supported.
    #[error("Operation not supported: {0}")]
    NotSupported(String),
}

/// BGI graphics error codes (for compatibility).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphicsError {
    /// No error.
    Ok = 0,
    /// Graphics not initialized.
    NoInitGraph = -1,
    /// Hardware not detected.
    NotDetected = -2,
    /// File not found.
    FileNotFound = -3,
    /// Invalid driver.
    InvalidDriver = -4,
    /// Not enough memory to load driver.
    NoLoadMem = -5,
    /// Not enough memory for scan fill.
    NoScanMem = -6,
    /// Not enough memory for flood fill.
    NoFloodMem = -7,
    /// Font file not found.
    FontNotFound = -8,
    /// Not enough memory for font.
    NoFontMem = -9,
    /// Invalid graphics mode.
    InvalidMode = -10,
    /// Generic error.
    Error = -11,
    /// I/O error.
    IoError = -12,
    /// Invalid font file.
    InvalidFont = -13,
    /// Invalid font number.
    InvalidFontNum = -14,
    /// Invalid version.
    InvalidVersion = -18,
}

impl GraphicsError {
    /// Get error message for a graphics error code.
    pub fn message(self) -> &'static str {
        match self {
            Self::Ok => "No error",
            Self::NoInitGraph => "Graphics not initialized",
            Self::NotDetected => "Graphics hardware not detected",
            Self::FileNotFound => "Device driver file not found",
            Self::InvalidDriver => "Invalid device driver",
            Self::NoLoadMem => "Not enough memory to load driver",
            Self::NoScanMem => "Out of memory in scan fill",
            Self::NoFloodMem => "Out of memory in flood fill",
            Self::FontNotFound => "Font file not found",
            Self::NoFontMem => "Not enough memory to load font",
            Self::InvalidMode => "Invalid graphics mode for selected driver",
            Self::Error => "Graphics error",
            Self::IoError => "Graphics I/O error",
            Self::InvalidFont => "Invalid font file",
            Self::InvalidFontNum => "Invalid font number",
            Self::InvalidVersion => "Invalid driver version",
        }
    }
}

impl From<GraphicsError> for BgiError {
    fn from(error: GraphicsError) -> Self {
        match error {
            GraphicsError::Ok => unreachable!("Ok is not an error"),
            GraphicsError::NoInitGraph => Self::NotInitialized,
            GraphicsError::InvalidDriver => Self::InvalidDriver,
            GraphicsError::InvalidMode => Self::InvalidMode { mode: -1 },
            GraphicsError::FontNotFound
            | GraphicsError::NoFontMem
            | GraphicsError::InvalidFont
            | GraphicsError::InvalidFontNum => Self::FontError {
                message: error.message().to_string(),
            },
            GraphicsError::NoLoadMem | GraphicsError::NoScanMem | GraphicsError::NoFloodMem => {
                Self::OutOfMemory {
                    operation: error.message().to_string(),
                }
            }
            GraphicsError::IoError => Self::IoError {
                message: error.message().to_string(),
            },
            _ => Self::General {
                message: error.message().to_string(),
            },
        }
    }
}

impl From<crate::types::GraphResult> for BgiError {
    fn from(result: crate::types::GraphResult) -> Self {
        use crate::types::GraphResult;

        match result {
            GraphResult::Ok => unreachable!("Ok is not an error"),
            GraphResult::NotInitialized | GraphResult::GraphicsNotInitialized => {
                Self::NotInitialized
            }
            GraphResult::NotDetected => Self::General {
                message: "Graphics hardware not detected".to_string(),
            },
            GraphResult::DeviceDriverFileNotFound => Self::General {
                message: "Device driver file not found".to_string(),
            },
            GraphResult::InvalidDriver => Self::InvalidDriver,
            GraphResult::NoLoadMem | GraphResult::NotEnoughMemoryForDriver => Self::OutOfMemory {
                operation: "loading driver".to_string(),
            },
            GraphResult::NoScanMem => Self::OutOfMemory {
                operation: "scan fill".to_string(),
            },
            GraphResult::NoFloodMem => Self::OutOfMemory {
                operation: "flood fill".to_string(),
            },
            GraphResult::FontNotFound => Self::FontError {
                message: "Font file not found".to_string(),
            },
            GraphResult::NoFontMem => Self::OutOfMemory {
                operation: "loading font".to_string(),
            },
            GraphResult::InvalidMode => Self::InvalidMode { mode: -1 },
            GraphResult::GraphicsError => Self::General {
                message: "Graphics error".to_string(),
            },
            GraphResult::IoError | GraphResult::IOError => Self::IoError {
                message: "I/O operation failed".to_string(),
            },
            GraphResult::InvalidFont => Self::FontError {
                message: "Invalid font file".to_string(),
            },
            GraphResult::InvalidFontNum => Self::FontError {
                message: "Invalid font number".to_string(),
            },
            GraphResult::InvalidVersion => Self::General {
                message: "Invalid device driver version".to_string(),
            },
            GraphResult::InvalidDeviceDriverFile => Self::General {
                message: "Invalid device driver file".to_string(),
            },
            GraphResult::OutOfMemory => Self::OutOfMemory {
                operation: "operation".to_string(),
            },
            GraphResult::FontFileError => Self::FontError {
                message: "Font file error".to_string(),
            },
        }
    }
}
