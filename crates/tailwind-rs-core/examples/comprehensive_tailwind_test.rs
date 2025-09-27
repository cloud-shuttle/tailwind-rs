//! Comprehensive Tailwind CSS Coverage Test
//!
//! This test covers the major categories of Tailwind CSS utilities to assess
//! our coverage against the full Tailwind CSS feature set.
//! The implementation has been modularized into separate files for better maintainability.

// Import the modularized components
mod comprehensive_tailwind_test_module;
use comprehensive_tailwind_test_module::{run_comprehensive_tests, print_test_results};

fn main() {
    let results = run_comprehensive_tests();
    print_test_results(&results);
}
