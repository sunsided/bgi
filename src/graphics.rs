//! Core graphics functions for BGI initialization and management.

use crate::types::MouseState;
use crate::{
    constants::*, Color, DrawingState, FontSettings, GraphResult, InputEvent, WindowState,
};
use std::cell::RefCell;
use std::collections::HashMap;

#[cfg(feature = "visual-backend")]
use crate::backend::{create_default_backend, Backend};
#[cfg(feature = "visual-backend")]
use crate::window::WindowId;

/// Global graphics context state.
pub struct GraphicsState {
    /// Window management state
    pub window_state: WindowState,
    /// Drawing state (line styles, fill patterns, etc.)
    pub drawing_state: DrawingState,
    /// Font and text settings
    pub font_settings: FontSettings,
    /// Input event handling
    pub input_event: InputEvent,
    /// Additional graphics state
    pub current_palette: Vec<Color>,
    /// Graphics pages for double buffering
    pub pages: HashMap<i32, Vec<u8>>,
    /// Backend for visual rendering (when visual-backend feature is enabled)
    #[cfg(feature = "visual-backend")]
    pub backend: Option<Box<dyn Backend>>,
    /// Current window for visual backend
    #[cfg(feature = "visual-backend")]
    pub current_window: Option<WindowId>,
}

impl Default for GraphicsState {
    fn default() -> Self {
        Self {
            window_state: WindowState::default(),
            drawing_state: DrawingState::default(),
            font_settings: FontSettings::default(),
            input_event: InputEvent::default(),
            current_palette: create_default_palette(),
            pages: HashMap::new(),
            #[cfg(feature = "visual-backend")]
            backend: None,
            #[cfg(feature = "visual-backend")]
            current_window: None,
        }
    }
}

impl GraphicsState {
    /// Create a new graphics state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Initialize graphics with specified driver and mode.
    pub fn init_graphics(&mut self, driver: i32, mode: i32, path: &str) -> Result<(), i32> {
        // Initialize window state
        self.window_state.init_graphics(driver, mode, path)?;

        // Reset drawing state
        self.drawing_state = DrawingState::default();

        // Reset font settings
        self.font_settings = FontSettings::default();

        // Clear input events
        self.input_event.clear_all();

        // Initialize palette
        self.current_palette = create_default_palette();

        // Initialize graphics pages
        let (width, height) = self.window_state.get_screen_size();
        let page_size = (width * height * 4) as usize; // 4 bytes per pixel (RGBA)

        for page in 0..self.window_state.pages.total_pages {
            self.pages.insert(page, vec![0; page_size]);
        }

        // Initialize visual backend if feature is enabled
        #[cfg(feature = "visual-backend")]
        {
            match create_default_backend() {
                Ok(mut backend) => {
                    if let Err(_) = backend.init() {
                        // Backend initialization failed, continue without visual output
                        self.backend = None;
                        self.current_window = None;
                    } else {
                        // Create a window for BGI graphics
                        match backend.create_window(
                            width as u32,
                            height as u32,
                            Some("BGI Graphics"),
                            crate::types::GraphicsMode::new(
                                crate::types::GraphicsDriver::Vga,
                                mode,
                            ),
                        ) {
                            Ok(window_id) => {
                                self.current_window = Some(window_id);
                                self.backend = Some(backend);
                            }
                            Err(_) => {
                                // Window creation failed, continue without visual output
                                self.backend = None;
                                self.current_window = None;
                            }
                        }
                    }
                }
                Err(_) => {
                    // Backend creation failed, continue without visual output
                    self.backend = None;
                    self.current_window = None;
                }
            }
        }

        Ok(())
    }

    /// Close graphics and clean up.
    pub fn close_graphics(&mut self) {
        self.window_state.close_graphics();
        self.pages.clear();

        // Clean up visual backend
        #[cfg(feature = "visual-backend")]
        {
            if let (Some(ref mut backend), Some(window_id)) =
                (&mut self.backend, self.current_window)
            {
                let _ = backend.close_window(window_id);
                let _ = backend.shutdown();
            }
            self.backend = None;
            self.current_window = None;
        }
    }

    /// Check if graphics is initialized.
    pub fn is_initialized(&self) -> bool {
        self.window_state.is_initialized()
    }

