# Tasks: Visual Backend with Input Support

**Input**: Design documents from `/specs/002-add-inputs-and/`
**Prerequisites**: plan.md (required), research.md, data-model.md, contracts/

## Execution Flow (main)
```
1. Load plan.md from feature directory
   → Extract: Rust 1.90+ Edition 2024, minifb, bgi-stroked-fonts
   → Structure: src/backend/, src/input/, src/graphics/
2. Load design documents:
   → data-model.md: Backend, GraphicsBuffer, InputEvent, InputEventQueue, WindowState, BackendSelector
   → contracts/backend-api.md: Backend trait contract
   → quickstart.md: Interactive examples and validation scenarios
3. Generate tasks by category:
   → Setup: project structure, dependencies, feature flags
   → Tests: backend contract tests, input tests, integration tests
   → Core: backend trait, minifb backend, PGM backend, input system
   → Integration: coordinate systems, graphics buffer, event handling
   → Polish: examples, performance tests, documentation
4. Apply task rules:
   → Different modules = mark [P] for parallel
   → Same module = sequential (no [P])
   → Tests before implementation (TDD)
5. Tasks numbered T001-T038
6. Backend implementations can run in parallel
7. Input system independent of specific backends
```

## Format: `[ID] [P?] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- Include exact file paths in descriptions

## Path Conventions
- **Backend modules**: `src/backend/mod.rs`, `src/backend/minifb.rs`, `src/backend/pgm.rs`
- **Input system**: `src/input/mod.rs`, `src/input/keyboard.rs`, `src/input/mouse.rs`
- **Graphics core**: `src/graphics/mod.rs`
- **Tests**: `tests/backend/`, `tests/input/`, `tests/integration/`
- **Examples**: `examples/visual/`, `examples/interactive/`

## Phase 3.1: Setup
- [x] T001 Create backend module structure in src/backend/mod.rs
- [x] T002 Create input module structure in src/input/mod.rs  
- [x] T003 [P] Add minifb dependency with feature flag "visual-backend" in Cargo.toml
- [x] T004 [P] Add bgi-stroked-fonts dependency in Cargo.toml
- [x] T005 [P] Configure feature flags for backend selection in Cargo.toml

## Phase 3.2: Tests First (TDD) ⚠️ MUST COMPLETE BEFORE 3.3
**CRITICAL: These tests MUST be written and MUST FAIL before ANY implementation**
- [x] T006 [P] Backend trait contract test in tests/backend/test_backend_trait.rs
- [x] T007 [P] Visual backend contract test in tests/backend/test_minifb_backend.rs
- [x] T008 [P] Headless backend contract test in tests/backend/test_pgm_backend.rs
- [x] T009 [P] Keyboard input integration test in tests/input/test_keyboard.rs
- [x] T010 [P] Mouse input integration test in tests/input/test_mouse.rs
- [x] T011 [P] Event queue management test in tests/input/test_event_queue.rs
- [x] T011a [P] Font loading and rendering test in tests/graphics/test_fonts.rs
- [x] T011b [P] Graphics state management test in tests/graphics/test_state.rs
- [x] T012 [P] Coordinate transformation test in tests/integration/test_coordinates.rs
- [x] T013 [P] Backend switching test in tests/integration/test_backend_selection.rs
- [x] T014 [P] Visual validation test in tests/integration/test_visual_output.rs

## Phase 3.3: Core Implementation (ONLY after tests are failing)
- [ ] T015 [P] Backend trait definition in src/backend/mod.rs
- [ ] T016 [P] GraphicsMode and Color types in src/backend/mod.rs  
- [ ] T017 [P] KeyEvent and MouseEvent types in src/input/mod.rs
- [ ] T018 [P] InputEventQueue implementation in src/input/mod.rs
- [ ] T019 [P] Graphics buffer management in src/graphics/mod.rs
- [ ] T019a [P] Font data integration module in src/graphics/fonts.rs  
- [ ] T019b [P] Font rendering interface using bgi-stroked-fonts in src/graphics/fonts.rs
- [ ] T020 MiniFB visual backend implementation in src/backend/minifb.rs
- [ ] T021 Window management and scaling in src/backend/minifb.rs
- [ ] T022 MiniFB input event capture in src/backend/minifb.rs
- [ ] T023 [P] PGM headless backend implementation in src/backend/pgm.rs
- [ ] T024 [P] PGM file output generation in src/backend/pgm.rs
- [ ] T025 [P] PGM stdin input simulation in src/backend/pgm.rs
- [ ] T026 Keyboard event processing in src/input/keyboard.rs
- [ ] T027 Mouse event processing in src/input/mouse.rs
- [ ] T028 Coordinate system transformation in src/graphics/mod.rs

