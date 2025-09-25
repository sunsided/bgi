//! Main graphics context for the BGI library.

use std::collections::VecDeque;

use crate::backend::{Backend, DrawCommand};
use crate::color::{Color, Palette, RgbColor, RgbPalette};
use crate::constants::{MAX_COLORS, VISUAL_PAGES};
use crate::drawing::{ArcInfo, DrawingPrimitive};
use crate::error::{BgiError, BgiResult, GraphicsError};
use crate::fill::{FillPattern, FillStyle, UserFillPattern};
use crate::font::{
    Font, HorizontalJustification, TextDirection, TextJustification, TextSettings, UserCharSize,
    VerticalJustification,
};
use crate::input::{InputEvent, Key, MouseEvent};
use crate::types::{
    ArcCoords, GraphicsMode, LineSettings, LineStyle, LineThickness, Point, Rect, WriteMode,
};
use crate::viewport::Viewport;
use crate::window::{Window, WindowId};

/// Main graphics context that manages the BGI graphics system.
pub struct GraphicsContext {
    /// Backend implementation.
    backend: Box<dyn Backend>,
    /// Current graphics mode.
    mode: GraphicsMode,
    /// All windows.
    windows: Vec<Option<Window>>,
    /// Current active window.
    current_window: Option<WindowId>,
    /// Current drawing position.
    current_position: Point,
    /// Current drawing color.
    current_color: Color,
    /// Current background color.
    background_color: Color,
    /// Current fill style.
    fill_style: FillStyle,
    /// User-defined fill pattern.
    user_fill_pattern: UserFillPattern,
    /// Current line settings.
    line_settings: LineSettings,
    /// Current text settings.
    text_settings: TextSettings,
    /// User character size settings.
    user_char_size: UserCharSize,
    /// Current viewport.
    viewport: Viewport,
    /// Current write mode.
    write_mode: WriteMode,
    /// Visual pages.
    visual_page: usize,
    /// Active page.
    active_page: usize,
    /// Color palette.
    palette: Palette,
    /// Extended RGB palette.
    rgb_palette: RgbPalette,
    /// Last graphics error.
    last_error: GraphicsError,
    /// Arc coordinates from last arc operation.
    arc_info: ArcInfo,
    /// Input event queue.
    event_queue: VecDeque<InputEvent>,
    /// Auto-refresh mode.
    auto_refresh: bool,
    /// Aspect ratio correction.
    aspect_ratio: (f64, f64),
}

impl GraphicsContext {
    /// Create a new graphics context with the specified mode.
    pub fn new(mode: GraphicsMode) -> BgiResult<Self> {
        let backend = crate::backend::create_default_backend()?;

        Ok(Self {
            backend,
            mode,
            windows: vec![None; 16], // Support up to 16 windows
            current_window: None,
            current_position: Point::new(0, 0),
            current_color: Color::WHITE,
            background_color: Color::BLACK,
            fill_style: FillStyle::default(),
            user_fill_pattern: UserFillPattern::default(),
            line_settings: LineSettings::default(),
            text_settings: TextSettings::default(),
            user_char_size: UserCharSize::default(),
            viewport: Viewport::default(),
            write_mode: WriteMode::Copy,
            visual_page: 0,
            active_page: 0,
            palette: Palette::default(),
            rgb_palette: RgbPalette::default(),
            last_error: GraphicsError::Ok,
            arc_info: ArcInfo::default(),
            event_queue: VecDeque::new(),
            auto_refresh: true,
            aspect_ratio: (1.0, 1.0),
        })
    }

    /// Initialize graphics system (equivalent to initgraph).
    pub fn init_graph(&mut self, driver: &mut i32, mode: &mut i32, _path: &str) -> BgiResult<()> {
        self.backend.init()?;

        // Auto-detect mode if requested
        if *driver == -1 {
            *driver = 0; // SDL driver
            *mode = GraphicsMode::Vga as i32;
            self.mode = GraphicsMode::Vga;
        } else {
            self.mode = GraphicsMode::from_int(*mode).unwrap_or(GraphicsMode::Vga);
        }

        // Create initial window
        let (width, height) = self.mode.resolution().unwrap_or((640, 480));
        let window_id =
            self.backend
                .create_window(width, height, Some("BGI Graphics"), self.mode)?;

        let window = Window::new(
            window_id,
            width,
            height,
            "BGI Graphics".to_string(),
            self.mode,
        );
        self.windows[0] = Some(window);
        self.current_window = Some(window_id);

        self.backend.set_current_window(window_id)?;

        // Set default viewport to full window
        self.viewport = Viewport::new(0, 0, width as i32 - 1, height as i32 - 1, true);
        self.backend
            .set_viewport(window_id, self.viewport.to_rect())?;

        // Clear the screen
        self.clear_device()?;

        self.last_error = GraphicsError::Ok;
        Ok(())
    }

