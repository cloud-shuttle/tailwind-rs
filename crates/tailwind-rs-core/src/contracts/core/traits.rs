//! Core Contract Traits
//!
//! This module defines the fundamental contract interfaces that ensure API stability
//! and provide the foundation for all contract implementations in Tailwind-RS.

use super::errors::ContractError;

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

/// Version-aware contract trait for backward compatibility
pub trait VersionedContract {
    /// Get current contract version
    fn version(&self) -> ApiVersion;

    /// Get all supported versions (for backward compatibility)
    fn supported_versions(&self) -> Vec<ApiVersion>;

    /// Check if contract is backward compatible with given version
    fn is_backward_compatible(&self, other: &ApiVersion) -> bool;
}

/// Testable contract trait for automated testing
pub trait TestableContract: ApiContract {
    /// Run contract-specific tests
    fn run_contract_tests(&self) -> Vec<TestResult>;

    /// Get contract test cases
    fn test_cases(&self) -> Vec<ContractTestCase>;
}

/// API version representation
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ApiVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl ApiVersion {
    /// Create a new API version
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self { major, minor, patch }
    }

    /// Check if this version is compatible with another (semantic versioning)
    pub fn is_compatible_with(&self, other: &ApiVersion) -> bool {
        self.major == other.major
    }
}

impl std::fmt::Display for ApiVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl Ord for ApiVersion {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.major.cmp(&other.major)
            .then(self.minor.cmp(&other.minor))
            .then(self.patch.cmp(&other.patch))
    }
}

impl PartialOrd for ApiVersion {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Test result for contract validation
#[derive(Debug, Clone)]
pub struct TestResult {
    pub test_name: String,
    pub passed: bool,
    pub error_message: Option<String>,
    pub execution_time: std::time::Duration,
}

/// Contract test case definition
#[derive(Debug, Clone)]
pub struct ContractTestCase {
    pub name: String,
    pub input: serde_json::Value,
    pub expected_output: Option<serde_json::Value>,
    pub should_fail: bool,
}

/// Contract validation result
#[derive(Debug, Clone)]
pub enum ValidationResult {
    Valid,
    Invalid(Vec<String>),
}

impl ValidationResult {
    pub fn is_valid(&self) -> bool {
        matches!(self, ValidationResult::Valid)
    }

    pub fn errors(&self) -> Vec<String> {
        match self {
            ValidationResult::Valid => vec![],
            ValidationResult::Invalid(errors) => errors.clone(),
        }
    }
}

/// Contract metadata for documentation and discovery
#[derive(Debug, Clone)]
pub struct ContractMetadata {
    pub name: String,
    pub description: String,
    pub version: ApiVersion,
    pub author: String,
    pub last_updated: chrono::DateTime<chrono::Utc>,
    pub tags: Vec<String>,
}

impl ContractMetadata {
    pub fn new(name: &str, description: &str, version: ApiVersion) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            version,
            author: env!("CARGO_PKG_AUTHORS").to_string(),
            last_updated: chrono::Utc::now(),
            tags: vec![],
        }
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }
}

/// Contract registry for managing multiple contracts
/// Note: Due to associated types in ApiContract, we use concrete contract types
pub struct ContractRegistry {
    class_builder_contracts: std::collections::HashMap<String, crate::contracts::class_builder::ClassBuilderContract>,
    css_generator_contracts: std::collections::HashMap<String, crate::contracts::css_generator::CssGeneratorContract>,
    theme_contracts: std::collections::HashMap<String, crate::contracts::theme::ThemeContract>,
    validation_contracts: std::collections::HashMap<String, crate::contracts::validation::ValidationContract>,
    metadata: std::collections::HashMap<String, ContractMetadata>,
}

