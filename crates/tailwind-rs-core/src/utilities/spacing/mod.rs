//! Spacing utilities for Tailwind CSS
//!
//! This module provides comprehensive spacing utilities including padding,
//! margin, gap, space-between, and divide utilities. The implementation
//! has been modularized into separate files for better maintainability.

pub mod spacing_values;
pub mod spacing_utilities;
pub mod spacing_tests;

// Re-export the main types and traits
pub use spacing_values::SpacingValue;
pub use spacing_utilities::*;
