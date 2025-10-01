//! Integration tests for backend functionality
//! This module contains all backend-related tests including trait contracts,
//! visual backend tests, and headless backend tests.

#[cfg(feature = "visual-backend")]
#[path = "backend/test_minifb_backend.rs"]
mod test_minifb_backend;

// Always include pixel buffer tests (headless)
#[path = "backend/test_pgm_backend.rs"]
mod test_pixel_buffer_backend;
