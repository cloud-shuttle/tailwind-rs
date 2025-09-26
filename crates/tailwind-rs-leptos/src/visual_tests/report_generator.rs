//! # Visual Test Report Generator
//!
//! This module provides functionality for generating and formatting visual test reports.

use super::test_utils::VisualComparison;

/// A comprehensive visual test report
#[derive(Debug, Clone)]
pub struct VisualTestReport {
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub test_results: Vec<VisualComparison>,
    pub execution_time: std::time::Duration,
    pub timestamp: std::time::SystemTime,
}

impl VisualTestReport {
    /// Create a new visual test report
    pub fn new(test_results: Vec<VisualComparison>, execution_time: std::time::Duration) -> Self {
        let total_tests = test_results.len();
        let passed_tests = test_results.iter().filter(|r| r.classes_match).count();
        let failed_tests = total_tests - passed_tests;

        Self {
            total_tests,
            passed_tests,
            failed_tests,
            test_results,
            execution_time,
            timestamp: std::time::SystemTime::now(),
        }
    }

    /// Get the success rate as a percentage
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

    /// Get failed test results
    pub fn get_failed_tests(&self) -> Vec<&VisualComparison> {
        self.test_results
            .iter()
            .filter(|r| !r.classes_match)
            .collect()
    }

    /// Get passed test results
    pub fn get_passed_tests(&self) -> Vec<&VisualComparison> {
        self.test_results
            .iter()
            .filter(|r| r.classes_match)
            .collect()
    }

    /// Format the report as a string
    pub fn format_report(&self) -> String {
        let mut output = String::new();

        output.push_str(&format!("Visual Test Report\n"));
        output.push_str(&format!("==================\n"));
        output.push_str(&format!("Total Tests: {}\n", self.total_tests));
        output.push_str(&format!("Passed: {}\n", self.passed_tests));
        output.push_str(&format!("Failed: {}\n", self.failed_tests));
        output.push_str(&format!("Success Rate: {:.1}%\n", self.success_rate()));
        output.push_str(&format!("Execution Time: {:?}\n", self.execution_time));

        if !self.failed_tests == 0 {
            output.push_str(&format!("\nFailed Tests:\n"));
            for (i, failed_test) in self.get_failed_tests().iter().enumerate() {
                output.push_str(&format!(
                    "  {}. {} - Classes don't match\n",
                    i + 1,
                    failed_test.component_name
                ));
                for diff in &failed_test.differences {
                    output.push_str(&format!(
                        "     {}: '{}' -> '{}'\n",
                        diff.field, diff.old_value, diff.new_value
                    ));
                }
            }
        }

        output
    }

    /// Generate a JSON representation of the report
    pub fn to_json(&self) -> String {
        // In a real implementation, this would use serde_json
        format!(
            r#"{{"total_tests":{},"passed_tests":{},"failed_tests":{},"success_rate":{:.1},"execution_time_ms":{}}}"#,
            self.total_tests,
            self.passed_tests,
            self.failed_tests,
            self.success_rate(),
            self.execution_time.as_millis()
        )
    }

    /// Save the report to a file
    pub fn save_to_file(&self, file_path: &str) -> Result<(), std::io::Error> {
        std::fs::write(file_path, self.format_report())
    }

    /// Merge this report with another report
    pub fn merge(&mut self, other: VisualTestReport) {
        self.total_tests += other.total_tests;
        self.passed_tests += other.passed_tests;
        self.failed_tests += other.failed_tests;
        self.test_results.extend(other.test_results);
        self.execution_time += other.execution_time;
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_utils::VisualTestUtils;
    use super::*;

    #[test]
    fn test_visual_test_report_creation() {
        let snapshot1 = VisualTestUtils::capture_component_state("Button", "px-4 py-2");
        let snapshot2 = VisualTestUtils::capture_component_state("Button", "px-4 py-2");
        let comparison = VisualTestUtils::compare_snapshots(&snapshot1, &snapshot2);

        let report = VisualTestReport::new(vec![comparison], std::time::Duration::from_secs(1));

        assert_eq!(report.total_tests, 1);
        assert_eq!(report.passed_tests, 1);
        assert_eq!(report.failed_tests, 0);
    }

    #[test]
    fn test_success_rate_calculation() {
        let snapshot1 = VisualTestUtils::capture_component_state("Button", "px-4 py-2");
        let snapshot2 = VisualTestUtils::capture_component_state("Button", "px-4 py-2");
        let comparison = VisualTestUtils::compare_snapshots(&snapshot1, &snapshot2);

        let report = VisualTestReport::new(vec![comparison], std::time::Duration::from_secs(1));

        assert_eq!(report.success_rate(), 100.0);
    }

    #[test]
    fn test_all_tests_passed() {
        let snapshot1 = VisualTestUtils::capture_component_state("Button", "px-4 py-2");
        let snapshot2 = VisualTestUtils::capture_component_state("Button", "px-4 py-2");
        let comparison = VisualTestUtils::compare_snapshots(&snapshot1, &snapshot2);

        let report = VisualTestReport::new(vec![comparison], std::time::Duration::from_secs(1));

        assert!(report.all_tests_passed());
    }

    #[test]
    fn test_get_failed_tests() {
        let snapshot1 = VisualTestUtils::capture_component_state("Button", "px-4 py-2");
        let snapshot2 = VisualTestUtils::capture_component_state("Button", "px-4 py-3");
        let comparison = VisualTestUtils::compare_snapshots(&snapshot1, &snapshot2);

        let report = VisualTestReport::new(vec![comparison], std::time::Duration::from_secs(1));

        assert_eq!(report.get_failed_tests().len(), 1);
        assert_eq!(report.get_passed_tests().len(), 0);
    }

    #[test]
    fn test_format_report() {
        let snapshot1 = VisualTestUtils::capture_component_state("Button", "px-4 py-2");
        let snapshot2 = VisualTestUtils::capture_component_state("Button", "px-4 py-2");
        let comparison = VisualTestUtils::compare_snapshots(&snapshot1, &snapshot2);

        let report = VisualTestReport::new(vec![comparison], std::time::Duration::from_secs(1));
        let formatted = report.format_report();

        assert!(formatted.contains("Visual Test Report"));
        assert!(formatted.contains("Total Tests: 1"));
        assert!(formatted.contains("Passed: 1"));
        assert!(formatted.contains("Failed: 0"));
        assert!(formatted.contains("Success Rate: 100.0%"));
    }

    #[test]
    fn test_merge_reports() {
        let snapshot1 = VisualTestUtils::capture_component_state("Button", "px-4 py-2");
        let snapshot2 = VisualTestUtils::capture_component_state("Button", "px-4 py-2");
        let comparison1 = VisualTestUtils::compare_snapshots(&snapshot1, &snapshot2);

        let snapshot3 = VisualTestUtils::capture_component_state("Input", "px-3 py-2");
        let snapshot4 = VisualTestUtils::capture_component_state("Input", "px-3 py-2");
        let comparison2 = VisualTestUtils::compare_snapshots(&snapshot3, &snapshot4);

        let mut report1 =
            VisualTestReport::new(vec![comparison1], std::time::Duration::from_secs(1));
        let report2 = VisualTestReport::new(vec![comparison2], std::time::Duration::from_secs(2));

        report1.merge(report2);

        assert_eq!(report1.total_tests, 2);
        assert_eq!(report1.passed_tests, 2);
        assert_eq!(report1.failed_tests, 0);
        assert_eq!(report1.execution_time, std::time::Duration::from_secs(3));
    }
}