    /// Get current error code.
    pub fn get_error_code(&self) -> i32 {
        self.window_state.get_error_code()
    }

    /// Set error code.
    pub fn set_error_code(&mut self, code: i32) {
        self.window_state.set_error_code(code);
    }
}

// Thread-local storage for global graphics state
thread_local! {
    static GRAPHICS_STATE: RefCell<Option<GraphicsState>> = RefCell::new(None);
}

/// Initialize graphics system (BGI-compatible).
pub fn initgraph(driver: &mut i32, mode: &mut i32, path: &str) {
    // Handle auto-detection - detect driver but preserve mode if valid
    if *driver == DETECT {
        let original_mode = *mode;
        detectgraph(driver, mode);
        // If user specified a mode, try to preserve it
        if original_mode >= 0 {
            *mode = original_mode;
        }
    }

    // Validate input parameters
    if *driver < 0 || *mode < 0 {
        *driver = -1;
        *mode = -1;
        return;
    }

    GRAPHICS_STATE.with(|state_ref| {
        let mut state_opt = state_ref.borrow_mut();
        let mut graphics_state = GraphicsState::new();

        match graphics_state.init_graphics(*driver, *mode, path) {
            Ok(()) => {
                // Explicitly set success code
                graphics_state.set_error_code(0);
                *state_opt = Some(graphics_state);
            }
            Err(error_code) => {
                graphics_state.set_error_code(error_code);
                *state_opt = Some(graphics_state);
                *driver = -1;
                *mode = -1;
            }
        }
    });
}

/// Close graphics system (BGI-compatible).
pub fn closegraph() {
    GRAPHICS_STATE.with(|state_ref| {
        if let Some(ref mut graphics_state) = state_ref.borrow_mut().as_mut() {
            graphics_state.close_graphics();
        }
        *state_ref.borrow_mut() = None;
    });
}

/// Get graphics error code.
pub fn graphresult() -> GraphResult {
    GRAPHICS_STATE.with(|state_ref| {
        if let Some(ref graphics_state) = state_ref.borrow().as_ref() {
            match graphics_state.get_error_code() {
                0 => GraphResult::Ok,
                -2 => GraphResult::InvalidDriver,
                -3 => GraphResult::DeviceDriverFileNotFound,
                -4 => GraphResult::InvalidDriver,
                -10 => GraphResult::InvalidMode,
                _ => GraphResult::GraphicsError, // Return error for unknown codes
            }
        } else {
            GraphResult::NotInitialized
        }
    })
}

/// Get error message for error code (BGI-compatible).
pub fn grapherrormsg<T: Into<i32>>(error_code: T) -> &'static str {
    let code = error_code.into();
    match code {
        0 => "No error",
        -2 => "Graphics hardware not detected",
        -3 => "Device driver file not found",
        -4 => "Invalid device driver",
        -5 => "Not enough memory to load driver",
        -6 => "Out of memory",
        -7 => "Out of memory in scanfill",
        -8 => "Path not found",
        -10 => "Invalid graphics mode",
        -11 => "Graphics I/O error",
        -15 => "Invalid font file",
        _ => "Unknown graphics error",
    }
}

/// Auto-detect graphics driver and mode.
pub fn detectgraph(driver: &mut i32, mode: &mut i32) {
    *driver = VGA;
    *mode = VGAHI;
}

/// Get current graphics mode.
pub fn getgraphmode() -> i32 {
    GRAPHICS_STATE.with(|state_ref| {
        if let Some(ref graphics_state) = state_ref.borrow().as_ref() {
            graphics_state.window_state.screen_mode.mode
        } else {
            -1
        }
    })
}

/// Set graphics mode.
pub fn setgraphmode(mode: i32) {
    GRAPHICS_STATE.with(|state_ref| {
        if let Some(ref mut graphics_state) = state_ref.borrow_mut().as_mut() {
            // For simplicity, just update the mode in screen_mode
            graphics_state.window_state.screen_mode.mode = mode;
        }
    });
}

