//! Unit tests for backend trait and implementations.

use bgi::{
    backend::*,
    color::RgbColor,
    error::{BgiError, BgiResult},
    types::{GraphicsMode, Point, Rect},
    window::WindowId,
};

#[test]
fn test_backend_capabilities() {
    let caps = BackendCapabilities {
        multi_window: true,
        hardware_acceleration: false,
        alpha_blending: true,
        fullscreen: true,
        resizable: false,
    };

    assert!(caps.multi_window);
    assert!(!caps.hardware_acceleration);
    assert!(caps.alpha_blending);
    assert!(caps.fullscreen);
    assert!(!caps.resizable);
}

#[test]
fn test_draw_command_variants() {
    let clear_cmd = DrawCommand::Clear {
        color: RgbColor::new(255, 0, 0),
    };

    match clear_cmd {
        DrawCommand::Clear { color } => {
            assert_eq!(color.r, 255);
            assert_eq!(color.g, 0);
            assert_eq!(color.b, 0);
        }
        _ => panic!("Expected Clear command"),
    }

    let pixel_cmd = DrawCommand::Pixel {
        x: 100,
        y: 200,
        color: RgbColor::new(0, 255, 0),
    };

    match pixel_cmd {
        DrawCommand::Pixel { x, y, color } => {
            assert_eq!(x, 100);
            assert_eq!(y, 200);
            assert_eq!(color.g, 255);
        }
        _ => panic!("Expected Pixel command"),
    }

    let line_cmd = DrawCommand::Line {
        x1: 0,
        y1: 0,
        x2: 100,
        y2: 100,
        color: RgbColor::new(0, 0, 255),
    };

    match line_cmd {
        DrawCommand::Line {
            x1,
            y1,
            x2,
            y2,
            color,
        } => {
            assert_eq!(x1, 0);
            assert_eq!(y1, 0);
            assert_eq!(x2, 100);
            assert_eq!(y2, 100);
            assert_eq!(color.b, 255);
        }
        _ => panic!("Expected Line command"),
    }
}

#[test]
fn test_draw_command_shapes() {
    let rect_cmd = DrawCommand::Rectangle {
        x1: 10,
        y1: 20,
        x2: 110,
        y2: 120,
        color: RgbColor::new(128, 128, 128),
        filled: true,
    };

    match rect_cmd {
        DrawCommand::Rectangle {
            x1,
            y1,
            x2,
            y2,
            color,
            filled,
        } => {
            assert_eq!(x1, 10);
            assert_eq!(y1, 20);
            assert_eq!(x2, 110);
            assert_eq!(y2, 120);
            assert_eq!(color.r, 128);
            assert!(filled);
        }
        _ => panic!("Expected Rectangle command"),
    }

    let circle_cmd = DrawCommand::Circle {
        x: 50,
        y: 75,
        radius: 25,
        color: RgbColor::new(255, 255, 0),
        filled: false,
    };

    match circle_cmd {
        DrawCommand::Circle {
            x,
            y,
            radius,
            color,
            filled,
        } => {
            assert_eq!(x, 50);
            assert_eq!(y, 75);
            assert_eq!(radius, 25);
            assert_eq!(color.r, 255);
            assert_eq!(color.g, 255);
            assert!(!filled);
        }
        _ => panic!("Expected Circle command"),
    }

    let ellipse_cmd = DrawCommand::Ellipse {
        x: 100,
        y: 150,
        rx: 30,
        ry: 20,
        color: RgbColor::new(255, 0, 255),
        filled: true,
    };

    match ellipse_cmd {
        DrawCommand::Ellipse {
            x,
            y,
            rx,
            ry,
            color,
            filled,
        } => {
            assert_eq!(x, 100);
            assert_eq!(y, 150);
            assert_eq!(rx, 30);
            assert_eq!(ry, 20);
            assert_eq!(color.r, 255);
            assert_eq!(color.b, 255);
            assert!(filled);
        }
        _ => panic!("Expected Ellipse command"),
    }
}

