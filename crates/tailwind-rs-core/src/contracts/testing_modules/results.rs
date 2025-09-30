//! Contract Testing Results Module
//!
//! Handles contract test results and reporting:
//! - ContractTestResult: Individual test result structure
//! - ContractTestReport: Comprehensive test report with summary
//! - TestSummary: Aggregated test statistics

use std::collections::HashMap;

/// Individual contract test result
#[derive(Debug, Clone)]
pub struct ContractTestResult {
    pub contract_name: String,
    pub test_name: String,
    pub passed: bool,
    pub error_message: Option<String>,
    pub execution_time: std::time::Duration,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl ContractTestResult {
    /// Create a new test result
    pub fn new(
        contract_name: String,
        test_name: String,
        passed: bool,
        error_message: Option<String>,
        execution_time: std::time::Duration,
    ) -> Self {
        Self {
            contract_name,
            test_name,
            passed,
            error_message,
            execution_time,
            timestamp: chrono::Utc::now(),
        }
    }

    /// Create a passing test result
    pub fn pass(contract_name: String, test_name: String, execution_time: std::time::Duration) -> Self {
        Self::new(contract_name, test_name, true, None, execution_time)
    }

    /// Create a failing test result
    pub fn fail(
        contract_name: String,
        test_name: String,
        error_message: String,
        execution_time: std::time::Duration,
    ) -> Self {
        Self::new(contract_name, test_name, false, Some(error_message), execution_time)
    }

    /// Check if the test passed
    pub fn is_pass(&self) -> bool {
        self.passed
    }

    /// Check if the test failed
    pub fn is_fail(&self) -> bool {
        !self.passed
    }

    /// Get the error message if the test failed
    pub fn error_message(&self) -> Option<&str> {
        self.error_message.as_deref()
    }

    /// Get a human-readable description of the result
    pub fn description(&self) -> String {
        if self.passed {
            format!("✅ {}::{} passed in {:?}", self.contract_name, self.test_name, self.execution_time)
        } else {
            format!("❌ {}::{} failed: {} (took {:?})",
                    self.contract_name,
                    self.test_name,
                    self.error_message.as_deref().unwrap_or("Unknown error"),
                    self.execution_time)
        }
    }
}

/// Contract test report with comprehensive results
#[derive(Debug, Clone)]
pub struct ContractTestReport {
    pub contract_results: HashMap<String, Vec<ContractTestResult>>,
    pub total_execution_time: std::time::Duration,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub summary: TestSummary,
}

impl ContractTestReport {
    /// Create a new empty test report
    pub fn new() -> Self {
        Self {
            contract_results: HashMap::new(),
            total_execution_time: std::time::Duration::default(),
            timestamp: chrono::Utc::now(),
            summary: TestSummary::default(),
        }
    }

    /// Add results for a specific contract
    pub fn add_contract_results(&mut self, contract_name: String, results: Vec<ContractTestResult>) {
        self.contract_results.insert(contract_name, results.clone());

        for result in results {
            self.summary.total_tests += 1;
            if result.passed {
                self.summary.passed_tests += 1;
            } else {
                self.summary.failed_tests += 1;
            }
            self.summary.total_execution_time += result.execution_time;
        }
    }

    /// Check if all tests passed
    pub fn all_passed(&self) -> bool {
        self.summary.failed_tests == 0
    }

    /// Get pass rate as percentage
    pub fn pass_rate(&self) -> f64 {
        if self.summary.total_tests == 0 {
            0.0
        } else {
            (self.summary.passed_tests as f64 / self.summary.total_tests as f64) * 100.0
        }
    }

    /// Get failed test details
    pub fn failed_tests(&self) -> Vec<&ContractTestResult> {
        self.contract_results.values()
            .flatten()
            .filter(|result| !result.passed)
            .collect()
    }

    /// Get successful test details
    pub fn passed_tests(&self) -> Vec<&ContractTestResult> {
        self.contract_results.values()
            .flatten()
            .filter(|result| result.passed)
            .collect()
    }

    /// Get all test results
    pub fn all_results(&self) -> Vec<&ContractTestResult> {
        self.contract_results.values()
            .flatten()
            .collect()
    }

    /// Get results for a specific contract
    pub fn contract_results(&self, contract_name: &str) -> Option<&Vec<ContractTestResult>> {
        self.contract_results.get(contract_name)
    }

    /// Get all contract names
    pub fn contract_names(&self) -> Vec<&String> {
        self.contract_results.keys().collect()
    }
}

impl Default for ContractTestReport {
    fn default() -> Self {
        Self::new()
    }
}

/// Aggregated test statistics
#[derive(Debug, Clone, Default)]
pub struct TestSummary {
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub total_execution_time: std::time::Duration,
}

impl TestSummary {
    /// Create a new test summary
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if all tests passed
    pub fn all_passed(&self) -> bool {
        self.failed_tests == 0 && self.total_tests > 0
    }

