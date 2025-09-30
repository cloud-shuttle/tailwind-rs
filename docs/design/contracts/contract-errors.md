# 🚨 Contract Errors Design

## 📋 Overview

**File**: `contracts/core/errors.rs` (80 lines)
**Purpose**: Define comprehensive error types for contract violations
**Status**: Ready for implementation

## 🎯 Error Taxonomy

### Primary Error Categories

#### 1. Input Validation Errors
```rust
pub enum ContractError {
    /// Input data violates contract requirements
    InvalidInput(String),

    /// Input data is structurally invalid
    MalformedInput {
        field: String,
        reason: String,
    },

    /// Input data violates business rules
    BusinessRuleViolation {
        rule: String,
        details: String,
    },
}
```

#### 2. Processing Errors
```rust
pub enum ContractError {
    /// Internal processing failure
    ProcessingError(String),

    /// External dependency failure
    DependencyError {
        dependency: String,
        error: String,
    },

    /// Resource exhaustion
    ResourceExhausted {
        resource: String,
        limit: u64,
        used: u64,
    },
}
```

#### 3. Output Validation Errors
```rust
pub enum ContractError {
    /// Output data violates contract guarantees
    InvalidOutput(String),

    /// Output incomplete or missing required fields
    IncompleteOutput {
        missing_fields: Vec<String>,
    },

    /// Output inconsistent with input promises
    InconsistentOutput {
        expected: String,
        actual: String,
    },
}
```

#### 4. Compatibility Errors
```rust
pub enum ContractError {
    /// Backward compatibility violation
    BackwardCompatibilityViolation(String),

    /// Version incompatibility
    VersionMismatch {
        expected: ApiVersion,
        actual: ApiVersion,
    },

    /// Breaking API change detected
    BreakingChange {
        component: String,
        change_type: BreakingChangeType,
    },
}
```

## 🏗️ Implementation Structure

### 1. Core Error Enum
```rust
// contracts/core/errors.rs
#[derive(Debug, Clone, PartialEq)]
pub enum ContractError {
    // Input validation errors
    InvalidInput(String),
    MalformedInput { field: String, reason: String },
    BusinessRuleViolation { rule: String, details: String },

    // Processing errors
    ProcessingError(String),
    DependencyError { dependency: String, error: String },
    ResourceExhausted { resource: String, limit: u64, used: u64 },

    // Output validation errors
    InvalidOutput(String),
    IncompleteOutput { missing_fields: Vec<String> },
    InconsistentOutput { expected: String, actual: String },

    // Compatibility errors
    BackwardCompatibilityViolation(String),
    VersionMismatch { expected: ApiVersion, actual: ApiVersion },
    BreakingChange { component: String, change_type: BreakingChangeType },
}
```

### 2. Supporting Types
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum BreakingChangeType {
    RemovedMethod,
    ChangedSignature,
    ChangedBehavior,
    RemovedField,
    ChangedFieldType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ApiVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}
```

### 3. Standard Trait Implementations
```rust
impl std::fmt::Display for ContractError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContractError::InvalidInput(msg) =>
                write!(f, "Invalid input: {}", msg),
            ContractError::MalformedInput { field, reason } =>
                write!(f, "Malformed input in field '{}': {}", field, reason),
            // ... other variants
        }
    }
}

impl std::error::Error for ContractError {}
```

## 🔄 Error Conversion Traits

### From Standard Library Errors
```rust
impl From<std::io::Error> for ContractError {
    fn from(err: std::io::Error) -> Self {
        ContractError::ProcessingError(format!("IO error: {}", err))
    }
}

impl From<serde_json::Error> for ContractError {
    fn from(err: serde_json::Error) -> Self {
        ContractError::MalformedInput {
            field: "json".to_string(),
            reason: err.to_string(),
        }
    }
}
```

### From Domain Errors
```rust
impl From<TailwindError> for ContractError {
    fn from(err: TailwindError) -> Self {
        match err {
            TailwindError::InvalidClassName(name) =>
                ContractError::InvalidInput(format!("Invalid class name: {}", name)),
            TailwindError::CssGenerationError(msg) =>
                ContractError::ProcessingError(format!("CSS generation failed: {}", msg)),
            // ... other mappings
        }
    }
}
```

## 📊 Error Context and Enrichment

### Context-Aware Errors
```rust
pub struct ErrorContext {
    pub operation: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub input_size: Option<usize>,
    pub processing_time: Option<std::time::Duration>,
}

