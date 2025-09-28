# 🔒 API Contracts - Complete Implementation Guide

## Overview

The Tailwind-RS API Contracts system provides comprehensive validation, testing, and stability guarantees for all major APIs. This contract-based approach ensures API stability, backward compatibility, and reliable behavior across versions.

## 🏗️ Architecture

### Core Components

```rust
pub trait ApiContract {
    type Input;
    type Output;
    type Error;

    fn validate_input(&self, input: &Self::Input) -> Result<(), ContractError>;
    fn process(&self, input: Self::Input) -> Result<Self::Output, Self::Error>;
    fn validate_output(&self, output: &Self::Output) -> Result<(), ContractError>;
}
```

### Available Contracts

| Contract | Purpose | Input Type | Output Type |
|----------|---------|------------|-------------|
| `ClassBuilderContract` | Class building operations | `ClassBuilderInput` | `ClassSet` |
| `CssGeneratorContract` | CSS generation operations | `CssGeneratorInput` | `String` |
| `ThemeContract` | Theme configuration operations | `ThemeInput` | `ThemeConfig` |
| `ValidationContract` | Class validation operations | `ValidationInput` | `ValidationResult` |

## 📋 Contract Implementation Details

### ClassBuilderContract

**Purpose**: Validates and processes class building operations with guaranteed output format.

**Input Validation:**
- Class names cannot be empty
- Class names cannot contain spaces
- Breakpoint values must be valid enum variants

**Processing:**
- Builds `ClassSet` from input specifications
- Handles responsive, conditional, and custom classes
- Maintains type safety throughout

**Output Validation:**
- Ensures CSS classes string format is valid
- Checks for double spaces and invalid characters

### CssGeneratorContract

**Purpose**: Validates CSS generation operations with guaranteed CSS output.

**Input Validation:**
- CSS selectors cannot be empty
- CSS properties must be present for each rule
- Media queries must follow `@media` format

**Processing:**
- Generates CSS based on specified format (Regular, Minified, WithSourceMaps)
- Handles CSS rules and media queries
- Applies format-specific transformations

**Output Validation:**
- Ensures CSS contains proper structure (`{` and `}`)
- Validates basic CSS syntax

### ThemeContract

**Purpose**: Validates theme configuration operations with guaranteed theme structure.

**Input Validation:**
- Theme names cannot be empty
- Color values must follow valid CSS color formats
- Spacing values must be valid CSS units

**Processing:**
- Creates `ThemeConfig` from input specifications
- Validates and applies color palettes
- Sets up spacing scales

**Output Validation:**
- Ensures theme has required fields
- Validates theme structure integrity

### ValidationContract

**Purpose**: Validates class validation operations with guaranteed validation results.

**Input Validation:**
- Class names to validate cannot be empty

**Processing:**
- Performs comprehensive validation checks
- Returns structured validation results

**Output Validation:**
- Ensures valid validation result format
- Checks error/warning consistency

## 🧪 Contract Testing Framework

### Test Execution

```rust
use tailwind_rs_core::api_contracts::{ContractTester, TestCase};

let mut tester = ContractTester::new();
tester.add_test_case(TestCase {
    name: "class_builder_validation".to_string(),
    input: "test_input".to_string(),
    expected_output: "test_output".to_string(),
    should_fail: false,
});

let results = tester.run_tests()?;
// Access: results.passed_tests, results.failed_tests, results.total_tests
```

### Runtime Validation

```rust
use tailwind_rs_core::api_contracts::ContractValidator;

let mut validator = ContractValidator::new();
validator.add_contract("my_api".to_string(), my_contract);

// Enable/disable validation
validator.enable_validation();
validator.disable_validation();

// Validate API calls
validator.validate_call("my_api", input_data)?;

// Check violations
let violations = validator.get_violations();
```

## 🎯 Error Handling

### ContractError Types

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum ContractError {
    InvalidInput(String),           // Input validation failure
    InvalidOutput(String),          // Output validation failure
    ContractViolation(String),      // Contract rule violation
    BackwardCompatibilityViolation(String), // Breaking change detected
}
```

### Error Propagation

All contract methods return `Result<T, ContractError>` for comprehensive error handling:

```rust
match contract.validate_input(&input) {
    Ok(_) => println!("Input is valid"),
    Err(ContractError::InvalidInput(msg)) => eprintln!("Invalid input: {}", msg),
    Err(ContractError::ContractViolation(msg)) => eprintln!("Contract violation: {}", msg),
    Err(_) => eprintln!("Other validation error"),
}
```

## 🔄 Version Management

### API Versioning

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum ApiVersion {
    V1_0_0,
    V1_1_0,
    V2_0_0,
}
```

