# bgi Development Guidelines

Auto-generated from all feature plans. Last updated: 2025-10-01

## Active Technologies
- Rust 1.90+ (Edition 2024) + minifb (visual backend), bgi-stroked-fonts (font rendering)
- PGM file output for debugging/testing

## Project Structure
```
src/
tests/
```

## Commands
```bash
cargo test                    # Run all tests
cargo test --features visual-backend  # Run tests with visual backend
cargo clippy                  # Run linter
cargo fmt                     # Format code
cargo build --features visual-backend # Build with visual backend
```

## Code Style
Rust 1.90+ (Edition 2024): Follow standard conventions

## Recent Changes
- 002-add-inputs-and: Added Rust 1.90+ (Edition 2024) + minifb (visual backend), bgi-stroked-fonts (font rendering)

<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->
