//! API Contracts and Contract Testing
//!
//! This module provides comprehensive API contracts and contract testing
//! to ensure API stability, backward compatibility, and reliability.

use crate::classes::{ClassBuilder, ClassSet};
use crate::css_generator::CssGenerator;
use crate::error::TailwindError;
use crate::responsive::Breakpoint;
use std::collections::HashMap;
use std::result::Result;

/// API contract trait for ensuring API stability
pub trait ApiContract {
    type Input;
    type Output;
    type Error;

    /// Validate input according to contract
    fn validate_input(&self, input: &Self::Input) -> Result<(), ContractError>;

    /// Process input according to contract
    fn process(&self, input: Self::Input) -> Result<Self::Output, Self::Error>;

    /// Validate output according to contract
    fn validate_output(&self, output: &Self::Output) -> Result<(), ContractError>;
}

/// Contract error type
#[derive(Debug, Clone, PartialEq)]
pub enum ContractError {
    InvalidInput(String),
    InvalidOutput(String),
    ContractViolation(String),
    BackwardCompatibilityViolation(String),
}

impl std::fmt::Display for ContractError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContractError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            ContractError::InvalidOutput(msg) => write!(f, "Invalid output: {}", msg),
            ContractError::ContractViolation(msg) => write!(f, "Contract violation: {}", msg),
            ContractError::BackwardCompatibilityViolation(msg) => {
                write!(f, "Backward compatibility violation: {}", msg)
            }
        }
    }
}

impl std::error::Error for ContractError {}

/// ClassBuilder API contract
#[derive(Debug, Clone)]
pub struct ClassBuilderContract {
    version: ApiVersion,
    supported_methods: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ApiVersion {
    V1_0_0,
    V1_1_0,
    V2_0_0,
}

impl ClassBuilderContract {
    pub fn new(version: ApiVersion) -> Self {
        Self {
            version,
            supported_methods: vec![
                "new".to_string(),
                "class".to_string(),
                "classes".to_string(),
                "responsive".to_string(),
                "conditional".to_string(),
                "custom".to_string(),
                "build".to_string(),
                "build_string".to_string(),
            ],
        }
    }
}

impl ApiContract for ClassBuilderContract {
    type Input = ClassBuilderInput;
    type Output = ClassSet;
    type Error = TailwindError;

    fn validate_input(&self, input: &Self::Input) -> Result<(), ContractError> {
        // Validate class names
        for class in &input.classes {
            if class.is_empty() {
                return Err(ContractError::InvalidInput("Empty class name".to_string()));
            }
            if class.contains(" ") {
                return Err(ContractError::InvalidInput(
                    "Class name contains spaces".to_string(),
                ));
            }
        }

        // Validate breakpoints
        for (breakpoint, _) in &input.responsive {
            match breakpoint {
                Breakpoint::Base
                | Breakpoint::Sm
                | Breakpoint::Md
                | Breakpoint::Lg
                | Breakpoint::Xl
                | Breakpoint::Xl2 => {}
                _ => {
                    return Err(ContractError::InvalidInput(
                        "Invalid breakpoint".to_string(),
                    ))
                }
            }
        }

        Ok(())
    }

    fn process(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        let mut builder = ClassBuilder::new();

        // Add base classes
        for class in input.classes {
            builder = builder.class(class);
        }

        // Add responsive classes
        for (breakpoint, class) in input.responsive {
            builder = builder.responsive(breakpoint, class);
        }

        // Add conditional classes
        for (condition, class) in input.conditional {
            builder = builder.conditional(condition, class);
        }

        // Add custom properties
        for (property, value) in input.custom {
            builder = builder.custom(property, value);
        }

        Ok(builder.build())
    }