/// Get maximum X coordinate.
pub fn getmaxx() -> i32 {
    GRAPHICS_STATE.with(|state_ref| {
        if let Some(ref graphics_state) = state_ref.borrow().as_ref() {
            let viewport = graphics_state.drawing_state.get_viewport();
            viewport.right
        } else {
            639 // Default VGA width - 1
        }
    })
}

/// Get maximum Y coordinate.
pub fn getmaxy() -> i32 {
    GRAPHICS_STATE.with(|state_ref| {
        if let Some(ref graphics_state) = state_ref.borrow().as_ref() {
            let viewport = graphics_state.drawing_state.get_viewport();
            viewport.bottom
        } else {
            479 // Default VGA height - 1
        }
    })
}

/// Set current drawing color.
pub fn setcolor(color: Color) {
    GRAPHICS_STATE.with(|state_ref| {
        if let Some(ref mut graphics_state) = state_ref.borrow_mut().as_mut() {
            graphics_state.drawing_state.set_color(color);
        }
    });
}

/// Get current drawing color.
pub fn getcolor() -> Color {
    GRAPHICS_STATE.with(|state_ref| {
        if let Some(ref graphics_state) = state_ref.borrow().as_ref() {
            graphics_state.drawing_state.get_color()
        } else {
            Color::WHITE
        }
    })
}

/// Set background color.
pub fn setbkcolor(color: Color) {
    GRAPHICS_STATE.with(|state_ref| {
        if let Some(ref mut graphics_state) = state_ref.borrow_mut().as_mut() {
            graphics_state.window_state.properties.background_color = color;
        }
    });
}

/// Get background color.
pub fn getbkcolor() -> Color {
    GRAPHICS_STATE.with(|state_ref| {
        if let Some(ref graphics_state) = state_ref.borrow().as_ref() {
            graphics_state.window_state.properties.background_color
        } else {
            Color::BLACK
        }
    })
}

/// Set line style.
pub fn setlinestyle(line_style: i32, pattern: u16, thickness: i32) {
    GRAPHICS_STATE.with(|state_ref| {
        if let Some(ref mut graphics_state) = state_ref.borrow_mut().as_mut() {
            graphics_state
                .drawing_state
                .set_line_style(line_style, pattern, thickness);
        }
    });
}

/// Get line style settings.
pub fn getlinesettings() -> crate::types::BgiLineSettings {
    GRAPHICS_STATE.with(|state_ref| {
        if let Some(ref graphics_state) = state_ref.borrow().as_ref() {
            let (linestyle, upattern, thickness) = graphics_state.drawing_state.get_line_style();
            crate::types::BgiLineSettings {
                linestyle,
                upattern,
                thickness,
            }
        } else {
            crate::types::BgiLineSettings::default()
        }
    })
}

/// Set fill style and color.
pub fn setfillstyle(pattern: i32, color: Color) {
    GRAPHICS_STATE.with(|state_ref| {
        if let Some(ref mut graphics_state) = state_ref.borrow_mut().as_mut() {
            graphics_state.drawing_state.set_fill_style(pattern, color);
        }
    });
}

/// Set custom fill pattern.
pub fn setfillpattern(pattern: &[u8; 8], color: Color) {
    GRAPHICS_STATE.with(|state_ref| {
        if let Some(ref mut graphics_state) = state_ref.borrow_mut().as_mut() {
            graphics_state
                .drawing_state
                .set_fill_pattern(pattern, color);
        }
    });
}

/// Get fill style settings.
pub fn getfillsettings() -> (i32, Color) {
    GRAPHICS_STATE.with(|state_ref| {
        if let Some(ref graphics_state) = state_ref.borrow().as_ref() {
            graphics_state.drawing_state.get_fill_style()
        } else {
            (SOLID_FILL, Color::WHITE)
        }
    })
}

/// Set write mode for drawing operations.
pub fn setwritemode(mode: i32) {
    GRAPHICS_STATE.with(|state_ref| {
        if let Some(ref mut graphics_state) = state_ref.borrow_mut().as_mut() {
            graphics_state.drawing_state.set_write_mode(mode);
        }
    });
}

/// Get current write mode.
pub fn getwritemode() -> i32 {
    GRAPHICS_STATE.with(|state_ref| {
        if let Some(ref graphics_state) = state_ref.borrow().as_ref() {
            graphics_state.drawing_state.get_write_mode()
        } else {
            COPY_PUT
        }
    })
}

