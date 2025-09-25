
# Implementation Plan: Complete BGI Graphics.h API Implementation

**Branch**: `001-implement-the-bgi` | **Date**: 2025-09-26 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-implement-the-bgi/spec.md`

## Execution Flow (/plan command scope)
```
1. Load feature spec from Input path
   → If not found: ERROR "No feature spec at {path}"
2. Fill Technical Context (scan for NEEDS CLARIFICATION)
   → Detect Project Type from file system structure or context (web=frontend+backend, mobile=app+api)
   → Set Structure Decision based on project type
3. Fill the Constitution Check section based on the content of the constitution document.
4. Evaluate Constitution Check section below
   → If violations exist: Document in Complexity Tracking
   → If no justification possible: ERROR "Simplify approach first"
   → Update Progress Tracking: Initial Constitution Check
5. Execute Phase 0 → research.md
   → If NEEDS CLARIFICATION remain: ERROR "Resolve unknowns"
6. Execute Phase 1 → contracts, data-model.md, quickstart.md, agent-specific template file (e.g., `CLAUDE.md` for Claude Code, `.github/copilot-instructions.md` for GitHub Copilot, `GEMINI.md` for Gemini CLI, `QWEN.md` for Qwen Code or `AGENTS.md` for opencode).
7. Re-evaluate Constitution Check section
   → If new violations: Refactor design, return to Phase 1
   → Update Progress Tracking: Post-Design Constitution Check
