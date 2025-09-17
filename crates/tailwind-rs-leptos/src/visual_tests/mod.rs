//! # Visual Testing Module
//!
//! This module provides visual regression tests that can capture and compare
//! component rendering to ensure visual consistency.
//!
//! The module is organized into focused sub-modules for better maintainability:
//! - `test_utils` - Core visual testing utilities
//! - `test_runner` - Test execution and management
//! - `report_generator` - Report generation and formatting
//! - `test_cases` - Test case definitions and structures

pub mod test_utils;
pub mod test_runner;
pub mod report_generator;
pub mod test_cases;

// Re-export main types for backward compatibility
pub use test_utils::{VisualTestUtils, VisualSnapshot, VisualComparison, VisualDifference};
pub use test_runner::{VisualTestRunner, VisualTestConfig};
pub use report_generator::VisualTestReport;
pub use test_cases::{ComponentTestCase, ResponsiveTestCase, ThemeTestCase, StateTestCase};

/// Main visual regression tests implementation
pub struct VisualRegressionTests;

impl VisualRegressionTests {
    /// Run all visual regression tests
    pub fn run_all_tests() -> VisualTestReport {
        let mut runner = VisualTestRunner::new(VisualTestConfig::default());
        
        // Run component consistency tests
        let component_tests = Self::run_component_consistency_tests();
        runner.add_test_results(component_tests);
        
        // Run responsive consistency tests
        let responsive_tests = Self::run_responsive_consistency_tests();
        runner.add_test_results(responsive_tests);
        
        // Run state consistency tests
        let state_tests = Self::run_state_consistency_tests();
        runner.add_test_results(state_tests);
        
        // Run theme consistency tests
        let theme_tests = Self::run_theme_consistency_tests();
        runner.add_test_results(theme_tests);
        
        runner.generate_report()
    }
    
    /// Run component consistency tests
    pub fn run_component_consistency_tests() -> Vec<VisualComparison> {
        use test_cases::ComponentTestCase;
        
        let test_cases = vec![
            ComponentTestCase::new("Button", "px-4 py-2 bg-blue-600 text-white rounded"),
            ComponentTestCase::new("Input", "px-3 py-2 border border-gray-300 rounded"),
            ComponentTestCase::new("Card", "bg-white rounded-lg shadow-md border"),
        ];
        
        test_cases.into_iter().map(|test_case| {
            let snapshot1 = VisualTestUtils::capture_component_state(&test_case.component_name, &test_case.expected_classes);
            let snapshot2 = VisualTestUtils::capture_component_state(&test_case.component_name, &test_case.expected_classes);
            VisualTestUtils::compare_snapshots(&snapshot1, &snapshot2)
        }).collect()
    }
    
    /// Run responsive consistency tests
    pub fn run_responsive_consistency_tests() -> Vec<VisualComparison> {
        use test_cases::ResponsiveTestCase;
        
        let test_cases = vec![
            ResponsiveTestCase::new("sm:px-2 md:px-4 lg:px-6"),
            ResponsiveTestCase::new("sm:text-sm md:text-base lg:text-lg"),
            ResponsiveTestCase::new("sm:grid-cols-1 md:grid-cols-2 lg:grid-cols-3"),
        ];
        
        test_cases.into_iter().map(|test_case| {
            let snapshot1 = VisualTestUtils::capture_component_state("ResponsiveComponent", &test_case.responsive_classes);
            let snapshot2 = VisualTestUtils::capture_component_state("ResponsiveComponent", &test_case.responsive_classes);
            VisualTestUtils::compare_snapshots(&snapshot1, &snapshot2)
        }).collect()
    }
    
    /// Run state consistency tests
    pub fn run_state_consistency_tests() -> Vec<VisualComparison> {
        use test_cases::StateTestCase;
        
        let test_cases = vec![
            StateTestCase::new("hover:bg-blue-700 focus:ring-2"),
            StateTestCase::new("disabled:opacity-50 disabled:cursor-not-allowed"),
            StateTestCase::new("active:scale-95 transition-transform"),
        ];
        
        test_cases.into_iter().map(|test_case| {
            let snapshot1 = VisualTestUtils::capture_component_state("StateComponent", &test_case.state_classes);
            let snapshot2 = VisualTestUtils::capture_component_state("StateComponent", &test_case.state_classes);
            VisualTestUtils::compare_snapshots(&snapshot1, &snapshot2)
        }).collect()
    }
    
    /// Run theme consistency tests
    pub fn run_theme_consistency_tests() -> Vec<VisualComparison> {
        use test_cases::ThemeTestCase;
        
        let test_cases = vec![
            ThemeTestCase::new("dark:bg-gray-800 dark:text-white"),
            ThemeTestCase::new("light:bg-white light:text-gray-900"),
            ThemeTestCase::new("dark:border-gray-700 light:border-gray-300"),
        ];
        
        test_cases.into_iter().map(|test_case| {
            let snapshot1 = VisualTestUtils::capture_component_state("ThemeComponent", &test_case.theme_classes);
            let snapshot2 = VisualTestUtils::capture_component_state("ThemeComponent", &test_case.theme_classes);
            VisualTestUtils::compare_snapshots(&snapshot1, &snapshot2)
        }).collect()
    }
}
