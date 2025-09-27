//! Enhanced Validation Module
//!
//! This module provides enhanced validation utilities for tailwind-rs,
//! including support for class validation, property validation, and validation rules.
//! The implementation has been modularized into separate files for better maintainability.

pub mod validation_types;
pub mod validation_traits;
#[cfg(test)]
pub mod validation_tests;

// Re-export the public API
pub use validation_types::*;
pub use validation_traits::*;
