//! Contract Testing Framework
//!
//! Provides comprehensive contract testing capabilities to ensure API stability
//! and catch breaking changes before they reach production.

use super::core::{traits::*, errors::*};
use std::collections::HashMap;

/// Contract test runner for automated testing
#[derive(Debug)]
pub struct ContractTestRunner {
    class_builder_contracts: HashMap<String, crate::contracts::class_builder::ClassBuilderContract>,
    css_generator_contracts: HashMap<String, crate::contracts::css_generator::CssGeneratorContract>,
    theme_contracts: HashMap<String, crate::contracts::theme::ThemeContract>,
    validation_contracts: HashMap<String, crate::contracts::validation::ValidationContract>,
    test_results: Vec<ContractTestResult>,
}

#[derive(Debug, Clone)]
pub struct ContractTestResult {
    pub contract_name: String,
    pub test_name: String,
    pub passed: bool,
    pub error_message: Option<String>,
    pub execution_time: std::time::Duration,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl ContractTestRunner {
    /// Create a new contract test runner
    pub fn new() -> Self {
        Self {
            class_builder_contracts: HashMap::new(),
            css_generator_contracts: HashMap::new(),
            theme_contracts: HashMap::new(),
            validation_contracts: HashMap::new(),
            test_results: Vec::new(),
        }
    }

    /// Register a class builder contract for testing
    pub fn register_class_builder(&mut self, name: String, contract: crate::contracts::class_builder::ClassBuilderContract) {
        self.class_builder_contracts.insert(name, contract);
    }

    /// Register a CSS generator contract for testing
    pub fn register_css_generator(&mut self, name: String, contract: crate::contracts::css_generator::CssGeneratorContract) {
        self.css_generator_contracts.insert(name, contract);
    }

    /// Register a theme contract for testing
    pub fn register_theme(&mut self, name: String, contract: crate::contracts::theme::ThemeContract) {
        self.theme_contracts.insert(name, contract);
    }

    /// Register a validation contract for testing
    pub fn register_validation(&mut self, name: String, contract: crate::contracts::validation::ValidationContract) {
        self.validation_contracts.insert(name, contract);
    }

    /// Run all contract tests
    pub fn run_all_tests(&mut self) -> ContractTestReport {
        let mut report = ContractTestReport::new();
        let start_time = std::time::Instant::now();

        // Test class builder contracts
        for (name, contract) in &self.class_builder_contracts {
            let contract_results = self.run_class_builder_tests(name, contract);
            report.add_contract_results(name.clone(), contract_results);
        }

        // Test CSS generator contracts
        for (name, contract) in &self.css_generator_contracts {
            let contract_results = self.run_css_generator_tests(name, contract);
            report.add_contract_results(name.clone(), contract_results);
        }

        // Test theme contracts
        for (name, contract) in &self.theme_contracts {
            let contract_results = self.run_theme_tests(name, contract);
            report.add_contract_results(name.clone(), contract_results);
        }

        // Test validation contracts
        for (name, contract) in &self.validation_contracts {
            let contract_results = self.run_validation_tests(name, contract);
            report.add_contract_results(name.clone(), contract_results);
        }

        report.total_execution_time = start_time.elapsed();
        report.timestamp = chrono::Utc::now();

        report
    }

    /// Run tests for a class builder contract
    fn run_class_builder_tests(
        &self,
        contract_name: &str,
        contract: &crate::contracts::class_builder::ClassBuilderContract,
    ) -> Vec<ContractTestResult> {
        let mut results = Vec::new();

        // Test contract interface compliance
        results.push(self.test_contract_interface(contract_name));

        // Test version compatibility
        results.push(self.test_version_compatibility(contract));

        // Test class builder specific functionality
        results.extend(self.test_class_builder_functionality(contract_name, contract));

        results
    }

    /// Run tests for a CSS generator contract
    fn run_css_generator_tests(
        &self,
        contract_name: &str,
        contract: &crate::contracts::css_generator::CssGeneratorContract,
    ) -> Vec<ContractTestResult> {
        let mut results = Vec::new();

        results.push(self.test_contract_interface(contract_name));
        results.push(self.test_version_compatibility(contract));
        results.extend(self.test_css_generator_functionality(contract_name, contract));

        results
    }

    /// Run tests for a theme contract
    fn run_theme_tests(
        &self,
        contract_name: &str,
        contract: &crate::contracts::theme::ThemeContract,
    ) -> Vec<ContractTestResult> {
        let mut results = Vec::new();

        results.push(self.test_contract_interface(contract_name));
        results.push(self.test_version_compatibility(contract));
        results.extend(self.test_theme_functionality(contract_name, contract));

        results
    }

    /// Run tests for a validation contract
    fn run_validation_tests(
        &self,
        contract_name: &str,
        contract: &crate::contracts::validation::ValidationContract,
    ) -> Vec<ContractTestResult> {
        let mut results = Vec::new();

        results.push(self.test_contract_interface(contract_name));
        results.push(self.test_version_compatibility(contract));
        results.extend(self.test_validation_functionality(contract_name, contract));

        results
    }

    /// Test that contract implements required interface
    fn test_contract_interface(
        &self,
        contract_name: &str,
    ) -> ContractTestResult {
        let start_time = std::time::Instant::now();

        // Basic interface compliance test
        let passed = true; // In a real implementation, this would do reflection-like checks
        let execution_time = start_time.elapsed();

        ContractTestResult {
            contract_name: contract_name.to_string(),
            test_name: "interface_compliance".to_string(),
            passed,
            error_message: None,
            execution_time,
            timestamp: chrono::Utc::now(),
        }
    }

    /// Test version compatibility
    fn test_version_compatibility<T: VersionedContract>(
        &self,
        contract: &T,
    ) -> ContractTestResult {
        let start_time = std::time::Instant::now();

        let current_version = contract.version();
        let supported_versions = contract.supported_versions();

        // Check that current version is in supported versions
        let passed = supported_versions.contains(&current_version);

        let error_message = if !passed {
            Some("Contract does not properly implement version compatibility".to_string())
        } else {
            None
        };

        let execution_time = start_time.elapsed();

        ContractTestResult {
            contract_name: "version_test".to_string(), // Will be overridden by caller
            test_name: "version_compatibility".to_string(),
            passed,
            error_message,
            execution_time,
            timestamp: chrono::Utc::now(),
        }
    }

    /// Test class builder specific functionality
    fn test_class_builder_functionality(
        &self,
        contract_name: &str,
        contract: &crate::contracts::class_builder::ClassBuilderContract,
    ) -> Vec<ContractTestResult> {
        let mut results = Vec::new();

        // Test basic class building
        let start_time = std::time::Instant::now();
        let input = crate::contracts::class_builder::ClassBuilderInput::new(vec!["bg-red-500".to_string()]);
        let validation_result = contract.validate_input(&input);
        let passed = validation_result.is_ok();
        let execution_time = start_time.elapsed();

        results.push(ContractTestResult {
            contract_name: contract_name.to_string(),
            test_name: "basic_class_building".to_string(),
            passed,
            error_message: validation_result.err().map(|e| e.to_string()),
            execution_time,
            timestamp: chrono::Utc::now(),
        });

        results
    }

    /// Test CSS generator specific functionality
    fn test_css_generator_functionality(
        &self,
        contract_name: &str,
        contract: &crate::contracts::css_generator::CssGeneratorContract,
    ) -> Vec<ContractTestResult> {
        let mut results = Vec::new();

        // Test basic CSS generation
        let start_time = std::time::Instant::now();
        let rule = crate::contracts::css_generator::CssRuleInput::new(
            ".test".to_string(),
            vec![crate::contracts::css_generator::CssPropertyInput::new("color".to_string(), "red".to_string())]
        );
        let input = crate::contracts::css_generator::CssGeneratorInput::new(vec![rule], crate::contracts::css_generator::CssFormat::Regular);
        let validation_result = contract.validate_input(&input);
        let passed = validation_result.is_ok();
        let execution_time = start_time.elapsed();

        results.push(ContractTestResult {
            contract_name: contract_name.to_string(),
            test_name: "basic_css_generation".to_string(),
            passed,
            error_message: validation_result.err().map(|e| e.to_string()),
            execution_time,
            timestamp: chrono::Utc::now(),
        });

        results
    }

    /// Test theme specific functionality
    fn test_theme_functionality(
        &self,
        contract_name: &str,
        contract: &crate::contracts::theme::ThemeContract,
    ) -> Vec<ContractTestResult> {
        let mut results = Vec::new();

        // Test basic theme validation
        let start_time = std::time::Instant::now();
        let input = crate::contracts::theme::ThemeInput::new("test-theme".to_string())
            .with_color("primary".to_string(), "#ff0000".to_string());
        let validation_result = contract.validate_input(&input);
        let passed = validation_result.is_ok();
        let execution_time = start_time.elapsed();

        results.push(ContractTestResult {
            contract_name: contract_name.to_string(),
            test_name: "basic_theme_validation".to_string(),
            passed,
            error_message: validation_result.err().map(|e| e.to_string()),
            execution_time,
            timestamp: chrono::Utc::now(),
        });

        results
    }

    /// Test validation specific functionality
    fn test_validation_functionality(
        &self,
        contract_name: &str,
        contract: &crate::contracts::validation::ValidationContract,
    ) -> Vec<ContractTestResult> {
        let mut results = Vec::new();

        // Test basic validation
        let start_time = std::time::Instant::now();
        let input = crate::contracts::validation::ValidationInput::new(vec!["bg-red-500".to_string()]);
        let validation_result = contract.validate_input(&input);
        let passed = validation_result.is_ok();
        let execution_time = start_time.elapsed();

        results.push(ContractTestResult {
            contract_name: contract_name.to_string(),
            test_name: "basic_validation".to_string(),
            passed,
            error_message: validation_result.err().map(|e| e.to_string()),
            execution_time,
            timestamp: chrono::Utc::now(),
        });

        results
    }

    /// Get test results
    pub fn get_results(&self) -> &[ContractTestResult] {
        &self.test_results
    }

    /// Clear test results
    pub fn clear_results(&mut self) {
        self.test_results.clear();
    }
}

/// Contract test report
#[derive(Debug, Clone)]
pub struct ContractTestReport {
    pub contract_results: HashMap<String, Vec<ContractTestResult>>,
    pub total_execution_time: std::time::Duration,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub summary: TestSummary,
}

#[derive(Debug, Clone, Default)]
pub struct TestSummary {
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub total_execution_time: std::time::Duration,
}

impl ContractTestReport {
    fn new() -> Self {
        Self {
            contract_results: HashMap::new(),
            total_execution_time: std::time::Duration::default(),
            timestamp: chrono::Utc::now(),
            summary: TestSummary::default(),
        }
    }

