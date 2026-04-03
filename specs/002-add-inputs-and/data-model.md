# Data Model: Visual Backend with Input Support

**Generated**: October 1, 2025
**Context**: BGI library visual backend and input system entities

## Core Entities

### Backend
**Purpose**: Abstract interface for graphics output and input handling
**Lifecycle**: Created at initgraph(), destroyed at closegraph()
**State**: Active/Inactive

#### Fields
- `graphics_mode: GraphicsMode` - Current BGI graphics mode configuration
- `width: u16` - Logical graphics width in pixels
- `height: u16` - Logical graphics height in pixels
- `is_initialized: bool` - Backend initialization state

#### Behaviors
- Initialize graphics system with specified mode
- Render pixel data to output (window or file)
- Capture and queue input events
- Handle window lifecycle events
- Coordinate system transformation (logical ↔ physical)

#### Relationships
- **Contains** GraphicsBuffer (1:1) - pixel data storage
- **Manages** InputEventQueue (1:1) - pending input events
- **Implements** Backend trait - rendering contract

### GraphicsBuffer
**Purpose**: Pixel data storage and manipulation
**Lifecycle**: Created with Backend, persists until closegraph()
**State**: Active (can be modified)

#### Fields
- `pixels: Vec<Color>` - Pixel data in row-major order
- `width: u16` - Buffer width in pixels
- `height: u16` - Buffer height in pixels
- `dirty_regions: Vec<Rectangle>` - Areas requiring display update

#### Behaviors
- Set/get individual pixel values
- Clear entire buffer or regions
- Mark regions as needing display refresh
- Coordinate bounds checking and clipping

#### Relationships
- **Owned by** Backend (1:1) - single buffer per backend
- **Contains** Color values - individual pixel data

### InputEvent
**Purpose**: Represents user interaction data
**Lifecycle**: Created on user action, consumed by BGI functions
**State**: Pending → Consumed

#### Variants
```rust
enum InputEvent {
    Keyboard { key: KeyCode, ascii: u8, pressed: bool },
    Mouse { x: i16, y: i16, button: MouseButton, pressed: bool },
    MouseMove { x: i16, y: i16 },
    WindowClose,
}
```

#### Fields (Keyboard)
- `key: KeyCode` - Virtual key identifier
- `ascii: u8` - ASCII character code (if applicable)
- `pressed: bool` - true for press, false for release

#### Fields (Mouse)
- `x: i16` - Mouse X coordinate (BGI logical coordinates)
- `y: i16` - Mouse Y coordinate (BGI logical coordinates)
- `button: MouseButton` - Left/Right/Middle button identifier
- `pressed: bool` - true for press, false for release

#### Relationships
- **Queued in** InputEventQueue - pending events
- **Consumed by** BGI input functions (getch, kbhit, mousex, mousey)

### InputEventQueue
**Purpose**: Manages sequence of pending input events
**Lifecycle**: Created with Backend, persists until closegraph()
**State**: Active (receiving/providing events)

#### Fields
- `events: VecDeque<InputEvent>` - FIFO queue of pending events
- `max_size: usize` - Maximum queue capacity (prevents memory growth)
- `mouse_position: (i16, i16)` - Current mouse position cache

#### Behaviors
- Push new input events from backend
- Pop events for BGI input functions
- Filter/deduplicate mouse move events
- Manage queue size limits (drop oldest when full)
- Cache current mouse position for mousex/mousey queries

#### Relationships
- **Owned by** Backend (1:1) - single queue per backend
- **Contains** InputEvent collection - pending user actions

### WindowState
**Purpose**: Tracks visual window properties and state
**Lifecycle**: Created with visual backend, destroyed on close
**State**: Open/Minimized/Closed

#### Fields
- `physical_width: u32` - Actual window width in screen pixels
- `physical_height: u32` - Actual window height in screen pixels
- `scale_x: f32` - Horizontal scaling factor (physical/logical)
- `scale_y: f32` - Vertical scaling factor (physical/logical)
- `is_visible: bool` - Window visibility state