/// Set viewport for clipping.
pub fn setviewport(left: i32, top: i32, right: i32, bottom: i32, clip: bool) {
    GRAPHICS_STATE.with(|state_ref| {
        if let Some(ref mut graphics_state) = state_ref.borrow_mut().as_mut() {
            graphics_state
                .drawing_state
                .set_viewport(left, top, right, bottom, clip);
        }
    });
}

/// Get current viewport settings.
pub fn getviewport() -> (i32, i32, i32, i32) {
    GRAPHICS_STATE.with(|state_ref| {
        if let Some(ref graphics_state) = state_ref.borrow().as_ref() {
            let viewport = graphics_state.drawing_state.get_viewport();
            (viewport.left, viewport.top, viewport.right, viewport.bottom)
        } else {
            (0, 0, 639, 479)
        }
    })
}

/// Get current viewport settings (extended version with clip flag).
pub fn getviewsettings() -> (i32, i32, i32, i32, bool) {
    GRAPHICS_STATE.with(|state_ref| {
        if let Some(ref graphics_state) = state_ref.borrow().as_ref() {
            let viewport = graphics_state.drawing_state.get_viewport();
            (
                viewport.left,
                viewport.top,
                viewport.right,
                viewport.bottom,
                viewport.clip,
            )
        } else {
            (0, 0, 639, 479, true)
        }
    })
}

/// Clear graphics screen with current background color.
pub fn cleardevice() {
    GRAPHICS_STATE.with(|state_ref| {
        if let Some(ref mut graphics_state) = state_ref.borrow_mut().as_mut() {
            let bg_color = graphics_state.window_state.properties.background_color;
            let active_page = graphics_state.window_state.pages.active_page;

            if let Some(page_data) = graphics_state.pages.get_mut(&active_page) {
                // Fill page with background color (simplified)
                page_data.fill(0); // Black for now
            }

            // Clear visual backend if available
            #[cfg(feature = "visual-backend")]
            {
                if let (Some(ref mut backend), Some(window_id)) =
                    (&mut graphics_state.backend, graphics_state.current_window)
                {
                    use crate::backend::DrawCommand;
                    let rgb_color = bg_color.to_rgb();
                    let commands = vec![DrawCommand::Clear { color: rgb_color }];
                    if let Err(_) = backend.draw(window_id, &commands) {
                        // Ignore draw errors to maintain BGI compatibility
                    }
                    if let Err(_) = backend.present(window_id) {
                        // Ignore present errors to maintain BGI compatibility
                    }
                }
            }
        }
    });
}

/// Set active page for drawing.
pub fn setactivepage(page: i32) {
    GRAPHICS_STATE.with(|state_ref| {
        if let Some(ref mut graphics_state) = state_ref.borrow_mut().as_mut() {
            let _ = graphics_state.window_state.set_active_page(page);
        }
    });
}

/// Set visual page for display.
pub fn setvisualpage(page: i32) {
    GRAPHICS_STATE.with(|state_ref| {
        if let Some(ref mut graphics_state) = state_ref.borrow_mut().as_mut() {
            let _ = graphics_state.window_state.set_visual_page(page);
        }
    });
}

/// Get current active page.
pub fn getactivepage() -> i32 {
    GRAPHICS_STATE.with(|state_ref| {
        if let Some(ref graphics_state) = state_ref.borrow().as_ref() {
            graphics_state.window_state.get_active_page()
        } else {
            0
        }
    })
}

/// Get current visual page.
pub fn getvisualpage() -> i32 {
    GRAPHICS_STATE.with(|state_ref| {
        if let Some(ref graphics_state) = state_ref.borrow().as_ref() {
            graphics_state.window_state.get_visual_page()
        } else {
            0
        }
    })
}

