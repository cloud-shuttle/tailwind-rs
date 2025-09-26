//! # End-to-End Tests Module
//!
//! This module provides comprehensive workflow tests that test the complete
//! integration between tailwind-rs and Leptos components.
//!
//! The module is organized into focused sub-modules for better maintainability:
//! - `workflow_tests` - Complete workflow component tests
//! - `signal_integration` - Signal-based component tests
//! - `responsive_integration` - Responsive component tests
//! - `theme_integration` - Theme component tests
//! - `state_management` - Complex state component tests

pub mod responsive_integration;
pub mod signal_integration;
pub mod state_management;
pub mod theme_integration;
pub mod workflow_tests;

// Re-export main components for backward compatibility
pub use responsive_integration::ResponsiveComponent;
pub use signal_integration::SignalBasedComponent;
pub use state_management::ComplexStateComponent;
pub use theme_integration::ThemeComponent;
pub use workflow_tests::TestWorkflowComponent;

/// Run all E2E tests
pub fn run_all_e2e_tests() -> E2ETestResults {
    let mut results = E2ETestResults::new();

    // Run workflow tests
    let workflow_results = workflow_tests::run_workflow_tests();
    results.add_results("workflow", workflow_results);

    // Run signal integration tests
    let signal_results = signal_integration::run_signal_tests();
    results.add_results("signal_integration", signal_results);

    // Run responsive integration tests
    let responsive_results = responsive_integration::run_responsive_tests();
    results.add_results("responsive_integration", responsive_results);

    // Run theme integration tests
    let theme_results = theme_integration::run_theme_tests();
    results.add_results("theme_integration", theme_results);

    // Run state management tests
    let state_results = state_management::run_state_tests();
    results.add_results("state_management", state_results);

    results
}

/// Results from E2E test execution
#[derive(Debug, Clone)]
pub struct E2ETestResults {
    pub test_suites: std::collections::HashMap<String, E2ETestSuite>,
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
}

impl E2ETestResults {
    /// Create new E2E test results
    pub fn new() -> Self {
        Self {
            test_suites: std::collections::HashMap::new(),
            total_tests: 0,
            passed_tests: 0,
            failed_tests: 0,
        }
    }

    /// Add test results for a specific suite
    pub fn add_results(&mut self, suite_name: &str, results: E2ETestSuite) {
        self.total_tests += results.total_tests;
        self.passed_tests += results.passed_tests;
        self.failed_tests += results.failed_tests;
        self.test_suites.insert(suite_name.to_string(), results);
    }

    /// Get success rate as percentage
    pub fn success_rate(&self) -> f64 {
        if self.total_tests == 0 {
            100.0
        } else {
            (self.passed_tests as f64 / self.total_tests as f64) * 100.0
        }
    }

    /// Check if all tests passed
    pub fn all_tests_passed(&self) -> bool {
        self.failed_tests == 0
    }
}

/// Results from a specific test suite
#[derive(Debug, Clone)]
pub struct E2ETestSuite {
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub test_details: Vec<E2ETestDetail>,
}

impl E2ETestSuite {
    /// Create new test suite results
    pub fn new() -> Self {
        Self {
            total_tests: 0,
            passed_tests: 0,
            failed_tests: 0,
            test_details: Vec::new(),
        }
    }

    /// Add a test result
    pub fn add_test(&mut self, test_name: String, passed: bool, details: Option<String>) {
        self.total_tests += 1;
        if passed {
            self.passed_tests += 1;
        } else {
            self.failed_tests += 1;
        }

        self.test_details.push(E2ETestDetail {
            test_name,
            passed,
            details,
        });
    }
}

/// Details of an individual test
#[derive(Debug, Clone)]
pub struct E2ETestDetail {
    pub test_name: String,
    pub passed: bool,
    pub details: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_e2e_test_results_creation() {
        let results = E2ETestResults::new();

        assert_eq!(results.total_tests, 0);
        assert_eq!(results.passed_tests, 0);
        assert_eq!(results.failed_tests, 0);
        assert_eq!(results.success_rate(), 100.0);
        assert!(results.all_tests_passed());
    }

    #[test]
    fn test_e2e_test_suite_creation() {
        let mut suite = E2ETestSuite::new();

        suite.add_test("test_1".to_string(), true, None);
        suite.add_test(
            "test_2".to_string(),
            false,
            Some("Failed assertion".to_string()),
        );

        assert_eq!(suite.total_tests, 2);
        assert_eq!(suite.passed_tests, 1);
        assert_eq!(suite.failed_tests, 1);
    }

    #[test]
    fn test_add_results() {
        let mut results = E2ETestResults::new();
        let mut suite = E2ETestSuite::new();

        suite.add_test("test_1".to_string(), true, None);
        results.add_results("test_suite", suite);

        assert_eq!(results.total_tests, 1);
        assert_eq!(results.passed_tests, 1);
        assert_eq!(results.failed_tests, 0);
        assert!(results.test_suites.contains_key("test_suite"));
    }
}