impl ContractRegistry {
    pub fn new() -> Self {
        Self {
            class_builder_contracts: std::collections::HashMap::new(),
            css_generator_contracts: std::collections::HashMap::new(),
            theme_contracts: std::collections::HashMap::new(),
            validation_contracts: std::collections::HashMap::new(),
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Register a class builder contract
    pub fn register_class_builder(&mut self, name: String, contract: crate::contracts::class_builder::ClassBuilderContract, metadata: ContractMetadata) {
        self.class_builder_contracts.insert(name.clone(), contract);
        self.metadata.insert(name, metadata);
    }

    /// Register a CSS generator contract
    pub fn register_css_generator(&mut self, name: String, contract: crate::contracts::css_generator::CssGeneratorContract, metadata: ContractMetadata) {
        self.css_generator_contracts.insert(name.clone(), contract);
        self.metadata.insert(name, metadata);
    }

    /// Register a theme contract
    pub fn register_theme(&mut self, name: String, contract: crate::contracts::theme::ThemeContract, metadata: ContractMetadata) {
        self.theme_contracts.insert(name.clone(), contract);
        self.metadata.insert(name, metadata);
    }

    /// Register a validation contract
    pub fn register_validation(&mut self, name: String, contract: crate::contracts::validation::ValidationContract, metadata: ContractMetadata) {
        self.validation_contracts.insert(name.clone(), contract);
        self.metadata.insert(name, metadata);
    }

    /// Get a class builder contract
    pub fn get_class_builder(&self, name: &str) -> Option<&crate::contracts::class_builder::ClassBuilderContract> {
        self.class_builder_contracts.get(name)
    }

    /// Get a CSS generator contract
    pub fn get_css_generator(&self, name: &str) -> Option<&crate::contracts::css_generator::CssGeneratorContract> {
        self.css_generator_contracts.get(name)
    }

    /// Get a theme contract
    pub fn get_theme(&self, name: &str) -> Option<&crate::contracts::theme::ThemeContract> {
        self.theme_contracts.get(name)
    }

    /// Get a validation contract
    pub fn get_validation(&self, name: &str) -> Option<&crate::contracts::validation::ValidationContract> {
        self.validation_contracts.get(name)
    }

    pub fn get_metadata(&self, name: &str) -> Option<&ContractMetadata> {
        self.metadata.get(name)
    }

    /// Generic contract getter that checks if a contract exists
    pub fn get_contract(&self, name: &str) -> Option<bool> {
        if self.class_builder_contracts.contains_key(name) {
            Some(true)
        } else if self.css_generator_contracts.contains_key(name) {
            Some(true)
        } else if self.theme_contracts.contains_key(name) {
            Some(true)
        } else if self.validation_contracts.contains_key(name) {
            Some(true)
        } else {
            None
        }
    }

    pub fn list_contracts(&self) -> Vec<String> {
        let mut contracts = Vec::new();
        contracts.extend(self.class_builder_contracts.keys().cloned());
        contracts.extend(self.css_generator_contracts.keys().cloned());
        contracts.extend(self.theme_contracts.keys().cloned());
        contracts.extend(self.validation_contracts.keys().cloned());
        contracts
    }

    pub fn validate_all(&self) -> Vec<(String, ValidationResult)> {
        let mut results = Vec::new();

        for (name, contract) in &self.class_builder_contracts {
            // Basic validation - in real implementation would call contract methods
            results.push((name.clone(), ValidationResult::Valid));
        }

        for (name, contract) in &self.css_generator_contracts {
            results.push((name.clone(), ValidationResult::Valid));
        }

        for (name, contract) in &self.theme_contracts {
            results.push((name.clone(), ValidationResult::Valid));
        }

        for (name, contract) in &self.validation_contracts {
            results.push((name.clone(), ValidationResult::Valid));
        }

        results
    }
}

impl Default for ContractRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_version_compatibility() {
        let v1_0_0 = ApiVersion::new(1, 0, 0);
        let v1_1_0 = ApiVersion::new(1, 1, 0);
        let v2_0_0 = ApiVersion::new(2, 0, 0);

        assert!(v1_0_0.is_compatible_with(&v1_1_0));
        assert!(!v1_0_0.is_compatible_with(&v2_0_0));
    }

    #[test]
    fn api_version_ordering() {
        let v1_0_0 = ApiVersion::new(1, 0, 0);
        let v1_1_0 = ApiVersion::new(1, 1, 0);
        let v2_0_0 = ApiVersion::new(2, 0, 0);

        assert!(v1_0_0 < v1_1_0);
        assert!(v1_1_0 < v2_0_0);
        assert!(v2_0_0 > v1_0_0);
    }

    #[test]
    fn contract_registry_operations() {
        let mut registry = ContractRegistry::new();

        // Test empty registry
        assert!(registry.list_contracts().is_empty());
        assert!(registry.get_contract("nonexistent").is_none());

        // Note: Full registry testing would require concrete contract implementations
        // This is tested in integration tests with actual contracts
    }
}