## Phase 3.4: Integration
- [ ] T029 Backend selector and runtime switching in src/lib.rs
- [ ] T030 BGI API integration (initgraph, closegraph) in src/lib.rs
- [ ] T030a Global graphics state initialization (GRAPHICS_STATE) in src/lib.rs
- [ ] T030b Graphics state management (setcolor, setlinestyle, etc.) in src/lib.rs  
- [ ] T031 BGI input functions (getch, kbhit, mousex, mousey) in src/lib.rs
- [ ] T032 Error handling via graphresult() mechanism in src/lib.rs
- [ ] T033 Feature flag compilation and dead code elimination in src/lib.rs

## Phase 3.5: Polish
- [ ] T034 [P] Visual hello world example in examples/visual/hello.rs
- [ ] T035 [P] Interactive input example in examples/interactive/keyboard_test.rs
- [ ] T036 [P] Mouse tracking example in examples/interactive/mouse_test.rs
- [ ] T037 [P] Performance benchmarking (minimum 30 FPS, <16ms flush time) in tests/performance/
- [ ] T038 [P] Update documentation and quickstart validation

## Dependencies
- Setup (T001-T005) before all other phases
- Tests (T006-T014) before implementation (T015-T028)
- T015-T016 (core types) before T017-T019 (component types)
- T015 (Backend trait) blocks T020, T023 (implementations)
- T017-T018 (input types) before T026-T027 (input processing)
- T019 (graphics buffer) before T028 (coordinate transformation)
- Implementation (T015-T028) before integration (T029-T033)
- Integration before polish (T034-T038)

## Parallel Example: Backend Implementations
```
# After T015 (Backend trait) is complete, launch T020 and T023 together:
Task: "MiniFB visual backend implementation in src/backend/minifb.rs"
Task: "PGM headless backend implementation in src/backend/pgm.rs"

# While those run, also launch input system tasks:
Task: "Keyboard event processing in src/input/keyboard.rs"  
Task: "Mouse event processing in src/input/mouse.rs"
```

## Parallel Example: Test Suite
```
# Launch T006-T014 together (all different files):
Task: "Backend trait contract test in tests/backend/test_backend_trait.rs"
Task: "Visual backend contract test in tests/backend/test_minifb_backend.rs"
Task: "Headless backend contract test in tests/backend/test_pgm_backend.rs"
Task: "Keyboard input integration test in tests/input/test_keyboard.rs"
Task: "Mouse input integration test in tests/input/test_mouse.rs"
Task: "Event queue management test in tests/input/test_event_queue.rs"
Task: "Coordinate transformation test in tests/integration/test_coordinates.rs"
Task: "Backend switching test in tests/integration/test_backend_selection.rs"
Task: "Visual validation test in tests/integration/test_visual_output.rs"
```

## Notes
- [P] tasks = different files/modules, no dependencies
- MiniFB and PGM backends can be developed in parallel after backend trait exists
- Input system (keyboard.rs, mouse.rs) independent and parallel
- Verify tests fail before implementing (TDD compliance)
- Each backend must pass the same contract tests
- Coordinate transformation critical for BGI compatibility
- Performance target: minimum 30 FPS, <16ms flush time, <1ms input latency

## Task Generation Rules
*Applied during main() execution*

1. **From Contracts**:
   - backend-api.md → Backend trait test (T006) and implementation (T015)
   - Each backend type → contract test [P] and implementation
   
2. **From Data Model**:
   - Backend entity → trait definition task (T015)
   - GraphicsBuffer → buffer management task (T019)
   - InputEvent → event type definitions (T017)
   - InputEventQueue → queue implementation (T018)
   - WindowState → window management (T021)
   - BackendSelector → runtime switching (T029)
   
3. **From Quickstart Scenarios**:
   - Visual hello example → T034
   - Keyboard test → T035  
   - Mouse test → T036
   - Performance validation → T037

4. **Ordering**:
   - Setup → Tests → Core Types → Implementations → Integration → Examples
   - Backend trait before backend implementations
   - Input types before input processing
   - Core before integration

## Validation Checklist
*GATE: Checked before task execution*

- [x] All backend contracts have corresponding tests (T006-T008)
- [x] All entities have implementation tasks (Backend→T015, GraphicsBuffer→T019, etc.)
- [x] All tests come before implementation (T006-T014 before T015-T028)
- [x] Parallel tasks truly independent ([P] tasks use different files)
- [x] Each task specifies exact file path
- [x] No task modifies same file as another [P] task
- [x] TDD ordering preserved (tests must fail before implementation)
- [x] Constitutional compliance maintained (BGI compatibility, zero-cost abstractions)
