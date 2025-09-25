//! Window state management for BGI graphics window.

use crate::{Color, constants::*};

/// Graphics driver information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DriverInfo {
    /// Driver type/ID
    pub driver: i32,
    /// Driver name
    pub name: String,
    /// Driver mode
    pub mode: i32,
    /// Mode name/description
    pub mode_name: String,
}

impl Default for DriverInfo {
    fn default() -> Self {
        Self {
            driver: VGA,
            name: "VGA".to_string(),
            mode: VGAHI,
            mode_name: "VGA High".to_string(),
        }
    }
}

/// Screen resolution and graphics mode information.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScreenMode {
    /// Screen width in pixels
    pub width: i32,
    /// Screen height in pixels
    pub height: i32,
    /// Color depth (bits per pixel)
    pub color_depth: i32,
    /// Maximum number of colors
    pub max_colors: i32,
    /// Graphics mode ID
    pub mode: i32,
}

impl Default for ScreenMode {
    fn default() -> Self {
        Self {
            width: 640,
            height: 480,
            color_depth: 4,
            max_colors: 16,
            mode: VGAHI,
        }
    }
}

/// Window properties and state.
#[derive(Debug, Clone)]
pub struct WindowProperties {
    /// Window title
    pub title: String,
    /// Whether window is resizable
    pub resizable: bool,
    /// Whether window has decorations (title bar, borders)
    pub decorated: bool,
    /// Whether window is always on top
    pub always_on_top: bool,
    /// Window background color
    pub background_color: Color,
}

impl Default for WindowProperties {
    fn default() -> Self {
        Self {
            title: "BGI Graphics".to_string(),
            resizable: false,
            decorated: true,
            always_on_top: false,
            background_color: Color::BLACK,
        }
    }
}

/// Graphics pages for double buffering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GraphicsPages {
    /// Currently active page for drawing
    pub active_page: i32,
    /// Currently visible page for display
    pub visual_page: i32,
    /// Total number of available pages
    pub total_pages: i32,
}

impl Default for GraphicsPages {
    fn default() -> Self {
        Self {
            active_page: 0,
            visual_page: 0,
            total_pages: 1,
        }
    }
}

/// Complete window state for BGI graphics.
#[derive(Debug, Clone)]
pub struct WindowState {
    /// Graphics driver information
    pub driver_info: DriverInfo,
    /// Screen mode and resolution
    pub screen_mode: ScreenMode,
    /// Window properties
    pub properties: WindowProperties,
    /// Graphics pages
    pub pages: GraphicsPages,
    /// Whether graphics system is initialized
    pub initialized: bool,
    /// Error state
    pub error_code: i32,
    /// Path to BGI driver files
    pub driver_path: String,
}

impl Default for WindowState {
    fn default() -> Self {
        Self {
            driver_info: DriverInfo::default(),
            screen_mode: ScreenMode::default(),
            properties: WindowProperties::default(),
            pages: GraphicsPages::default(),
            initialized: false,
            error_code: 0, // grOk
            driver_path: String::new(),
        }
    }
}

impl WindowState {
    /// Create a new window state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Initialize graphics with specified driver and mode.
    pub fn init_graphics(&mut self, driver: i32, mode: i32, path: &str) -> Result<(), i32> {
        // Validate driver and mode
        if !self.is_valid_driver(driver) {
            self.error_code = -2; // grInvalidDriver
            return Err(self.error_code);
        }

        if !self.is_valid_mode(driver, mode) {
            self.error_code = -10; // grInvalidMode
            return Err(self.error_code);
        }

        // Set up driver info
        self.driver_info.driver = driver;
        self.driver_info.mode = mode;
        self.driver_info.name = self.get_driver_name(driver);
        self.driver_info.mode_name = self.get_mode_name(driver, mode);

        // Set up screen mode
        self.screen_mode = self.get_screen_mode(driver, mode)?;

        // Set driver path
        self.driver_path = path.to_string();

        // Initialize successfully
        self.initialized = true;
        self.error_code = 0; // grOk

        Ok(())
    }

    /// Close graphics and clean up.
    pub fn close_graphics(&mut self) {
        self.initialized = false;
        self.error_code = 0;
        self.pages.active_page = 0;
        self.pages.visual_page = 0;
    }

    /// Check if graphics is initialized.
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Get current error code.
    pub fn get_error_code(&self) -> i32 {
        self.error_code
    }

    /// Set error code.
    pub fn set_error_code(&mut self, code: i32) {
        self.error_code = code;
    }

    /// Get screen dimensions.
    pub fn get_screen_size(&self) -> (i32, i32) {
        (self.screen_mode.width, self.screen_mode.height)
    }

    /// Get maximum X coordinate.
    pub fn get_max_x(&self) -> i32 {
        self.screen_mode.width - 1
    }