impl ContractError {
    pub fn with_context(self, context: ErrorContext) -> ContextualError {
        ContextualError {
            error: self,
            context,
        }
    }
}

pub struct ContextualError {
    pub error: ContractError,
    pub context: ErrorContext,
}
```

### Error Chains
```rust
impl ContractError {
    pub fn chain(self, additional_info: String) -> ChainedError {
        ChainedError {
            errors: vec![self],
            additional_info,
        }
    }
}

pub struct ChainedError {
    pub errors: Vec<ContractError>,
    pub additional_info: String,
}

impl ChainedError {
    pub fn chain(mut self, error: ContractError) -> Self {
        self.errors.push(error);
        self
    }
}
```

## 🧪 Testing Strategy

### Error Creation Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_creation() {
        let err = ContractError::InvalidInput("test".to_string());
        assert_eq!(err.to_string(), "Invalid input: test");
    }

    #[test]
    fn error_conversion() {
        let tailwind_err = TailwindError::InvalidClassName("invalid".to_string());
        let contract_err: ContractError = tailwind_err.into();
        assert!(matches!(contract_err, ContractError::InvalidInput(_)));
    }
}
```

### Property-Based Testing
```rust
#[cfg(test)]
mod property_tests {
    use proptest::prelude::*;
    use super::*;

    proptest! {
        #[test]
        fn error_display_roundtrip(err in any::<ContractError>()) {
            let display = err.to_string();
            assert!(!display.is_empty());
        }
    }
}
```

## 📈 Error Metrics and Monitoring

### Error Aggregation
```rust
pub struct ErrorMetrics {
    pub error_counts: HashMap<String, u64>,
    pub error_rates: HashMap<String, f64>,
    pub recent_errors: VecDeque<ContractError>,
}

impl ErrorMetrics {
    pub fn record_error(&mut self, error: &ContractError) {
        let key = format!("{:?}", error);
        *self.error_counts.entry(key).or_insert(0) += 1;
        self.recent_errors.push_back(error.clone());

        if self.recent_errors.len() > 1000 {
            self.recent_errors.pop_front();
        }
    }
}
```

### Error Thresholds
```rust
pub struct ErrorThresholds {
    pub max_errors_per_minute: u32,
    pub max_error_rate: f64,
    pub alert_on_breaking_changes: bool,
}

impl ErrorThresholds {
    pub fn check_thresholds(&self, metrics: &ErrorMetrics) -> Vec<ThresholdViolation> {
        // Check error rates and thresholds
    }
}
```

## 🚨 Error Recovery Strategies

### Automatic Recovery
```rust
pub trait RecoverableError {
    fn can_recover(&self) -> bool;
    fn recovery_strategy(&self) -> RecoveryStrategy;
}

pub enum RecoveryStrategy {
    Retry { max_attempts: u32, backoff: std::time::Duration },
    Fallback { alternative: Box<dyn Fn() -> Result<(), ContractError>> },
    Skip { reason: String },
    Fail { critical: bool },
}

impl ContractError {
    pub fn recovery_strategy(&self) -> RecoveryStrategy {
        match self {
            ContractError::ProcessingError(_) =>
                RecoveryStrategy::Retry { max_attempts: 3, backoff: Duration::from_millis(100) },
            ContractError::BreakingChange { .. } =>
                RecoveryStrategy::Fail { critical: true },
            // ... other strategies
        }
    }
}
```

## 📋 Migration and Compatibility

### Error Code Stability
- **Error variants**: Never remove, only add new ones
- **String formats**: Maintain backward compatibility
- **Error codes**: Assign numeric codes for external systems

### Version Compatibility
```rust
impl ContractError {
    pub fn error_code(&self) -> u32 {
        match self {
            ContractError::InvalidInput(_) => 1001,
            ContractError::ProcessingError(_) => 2001,
            // ... stable error codes
        }
    }
}
```

## 🎯 Success Criteria

### Functional Requirements
- ✅ All error types covered
- ✅ Proper error conversion
- ✅ Context preservation
- ✅ Recovery strategies

### Non-Functional Requirements
- ✅ File size: <80 lines
- ✅ Performance: Minimal overhead
- ✅ Memory safety: No unsafe code
- ✅ Thread safety: Send + Sync

### Quality Metrics
- ✅ Test coverage: >95%
- ✅ Documentation: Complete error docs
- ✅ Error codes: Stable and documented
- ✅ Monitoring: Integrated metrics

---

*Contract Errors Design - September 2025*
*File: contracts/core/errors.rs (80 lines)*
*Status: Ready for implementation*
*Risk Level: Low (error handling only)*
