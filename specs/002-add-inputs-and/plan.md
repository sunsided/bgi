
# Implementation Plan: Visual Backend with Input Support

**Branch**: `002-add-inputs-and` | **Date**: October 1, 2025 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/002-add-inputs-and/spec.md`

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
Add visual window backend using minifb and interactive input support to BGI library for visual validation of graphics examples. Includes PGM file output fallback for headless debugging scenarios.

## Technical Context
**Language/Version**: Rust 1.90+ (Edition 2024)  
**Primary Dependencies**: minifb (visual backend), bgi-stroked-fonts (font rendering)  
**Storage**: PGM file output for debugging/testing  
**Testing**: cargo test with visual validation tests  
**Target Platform**: Cross-platform (Linux, Windows, macOS)
**Project Type**: Rust library with backend trait system  
**Performance Goals**: 30 fps minimum graphics rendering, <1ms input latency  
**Constraints**: BGI API compatibility, zero runtime overhead, non-blocking input  
**Scale/Scope**: 16 functional requirements, dual backend system (visual + headless)

## Constitution Check
*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**Backend Extensibility**: ✅ PASS - Feature implements dual backend system (minifb visual + PGM headless) via Backend trait. No BGI function bypasses backend abstraction.

**BGI API Compatibility**: ✅ PASS - All existing BGI functions (getch, kbhit, mousex, mousey) maintain identical signatures. Input coordinates returned in BGI logical space, not physical pixels. No Result types introduced for error handling.

**Modern Rust Standards**: ✅ PASS - Uses Edition 2024, MSRV 1.90+, proper error handling patterns. All public APIs follow Rust conventions while preserving BGI compatibility at function level.

**Zero-Cost Abstractions**: ✅ PASS - Backend selection via compile-time feature flags enables dead code elimination. Input event handling designed as non-blocking polling to avoid runtime overhead.

**Test-First Development**: ✅ PASS - Plan requires comprehensive test suite covering visual validation, input simulation, and cross-backend compatibility before implementation.

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
├── backend/           # Backend trait and implementations
│   ├── mod.rs
│   ├── minifb.rs     # Visual window backend
│   └── pgm.rs        # Headless PGM file backend
├── input/            # Input handling system
│   ├── mod.rs
│   ├── keyboard.rs   # Keyboard event processing
│   └── mouse.rs      # Mouse event processing
├── graphics/         # Core graphics state management
│   └── mod.rs        # Graphics buffer and coordinate system
└── lib.rs            # Public API and backend selection

tests/
├── # End-to-end BGI compatibility tests
├── # Visual validation tests
└── # Component unit tests

examples/            # Demo programs for validation
├── interactive/     # Programs using input features
└── visual/          # Programs demonstrating graphics output
```

**Structure Decision**: Single Rust library project with modular backend system. The existing BGI library codebase will be extended with new backend and input modules while preserving the current API surface and global graphics state architecture.

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
- Generate tasks from Phase 1 design docs (contracts, data model, quickstart)
- Backend trait implementation tasks (minifb backend, PGM backend) [P]
- Input system tasks (keyboard events, mouse events, event queue) [P]
- Graphics integration tasks (coordinate transformation, buffer management)
- Test tasks (unit tests, integration tests, visual validation tests)
- Example programs (interactive demos, validation scenarios)

**Ordering Strategy**:
- TDD order: Tests before implementation
- Dependency order: Core traits → Backend implementations → Input system → Integration
- Parallel opportunities: Backend implementations can be developed independently [P]
- Testing integration: Each implementation task paired with corresponding test task

**Estimated Output**: 35-40 numbered, ordered tasks covering:
- Core backend trait definition (3 tasks)
- minifb visual backend implementation (8-10 tasks)
- PGM headless backend implementation (6-8 tasks)
- Input event system (8-10 tasks)
- Coordinate system and graphics integration (6-8 tasks)
- Comprehensive testing suite (8-10 tasks)
- Example programs and documentation (4-6 tasks)

**Task Categories**:
1. **Foundation [P]**: Backend trait, core types, error handling
2. **Visual Backend**: minifb integration, window management, input capture
3. **Headless Backend [P]**: PGM output, stdin input simulation
4. **Input System**: Event queuing, coordinate transformation, BGI API integration
5. **Testing**: Unit tests, integration tests, visual validation
6. **Examples**: Interactive demos, quickstart validation, performance tests

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
- [x] Phase 0: Research complete (/plan command)
- [x] Phase 1: Design complete (/plan command)
- [x] Phase 2: Task planning complete (/plan command - describe approach only)
- [x] Phase 3: Tasks generated (/tasks command)
- [ ] Phase 4: Implementation complete
- [ ] Phase 5: Validation passed

**Gate Status**:
- [x] Initial Constitution Check: PASS
- [x] Post-Design Constitution Check: PASS
- [x] All NEEDS CLARIFICATION resolved
- [x] Complexity deviations documented

---
*Based on Constitution v1.0.0 - See `/memory/constitution.md`*
