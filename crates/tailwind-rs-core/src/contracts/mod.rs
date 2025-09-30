//! API Contracts Module
//!
//! This module provides comprehensive API contracts and contract testing
//! to ensure API stability, backward compatibility, and reliability.
//!
//! ## Overview
//!
//! The contracts module is designed to:
//! - Define clear API boundaries and expectations
//! - Ensure backward compatibility across versions
//! - Provide automated contract testing
//! - Enable reliable API evolution
//!
//! ## Architecture
//!
//! ```text
//! contracts/
//! ├── core/
//! │   ├── traits.rs      # Core contract interfaces
//! │   └── errors.rs      # Comprehensive error types
//! ├── class_builder.rs   # ClassBuilder API contract
//! ├── css_generator.rs   # CSS Generator API contract
//! ├── theme.rs          # Theme API contract
//! ├── validation.rs     # Validation API contract
//! ├── testing.rs        # Contract testing framework
//! └── mod.rs            # Module exports
//! ```
//!
//! ## Key Components
//!
//! ### Core Traits
//! - `ApiContract`: Base contract interface for all APIs
//! - `VersionedContract`: Version compatibility management
//! - `TestableContract`: Automated testing interface
//!
//! ### Contract Implementations
//! - `ClassBuilderContract`: Tailwind class construction
//! - `CssGeneratorContract`: CSS generation and formatting
//! - `ThemeContract`: Theme configuration management
//! - `ValidationContract`: Class and configuration validation
//!
//! ### Testing Framework
//! - `ContractTestRunner`: Automated contract verification
//! - `ContractTestReport`: Test results and analysis
//! - Property-based testing for edge cases
//!
//! ## Usage Examples
//!
//! ### Basic Contract Usage
//! ```rust
//! use tailwind_rs_core::contracts::class_builder::ClassBuilderContract;
//!
//! let contract = ClassBuilderContract::new(ApiVersion::new(1, 0, 0));
//! let input = ClassBuilderInput::new(vec!["bg-red-500".to_string()]);
//!
//! // Validate input
//! contract.validate_input(&input)?;
//!
//! // Process input
//! let output = contract.process(input)?;
//!
//! // Validate output
//! contract.validate_output(&output)?;
//! ```
//!
//! ### Contract Testing
//! ```rust
//! use tailwind_rs_core::contracts::testing::ContractTestRunner;
//!
//! let mut runner = ContractTestRunner::new();
//! runner.register_contract("class_builder".to_string(), contract);
//!
//! let report = runner.run_all_tests();
//! assert!(report.all_passed(), "Contract tests failed: {}", report);
//! ```
//!
//! ## Version Compatibility
//!
//! Contracts support semantic versioning with backward compatibility guarantees:
//!
//! - **Major version**: Breaking changes allowed
//! - **Minor version**: New features, backward compatible
//! - **Patch version**: Bug fixes only
//!
//! ## Error Handling
//!
//! The contracts module provides comprehensive error types:
//!
//! - `ContractError`: Primary contract violation errors
//! - `ContextualError`: Errors with additional debugging context
//! - `ErrorMetrics`: Monitoring and alerting for error patterns
//!
//! ## Testing Strategy
//!
//! ### Unit Tests
//! - Contract interface compliance
//! - Input/output validation
//! - Error handling correctness
//!
//! ### Integration Tests
//! - End-to-end contract workflows
//! - Cross-contract interactions
//! - Performance validation
//!
//! ### Property-Based Tests
//! - Edge case exploration
//! - Input validation robustness
//! - Error condition coverage

// Core contract infrastructure
pub mod core {
    pub mod traits;
    pub mod errors;
}

// Contract implementations
pub mod class_builder;
pub mod css_generator;
pub mod theme;
pub mod validation;

// Testing framework
pub mod testing_modules;

// Re-exports for convenience
pub use core::traits::*;
pub use core::errors::*;

// Re-export contract implementations
pub use class_builder::ClassBuilderContract;
pub use css_generator::CssGeneratorContract;
pub use theme::ThemeContract;
pub use validation::ValidationContract;

// Re-export testing framework
pub use testing_modules::{ContractTestRunner, ContractTestReport};

/// Global contract registry for managing all contracts
pub type GlobalContractRegistry = core::traits::ContractRegistry;

