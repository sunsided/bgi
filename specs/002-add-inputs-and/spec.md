# Feature Specification: Visual Backend with Input Support

**Feature Branch**: `002-add-inputs-and`  
**Created**: October 1, 2025  
**Status**: Draft  
**Input**: User description: "Add inputs and minifb backend. The examples currently do not provide any visual interaction, which makes them hard to validate. I would like to use minifb (instead of winit) as a starting point. For debugging, rendering the graphics buffer via PGM is also a viable option, if it fits."

## User Scenarios & Testing

### Primary User Story
As a developer using the BGI library, I want to run examples that display graphics in a window with interactive input capabilities so that I can visually validate that the graphics functions work correctly and test interactive features like mouse clicks and keyboard input.

### Acceptance Scenarios
1. **Given** a BGI example program, **When** I run it, **Then** it opens a window displaying the rendered graphics output
2. **Given** a graphics window is open, **When** I press a key, **Then** the BGI program receives the key press event
3. **Given** a graphics window is open, **When** I click the mouse, **Then** the BGI program receives the mouse click coordinates
4. **Given** a graphics window is open, **When** I move the mouse, **Then** the BGI program can query the current mouse position
5. **Given** a BGI program finishes execution, **When** it calls closegraph(), **Then** the graphics window closes properly
6. **Given** debugging is needed, **When** graphics buffer is rendered to PGM format, **Then** the output can be viewed and validated without a display window

### Edge Cases
- What happens when the window is resized by the user?
- How does the system handle window close events?
- What occurs when multiple windows are requested?
- How does input handling work when the window loses focus?

## Requirements

### Functional Requirements

- **FR-001**: System MUST display BGI graphics output in a visual window that users can see
- **FR-002**: System MUST capture keyboard input events and make them available to BGI programs via getch() and kbhit()
- **FR-003**: System MUST capture mouse input events including clicks, movements, and button states
- **FR-004**: System MUST provide mouse position querying capabilities via mousex() and mousey() functions
- **FR-005**: System MUST support proper window lifecycle management (open, display, close)
- **FR-006**: System MUST provide alternative PGM file output for debugging scenarios where visual display is not available
- **FR-007**: Graphics rendering MUST maintain pixel-perfect accuracy compared to classic BGI behavior (exact RGB values, no anti-aliasing differences)
- **FR-008**: Input event handling MUST be non-blocking to preserve BGI's polling-based input model
- **FR-009**: System MUST support standard BGI graphics modes and resolutions in the visual window
- **FR-010**: Window display MUST refresh at minimum 30 FPS during active graphics operations and complete flush operations within 16ms
- **FR-011**: Window resizing MUST scale graphics content while preserving BGI's logical coordinate system and dimensions
- **FR-012**: System MUST support runtime switching between visual window and PGM file output backends
- **FR-013**: System MUST automatically fallback to PGM output when visual display is unavailable
- **FR-014**: PGM backend MUST handle keyboard input functions by reading from stdin for simulation
- **FR-015**: Window dimensions MUST use 1024x768 default size with 2:1 minimum scaling factor for BGI graphics content, maintaining aspect-correct scaling
- **FR-016**: Mouse coordinate functions MUST return positions in BGI logical coordinates matching the graphics coordinate system

### Key Entities

- **Graphics Window**: Visual display container showing rendered BGI graphics with configurable dimensions based on graphics mode
- **Input Event**: User interaction data including keyboard key codes, mouse coordinates, and button states
- **Graphics Buffer**: Pixel data representation that can be displayed in window or exported to PGM format
- **Backend Provider**: Rendering system that handles window creation, input capture, and graphics display (visual backend) or PGM file export (headless backend)
- **Backend Selector**: Runtime mechanism for choosing between visual window display and PGM file output modes

## Implementation Notes

### Font Handling Resource
For any font implementation needs, the `bgi-stroked-fonts` crate is available and provides:
- Complete BGI font data without need for reimplementation
- Direct integration capability for BGI font rendering
- Usage examples available at: https://github.com/sunsided/bgi-stroked-fonts/blob/main/examples/showcase.rs

This crate should be considered for text rendering features to maintain BGI font compatibility and avoid duplicating font data implementation.

## Clarifications

### Session 2025-10-01
- Q: For window resize events, what should happen to the graphics content? → A: Scale graphics content to fit new window size
- Q: How should the system determine when to output PGM files versus displaying in a window? → A: Runtime API call allows switching between modes AND automatic fallback to PGM when display unavailable
- Q: For input handling in the PGM backend (where there's no visual window), what should happen when BGI programs call input functions like getch() or kbhit()? → A: Read from stdin for keyboard simulation
- Q: For BGI graphics modes (like VGA, EGA, etc.), how should the minifb window dimensions be determined? → A: Use 1024x768 default window size, scale BGI content to fit with aspect-correct scaling (minimum 2x scaling factor)
- Q: For mouse input functions like mousex() and mousey(), should the coordinates returned be in BGI logical coordinates or physical window coordinates? → A: BGI logical coordinates (matches graphics coordinate system)

## Review & Acceptance Checklist

### Content Quality

- [x] No implementation details (languages, frameworks, APIs)
- [x] Focused on user value and business needs
- [x] Written for non-technical stakeholders
- [x] All mandatory sections completed

### Requirement Completeness

- [x] No [NEEDS CLARIFICATION] markers remain
- [x] Requirements are testable and unambiguous  
- [x] Success criteria are measurable
- [x] Scope is clearly bounded
- [x] Dependencies and assumptions identified

## Execution Status

- [x] User description parsed
- [x] Key concepts extracted
- [x] Ambiguities marked
- [x] User scenarios defined
- [x] Requirements generated
- [x] Entities identified
- [x] Review checklist passed