/// Create default EGA/VGA color palette.
fn create_default_palette() -> Vec<Color> {
    vec![
        Color::BLACK,        // 0
        Color::BLUE,         // 1
        Color::GREEN,        // 2
        Color::CYAN,         // 3
        Color::RED,          // 4
        Color::MAGENTA,      // 5
        Color::BROWN,        // 6
        Color::LIGHTGRAY,    // 7
        Color::DARKGRAY,     // 8
        Color::LIGHTBLUE,    // 9
        Color::LIGHTGREEN,   // 10
        Color::LIGHTCYAN,    // 11
        Color::LIGHTRED,     // 12
        Color::LIGHTMAGENTA, // 13
        Color::YELLOW,       // 14
        Color::WHITE,        // 15
    ]
}

/// Helper function to check if graphics is initialized.
pub fn is_graphics_initialized() -> bool {
    GRAPHICS_STATE.with(|state_ref| {
        state_ref
            .borrow()
            .as_ref()
            .map_or(false, |gs| gs.is_initialized())
    })
}

/// Helper function to get graphics state for other modules.
pub fn with_graphics_state<F, R>(f: F) -> Option<R>
where
    F: FnOnce(&GraphicsState) -> R,
{
    GRAPHICS_STATE.with(|state_ref| state_ref.borrow().as_ref().map(f))
}

/// Helper function to get mutable graphics state for other modules.
pub fn with_graphics_state_mut<F, R>(f: F) -> Option<R>
where
    F: FnOnce(&mut GraphicsState) -> R,
{
    GRAPHICS_STATE.with(|state_ref| state_ref.borrow_mut().as_mut().map(f))
}

/// Move to absolute position.
pub fn moveto(x: i32, y: i32) {
    GRAPHICS_STATE.with(|state_ref| {
        if let Some(ref mut graphics_state) = state_ref.borrow_mut().as_mut() {
            graphics_state.drawing_state.move_to(x, y);
        }
    });
}

/// Move relatively from current position.
pub fn moverel(dx: i32, dy: i32) {
    GRAPHICS_STATE.with(|state_ref| {
        if let Some(ref mut graphics_state) = state_ref.borrow_mut().as_mut() {
            graphics_state.drawing_state.move_rel(dx, dy);
        }
    });
}

/// Get current X position.
pub fn getx() -> i32 {
    GRAPHICS_STATE.with(|state_ref| {
        if let Some(ref graphics_state) = state_ref.borrow().as_ref() {
            graphics_state.drawing_state.get_position().0
        } else {
            0
        }
    })
}

/// Get current Y position.
pub fn gety() -> i32 {
    GRAPHICS_STATE.with(|state_ref| {
        if let Some(ref graphics_state) = state_ref.borrow().as_ref() {
            graphics_state.drawing_state.get_position().1
        } else {
            0
        }
    })
}

/// Get mouse X coordinate.
pub fn mousex() -> i32 {
    GRAPHICS_STATE.with(|state_ref| {
        if let Some(ref graphics_state) = state_ref.borrow().as_ref() {
            graphics_state.input_event.get_mouse_position().0
        } else {
            0
        }
    })
}

/// Get mouse Y coordinate.
pub fn mousey() -> i32 {
    GRAPHICS_STATE.with(|state_ref| {
        if let Some(ref graphics_state) = state_ref.borrow().as_ref() {
            graphics_state.input_event.get_mouse_position().1
        } else {
            0
        }
    })
}

/// Get mouse button state and position.
pub fn getmouse() -> MouseState {
    GRAPHICS_STATE.with(|state_ref| {
        if let Some(ref graphics_state) = state_ref.borrow().as_ref() {
            let (x, y) = graphics_state.input_event.get_mouse_position();
            let buttons = graphics_state.input_event.get_mouse_buttons();
            MouseState {
                x,
                y,
                left: buttons.0,
                right: buttons.1,
                middle: buttons.2,
            }
        } else {
            MouseState {
                x: 0,
                y: 0,
                left: false,
                right: false,
                middle: false,
            }
        }
    })
}

/// Check if mouse button was clicked.
pub fn ismouseclick(button: i32) -> bool {
    GRAPHICS_STATE.with(|state_ref| {
        if let Some(ref graphics_state) = state_ref.borrow().as_ref() {
            match button {
                1 => graphics_state.input_event.has_left_click(),
                2 => graphics_state.input_event.has_right_click(),
                4 => graphics_state.input_event.has_middle_click(),
                _ => false,
            }
        } else {
            false
        }
    })
}

