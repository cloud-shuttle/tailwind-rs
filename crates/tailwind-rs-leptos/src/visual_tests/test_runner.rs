//! # Visual Test Runner
//!
//! This module provides test execution and management functionality for visual tests.

use super::report_generator::VisualTestReport;
use super::test_utils::VisualComparison;

/// Configuration for visual test execution
#[derive(Debug, Clone)]
pub struct VisualTestConfig {
    pub max_retries: u32,
    pub timeout_seconds: u64,
    pub parallel_execution: bool,
    pub save_snapshots: bool,
}

impl Default for VisualTestConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            timeout_seconds: 30,
            parallel_execution: true,
            save_snapshots: true,
        }
    }
}

/// Visual test runner for executing and managing visual tests
pub struct VisualTestRunner {
    config: VisualTestConfig,
    test_results: Vec<VisualComparison>,
}

impl VisualTestRunner {
    /// Create a new visual test runner with the given configuration
    pub fn new(config: VisualTestConfig) -> Self {
        Self {
            config,
            test_results: Vec::new(),
        }
    }

    /// Add test results to the runner
    pub fn add_test_results(&mut self, results: Vec<VisualComparison>) {
        self.test_results.extend(results);
    }

    /// Run a single test with retry logic
    pub fn run_test_with_retry<F>(&self, test_fn: F) -> VisualComparison
    where
        F: Fn() -> VisualComparison,
    {
        let mut last_result = test_fn();
        let mut retries = 0;

        while !last_result.classes_match && retries < self.config.max_retries {
            last_result = test_fn();
            retries += 1;
        }

        last_result
    }

    /// Run multiple tests in parallel or sequentially
    pub fn run_tests<F>(&self, test_functions: Vec<F>) -> Vec<VisualComparison>
    where
        F: Fn() -> VisualComparison + Send + Sync,
    {
        if self.config.parallel_execution {
            // In a real implementation, this would use rayon or similar for parallel execution
            test_functions.into_iter().map(|f| f()).collect()
        } else {
            test_functions.into_iter().map(|f| f()).collect()
        }
    }

    /// Generate a comprehensive test report
    pub fn generate_report(&self) -> VisualTestReport {
        let total_tests = self.test_results.len();
        let passed_tests = self.test_results.iter().filter(|r| r.classes_match).count();
        let failed_tests = total_tests - passed_tests;

        VisualTestReport {
            total_tests,
            passed_tests,
            failed_tests,
            test_results: self.test_results.clone(),
            execution_time: std::time::Duration::from_secs(0), // Would be calculated in real implementation
            timestamp: std::time::SystemTime::now(),
        }
    }

    /// Get the current test results
    pub fn get_test_results(&self) -> &[VisualComparison] {
        &self.test_results
    }

    /// Clear all test results
    pub fn clear_results(&mut self) {
        self.test_results.clear();
    }

    /// Get the configuration
    pub fn get_config(&self) -> &VisualTestConfig {
        &self.config
    }

    /// Update the configuration
    pub fn update_config(&mut self, config: VisualTestConfig) {
        self.config = config;
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_utils::VisualTestUtils;
    use super::*;

    #[test]
    fn test_visual_test_runner_creation() {
        let config = VisualTestConfig::default();
        let runner = VisualTestRunner::new(config);

        assert_eq!(runner.get_test_results().len(), 0);
        assert_eq!(runner.get_config().max_retries, 3);
    }

    #[test]
    fn test_add_test_results() {
        let config = VisualTestConfig::default();
        let mut runner = VisualTestRunner::new(config);

        let snapshot1 = VisualTestUtils::capture_component_state("Button", "px-4 py-2");
        let snapshot2 = VisualTestUtils::capture_component_state("Button", "px-4 py-2");
        let comparison = VisualTestUtils::compare_snapshots(&snapshot1, &snapshot2);

        runner.add_test_results(vec![comparison.clone()]);

        assert_eq!(runner.get_test_results().len(), 1);
        assert!(runner.get_test_results()[0].classes_match);
    }

    #[test]
    fn test_run_test_with_retry() {
        let config = VisualTestConfig::default();
        let runner = VisualTestRunner::new(config);

        let snapshot1 = VisualTestUtils::capture_component_state("Button", "px-4 py-2");
        let snapshot2 = VisualTestUtils::capture_component_state("Button", "px-4 py-2");

        let result = runner
            .run_test_with_retry(|| VisualTestUtils::compare_snapshots(&snapshot1, &snapshot2));

        assert!(result.classes_match);
    }

    #[test]
    fn test_generate_report() {
        let config = VisualTestConfig::default();
        let mut runner = VisualTestRunner::new(config);

        let snapshot1 = VisualTestUtils::capture_component_state("Button", "px-4 py-2");
        let snapshot2 = VisualTestUtils::capture_component_state("Button", "px-4 py-2");
        let comparison = VisualTestUtils::compare_snapshots(&snapshot1, &snapshot2);

        runner.add_test_results(vec![comparison]);
        let report = runner.generate_report();

        assert_eq!(report.total_tests, 1);
        assert_eq!(report.passed_tests, 1);
        assert_eq!(report.failed_tests, 0);
    }

    #[test]
    fn test_clear_results() {
        let config = VisualTestConfig::default();
        let mut runner = VisualTestRunner::new(config);

        let snapshot1 = VisualTestUtils::capture_component_state("Button", "px-4 py-2");
        let snapshot2 = VisualTestUtils::capture_component_state("Button", "px-4 py-2");
        let comparison = VisualTestUtils::compare_snapshots(&snapshot1, &snapshot2);

        runner.add_test_results(vec![comparison]);
        assert_eq!(runner.get_test_results().len(), 1);

        runner.clear_results();
        assert_eq!(runner.get_test_results().len(), 0);
    }
}