    /// Get pass rate as percentage
    pub fn pass_rate(&self) -> f64 {
        if self.total_tests == 0 {
            0.0
        } else {
            (self.passed_tests as f64 / self.total_tests as f64) * 100.0
        }
    }

    /// Get average execution time per test
    pub fn average_execution_time(&self) -> std::time::Duration {
        if self.total_tests == 0 {
            std::time::Duration::default()
        } else {
            self.total_execution_time / self.total_tests as u32
        }
    }

    /// Get a human-readable summary
    pub fn summary_text(&self) -> String {
        format!(
            "Tests: {} total, {} passed, {} failed ({:.1}% pass rate, avg: {:?})",
            self.total_tests,
            self.passed_tests,
            self.failed_tests,
            self.pass_rate(),
            self.average_execution_time()
        )
    }
}

impl std::fmt::Display for ContractTestReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Contract Test Report")?;
        writeln!(f, "===================")?;
        writeln!(f, "Timestamp: {}", self.timestamp)?;
        writeln!(f, "Total Execution Time: {:?}", self.total_execution_time)?;
        writeln!(f, "Summary: {}", self.summary.summary_text())?;
        writeln!(f)?;

        if self.contract_results.is_empty() {
            writeln!(f, "No contracts tested.")?;
        } else {
            writeln!(f, "Contract Details:")?;
            for (contract_name, results) in &self.contract_results {
                writeln!(f, "  {}: {} tests", contract_name, results.len())?;
                let passed = results.iter().filter(|r| r.passed).count();
                let failed = results.len() - passed;
                writeln!(f, "    Passed: {}, Failed: {}", passed, failed)?;

                if !results.iter().all(|r| r.passed) {
                    writeln!(f, "    Failed tests:")?;
                    for result in results.iter().filter(|r| !r.passed) {
                        writeln!(f, "      - {}: {}", result.test_name, result.error_message.as_deref().unwrap_or("Unknown error"))?;
                    }
                }
            }
        }

        if self.all_passed() {
            writeln!(f, "\n✅ All contract tests passed!")?;
        } else {
            writeln!(f, "\n❌ Some contract tests failed. See details above.")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contract_test_result_creation() {
        let result = ContractTestResult::pass(
            "test_contract".to_string(),
            "test_functionality".to_string(),
            std::time::Duration::from_millis(100),
        );

        assert_eq!(result.contract_name, "test_contract");
        assert_eq!(result.test_name, "test_functionality");
        assert!(result.is_pass());
        assert!(!result.is_fail());
        assert!(result.error_message().is_none());
    }

    #[test]
    fn contract_test_result_failure() {
        let result = ContractTestResult::fail(
            "test_contract".to_string(),
            "test_functionality".to_string(),
            "Test failed".to_string(),
            std::time::Duration::from_millis(100),
        );

        assert!(!result.is_pass());
        assert!(result.is_fail());
        assert_eq!(result.error_message(), Some("Test failed"));
    }

    #[test]
    fn contract_test_report_operations() {
        let mut report = ContractTestReport::new();

        let results = vec![
            ContractTestResult::pass("contract1".to_string(), "test1".to_string(), std::time::Duration::from_millis(10)),
            ContractTestResult::fail("contract1".to_string(), "test2".to_string(), "Error".to_string(), std::time::Duration::from_millis(20)),
        ];

        report.add_contract_results("contract1".to_string(), results);

        assert_eq!(report.summary.total_tests, 2);
        assert_eq!(report.summary.passed_tests, 1);
        assert_eq!(report.summary.failed_tests, 1);
        assert!(!report.all_passed());
        assert_eq!(report.pass_rate(), 50.0);
        assert_eq!(report.failed_tests().len(), 1);
        assert_eq!(report.passed_tests().len(), 1);
    }

    #[test]
    fn test_summary_calculations() {
        let mut summary = TestSummary::new();
        summary.total_tests = 10;
        summary.passed_tests = 8;
        summary.failed_tests = 2;
        summary.total_execution_time = std::time::Duration::from_millis(100);

        assert!(!summary.all_passed());
        assert_eq!(summary.pass_rate(), 80.0);
        assert_eq!(summary.average_execution_time(), std::time::Duration::from_millis(10));
    }

    #[test]
    fn empty_test_report() {
        let report = ContractTestReport::new();

        assert!(report.all_passed());
        assert_eq!(report.pass_rate(), 0.0);
        assert!(report.failed_tests().is_empty());
        assert!(report.contract_names().is_empty());
    }
}