/// Get mouse click information.
pub fn mouseclick(button: i32) -> bool {
    GRAPHICS_STATE.with(|state_ref| {
        if let Some(ref graphics_state) = state_ref.borrow().as_ref() {
            match button {
                1 => graphics_state.input_event.has_left_click(),
                2 => graphics_state.input_event.has_right_click(),
                3 => graphics_state.input_event.has_middle_click(),
                _ => false,
            }
        } else {
            false
        }
    })
}

/// Set mouse position.
pub fn setmouse(x: i32, y: i32) {
    GRAPHICS_STATE.with(|state_ref| {
        if let Some(ref mut graphics_state) = state_ref.borrow_mut().as_mut() {
            graphics_state.input_event.set_mouse_position(x, y);
        }
    });
}

/// Clear mouse click information.
pub fn clearmouseclick(button: i32) {
    GRAPHICS_STATE.with(|state_ref| {
        if let Some(ref mut graphics_state) = state_ref.borrow_mut().as_mut() {
            match button {
                1 => graphics_state.input_event.clear_left_clicks(),
                2 => graphics_state.input_event.clear_right_clicks(),
                4 => graphics_state.input_event.clear_middle_clicks(),
                _ => {}
            }
        }
    });
}

/// Check if a key was pressed.
pub fn kbhit() -> bool {
    GRAPHICS_STATE.with(|state_ref| {
        if let Some(ref mut graphics_state) = state_ref.borrow_mut().as_mut() {
            // Poll events from backend and feed them into input system
            #[cfg(feature = "visual-backend")]
            if let Some(ref mut backend) = graphics_state.backend {
                let events = backend.poll_events();
                for event in events {
                    match event {
                        crate::backend::InputEvent::Key {
                            key_code, extended, ..
                        } => {
                            // Feed key event into input system
                            graphics_state.input_event.add_key_event(key_code, extended);
                        }
                        _ => {} // Handle other events if needed
                    }
                }
                return graphics_state.input_event.has_key_event();
            }

            // Return false if no backend or in headless mode
            false
        } else {
            false
        }
    })
}

/// Get a character from keyboard input.
pub fn getch() -> Option<char> {
    GRAPHICS_STATE.with(|state_ref| {
        if let Some(ref mut graphics_state) = state_ref.borrow_mut().as_mut() {
            // Poll events from backend and feed them into input system
            #[cfg(feature = "visual-backend")]
            if let Some(ref mut backend) = graphics_state.backend {
                let events = backend.poll_events();
                for event in events {
                    match event {
                        crate::backend::InputEvent::Key {
                            key_code, extended, ..
                        } => {
                            // Feed key event into input system
                            graphics_state.input_event.add_key_event(key_code, extended);
                        }
                        _ => {} // Handle other events if needed
                    }
                }
            }

            graphics_state.input_event.get_next_key().map(|event| {
                // Convert key_code to character (simplified)
                if event.key_code >= 32 && event.key_code <= 126 {
                    event.key_code as u8 as char
                } else {
                    '\0'
                }
            })
        } else {
            None
        }
    })
}

/// Check if the graphics system is running in headless mode (no visual output).
/// This is useful for examples to adapt their behavior accordingly.
pub fn is_headless() -> bool {
    #[cfg(feature = "visual-backend")]
    {
        false
    }
    #[cfg(not(feature = "visual-backend"))]
    {
        true
    }
}

/// Flood fill an area with the current fill color (stub implementation).
pub fn floodfill(x: i32, y: i32, border_color: Color) {
    // TODO: Implement flood fill algorithm
    // For now, just a stub to allow compilation
    let _ = (x, y, border_color);
}

/// Delay execution for the specified number of milliseconds.
pub fn delay(milliseconds: u32) {
    std::thread::sleep(std::time::Duration::from_millis(milliseconds as u64));
}

/// Clear the current viewport.
pub fn clearviewport() {
    GRAPHICS_STATE.with(|state_ref| {
        if let Some(ref mut graphics_state) = state_ref.borrow_mut().as_mut() {
            let viewport = graphics_state.drawing_state.viewport;
            // Clear the viewport area by filling it with background color
            let bg_color = graphics_state.drawing_state.background_color;

            // For now, just a stub - should clear the viewport rectangle
            let _ = (viewport, bg_color);
        }
    });
}