#### Behaviors
- Handle window resize events
- Calculate coordinate transformations
- Manage window visibility and focus
- Update scaling factors on resize

#### Relationships
- **Owned by** VisualBackend (1:1) - one window per visual backend
- **Used by** Backend for coordinate transformations

### BackendSelector
**Purpose**: Runtime selection between visual and headless backends
**Lifecycle**: Global, persists for application lifetime
**State**: Selected backend type

#### Fields
- `backend_type: BackendType` - Currently selected backend
- `auto_fallback: bool` - Whether to fallback to headless if visual fails

#### Variants
```rust
enum BackendType {
    Visual(VisualBackend),     // minifb window backend
    Headless(HeadlessBackend), // PGM file backend
}
```

#### Behaviors
- Select appropriate backend at runtime
- Handle backend initialization failures
- Coordinate backend switching (if supported)
- Apply fallback logic when visual backend unavailable

#### Relationships
- **Manages** Backend implementations - active backend instance
- **Used by** BGI initialization functions

## Entity Relationships

```
┌─────────────────┐    ┌─────────────────┐
│ BackendSelector │────│ Backend (trait) │
└─────────────────┘    └─────────────────┘
                                │
                       ┌────────┴────────┐
                       │                 │
            ┌──────────────────┐  ┌─────────────────┐
            │ VisualBackend    │  │ HeadlessBackend │
            │ (minifb)         │  │ (PGM files)     │
            └──────────────────┘  └─────────────────┘
                       │
              ┌────────┼────────┐
              │        │        │
    ┌─────────────┐ ┌──────────────┐ ┌─────────────┐
    │WindowState  │ │GraphicsBuffer│ │InputEventQueue│
    └─────────────┘ └──────────────┘ └─────────────┘
                           │               │
                    ┌─────────────┐ ┌─────────────┐
                    │ Color       │ │ InputEvent  │
                    │ (pixels)    │ │ (user data) │
                    └─────────────┘ └─────────────┘
```

## State Transitions

### Backend Lifecycle
```
Uninitialized → initgraph() → Initialized → closegraph() → Destroyed
                    ↓              ↓
              [Error State] ← [Runtime Error]
```

### Input Event Flow
```
User Action → Backend Capture → Queue → BGI Function → Consumed
    ↓              ↓             ↓         ↓           ↓
 [Mouse Move]  [Filter Move]  [FIFO]   [getch()]   [Removed]
 [Key Press]   [Add to Queue] [Limit]  [kbhit()]   [Returned]
```

### Window State Changes
```
Created → Visible → [Resize] → Updated Scale → Refresh Display
   ↓         ↓         ↓           ↓              ↓
[Init]   [Show]   [User Action] [Recalculate] [Redraw Buffer]
```

## Validation Rules

### GraphicsBuffer
- Width and height must be > 0
- Pixel coordinates must be within bounds (0 ≤ x < width, 0 ≤ y < height)
- Color values must be valid BGI color codes

### InputEventQueue
- Queue size must not exceed max_size limit
- Mouse coordinates must be in valid logical coordinate range
- ASCII codes must be valid (0-255)

### Backend
- Can only have one active backend at a time
- Backend must be initialized before graphics operations
- Coordinate transformations must preserve aspect ratio

### WindowState (Visual Backend)
- Physical dimensions must be > 0
- Scale factors must be > 0
- Logical-to-physical coordinate mapping must be bijective

## Performance Constraints

### Memory
- GraphicsBuffer size scales with graphics mode (typical: 640x480 = 1.2MB)
- InputEventQueue bounded to 1000 events max
- No dynamic allocations during normal operation

### Processing
- Coordinate transformations must be O(1)
- Event queue operations must be O(1) amortized
- Pixel operations must be cache-friendly (row-major access)

### Threading
- All operations are single-threaded (BGI compatibility)
- No locks or synchronization primitives
- Backend switching must be atomic at BGI function boundaries