8. Plan Phase 2 → Describe task generation approach (DO NOT create tasks.md)
9. STOP - Ready for /tasks command
```

**IMPORTANT**: The /plan command STOPS at step 7. Phases 2-4 are executed by other commands:
- Phase 2: /tasks command creates tasks.md
- Phase 3-4: Implementation execution (manual or via tools)

## Summary
Complete implementation of the classic Borland Graphics Interface (BGI) API in modern Rust with extensible backend architecture. Primary requirement: pixel-perfect compatibility with original BGI functions while enabling multiple rendering backends through compile-time feature selection. Technical approach: Backend trait abstraction with winit-based initial implementation, software emulation for complete feature parity, and hybrid error handling maintaining BGI semantics.

## Technical Context
**Language/Version**: Rust 1.80+ (Edition 2021)  
**Primary Dependencies**: winit 0.29, pixels 0.13, thiserror 1.0, bitflags 2.4  
**Storage**: In-memory graphics state and pixel buffers (no persistent storage)  
**Testing**: cargo test with unit tests, integration tests, and visual regression testing  
**Target Platform**: Cross-platform (Windows, Linux, macOS) with additional backends possible
**Project Type**: Single library crate with optional backend features  
**Performance Goals**: 30 FPS for typical graphics operations matching retro hardware capabilities  
**Constraints**: Zero-cost abstractions, pixel-perfect compatibility, BGI-only coordinate system  
**Scale/Scope**: Complete graphics.h API surface (~100 functions), multiple graphics modes, extensible to additional backends

## Constitution Check
*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**Backend Extensibility**: Feature MUST be implementable via Backend trait - no direct platform dependencies.
**BGI API Compatibility**: Changes MUST NOT break classic BGI function signatures or semantics.
**Modern Rust Standards**: Code MUST use Edition 2021+, MSRV 1.80+, proper error handling with Results.
**Zero-Cost Abstractions**: Design MUST NOT impose runtime overhead through abstraction layers.
**Test-First Development**: Comprehensive tests MUST be written before implementation begins.

## Project Structure

### Documentation (this feature)
```
specs/[###-feature]/
├── plan.md              # This file (/plan command output)
├── research.md          # Phase 0 output (/plan command)
├── data-model.md        # Phase 1 output (/plan command)
├── quickstart.md        # Phase 1 output (/plan command)
├── contracts/           # Phase 1 output (/plan command)
└── tasks.md             # Phase 2 output (/tasks command - NOT created by /plan)
```

### Source Code (repository root)
```
src/
├── lib.rs                 # Main library entry point with public API exports
├── error.rs              # Error types and BGI error system
├── types.rs              # BGI data types and enums  
├── color.rs              # Color system and palette management
├── constants.rs          # BGI constants and definitions
├── context.rs            # Graphics context management
├── graphics.rs           # Graphics initialization (initgraph, closegraph, etc.)
├── drawing.rs            # Drawing primitives (line, circle, rectangle, etc.)
├── shapes.rs             # Filled shapes (bar, bar3d, ellipse, polygon, etc.)
├── fill.rs               # Fill patterns and flood fill
├── text.rs               # Text rendering and font support
├── input.rs              # Input handling (keyboard/mouse)
├── viewport.rs           # Viewport and coordinate management
├── image.rs              # Image operations (getimage, putimage, etc.)
├── palette.rs            # Palette management functions
├── window.rs             # Multi-window management
├── performance.rs        # Performance control (fast/slow modes)
└── backend/
    ├── mod.rs            # Backend trait definition
    └── winit.rs          # Winit backend implementation

tests/
├── integration/          # Cross-backend integration tests
│   ├── drawing_tests.rs
│   ├── input_tests.rs
│   └── compatibility_tests.rs
└── unit/                 # Unit tests per module
    ├── color_tests.rs
    ├── drawing_tests.rs
    └── backend_tests.rs

examples/
├── simple.rs             # Basic BGI usage example
├── mandelbrot.rs         # Complex graphics demonstration
└── compatibility.rs      # BGI compatibility showcase
```

**Structure Decision**: Single library project structure selected. BGI is a graphics library, not a web or mobile application. The modular source structure separates BGI API concerns (drawing, colors, input) from backend abstraction, with comprehensive test coverage and practical examples demonstrating API usage.

## Phase 0: Outline & Research
1. **Extract unknowns from Technical Context** above:
   - For each NEEDS CLARIFICATION → research task
   - For each dependency → best practices task
   - For each integration → patterns task

2. **Generate and dispatch research agents**:
   ```
   For each unknown in Technical Context:
     Task: "Research {unknown} for {feature context}"
   For each technology choice:
     Task: "Find best practices for {tech} in {domain}"
   ```

3. **Consolidate findings** in `research.md` using format:
   - Decision: [what was chosen]
   - Rationale: [why chosen]
   - Alternatives considered: [what else evaluated]

**Output**: research.md with all NEEDS CLARIFICATION resolved

## Phase 1: Design & Contracts
*Prerequisites: research.md complete*

1. **Extract entities from feature spec** → `data-model.md`:
   - Entity name, fields, relationships
   - Validation rules from requirements
   - State transitions if applicable

2. **Generate API contracts** from functional requirements:
   - For each user action → endpoint
   - Use standard REST/GraphQL patterns
   - Output OpenAPI/GraphQL schema to `/contracts/`

3. **Generate contract tests** from contracts:
   - One test file per endpoint
   - Assert request/response schemas
   - Tests must fail (no implementation yet)

4. **Extract test scenarios** from user stories:
   - Each story → integration test scenario
   - Quickstart test = story validation steps

5. **Update agent file incrementally** (O(1) operation):
   - Run `.specify/scripts/bash/update-agent-context.sh copilot`
     **IMPORTANT**: Execute it exactly as specified above. Do not add or remove any arguments.
   - If exists: Add only NEW tech from current plan
   - Preserve manual additions between markers
   - Update recent changes (keep last 3)
   - Keep under 150 lines for token efficiency
   - Output to repository root

**Output**: data-model.md, /contracts/*, failing tests, quickstart.md, agent-specific file

## Phase 2: Task Planning Approach
*This section describes what the /tasks command will do - DO NOT execute during /plan*

**Task Generation Strategy**:
- Load `.specify/templates/tasks-template.md` as base
- Generate tasks from Phase 1 design docs (contracts in `/contracts/`, data-model.md, quickstart.md)
- Each BGI API contract group → contract test task [P]
  - Graphics init contract → test_graphics_init.rs [P]
  - Drawing primitives contract → test_drawing_primitives.rs [P]
  - Fill and color contract → test_fill_color.rs [P]
  - Text rendering contract → test_text_rendering.rs [P]
  - Input interaction contract → test_input_interaction.rs [P]
  - Viewport coordinate contract → test_viewport_coordinate.rs [P]
- Each entity from data-model.md → model creation task [P]
  - GraphicsContext entity → src/context.rs [P]
  - Backend trait → src/backend/mod.rs [P]
  - Color system → src/color.rs [P]
  - Error types → src/error.rs [P]
- Quickstart examples → integration test tasks
  - Basic drawing example → integration test
  - Interactive graphics → integration test
  - Text and font example → integration test
- Implementation tasks to make contract tests pass (ordered by dependency)

**Ordering Strategy**:
- TDD order: Contract tests before implementation
- Dependency order: Core types → Backend trait → Context → API implementations
- Mark [P] for parallel execution (independent modules)
- BGI initialization must come first (other APIs depend on GraphicsContext)

**Estimated Output**: 35-40 numbered, ordered tasks covering:
- 6 contract test tasks (parallel)
- 9 core entity implementation tasks
- 19 BGI API function groups implementation
- 6 quickstart example validation tasks
- Integration and performance validation tasks

**IMPORTANT**: This phase is executed by the /tasks command, NOT by /plan

## Phase 3+: Future Implementation
*These phases are beyond the scope of the /plan command*

**Phase 3**: Task execution (/tasks command creates tasks.md)  
**Phase 4**: Implementation (execute tasks.md following constitutional principles)  
**Phase 5**: Validation (run tests, execute quickstart.md, performance validation)

## Complexity Tracking
*Fill ONLY if Constitution Check has violations that must be justified*

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |


## Progress Tracking
*This checklist is updated during execution flow*

**Phase Status**:

- [x] Phase 0: Research complete (/plan command) - See research.md
- [x] Phase 1: Design complete (/plan command) - See data-model.md, contracts/, quickstart.md, contract_tests.md
- [x] Phase 2: Task planning complete (/plan command - describe approach only) - Strategy documented above
- [x] Phase 3: Tasks generated (/tasks command) - See tasks.md with 51 numbered tasks
- [ ] Phase 4: Implementation complete
- [ ] Phase 5: Validation passed

**Gate Status**:

- [x] Initial Constitution Check: PASS - All principles respected in design
- [x] Post-Design Constitution Check: PASS - Backend trait enables extensibility, BGI compatibility maintained
- [x] All NEEDS CLARIFICATION resolved - Technical Context fully specified
- [x] Complexity deviations documented - No violations requiring justification

---
*Based on Constitution v1.0.0 - See `/memory/constitution.md`*