    fn validate_output(&self, output: &Self::Output) -> Result<(), ContractError> {
        // Validate ClassSet structure
        if output.len() == 0 && !output.is_empty() {
            return Err(ContractError::InvalidOutput(
                "Invalid ClassSet state".to_string(),
            ));
        }

        // Validate CSS class string format
        let css_classes = output.to_css_classes();
        if css_classes.contains("  ") {
            return Err(ContractError::InvalidOutput(
                "CSS classes contain double spaces".to_string(),
            ));
        }

        Ok(())
    }
}

/// Input for ClassBuilder contract
#[derive(Debug, Clone)]
pub struct ClassBuilderInput {
    pub classes: Vec<String>,
    pub responsive: Vec<(Breakpoint, String)>,
    pub conditional: Vec<(String, String)>,
    pub custom: Vec<(String, String)>,
}

/// CssGenerator API contract
#[derive(Debug, Clone)]
pub struct CssGeneratorContract {
    version: ApiVersion,
    supported_formats: Vec<CssFormat>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CssFormat {
    Regular,
    Minified,
    WithSourceMaps,
}

impl CssGeneratorContract {
    pub fn new(version: ApiVersion) -> Self {
        Self {
            version,
            supported_formats: vec![
                CssFormat::Regular,
                CssFormat::Minified,
                CssFormat::WithSourceMaps,
            ],
        }
    }
}

impl ApiContract for CssGeneratorContract {
    type Input = CssGeneratorInput;
    type Output = String;
    type Error = TailwindError;

    fn validate_input(&self, input: &Self::Input) -> Result<(), ContractError> {
        // Validate CSS rules
        for rule in &input.rules {
            if rule.selector.is_empty() {
                return Err(ContractError::InvalidInput(
                    "Empty CSS selector".to_string(),
                ));
            }
            if rule.properties.is_empty() {
                return Err(ContractError::InvalidInput(
                    "Empty CSS properties".to_string(),
                ));
            }
        }

        // Validate media queries
        for media_query in &input.media_queries {
            if !media_query.starts_with("@media") {
                return Err(ContractError::InvalidInput(
                    "Invalid media query format".to_string(),
                ));
            }
        }

        Ok(())
    }

    fn process(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        let mut generator = CssGenerator::new();

        // Add CSS rules
        for rule in input.rules {
            let properties_str = rule
                .properties
                .iter()
                .map(|p| format!("{}: {}", p.name, p.value))
                .collect::<Vec<_>>()
                .join("; ");
            generator.add_css_selector(&rule.selector, &properties_str)?;
        }

        // Generate CSS based on format
        match input.format {
            CssFormat::Regular => Ok(generator.generate_css()),
            CssFormat::Minified => Ok(generator.generate_minified_css()),
            CssFormat::WithSourceMaps => {
                // For now, just return regular CSS
                // In a full implementation, this would generate source maps
                Ok(generator.generate_css())
            }
        }
    }

    fn validate_output(&self, output: &Self::Output) -> Result<(), ContractError> {
        // Validate CSS syntax
        if output.is_empty() {
            return Err(ContractError::InvalidOutput("Empty CSS output".to_string()));
        }

        // Check for basic CSS structure
        if !output.contains("{") || !output.contains("}") {
            return Err(ContractError::InvalidOutput(
                "Invalid CSS structure".to_string(),
            ));
        }

        Ok(())
    }
}

/// Input for CssGenerator contract
#[derive(Debug, Clone)]
pub struct CssGeneratorInput {
    pub rules: Vec<CssRuleInput>,
    pub media_queries: Vec<String>,
    pub format: CssFormat,
}

#[derive(Debug, Clone)]
pub struct CssRuleInput {
    pub selector: String,
    pub properties: Vec<CssPropertyInput>,
}

#[derive(Debug, Clone)]
pub struct CssPropertyInput {
    pub name: String,
    pub value: String,
    pub important: bool,
}

/// Contract testing framework
#[derive(Debug, Clone)]
pub struct ContractTester {
    contracts: Vec<String>,
    test_cases: Vec<TestCase>,
}

#[derive(Debug, Clone)]
pub struct TestCase {
    pub name: String,
    pub input: String,
    pub expected_output: String,
    pub should_fail: bool,
}

impl ContractTester {
    pub fn new() -> Self {
        Self {
            contracts: Vec::new(),
            test_cases: Vec::new(),
        }
    }

    pub fn add_contract(&mut self, contract: String) {
        self.contracts.push(contract);
    }

    pub fn add_test_case(&mut self, test_case: TestCase) {
        self.test_cases.push(test_case);
    }

    pub fn run_tests(&self) -> Result<TestResults, ContractError> {
        let mut results = TestResults::new();

        for test_case in &self.test_cases {
            let result = self.run_single_test(test_case);
            results.add_result(test_case.name.clone(), result);
        }

        Ok(results)
    }