#[test]
fn test_draw_command_arc() {
    let arc_cmd = DrawCommand::Arc {
        x: 200,
        y: 250,
        start_angle: 0,
        end_angle: 90,
        radius: 40,
        color: RgbColor::new(0, 255, 255),
    };

    match arc_cmd {
        DrawCommand::Arc {
            x,
            y,
            start_angle,
            end_angle,
            radius,
            color,
        } => {
            assert_eq!(x, 200);
            assert_eq!(y, 250);
            assert_eq!(start_angle, 0);
            assert_eq!(end_angle, 90);
            assert_eq!(radius, 40);
            assert_eq!(color.g, 255);
            assert_eq!(color.b, 255);
        }
        _ => panic!("Expected Arc command"),
    }
}

#[test]
fn test_draw_command_text() {
    let text_cmd = DrawCommand::Text {
        x: 300,
        y: 350,
        text: "Hello World".to_string(),
        color: RgbColor::new(64, 128, 192),
    };

    match text_cmd {
        DrawCommand::Text { x, y, text, color } => {
            assert_eq!(x, 300);
            assert_eq!(y, 350);
            assert_eq!(text, "Hello World");
            assert_eq!(color.r, 64);
            assert_eq!(color.g, 128);
            assert_eq!(color.b, 192);
        }
        _ => panic!("Expected Text command"),
    }
}

#[test]
fn test_draw_command_image() {
    let pixels = vec![0xFFFF0000, 0xFF00FF00, 0xFF0000FF, 0xFFFFFFFF];
    let image_cmd = DrawCommand::Image {
        x: 400,
        y: 450,
        width: 2,
        height: 2,
        pixels: pixels.clone(),
    };

    match image_cmd {
        DrawCommand::Image {
            x,
            y,
            width,
            height,
            pixels,
        } => {
            assert_eq!(x, 400);
            assert_eq!(y, 450);
            assert_eq!(width, 2);
            assert_eq!(height, 2);
            assert_eq!(pixels.len(), 4);
            assert_eq!(pixels[0], 0xFFFF0000); // Red
            assert_eq!(pixels[1], 0xFF00FF00); // Green
            assert_eq!(pixels[2], 0xFF0000FF); // Blue
            assert_eq!(pixels[3], 0xFFFFFFFF); // White
        }
        _ => panic!("Expected Image command"),
    }
}

#[test]
fn test_draw_command_clone() {
    let original = DrawCommand::Circle {
        x: 50,
        y: 75,
        radius: 25,
        color: RgbColor::new(255, 0, 0),
        filled: true,
    };

    let cloned = original.clone();

    match (original, cloned) {
        (
            DrawCommand::Circle {
                x: x1,
                y: y1,
                radius: r1,
                ..
            },
            DrawCommand::Circle {
                x: x2,
                y: y2,
                radius: r2,
                ..
            },
        ) => {
            assert_eq!(x1, x2);
            assert_eq!(y1, y2);
            assert_eq!(r1, r2);
        }
        _ => panic!("Clone should preserve command type"),
    }
}

#[test]
fn test_create_default_backend() {
    let result = create_default_backend();

    // Should not panic and should return some backend
    // The actual backend type depends on feature flags
    match result {
        Ok(backend) => {
            // Test that we can get capabilities
            let _caps = backend.capabilities();
        }
        Err(_) => {
            // It's ok if backend creation fails in test environment
            // as long as it doesn't panic
        }
    }
}

#[test]
fn test_create_pixel_buffer_backend() {
    let result = create_pixel_buffer_backend();

    assert!(result.is_ok());
    let backend = result.unwrap();

    // Test capabilities of pixel buffer backend
    let caps = backend.capabilities();

    // Pixel buffer backend has specific characteristics
    assert!(caps.multi_window); // Should support multiple windows
    assert!(!caps.hardware_acceleration); // Software rendering
    // Other capabilities depend on implementation
}

// Mock backend for testing trait functionality
struct MockBackend {
    initialized: bool,
    windows: std::collections::HashMap<WindowId, (u32, u32)>,
    current_window: Option<WindowId>,
    error_on_init: bool,
}

impl MockBackend {
    fn new() -> Self {
        Self {
            initialized: false,
            windows: std::collections::HashMap::new(),
            current_window: None,
            error_on_init: false,
        }
    }