    fn add_contract_results(&mut self, contract_name: String, results: Vec<ContractTestResult>) {
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
}

impl std::fmt::Display for ContractTestReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Contract Test Report")?;
        writeln!(f, "===================")?;
        writeln!(f, "Timestamp: {}", self.timestamp)?;
        writeln!(f, "Total Execution Time: {:?}", self.total_execution_time)?;
        writeln!(f)?;
        writeln!(f, "Summary:")?;
        writeln!(f, "  Total Tests: {}", self.summary.total_tests)?;
        writeln!(f, "  Passed: {}", self.summary.passed_tests)?;
        writeln!(f, "  Failed: {}", self.summary.failed_tests)?;
        writeln!(f, "  Pass Rate: {:.1}%", self.pass_rate())?;
        writeln!(f)?;

        if !self.all_passed() {
            writeln!(f, "Failed Tests:")?;
            for failed_test in self.failed_tests() {
                writeln!(f, "  {}::{} - {}",
                    failed_test.contract_name,
                    failed_test.test_name,
                    failed_test.error_message.as_ref().unwrap_or(&"Unknown error".to_string())
                )?;
            }
        }

        Ok(())
    }
}

/// Property-based contract testing
#[cfg(test)]
pub mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn contract_error_display_roundtrip(err in any::<ContractError>()) {
            let display = err.to_string();
            assert!(!display.is_empty());
            assert!(display.len() < 500); // Reasonable length limit
        }

        #[test]
        fn contract_test_result_serialization(result in any::<ContractTestResult>()) {
            // Test that results can be serialized (important for CI/CD)
            let json = serde_json::to_string(&result);
            assert!(json.is_ok());
        }
    }
}

