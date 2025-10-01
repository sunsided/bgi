# Backend API Contract

**Purpose**: Define the interface contract for all BGI backend implementations
**Version**: 1.0.0

## Backend Trait Contract

```rust
/// Core backend interface for BGI graphics and input
pub trait Backend {
    type Error: std::fmt::Debug;

    /// Initialize graphics system with specified mode
    /// Returns error if backend cannot support the requested mode
    fn init_graphics(&mut self, mode: GraphicsMode) -> Result<(), Self::Error>;

    /// Shutdown graphics system and release resources
    fn close_graphics(&mut self);

    /// Set pixel at logical coordinates to specified color
    /// Must handle coordinate transformation and bounds checking
    fn set_pixel(&mut self, x: i16, y: i16, color: Color);

    /// Get pixel color at logical coordinates
    /// Must handle coordinate transformation and bounds checking
    fn get_pixel(&self, x: i16, y: i16) -> Color;

    /// Flush graphics buffer to display/output
    /// For visual backends: update window display
    /// For headless backends: write to file/stream
    fn flush(&mut self);

    /// Check if graphics system is initialized
    fn is_initialized(&self) -> bool;

    /// Get current graphics mode information
    fn get_mode_info(&self) -> Option<GraphicsMode>;

    // Input handling

    /// Poll for next keyboard event
    /// Returns None if no events pending
    fn poll_keyboard(&mut self) -> Option<KeyEvent>;

    /// Check if keyboard events are available
    /// Non-blocking check for event availability
    fn has_keyboard_input(&self) -> bool;

    /// Poll for next mouse event
    /// Returns None if no events pending
    fn poll_mouse(&mut self) -> Option<MouseEvent>;

    /// Get current mouse position in logical coordinates
    /// Returns last known position if no recent events
    fn get_mouse_position(&self) -> (i16, i16);

    /// Check for window close request
    /// Returns true if user requested window close
    fn should_close(&self) -> bool;
}
```

## Data Types Contract

```rust
/// BGI graphics mode specification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GraphicsMode {
    pub width: u16,
    pub height: u16,
    pub colors: u16,
    pub mode_id: u16,
}

/// BGI color representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub bgi_index: u8, // Original BGI color index
}

/// Keyboard input event
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyEvent {
    pub key_code: KeyCode,
    pub ascii_code: u8,
    pub is_pressed: bool,
    pub modifiers: KeyModifiers,
}

/// Mouse input event
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MouseEvent {
    pub x: i16,           // Logical coordinates
    pub y: i16,           // Logical coordinates
    pub button: MouseButton,
    pub is_pressed: bool,
}

/// Virtual key codes (subset for BGI compatibility)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyCode {
    // ASCII printable keys use ASCII values directly
    Ascii(u8),

    // Special keys
    Enter,
    Backspace,
    Tab,
    Escape,
    Space,

    // Arrow keys
    Up,
    Down,
    Left,
    Right,

    // Function keys
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10,

    // Other special keys
    Insert,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
}

/// Mouse button identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

/// Keyboard modifier state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyModifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
}
```

## Implementation Requirements

### Coordinate System
- All pixel operations use BGI logical coordinates (origin top-left)
- Backend must handle transformation to physical coordinates
- Mouse events must be transformed to logical coordinates
- Bounds checking must be performed in logical coordinate space

### Error Handling
- Backend errors must not panic
- Invalid coordinates should be silently clipped
- Unsupported operations should return appropriate error values
- Graphics errors should be reportable via BGI's graphresult() mechanism

### Performance
- set_pixel/get_pixel must be O(1) operations
- Input polling must be non-blocking
- flush() should batch updates for efficiency
- No dynamic memory allocation in hot paths

### Thread Safety
- Backend implementations need not be thread-safe
- All operations assumed to be on single thread
- No concurrent access patterns required

### Resource Management
- init_graphics() must be idempotent (safe to call multiple times)
- close_graphics() must clean up all resources
- Backends must handle partial initialization failures gracefully

## Behavioral Contracts

### Graphics Operations
1. **Pixel Setting**: set_pixel(x, y, color) must be visible after next flush()
2. **Pixel Getting**: get_pixel(x, y) must return last set color or background
3. **Bounds Clipping**: Operations outside logical bounds are silently ignored
4. **Color Mapping**: BGI color indices must map consistently to RGB values

### Input Handling
1. **Keyboard Polling**: poll_keyboard() returns events in chronological order
2. **Non-blocking**: All input functions must return immediately
3. **Event Persistence**: Events remain available until explicitly consumed
4. **Mouse Tracking**: Mouse position always reflects most recent location

### Window Management
1. **Lifecycle**: Window exists from init_graphics() to close_graphics()
2. **Visibility**: Graphics output should be visible immediately after flush()
3. **Responsiveness**: Window should respond to system close requests
4. **Resizing**: Window resize should update coordinate transformation

### Error Reporting
1. **Initialization Errors**: init_graphics() failure should set appropriate error code
2. **Runtime Errors**: Operational failures should not crash the program
3. **State Errors**: Operations on uninitialized backend should fail gracefully
4. **Resource Errors**: Out of memory/resources should be handled appropriately

## Testing Contract

All backend implementations must pass the following test scenarios:

### Basic Graphics
- Set and retrieve pixels at various coordinates
- Handle out-of-bounds coordinates gracefully
- Flush operations complete without error
- Color mapping maintains BGI compatibility

### Input Processing
- Keyboard events are captured and retrievable
- Mouse events provide correct logical coordinates
- Event ordering is preserved (FIFO)
- Non-blocking behavior is maintained

### Lifecycle Management
- Multiple init/close cycles work correctly
- Resource cleanup is complete
- Error states are recoverable
- Backend state is consistent

### Edge Cases
- Zero-size graphics modes
- Maximum coordinate values
- High-frequency input events
- Resource exhaustion scenarios