    fn with_error_on_init() -> Self {
        Self {
            initialized: false,
            windows: std::collections::HashMap::new(),
            current_window: None,
            error_on_init: true,
        }
    }
}

impl Backend for MockBackend {
    fn capabilities(&self) -> BackendCapabilities {
        BackendCapabilities {
            multi_window: true,
            hardware_acceleration: false,
            alpha_blending: true,
            fullscreen: false,
            resizable: true,
        }
    }

    fn init(&mut self) -> BgiResult<()> {
        if self.error_on_init {
            return Err(BgiError::InitializationError("Mock error".to_string()));
        }
        self.initialized = true;
        Ok(())
    }

    fn shutdown(&mut self) -> BgiResult<()> {
        self.initialized = false;
        self.windows.clear();
        self.current_window = None;
        Ok(())
    }

    fn create_window(
        &mut self,
        width: u32,
        height: u32,
        _title: Option<&str>,
        _mode: GraphicsMode,
    ) -> BgiResult<WindowId> {
        if !self.initialized {
            return Err(BgiError::NotInitialized);
        }

        let window_id = WindowId(self.windows.len() as u32);
        self.windows.insert(window_id, (width, height));

        if self.current_window.is_none() {
            self.current_window = Some(window_id);
        }

        Ok(window_id)
    }

    fn close_window(&mut self, window_id: WindowId) -> BgiResult<()> {
        if !self.windows.contains_key(&window_id) {
            return Err(BgiError::InvalidWindow);
        }

        self.windows.remove(&window_id);

        if self.current_window == Some(window_id) {
            self.current_window = self.windows.keys().next().copied();
        }

        Ok(())
    }

    fn set_current_window(&mut self, window_id: WindowId) -> BgiResult<()> {
        if !self.windows.contains_key(&window_id) {
            return Err(BgiError::InvalidWindow);
        }

        self.current_window = Some(window_id);
        Ok(())
    }

    fn current_window(&self) -> Option<WindowId> {
        self.current_window
    }

    fn window_size(&self, window_id: WindowId) -> BgiResult<(u32, u32)> {
        self.windows
            .get(&window_id)
            .copied()
            .ok_or(BgiError::InvalidWindow)
    }

    fn set_window_title(&mut self, window_id: WindowId, _title: &str) -> BgiResult<()> {
        if !self.windows.contains_key(&window_id) {
            return Err(BgiError::InvalidWindow);
        }
        Ok(())
    }

    fn is_window_valid(&self, window_id: WindowId) -> bool {
        self.windows.contains_key(&window_id)
    }

    fn draw(&mut self, window_id: WindowId, _commands: &[DrawCommand]) -> BgiResult<()> {
        if !self.windows.contains_key(&window_id) {
            return Err(BgiError::InvalidWindow);
        }
        Ok(())
    }

    fn present(&mut self, window_id: WindowId) -> BgiResult<()> {
        if !self.windows.contains_key(&window_id) {
            return Err(BgiError::InvalidWindow);
        }
        Ok(())
    }

    fn get_pixel(&self, window_id: WindowId, _x: i32, _y: i32) -> BgiResult<RgbColor> {
        if !self.windows.contains_key(&window_id) {
            return Err(BgiError::InvalidWindow);
        }
        Ok(RgbColor::new(0, 0, 0))
    }

    fn set_viewport(&mut self, window_id: WindowId, _rect: Rect) -> BgiResult<()> {
        if !self.windows.contains_key(&window_id) {
            return Err(BgiError::InvalidWindow);
        }
        Ok(())
    }

    fn viewport(&self, window_id: WindowId) -> BgiResult<Rect> {
        if !self.windows.contains_key(&window_id) {
            return Err(BgiError::InvalidWindow);
        }
        Ok(Rect::new(0, 0, 639, 479))
    }

    fn poll_events(&mut self) -> Vec<InputEvent> {
        vec![]
    }

    fn has_events(&self) -> bool {
        false
    }

    fn set_fullscreen(&mut self, window_id: WindowId, _fullscreen: bool) -> BgiResult<()> {
        if !self.windows.contains_key(&window_id) {
            return Err(BgiError::InvalidWindow);
        }
        Ok(())
    }