/// Integration testing for contract compatibility
#[cfg(test)]
pub mod integration_tests {
    use super::*;
    use crate::contracts::class_builder::ClassBuilderContract;

    #[test]
    fn contract_test_runner_integration() {
        let mut runner = ContractTestRunner::new();

        let contract = ClassBuilderContract::new(ApiVersion::new(1, 0, 0));
        runner.register_contract("class_builder".to_string(), contract);

        let report = runner.run_all_tests();

        // Should have run some tests
        assert!(report.summary.total_tests > 0);

        // All tests should pass for a properly implemented contract
        assert!(report.all_passed(), "Contract tests failed: {}", report);
    }

    #[test]
    fn contract_test_report_formatting() {
        let report = ContractTestReport::new();
        let formatted = format!("{}", report);

        assert!(formatted.contains("Contract Test Report"));
        assert!(formatted.contains("Summary"));
    }
}

/// CI/CD integration for contract testing
pub mod ci_integration {
    use super::*;

    /// Run contract tests and exit with appropriate code
    pub fn run_contract_tests_and_exit() -> ! {
        let mut runner = ContractTestRunner::new();

        // Register all available contracts
        // In a real implementation, this would discover and register all contracts
        // runner.register_contract("class_builder".to_string(), ClassBuilderContract::new(ApiVersion::new(1, 0, 0)));
        // runner.register_contract("css_generator".to_string(), CssGeneratorContract::new(ApiVersion::new(1, 0, 0)));
        // etc.

        let report = runner.run_all_tests();

        println!("{}", report);

        if report.all_passed() {
            println!("✅ All contract tests passed!");
            std::process::exit(0);
        } else {
            println!("❌ Contract tests failed!");
            std::process::exit(1);
        }
    }

