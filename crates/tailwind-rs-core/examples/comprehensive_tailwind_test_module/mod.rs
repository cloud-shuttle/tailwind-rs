//! Comprehensive Tailwind CSS Coverage Test
//!
//! This module provides comprehensive testing of Tailwind CSS utilities
//! to assess coverage against the full Tailwind CSS feature set.
//! The implementation has been modularized into separate files for better maintainability.

pub mod test_categories;
pub mod advanced_categories;
pub mod special_categories;
pub mod test_runner;

// Re-export the main functionality
pub use test_runner::{run_comprehensive_tests, print_test_results, TestResults, CategoryResult};