    fn screen_size(&self) -> BgiResult<(u32, u32)> {
        Ok((1920, 1080))
    }

    fn copy_surface(
        &mut self,
        window_id: WindowId,
        _src_rect: Rect,
        _dst_x: i32,
        _dst_y: i32,
    ) -> BgiResult<()> {
        if !self.windows.contains_key(&window_id) {
            return Err(BgiError::InvalidWindow);
        }
        Ok(())
    }

    fn get_buffer(&self, window_id: WindowId) -> BgiResult<Vec<u32>> {
        if !self.windows.contains_key(&window_id) {
            return Err(BgiError::InvalidWindow);
        }
        Ok(vec![0; 640 * 480])
    }

    fn set_buffer(&mut self, window_id: WindowId, _buffer: &[u32]) -> BgiResult<()> {
        if !self.windows.contains_key(&window_id) {
            return Err(BgiError::InvalidWindow);
        }
        Ok(())
    }

    fn load_image(&mut self, _filename: &str) -> BgiResult<(u32, u32, Vec<u32>)> {
        Ok((10, 10, vec![0xFFFFFFFF; 100]))
    }

    fn save_image(
        &self,
        _filename: &str,
        _width: u32,
        _height: u32,
        _pixels: &[u32],
    ) -> BgiResult<()> {
        Ok(())
    }
}

#[test]
fn test_mock_backend_basic_operations() {
    let mut backend = MockBackend::new();

    // Test capabilities
    let caps = backend.capabilities();
    assert!(caps.multi_window);
    assert!(!caps.hardware_acceleration);
    assert!(caps.alpha_blending);

    // Test initialization
    assert!(backend.init().is_ok());

    // Test window creation
    let window1 = backend.create_window(640, 480, Some("Test Window"), GraphicsMode::default());
    assert!(window1.is_ok());
    let window1 = window1.unwrap();

    // Should be current window
    assert_eq!(backend.current_window(), Some(window1));

    // Test window size
    let size = backend.window_size(window1);
    assert!(size.is_ok());
    assert_eq!(size.unwrap(), (640, 480));

    // Test window validity
    assert!(backend.is_window_valid(window1));
    assert!(!backend.is_window_valid(WindowId(999)));
}

#[test]
fn test_mock_backend_multiple_windows() {
    let mut backend = MockBackend::new();
    backend.init().unwrap();

    // Create multiple windows
    let window1 = backend
        .create_window(640, 480, None, GraphicsMode::default())
        .unwrap();
    let window2 = backend
        .create_window(800, 600, None, GraphicsMode::default())
        .unwrap();

    // Test switching current window
    assert!(backend.set_current_window(window2).is_ok());
    assert_eq!(backend.current_window(), Some(window2));

    // Test different window sizes
    assert_eq!(backend.window_size(window1).unwrap(), (640, 480));
    assert_eq!(backend.window_size(window2).unwrap(), (800, 600));

    // Close a window
    assert!(backend.close_window(window1).is_ok());
    assert!(!backend.is_window_valid(window1));
    assert!(backend.is_window_valid(window2));

    // Current window should remain valid
    assert_eq!(backend.current_window(), Some(window2));
}

#[test]
fn test_mock_backend_error_conditions() {
    let mut backend = MockBackend::new();

    // Operations without initialization should fail
    let result = backend.create_window(640, 480, None, GraphicsMode::default());
    assert!(result.is_err());

    // Initialize and create window
    backend.init().unwrap();
    let window = backend
        .create_window(640, 480, None, GraphicsMode::default())
        .unwrap();

    // Operations on invalid window should fail
    let invalid_window = WindowId(999);
    assert!(backend.window_size(invalid_window).is_err());
    assert!(backend.set_current_window(invalid_window).is_err());
    assert!(backend.close_window(invalid_window).is_err());
    assert!(backend.draw(invalid_window, &[]).is_err());
    assert!(backend.present(invalid_window).is_err());
    assert!(backend.get_pixel(invalid_window, 0, 0).is_err());
    assert!(
        backend
            .set_viewport(invalid_window, Rect::new(0, 0, 100, 100))
            .is_err()
    );
    assert!(backend.viewport(invalid_window).is_err());
}