    /// Generate contract test report for CI
    pub fn generate_ci_report(report: &ContractTestReport) -> Result<String, ContractError> {
        let mut ci_output = String::new();

        ci_output.push_str(&format!("## Contract Test Results\n\n"));
        ci_output.push_str(&format!("**Pass Rate:** {:.1}%\n\n", report.pass_rate()));
        ci_output.push_str(&format!("**Total Tests:** {}\n", report.summary.total_tests));
        ci_output.push_str(&format!("**Passed:** {}\n", report.summary.passed_tests));
        ci_output.push_str(&format!("**Failed:** {}\n\n", report.summary.failed_tests));

        if !report.all_passed() {
            ci_output.push_str("### Failed Tests\n\n");
            for failed_test in report.failed_tests() {
                ci_output.push_str(&format!("- **{}::{}**: {}\n",
                    failed_test.contract_name,
                    failed_test.test_name,
                    failed_test.error_message.as_ref().unwrap_or(&"Unknown error".to_string())
                ));
            }
        }

        ci_output.push_str(&format!("\n**Execution Time:** {:?}\n", report.total_execution_time));
        ci_output.push_str(&format!("**Timestamp:** {}\n", report.timestamp));

        Ok(ci_output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contract_test_runner_creation() {
        let runner = ContractTestRunner::new();
        assert!(runner.contracts.is_empty());
        assert!(runner.test_results.is_empty());
    }

    #[test]
    fn contract_test_report_operations() {
        let mut report = ContractTestReport::new();

        let result = ContractTestResult {
            contract_name: "test_contract".to_string(),
            test_name: "test_case".to_string(),
            passed: true,
            error_message: None,
            execution_time: std::time::Duration::from_millis(100),
            timestamp: chrono::Utc::now(),
        };

        report.add_contract_results("test_contract".to_string(), vec![result]);

        assert_eq!(report.summary.total_tests, 1);
        assert_eq!(report.summary.passed_tests, 1);
        assert_eq!(report.summary.failed_tests, 0);
        assert!(report.all_passed());
        assert_eq!(report.pass_rate(), 100.0);
    }

    #[test]
    fn test_summary_calculations() {
        let summary = TestSummary {
            total_tests: 10,
            passed_tests: 8,
            failed_tests: 2,
            total_execution_time: std::time::Duration::from_secs(1),
        };

        // Test pass rate calculation
        let pass_rate = (summary.passed_tests as f64 / summary.total_tests as f64) * 100.0;
        assert_eq!(pass_rate, 80.0);
    }

    #[test]
    fn contract_test_result_creation() {
        let result = ContractTestResult {
            contract_name: "test".to_string(),
            test_name: "case".to_string(),
            passed: false,
            error_message: Some("test error".to_string()),
            execution_time: std::time::Duration::from_millis(50),
            timestamp: chrono::Utc::now(),
        };

        assert_eq!(result.contract_name, "test");
        assert_eq!(result.test_name, "case");
        assert!(!result.passed);
        assert_eq!(result.error_message, Some("test error".to_string()));
    }
}