    /// Get maximum Y coordinate.
    pub fn get_max_y(&self) -> i32 {
        self.screen_mode.height - 1
    }

    /// Get maximum color value.
    pub fn get_max_color(&self) -> i32 {
        self.screen_mode.max_colors - 1
    }

    /// Set active page for drawing.
    pub fn set_active_page(&mut self, page: i32) -> Result<(), &'static str> {
        if page < 0 || page >= self.pages.total_pages {
            return Err("Invalid page number");
        }
        self.pages.active_page = page;
        Ok(())
    }

    /// Set visual page for display.
    pub fn set_visual_page(&mut self, page: i32) -> Result<(), &'static str> {
        if page < 0 || page >= self.pages.total_pages {
            return Err("Invalid page number");
        }
        self.pages.visual_page = page;
        Ok(())
    }

    /// Get current active page.
    pub fn get_active_page(&self) -> i32 {
        self.pages.active_page
    }

    /// Get current visual page.
    pub fn get_visual_page(&self) -> i32 {
        self.pages.visual_page
    }

    /// Set window title.
    pub fn set_window_title(&mut self, title: String) {
        self.properties.title = title;
    }

    /// Get window title.
    pub fn get_window_title(&self) -> &str {
        &self.properties.title
    }

    /// Check if driver is valid.
    pub fn is_valid_driver(&self, driver: i32) -> bool {
        matches!(driver, DETECT | CGA | MCGA | EGA | EGA64 | EGAMONO | IBM8514 | HERCMONO | ATT400 | VGA | PC3270)
    }

    /// Check if mode is valid for driver.
    pub fn is_valid_mode(&self, driver: i32, mode: i32) -> bool {
        use crate::constants::*;
        match driver {
            VGA => {
                // VGA-specific modes only  
                matches!(mode, VGALO | VGAMED | VGAHI | MCGAHI)
            },
            EGA => {
                // EGA-specific modes only
                matches!(mode, EGALO | EGAHI | EGA64LO | EGAMED)
            },
            CGA => {
                // CGA-specific modes only
                matches!(mode, CGALO | CGAMED | CGAHI)
            },
            MCGA => {
                // MCGA-specific modes only
                matches!(mode, MCGAHI)
            },
            _ => false, // Invalid drivers should return false
        }
    }

    /// Get driver name string.
    pub fn get_driver_name(&self, driver: i32) -> String {
        match driver {
            VGA => "VGA".to_string(),
            EGA => "EGA".to_string(),
            CGA => "CGA".to_string(),
            MCGA => "MCGA".to_string(),
            HERCMONO => "Hercules".to_string(),
            _ => format!("Driver {}", driver),
        }
    }

    /// Get mode name string.
    pub fn get_mode_name(&self, driver: i32, mode: i32) -> String {
        use crate::constants::*;
        match (driver, mode) {
            (VGA, VGALO) => "VGA Low (640x200)".to_string(),
            (VGA, VGAMED) => "VGA Medium (640x350)".to_string(),
            (VGA, VGAHI) => "VGA High (640x480)".to_string(),
            (EGA, EGALO) => "EGA Low (640x200)".to_string(),
            (EGA, EGAHI) => "EGA High (640x350)".to_string(),
            (CGA, CGAHI) => "CGA High (640x200)".to_string(),
            _ => format!("Mode {}", mode),
        }
    }

    /// Get screen mode for driver/mode combination.
    pub fn get_screen_mode(&self, driver: i32, mode: i32) -> Result<ScreenMode, i32> {
        use crate::constants::*;
        let screen_mode = match (driver, mode) {
            (VGA, VGALO) => ScreenMode {
                width: 640,
                height: 200,
                color_depth: 4,
                max_colors: 16,
                mode,
            },
            (VGA, VGAMED) => ScreenMode {
                width: 640,
                height: 350,
                color_depth: 4,
                max_colors: 16,
                mode,
            },
            (VGA, VGAHI) => ScreenMode {
                width: 640,
                height: 480,
                color_depth: 4,
                max_colors: 16,
                mode,
            },
            (VGA, MCGAHI) => ScreenMode {
                width: 320,
                height: 200,
                color_depth: 8,
                max_colors: 256,
                mode,
            },
            (EGA, EGALO) => ScreenMode {
                width: 640,
                height: 200,
                color_depth: 4,
                max_colors: 16,
                mode,
            },
            (EGA, EGAHI) => ScreenMode {
                width: 640,
                height: 350,
                color_depth: 4,
                max_colors: 16,
                mode,
            },
            (CGA, CGAHI) => ScreenMode {
                width: 640,
                height: 200,
                color_depth: 2,
                max_colors: 4,
                mode,
            },
            _ => return Err(-10), // grInvalidMode
        };

        Ok(screen_mode)
    }
}