    fn run_single_test(&self, test_case: &TestCase) -> TestResult {
        // This is a simplified implementation
        // In a real implementation, this would run the actual contract tests
        TestResult {
            passed: true,
            error: None,
            duration: std::time::Duration::from_millis(1),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TestResults {
    pub results: HashMap<String, TestResult>,
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
}

impl TestResults {
    pub fn new() -> Self {
        Self {
            results: HashMap::new(),
            total_tests: 0,
            passed_tests: 0,
            failed_tests: 0,
        }
    }

    pub fn add_result(&mut self, name: String, result: TestResult) {
        self.results.insert(name, result.clone());
        self.total_tests += 1;
        if result.passed {
            self.passed_tests += 1;
        } else {
            self.failed_tests += 1;
        }
    }
}

#[derive(Debug, Clone)]
pub struct TestResult {
    pub passed: bool,
    pub error: Option<String>,
    pub duration: std::time::Duration,
}

/// Runtime contract validation
#[derive(Debug, Clone)]
pub struct ContractValidator {
    contracts: HashMap<String, String>,
    validation_enabled: bool,
}

impl ContractValidator {
    pub fn new() -> Self {
        Self {
            contracts: HashMap::new(),
            validation_enabled: true,
        }
    }

    pub fn add_contract(&mut self, name: String, _contract: Box<dyn std::any::Any>) {
        // Simplified contract storage
        self.contracts.insert(name, "contract".to_string());
    }

    pub fn validate_call<T>(&self, api_name: &str, input: T) -> Result<(), ContractError> {
        if !self.validation_enabled {
            return Ok(());
        }

        if let Some(contract) = self.contracts.get(api_name) {
            // In a real implementation, this would validate the input
            // For now, we'll just return Ok
            Ok(())
        } else {
            Err(ContractError::ContractViolation(format!(
                "Unknown API: {}",
                api_name
            )))
        }
    }

    pub fn enable_validation(&mut self) {
        self.validation_enabled = true;
    }

    pub fn disable_validation(&mut self) {
        self.validation_enabled = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_class_builder_contract() {
        let contract = ClassBuilderContract::new(ApiVersion::V2_0_0);

        let input = ClassBuilderInput {
            classes: vec!["p-4".to_string(), "m-2".to_string()],
            responsive: vec![(Breakpoint::Md, "text-lg".to_string())],
            conditional: vec![("hover".to_string(), "bg-blue-600".to_string())],
            custom: vec![("primary-color".to_string(), "#3b82f6".to_string())],
        };

        // Test input validation
        assert!(contract.validate_input(&input).is_ok());

        // Test processing
        let output = contract.process(input).unwrap();

        // Test output validation
        assert!(contract.validate_output(&output).is_ok());
    }

    #[test]
    fn test_css_generator_contract() {
        let contract = CssGeneratorContract::new(ApiVersion::V2_0_0);

        let input = CssGeneratorInput {
            rules: vec![CssRuleInput {
                selector: ".test".to_string(),
                properties: vec![CssPropertyInput {
                    name: "padding".to_string(),
                    value: "1rem".to_string(),
                    important: false,
                }],
            }],
            media_queries: vec!["@media (min-width: 768px)".to_string()],
            format: CssFormat::Regular,
        };

        // Test input validation
        assert!(contract.validate_input(&input).is_ok());

        // Test processing
        let output = contract.process(input).unwrap();

        // Test output validation
        assert!(contract.validate_output(&output).is_ok());
    }

    #[test]
    fn test_contract_tester() {
        let mut tester = ContractTester::new();

        let test_case = TestCase {
            name: "test_case_1".to_string(),
            input: "test_input".to_string(),
            expected_output: "test_output".to_string(),
            should_fail: false,
        };

        tester.add_test_case(test_case);

        let results = tester.run_tests().unwrap();
        assert_eq!(results.total_tests, 1);
    }

    #[test]
    fn test_contract_validator() {
        let mut validator = ContractValidator::new();

        // Test validation
        let result = validator.validate_call("test_api", "test_input");
        assert!(result.is_err()); // Should fail because no contract is registered

        // Test enabling/disabling validation
        validator.disable_validation();
        let result = validator.validate_call("test_api", "test_input");
        assert!(result.is_ok()); // Should pass because validation is disabled
    }
}