    /// Close graphics system (equivalent to closegraph).
    pub fn close_graph(&mut self) -> BgiResult<()> {
        // Close all windows
        for window in &mut self.windows {
            if let Some(ref win) = window {
                self.backend.close_window(win.id)?;
                *window = None;
            }
        }

        self.current_window = None;
        self.backend.shutdown()?;
        Ok(())
    }

    /// Get last graphics error result.
    pub fn graph_result(&self) -> GraphicsError {
        self.last_error
    }

    /// Get graphics error message.
    pub fn graph_error_msg(&self, error_code: GraphicsError) -> &'static str {
        error_code.message()
    }

    /// Clear the device (equivalent to cleardevice).
    pub fn clear_device(&mut self) -> BgiResult<()> {
        let window_id = self.require_window()?;
        let bg_color = self.background_color.to_rgb();

        let command = DrawCommand::Clear { color: bg_color };
        self.backend.draw(window_id, &[command])?;

        if self.auto_refresh {
            self.refresh()?;
        }

        Ok(())
    }

    /// Clear the current viewport (equivalent to clearviewport).
    pub fn clear_viewport(&mut self) -> BgiResult<()> {
        let window_id = self.require_window()?;
        let bg_color = self.background_color.to_rgb();

        // Fill viewport rectangle with background color
        let command = DrawCommand::Rectangle {
            x1: self.viewport.left,
            y1: self.viewport.top,
            x2: self.viewport.right,
            y2: self.viewport.bottom,
            color: bg_color,
            filled: true,
        };

        self.backend.draw(window_id, &[command])?;

        if self.auto_refresh {
            self.refresh()?;
        }

        Ok(())
    }

    /// Set current drawing color.
    pub fn set_color(&mut self, color: Color) -> BgiResult<()> {
        self.current_color = color;
        Ok(())
    }

    /// Get current drawing color.
    pub fn get_color(&self) -> Color {
        self.current_color
    }

    /// Set background color.
    pub fn set_bk_color(&mut self, color: Color) -> BgiResult<()> {
        self.background_color = color;
        Ok(())
    }

    /// Get background color.
    pub fn get_bk_color(&self) -> Color {
        self.background_color
    }

    /// Draw a line (equivalent to line).
    pub fn line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) -> BgiResult<()> {
        let window_id = self.require_window()?;
        let color = self.current_color.to_rgb();

        let command = DrawCommand::Line {
            x1,
            y1,
            x2,
            y2,
            color,
        };
        self.backend.draw(window_id, &[command])?;

        // Update current position to end of line
        self.current_position = Point::new(x2, y2);

        if self.auto_refresh {
            self.refresh()?;
        }

        Ok(())
    }

    /// Draw a line from current position (equivalent to lineto).
    pub fn line_to(&mut self, x: i32, y: i32) -> BgiResult<()> {
        let start_pos = self.current_position;
        self.line(start_pos.x, start_pos.y, x, y)
    }

    /// Draw a relative line (equivalent to linerel).
    pub fn line_rel(&mut self, dx: i32, dy: i32) -> BgiResult<()> {
        let start_pos = self.current_position;
        self.line_to(start_pos.x + dx, start_pos.y + dy)
    }

    /// Move to position (equivalent to moveto).
    pub fn move_to(&mut self, x: i32, y: i32) -> BgiResult<()> {
        self.current_position = Point::new(x, y);
        Ok(())
    }

    /// Move relatively (equivalent to moverel).
    pub fn move_rel(&mut self, dx: i32, dy: i32) -> BgiResult<()> {
        let current = self.current_position;
        self.move_to(current.x + dx, current.y + dy)
    }

    /// Get current X position.
    pub fn get_x(&self) -> i32 {
        self.current_position.x
    }

    /// Get current Y position.
    pub fn get_y(&self) -> i32 {
        self.current_position.y
    }

    /// Draw a circle (equivalent to circle).
    pub fn circle(&mut self, x: i32, y: i32, radius: i32) -> BgiResult<()> {
        let window_id = self.require_window()?;
        let color = self.current_color.to_rgb();

        let command = DrawCommand::Circle {
            x,
            y,
            radius,
            color,
            filled: false,
        };

        self.backend.draw(window_id, &[command])?;

        if self.auto_refresh {
            self.refresh()?;
        }

        Ok(())
    }

    /// Draw a filled circle (equivalent to fillellipse with equal radii).
    pub fn fill_circle(&mut self, x: i32, y: i32, radius: i32) -> BgiResult<()> {
        let window_id = self.require_window()?;
        let color = self.current_color.to_rgb();

        let command = DrawCommand::Circle {
            x,
            y,
            radius,
            color,
            filled: true,
        };

        self.backend.draw(window_id, &[command])?;

        if self.auto_refresh {
            self.refresh()?;
        }

        Ok(())
    }

    /// Draw a rectangle (equivalent to rectangle).
    pub fn rectangle(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) -> BgiResult<()> {
        let window_id = self.require_window()?;
        let color = self.current_color.to_rgb();

        let command = DrawCommand::Rectangle {
            x1,
            y1,
            x2,
            y2,
            color,
            filled: false,
        };

        self.backend.draw(window_id, &[command])?;

        if self.auto_refresh {
            self.refresh()?;
        }

        Ok(())
    }

    /// Draw a filled rectangle (equivalent to bar).
    pub fn bar(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) -> BgiResult<()> {
        let window_id = self.require_window()?;
        let color = self.current_color.to_rgb();

        let command = DrawCommand::Rectangle {
            x1,
            y1,
            x2,
            y2,
            color,
            filled: true,
        };

        self.backend.draw(window_id, &[command])?;

        if self.auto_refresh {
            self.refresh()?;
        }

        Ok(())
    }

    /// Draw a pixel (equivalent to putpixel).
    pub fn put_pixel(&mut self, x: i32, y: i32, color: Color) -> BgiResult<()> {
        let window_id = self.require_window()?;
        let rgb_color = color.to_rgb();

        let command = DrawCommand::Pixel {
            x,
            y,
            color: rgb_color,
        };

        self.backend.draw(window_id, &[command])?;

        if self.auto_refresh {
            self.refresh()?;
        }

        Ok(())
    }

    /// Get pixel color (equivalent to getpixel).
    pub fn get_pixel(&self, x: i32, y: i32) -> BgiResult<RgbColor> {
        let window_id = self.require_window()?;
        self.backend.get_pixel(window_id, x, y)
    }

    /// Present rendered content to screen (equivalent to refresh in SDL_bgi).
    pub fn refresh(&mut self) -> BgiResult<()> {
        let window_id = self.require_window()?;
        self.backend.present(window_id)
    }

    /// Set viewport (equivalent to setviewport).
    pub fn set_viewport(
        &mut self,
        left: i32,
        top: i32,
        right: i32,
        bottom: i32,
        clip: bool,
    ) -> BgiResult<()> {
        self.viewport = Viewport::new(left, top, right, bottom, clip);

        if let Some(window_id) = self.current_window {
            self.backend
                .set_viewport(window_id, self.viewport.to_rect())?;
        }

        Ok(())
    }

    /// Get viewport settings.
    pub fn get_viewport_settings(&self) -> Viewport {
        self.viewport
    }

    /// Get maximum X coordinate.
    pub fn get_max_x(&self) -> BgiResult<i32> {
        let window_id = self.require_window()?;
        let (width, _) = self.backend.window_size(window_id)?;
        Ok(width as i32 - 1)
    }

    /// Get maximum Y coordinate.
    pub fn get_max_y(&self) -> BgiResult<i32> {
        let window_id = self.require_window()?;
        let (_, height) = self.backend.window_size(window_id)?;
        Ok(height as i32 - 1)
    }

    /// Wait for key press (equivalent to getch).
    pub fn getch(&mut self) -> BgiResult<Key> {
        loop {
            // Poll for events
            let events = self.backend.poll_events();
            self.event_queue.extend(events);

            // Look for key press events
            while let Some(event) = self.event_queue.pop_front() {
                if let InputEvent::Key {
                    key, pressed: true, ..
                } = event
                {
                    return Ok(key);
                }
            }

            // Small delay to avoid busy waiting
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }

    /// Check if key is available (equivalent to kbhit).
    pub fn kbhit(&mut self) -> bool {
        // Poll for new events
        let events = self.backend.poll_events();
        self.event_queue.extend(events);

        // Check if there are any key press events in queue
        self.event_queue
            .iter()
            .any(|event| matches!(event, InputEvent::Key { pressed: true, .. }))
    }

    /// Set auto-refresh mode.
    pub fn set_auto_refresh(&mut self, auto: bool) {
        self.auto_refresh = auto;
    }

    /// Get current window ID (helper method).
    fn require_window(&self) -> BgiResult<WindowId> {
        self.current_window.ok_or_else(|| BgiError::NotInitialized)
    }
}