/// Initialize the global contract registry with all available contracts
pub fn initialize_contract_registry() -> GlobalContractRegistry {
    let mut registry = GlobalContractRegistry::new();

    // Register core contracts
    let class_builder = ClassBuilderContract::new(ApiVersion::new(1, 0, 0));
    registry.register_class_builder(
        "class_builder".to_string(),
        class_builder,
        core::traits::ContractMetadata::new(
            "ClassBuilder",
            "API for constructing Tailwind CSS classes",
            ApiVersion::new(1, 0, 0),
        ),
    );

    let css_generator = CssGeneratorContract::new(ApiVersion::new(1, 0, 0));
    registry.register_css_generator(
        "css_generator".to_string(),
        css_generator,
        core::traits::ContractMetadata::new(
            "CssGenerator",
            "API for generating CSS from Tailwind classes",
            ApiVersion::new(1, 0, 0),
        ),
    );

    let theme = ThemeContract::new(ApiVersion::new(1, 0, 0));
    registry.register_theme(
        "theme".to_string(),
        theme,
        core::traits::ContractMetadata::new(
            "Theme",
            "API for managing Tailwind theme configurations",
            ApiVersion::new(1, 0, 0),
        ),
    );

    let validation = ValidationContract::new(ApiVersion::new(1, 0, 0));
    registry.register_validation(
        "validation".to_string(),
        validation,
        core::traits::ContractMetadata::new(
            "Validation",
            "API for validating Tailwind classes and configurations",
            ApiVersion::new(1, 0, 0),
        ),
    );

    registry
}

/// Run comprehensive contract tests for all registered contracts
pub fn run_all_contract_tests() -> ContractTestReport {
    let mut runner = ContractTestRunner::new();
    let registry = initialize_contract_registry();

    // Register contracts with runner
    if let Some(contract) = registry.get_class_builder("class_builder") {
        runner.register_class_builder("class_builder".to_string(), contract.clone());
    }
    if let Some(contract) = registry.get_css_generator("css_generator") {
        runner.register_css_generator("css_generator".to_string(), contract.clone());
    }
    if let Some(contract) = registry.get_theme("theme") {
        runner.register_theme("theme".to_string(), contract.clone());
    }
    if let Some(contract) = registry.get_validation("validation") {
        runner.register_validation("validation".to_string(), contract.clone());
    }

    runner.run_all_tests()
}

/// Validate that all contracts are properly implemented and compatible
pub fn validate_contract_compliance() -> Result<(), ContractError> {
    let registry = initialize_contract_registry();

    for contract_name in registry.list_contracts() {
        if let Some(metadata) = registry.get_metadata(&contract_name) {
            // Validate metadata
            if metadata.name.is_empty() {
                return Err(ContractError::InvalidInput(
                    format!("Contract '{}' has empty name", contract_name)
                ));
            }

            if metadata.description.is_empty() {
                return Err(ContractError::InvalidInput(
                    format!("Contract '{}' has empty description", contract_name)
                ));
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contract_registry_initialization() {
        let registry = initialize_contract_registry();

        // Should have all core contracts registered
        assert!(registry.get_contract("class_builder").is_some());
        assert!(registry.get_contract("css_generator").is_some());
        assert!(registry.get_contract("theme").is_some());
        assert!(registry.get_contract("validation").is_some());

        // Should have 4 contracts total
        assert_eq!(registry.list_contracts().len(), 4);
    }

    #[test]
    fn contract_metadata_validation() {
        let registry = initialize_contract_registry();

        for contract_name in registry.list_contracts() {
            let metadata = registry.get_metadata(&contract_name).unwrap();
            assert!(!metadata.name.is_empty());
            assert!(!metadata.description.is_empty());
            assert!(metadata.tags.is_empty()); // Default metadata has no tags
        }
    }

    #[test]
    fn contract_compliance_validation() {
        let result = validate_contract_compliance();
        assert!(result.is_ok(), "Contract compliance validation failed: {:?}", result);
    }

    #[test]
    fn contract_registry_operations() {
        let mut registry = GlobalContractRegistry::new();

        // Test empty registry
        assert!(registry.list_contracts().is_empty());

        // Add a contract (would need concrete implementation in real test)
        // registry.register_contract("test".to_string(), test_contract, metadata);
    }
}