#[test]
fn test_mock_backend_drawing_operations() {
    let mut backend = MockBackend::new();
    backend.init().unwrap();
    let window = backend
        .create_window(640, 480, None, GraphicsMode::default())
        .unwrap();

    // Test drawing commands
    let commands = vec![
        DrawCommand::Clear {
            color: RgbColor::new(0, 0, 0),
        },
        DrawCommand::Pixel {
            x: 100,
            y: 200,
            color: RgbColor::new(255, 0, 0),
        },
        DrawCommand::Line {
            x1: 0,
            y1: 0,
            x2: 100,
            y2: 100,
            color: RgbColor::new(0, 255, 0),
        },
    ];

    assert!(backend.draw(window, &commands).is_ok());
    assert!(backend.present(window).is_ok());

    // Test pixel operations
    assert!(backend.get_pixel(window, 50, 75).is_ok());

    // Test buffer operations
    let buffer = backend.get_buffer(window);
    assert!(buffer.is_ok());
    let buffer = buffer.unwrap();
    assert_eq!(buffer.len(), 640 * 480);

    assert!(backend.set_buffer(window, &buffer).is_ok());
}

#[test]
fn test_mock_backend_viewport_operations() {
    let mut backend = MockBackend::new();
    backend.init().unwrap();
    let window = backend
        .create_window(640, 480, None, GraphicsMode::default())
        .unwrap();

    // Test viewport operations
    let viewport = Rect::new(10, 20, 300, 400);
    assert!(backend.set_viewport(window, viewport).is_ok());

    let current_viewport = backend.viewport(window);
    assert!(current_viewport.is_ok());
}

#[test]
fn test_mock_backend_fullscreen_and_screen_size() {
    let mut backend = MockBackend::new();
    backend.init().unwrap();
    let window = backend
        .create_window(640, 480, None, GraphicsMode::default())
        .unwrap();

    // Test screen size
    let screen_size = backend.screen_size();
    assert!(screen_size.is_ok());
    assert_eq!(screen_size.unwrap(), (1920, 1080));

    // Test fullscreen
    assert!(backend.set_fullscreen(window, true).is_ok());
    assert!(backend.set_fullscreen(window, false).is_ok());
}

#[test]
fn test_mock_backend_image_operations() {
    let mut backend = MockBackend::new();
    backend.init().unwrap();

    // Test image loading
    let image = backend.load_image("test.png");
    assert!(image.is_ok());
    let (width, height, pixels) = image.unwrap();
    assert_eq!(width, 10);
    assert_eq!(height, 10);
    assert_eq!(pixels.len(), 100);

    // Test image saving
    assert!(
        backend
            .save_image("output.png", width, height, &pixels)
            .is_ok()
    );
}

#[test]
fn test_mock_backend_shutdown() {
    let mut backend = MockBackend::new();
    backend.init().unwrap();

    let _window1 = backend
        .create_window(640, 480, None, GraphicsMode::default())
        .unwrap();
    let _window2 = backend
        .create_window(800, 600, None, GraphicsMode::default())
        .unwrap();

    // Shutdown should clear everything
    assert!(backend.shutdown().is_ok());
    assert_eq!(backend.current_window(), None);

    // Creating windows after shutdown should fail
    let result = backend.create_window(640, 480, None, GraphicsMode::default());
    assert!(result.is_err());
}

#[test]
fn test_backend_init_error() {
    let mut backend = MockBackend::with_error_on_init();

    let result = backend.init();
    assert!(result.is_err());

    // Should not be initialized
    let create_result = backend.create_window(640, 480, None, GraphicsMode::default());
    assert!(create_result.is_err());
}

#[test]
fn test_input_event_placeholder() {
    let input_event = InputEvent::Placeholder;

    // Just test that it can be created and matched
    match input_event {
        InputEvent::Placeholder => {
            // This is expected
        }
        InputEvent::Key { .. } => {
            // Key events from actual backend
        }
        InputEvent::Mouse { .. } => {
            // Mouse events from actual backend
        }
    }
}