### Backward Compatibility

- Contracts specify their supported API version
- Version-specific validation rules ensure compatibility
- Breaking changes require new contract versions

## 📊 Performance Characteristics

### Validation Performance

- **Input Validation**: O(1) for most operations, O(n) for collection validation
- **Output Validation**: O(1) for format checks, O(m) for content validation
- **Contract Testing**: O(t) where t is number of test cases

### Memory Usage

- **Minimal Overhead**: Contracts add <1KB per API endpoint
- **Zero-Copy Validation**: Most validations work on references
- **Efficient Storage**: Contract storage uses minimal memory

## 🛠️ Usage Examples

### Basic Contract Usage

```rust
use tailwind_rs_core::api_contracts::*;

// Create and use a contract
let contract = ClassBuilderContract::new(ApiVersion::V2_0_0);
let input = ClassBuilderInput {
    classes: vec!["p-4".to_string(), "bg-blue-500".to_string()],
    responsive: vec![],
    conditional: vec![],
    custom: vec![],
};

// Full contract lifecycle
contract.validate_input(&input)?;
let output = contract.process(input)?;
contract.validate_output(&output)?;
```

### Advanced Testing

```rust
use tailwind_rs_core::api_contracts::{ContractTester, TestCase};

// Set up comprehensive testing
let mut tester = ContractTester::new();

// Add multiple test cases
tester.add_test_case(TestCase {
    name: "valid_classes".to_string(),
    input: "p-4 bg-blue-500".to_string(),
    expected_output: "valid".to_string(),
    should_fail: false,
});

tester.add_test_case(TestCase {
    name: "invalid_classes".to_string(),
    input: "invalid-class".to_string(),
    expected_output: "error".to_string(),
    should_fail: true,
});

// Run tests and get results
let results = tester.run_tests()?;
println!("Contract tests: {}/{} passed",
    results.passed_tests, results.total_tests);
```

### Runtime Monitoring

```rust
use tailwind_rs_core::api_contracts::ContractValidator;

// Set up runtime validation
let mut validator = ContractValidator::new();
validator.add_contract("class_builder".to_string(),
    ClassBuilderContract::new(ApiVersion::V2_0_0));

// Monitor API usage
for api_call in api_calls {
    if let Err(error) = validator.validate_call("class_builder", api_call.input) {
        validator.record_violation("class_builder".to_string(), error);
    }
}

// Report violations
for violation in validator.get_violations() {
    println!("API violation in {}: {}", violation.api_name, violation.error);
}
```

## 🔧 Integration Guidelines

### Adding New Contracts

1. **Implement ApiContract Trait**: Define Input, Output, and Error types
2. **Add Validation Logic**: Implement input/output validation methods
3. **Handle Processing**: Implement core business logic in `process` method
4. **Add Tests**: Create comprehensive test coverage for the contract
5. **Update Documentation**: Add contract to API reference

### Contract Maintenance

- **Version Bumps**: Update contract version for breaking changes
- **Validation Updates**: Modify validation rules as APIs evolve
- **Performance Monitoring**: Track contract validation performance
- **Error Analysis**: Monitor contract violations for API issues

## 📈 Benefits

### For Developers
- ✅ **Type Safety**: Compile-time guarantees about API usage
- ✅ **Clear Contracts**: Explicit input/output expectations
- ✅ **Better Errors**: Specific error messages for debugging
- ✅ **API Stability**: Contract-based versioning prevents breaking changes

### For Applications
- ✅ **Runtime Safety**: Optional runtime validation for dynamic inputs
- ✅ **Reliability**: Guaranteed behavior across API versions
- ✅ **Monitoring**: Contract violation tracking for production issues
- ✅ **Testing**: Comprehensive testing framework for API validation

### For Maintenance
- ✅ **Evolvability**: Clear upgrade paths with version management
- ✅ **Documentation**: Self-documenting APIs through contracts
- ✅ **Refactoring**: Safe refactoring with contract guarantees
- ✅ **Compatibility**: Backward compatibility validation
