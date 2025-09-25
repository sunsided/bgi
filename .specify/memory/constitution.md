<!--
Sync Impact Report:
Version change: 0.0.0 → 1.0.0
Added sections:
- Complete BGI constitution with 5 core principles
- API Compatibility requirements
- Quality Standards section
- Governance rules
Modified principles:
- NEW: Backend Extensibility
- NEW: BGI API Compatibility (NON-NEGOTIABLE)
- NEW: Modern Rust Standards
- NEW: Zero-Cost Abstractions
- NEW: Test-First Development
Templates requiring updates: ⚠ pending review of dependent templates
-->

# BGI Constitution

## Core Principles

### I. Backend Extensibility

All graphics functionality MUST be implementable through the Backend trait system. No BGI feature may bypass backend abstraction. Backends are selected at compile-time via feature flags (e.g., `winit-backend`, `sdl-backend`). Each backend MUST implement the complete Backend trait contract with pixel-perfect compatibility. This ensures the library remains truly cross-platform and allows custom rendering implementations.

### II. BGI API Compatibility (NON-NEGOTIABLE)

Complete compatibility with the classic Borland Graphics Interface API is mandatory. Every BGI function, constant, and behavior MUST be faithfully reproduced. Function signatures MUST match original BGI exactly - no Result types for error handling, use original BGI error mechanisms (graphresult(), etc.). Return types, parameter types, and calling conventions MUST be identical to maintain drop-in compatibility. Breaking API compatibility for convenience is forbidden - create separate convenience APIs instead.

**Global Graphics State Architecture**: BGI maintains a single, global graphics state (GRAPHICS_STATE) that MUST be initialized via `initgraph()` before any graphics operations. This state includes drawing color, line styles, fill patterns, viewport settings, and pixel buffers. All BGI drawing functions (line, circle, setcolor, etc.) operate on this unified global state - there SHALL be no duplicate or parallel state systems. The graphics system MUST be properly closed with `closegraph()` when no longer required. Functions that require graphics initialization MUST fail appropriately (returning error codes via graphresult()) when called before `initgraph()` - this is fundamental BGI behavior and cannot be bypassed for convenience. Line patterns, color settings, and all drawing state MUST be synchronized through this single source of truth.

### III. Modern Rust Standards

Code MUST follow modern Rust practices: Edition 2021 or later, MSRV 1.80+, zero unsafe code unless absolutely necessary with explicit justification. All public APIs MUST use appropriate Rust types (Results for errors, Options for nullability, iterators over indices). Clippy warnings MUST be addressed. Code MUST be formatted with rustfmt. All dependencies MUST be actively maintained and security-audited.

### IV. Zero-Cost Abstractions

Backend abstraction and BGI compatibility MUST NOT impose runtime overhead. Generic trait methods should monomorphize to direct function calls. Feature flags MUST enable compile-time dead code elimination. No dynamic dispatch unless required for extensibility. Memory allocations MUST be minimized and predictable. Performance MUST match or exceed equivalent C implementations.

### V. Test-First Development

TDD is mandatory for all BGI API implementations: Write comprehensive tests covering BGI behavior → Implement backend methods → Verify tests pass. Each BGI function MUST have unit tests verifying correct parameters, error conditions, and visual output (where applicable). Integration tests MUST verify cross-backend compatibility. Example programs serve as acceptance tests and MUST run without modification on all backends.

## API Compatibility Requirements

All BGI functions, constants, and data structures MUST be implemented with identical semantics to the original Turbo C++ BGI library. Graphics output MUST be pixel-perfect when using identical parameters. Color palettes MUST match exactly (16-color BGI + ARGB extensions). Coordinate systems, clipping behavior, and fill patterns MUST be identical. Error conditions MUST use original BGI error reporting mechanisms (graphresult() function) rather than Rust Result types to maintain API compatibility.

## Architecture Constraints

**Single State System**: There SHALL be only one graphics state system. Creating parallel or duplicate state management systems (e.g., separate GLOBAL_CONTEXT alongside GRAPHICS_STATE) is explicitly forbidden as it leads to synchronization bugs and broken functionality like line patterns not working properly. All graphics functions MUST operate through the unified GRAPHICS_STATE.

**Proper Initialization Contract**: Tests that expect BGI functions to work without calling `initgraph()` first are INCORRECT and violate BGI principles. Such tests MUST be fixed to properly initialize graphics before use, not worked around with fallback mechanisms.

## Quality Standards

Code coverage MUST exceed 80% for all BGI API implementations. Documentation MUST include runnable examples for every public function. All examples in documentation MUST be tested in CI. Performance regression testing MUST verify graphics operations complete within acceptable timeframes. Memory usage MUST be bounded and predictable. All commits MUST pass Clippy lints and rustfmt checks.

## Governance

This constitution supersedes all other development practices. All code reviews MUST verify constitutional compliance. Any complexity or architectural deviation MUST be explicitly justified with technical rationale. Breaking changes require major version bumps and migration documentation. Backend implementations require comprehensive testing across multiple platforms before acceptance.

**Version**: 1.0.0 | **Ratified**: 2024-12-19 | **Last Amended**: 2024-12-19
