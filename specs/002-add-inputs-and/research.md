# Research: Visual Backend with Input Support

**Generated**: October 1, 2025
**Context**: BGI library visual backend and interactive input implementation

## Technology Decisions

### Visual Backend: minifb

**Decision**: Use `minifb` crate for window creation and graphics display

**Rationale**:
- Lightweight, minimal dependencies compared to winit/SDL
- Direct framebuffer access aligns with BGI's pixel-based graphics model
- Cross-platform support (Windows, macOS, Linux)
- Simple API for BGI's requirements (create window, display buffer, handle events)
- No complex UI framework overhead - just raw pixel display
- Active maintenance and proven stability

**Alternatives Considered**:
- `winit`: More complex windowing abstraction, event loop complexity
- `sdl2`: Heavier dependency, C library bindings, SDL-specific patterns
- `pixels`: WebGPU-based, too modern/complex for BGI compatibility needs
- Custom platform code: Excessive maintenance burden, poor portability

### Headless Backend: PGM Format

**Decision**: Implement PGM (Portable Graymap) file output for headless scenarios

**Rationale**:
- Simple ASCII format, easy to generate and validate
- Viewable with standard image tools (GIMP, ImageMagick, web browsers)
- Perfect for automated testing and CI/CD validation
- Minimal implementation complexity
- Human-readable for debugging purposes

**Alternatives Considered**:
- PNG output: Requires compression library, more complexity
- BMP format: Binary format harder to debug
- Raw pixel dumps: Not viewable without custom tools
- No headless support: Blocks testing in CI environments

### Input Event System

**Decision**: Non-blocking event polling with internal event queue

**Rationale**:
- Preserves BGI's polling-based input model (`kbhit()`, `getch()`)
- Zero-cost when no input functions called
- Compatible with existing BGI programs expecting polling behavior
- Event queue prevents input loss during frame rendering

**Alternatives Considered**:
- Callback-based events: Breaks BGI API compatibility
- Synchronous blocking input: Breaks BGI non-blocking semantics
- No input support: Eliminates interactive validation capability

### Coordinate System Handling

**Decision**: Automatic scaling between physical window and BGI logical coordinates

**Rationale**:
- Maintains BGI API compatibility (mousex/mousey return logical coordinates)
- Enables modern window sizes while preserving classic BGI behavior
- Aspect-correct scaling prevents distortion
- Allows window resizing without breaking coordinate system

**Alternatives Considered**:
- Fixed window sizes: Poor modern UX, tiny windows on high-DPI displays
- Physical coordinates: Breaks BGI API compatibility
- No scaling: Graphics too small to see effectively

### Font Integration

**Decision**: Use existing `bgi-stroked-fonts` crate for font data

**Rationale**:
- Provides complete BGI font data without reimplementation
- Proven compatibility with classic BGI fonts
- Reduces development scope and maintenance burden
- Already available as external resource

**Alternatives Considered**:
- Reimplement font data: Unnecessary duplication of effort
- Modern font rendering: Breaks BGI visual compatibility
- No font support: Incomplete BGI implementation

## Integration Patterns

### Backend Trait Design

```rust
trait Backend {
    fn init_graphics(&mut self, width: u16, height: u16) -> Result<(), GraphicsError>;
    fn close_graphics(&mut self);
    fn set_pixel(&mut self, x: i16, y: i16, color: Color);
    fn get_pixel(&self, x: i16, y: i16) -> Color;
    fn flush(&mut self);

    // Input support
    fn poll_keyboard(&mut self) -> Option<KeyEvent>;
    fn poll_mouse(&mut self) -> Option<MouseEvent>;
}
```

### Event Queue Strategy

- Internal VecDeque for event buffering
- Backend implementations push events to queue
- BGI functions (getch, kbhit) poll from queue
- Automatic queue management (size limits, aging)

### Error Handling Approach

- Backend errors use BGI's `graphresult()` mechanism
- No panics or Result types in public BGI API
- Internal error state management
- Graceful degradation when backend unavailable

## Performance Characteristics

### Expected Performance Targets

- **Frame Rate**: 60 FPS for smooth visual feedback
- **Input Latency**: <1ms from event to availability via getch()
- **Memory Usage**: <10MB additional for typical BGI graphics modes
- **Startup Time**: <100ms for window creation and initialization

### Scaling Considerations

- Event queue bounded to prevent memory growth
- Graphics buffer size scales with BGI graphics mode
- Backend selection at compile time for zero-cost abstraction
- No dynamic dispatch in critical rendering paths

## Risk Assessment

### Technical Risks

- **minifb compatibility**: Low risk - well-established crate
- **Input simulation in PGM mode**: Medium risk - stdin interaction complexity
- **Coordinate system edge cases**: Low risk - well-defined scaling math
- **Font integration complexity**: Low risk - existing crate integration

### Mitigation Strategies

- Comprehensive test suite covering edge cases
- PGM mode testing via scripted input files
- Visual regression testing for coordinate accuracy
- Gradual rollout with feature flags for backend selection

## Testing Strategy

### Unit Tests
- Backend trait implementations
- Coordinate system transformations
- Event queue management
- Error handling paths

### Integration Tests
- End-to-end BGI program execution
- Cross-backend compatibility validation
- Visual output verification (pixel-perfect)
- Input simulation and response testing

### Validation Approach
- Existing BGI examples as acceptance tests
- Visual comparison with reference outputs
- Interactive testing for input responsiveness
- Performance benchmarking vs baseline