/// Draw a line relatively from current position.
pub fn linerel(dx: i32, dy: i32) {
    GRAPHICS_STATE.with(|state_ref| {
        if let Some(ref mut graphics_state) = state_ref.borrow_mut().as_mut() {
            let current_pos = graphics_state.drawing_state.position;
            let new_x = current_pos.x + dx;
            let new_y = current_pos.y + dy;

            // Draw line from current position to new position
            graphics_state.drawing_state.position.x = new_x;
            graphics_state.drawing_state.position.y = new_y;

            // TODO: Actually draw the line using the backend
        }
    });
}

/// Set text style (font, direction, char size).
pub fn settextstyle(font: i32, direction: i32, char_size: i32) {
    GRAPHICS_STATE.with(|state_ref| {
        if let Some(ref mut graphics_state) = state_ref.borrow_mut().as_mut() {
            graphics_state
                .font_settings
                .set_text_style(font, direction, char_size);
        }
    });
}

/// Set text justification.
pub fn settextjustify(horiz: i32, vert: i32) {
    GRAPHICS_STATE.with(|state_ref| {
        if let Some(ref mut graphics_state) = state_ref.borrow_mut().as_mut() {
            graphics_state.font_settings.alignment.horizontal = horiz;
            graphics_state.font_settings.alignment.vertical = vert;
        }
    });
}

/// Get current text settings.
pub fn gettextsettings() -> crate::types::BgiTextSettings {
    GRAPHICS_STATE.with(|state_ref| {
        if let Some(ref graphics_state) = state_ref.borrow().as_ref() {
            let text_style = &graphics_state.font_settings.style;
            let alignment = &graphics_state.font_settings.alignment;
            crate::types::BgiTextSettings {
                font: text_style.font,
                direction: text_style.direction,
                charsize: text_style.char_size,
                horiz: alignment.horizontal,
                vert: alignment.vertical,
            }
        } else {
            crate::types::BgiTextSettings {
                font: 0,
                direction: 0,
                charsize: 1,
                horiz: 0,
                vert: 0,
            }
        }
    })
}

/// Output text at specified position.
/// Currently a stub - will be implemented with bgi-stroked-fonts.
pub fn outtextxy(_x: i32, _y: i32, _text: &str) {
    // TDD stub - will implement with actual font rendering
}

/// Calculate the width of a text string in pixels.
pub fn textwidth(text: &str) -> i32 {
    GRAPHICS_STATE.with(|state_ref| {
        if let Some(ref graphics_state) = state_ref.borrow().as_ref() {
            graphics_state.font_settings.text_width(text)
        } else {
            // Fallback: 8 pixels per character
            text.len() as i32 * 8
        }
    })
}

/// Calculate the height of a text string in pixels.
pub fn textheight(text: &str) -> i32 {
    GRAPHICS_STATE.with(|state_ref| {
        if let Some(ref graphics_state) = state_ref.borrow().as_ref() {
            graphics_state.font_settings.text_height(text)
        } else {
            // Fallback: 16 pixels height
            if text.is_empty() {
                0
            } else {
                16
            }
        }
    })
}

/// Set batch mode to optimize bulk drawing operations.
/// When batch mode is enabled, individual draw operations (like putpixel)
/// will not immediately present to the screen, allowing for much faster
/// bulk operations. Call refresh() to present all changes at once.
pub fn set_batch_mode(enabled: bool) {
    with_graphics_state_mut(|state| {
        state.drawing_state.batch_mode = enabled;
    });
}

/// Check if batch mode is currently enabled.
pub fn is_batch_mode() -> bool {
    with_graphics_state(|state| state.drawing_state.batch_mode).unwrap_or(false)
}

/// Force a refresh/present of the current window.
/// This is useful when in batch mode to present all accumulated changes.
pub fn refresh() {
    #[cfg(feature = "visual-backend")]
    with_graphics_state_mut(|state| {
        if let (Some(ref mut backend), Some(window_id)) = (&mut state.backend, state.current_window)
        {
            if let Err(_) = backend.present(window_id) {
                // Ignore present errors to maintain BGI compatibility
            }
        }
    });
}
