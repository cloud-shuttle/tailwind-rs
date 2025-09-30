//! Validation API Contract
//!
//! Defines the contract for the Validation API, ensuring stable and reliable
//! validation of Tailwind classes and configurations.

use crate::error::TailwindError;
use super::core::{traits::*, errors::*};

/// Validation API contract implementation
#[derive(Debug, Clone)]
pub struct ValidationContract {
    version: ApiVersion,
    validation_rules: Vec<String>,
    capabilities: ValidationCapabilities,
}

#[derive(Debug, Clone)]
pub struct ValidationCapabilities {
    pub supports_class_name_validation: bool,
    pub supports_property_conflicts: bool,
    pub supports_responsive_validation: bool,
    pub supports_custom_variant_validation: bool,
    pub supports_theme_validation: bool,
    pub strict_mode_available: bool,
    pub max_classes_per_validation: usize,
}

impl Default for ValidationCapabilities {
    fn default() -> Self {
        Self {
            supports_class_name_validation: true,
            supports_property_conflicts: true,
            supports_responsive_validation: true,
            supports_custom_variant_validation: false, // Not yet implemented
            supports_theme_validation: false, // Not yet implemented
            strict_mode_available: true,
            max_classes_per_validation: 1000,
        }
    }
}

impl ValidationContract {
    /// Create a new Validation contract with specified version
    pub fn new(version: ApiVersion) -> Self {
        Self {
            version,
            validation_rules: vec![
                "class_name_format".to_string(),
                "property_conflicts".to_string(),
                "responsive_breakpoints".to_string(),
                "custom_variant_syntax".to_string(),
                "theme_consistency".to_string(),
            ],
            capabilities: ValidationCapabilities::default(),
        }
    }

    /// Create a contract with custom capabilities
    pub fn with_capabilities(version: ApiVersion, capabilities: ValidationCapabilities) -> Self {
        Self {
            version,
            validation_rules: vec![
                if capabilities.supports_class_name_validation { Some("class_name_format".to_string()) } else { None },
                if capabilities.supports_property_conflicts { Some("property_conflicts".to_string()) } else { None },
                if capabilities.supports_responsive_validation { Some("responsive_breakpoints".to_string()) } else { None },
                if capabilities.supports_custom_variant_validation { Some("custom_variant_syntax".to_string()) } else { None },
                if capabilities.supports_theme_validation { Some("theme_consistency".to_string()) } else { None },
            ].into_iter().flatten().collect(),
            capabilities,
        }
    }

    /// Get validation rules
    pub fn validation_rules(&self) -> &[String] {
        &self.validation_rules
    }

    /// Get capabilities
    pub fn capabilities(&self) -> &ValidationCapabilities {
        &self.capabilities
    }

    /// Check if rule is supported
    pub fn supports_rule(&self, rule: &str) -> bool {
        self.validation_rules.contains(&rule.to_string())
    }

    /// Validate class name format
    fn validate_class_name(&self, class: &str) -> Result<(), ContractError> {
        if !self.capabilities.supports_class_name_validation {
            return Ok(());
        }

        if class.trim().is_empty() {
            return Err(ContractError::InvalidInput("Empty class name".to_string()));
        }

        // Check for valid Tailwind class format
        if !class.chars().all(|c| c.is_alphanumeric() || c == '-' || c == ':' || c == '/' || c == '[' || c == ']' || c == '(' || c == ')' || c == '.' || c == '@') {
            return Err(ContractError::InvalidInput(format!("Invalid class name format: {}", class)));
        }

        // Check for balanced brackets in arbitrary values
        let bracket_count = class.chars().fold(0, |count, c| {
            match c {
                '[' => count + 1,
                ']' => count - 1,
                _ => count,
            }
        });

        if bracket_count != 0 {
            return Err(ContractError::InvalidInput("Unbalanced brackets in class name".to_string()));
        }

        Ok(())
    }

    /// Validate responsive classes
    fn validate_responsive_classes(&self, classes: &[String]) -> Result<(), ContractError> {
        if !self.capabilities.supports_responsive_validation {
            return Ok(());
        }

        let valid_breakpoints = ["sm", "md", "lg", "xl", "2xl"];

        for class in classes {
            if class.contains(':') {
                let parts: Vec<&str> = class.split(':').collect();
                if parts.len() >= 2 {
                    let breakpoint = parts[0];
                    if !valid_breakpoints.contains(&breakpoint) {
                        // Allow custom breakpoints, just warn
                        // In strict mode, this would be an error
                        if self.capabilities.strict_mode_available {
                            return Err(ContractError::InvalidInput(
                                format!("Invalid breakpoint: {} in class: {}", breakpoint, class)
                            ));
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Check for property conflicts
    fn check_property_conflicts(&self, classes: &[String]) -> Result<(), ContractError> {
        if !self.capabilities.supports_property_conflicts {
            return Ok(());
        }

        // Basic conflict detection - check for obvious conflicts
        let mut display_values = Vec::new();
        let mut position_values = Vec::new();

        for class in classes {
            if class.starts_with("block") || class.starts_with("inline") || class.starts_with("flex") || class.starts_with("grid") {
                display_values.push(class.clone());
            }
            if class.starts_with("static") || class.starts_with("relative") || class.starts_with("absolute") || class.starts_with("fixed") {
                position_values.push(class.clone());
            }
        }

        // In a real implementation, this would do sophisticated conflict detection
        // For now, we just check for multiple conflicting properties

        Ok(())
    }
}

impl ApiContract for ValidationContract {
    type Input = ValidationInput;
    type Output = ValidationResult;
    type Error = TailwindError;

    fn validate_input(&self, input: &Self::Input) -> Result<(), ContractError> {
        // Validate classes to check
        for class in &input.classes {
            self.validate_class_name(class)?;
        }

        // Check size limits
        if input.classes.len() > self.capabilities.max_classes_per_validation {
            return Err(ContractError::ResourceExhausted {
                resource: "classes_for_validation".to_string(),
                limit: self.capabilities.max_classes_per_validation as u64,
                used: input.classes.len() as u64,
            });
        }

        // Validate responsive classes if requested
        if input.validate_responsive {
            self.validate_responsive_classes(&input.classes)?;
        }

        // Check property conflicts if requested
        if input.check_conflicts {
            self.check_property_conflicts(&input.classes)?;
        }

        Ok(())
    }

    fn process(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // In a real implementation, this would use ValidationEngine
        // For now, we perform basic validation

        for class in &input.classes {
            // Basic validation - check if class looks reasonable
            if class.contains("  ") {
                errors.push(format!("Class contains double spaces: {}", class));
            }

            if class.len() > 200 {
                warnings.push(format!("Very long class name: {}", class));
            }
        }

        // Check for duplicates if requested
        if input.check_duplicates {
            let mut seen = std::collections::HashSet::new();
            for class in &input.classes {
                if !seen.insert(class) {
                    warnings.push(format!("Duplicate class: {}", class));
                }
            }
        }

        let is_valid = errors.is_empty();

        Ok(ValidationResult {
            is_valid,
            errors,
            warnings,
        })
    }

    fn validate_output(&self, output: &Self::Output) -> Result<(), ContractError> {
        // Validate that the result structure is consistent
        if !output.is_valid && output.errors.is_empty() {
            return Err(ContractError::InvalidOutput(
                "Invalid validation result: errors should be present when is_valid is false".to_string()
            ));
        }

        // Check that all errors are non-empty strings
        for error in &output.errors {
            if error.trim().is_empty() {
                return Err(ContractError::InvalidOutput(
                    "Validation result contains empty error message".to_string()
                ));
            }
        }

        // Check that all warnings are non-empty strings
        for warning in &output.warnings {
            if warning.trim().is_empty() {
                return Err(ContractError::InvalidOutput(
                    "Validation result contains empty warning message".to_string()
                ));
            }
        }

        Ok(())
    }
}

impl VersionedContract for ValidationContract {
    fn version(&self) -> ApiVersion {
        self.version.clone()
    }

    fn supported_versions(&self) -> Vec<ApiVersion> {
        match self.version {
            ApiVersion { major: 1, minor: 0, patch: 0 } => vec![
                ApiVersion::new(1, 0, 0),
            ],
            ApiVersion { major: 1, minor: 1, patch: 0 } => vec![
                ApiVersion::new(1, 0, 0),
                ApiVersion::new(1, 1, 0),
            ],
            ApiVersion { major: 2, minor: 0, patch: 0 } => vec![
                ApiVersion::new(1, 0, 0),
                ApiVersion::new(1, 1, 0),
                ApiVersion::new(2, 0, 0),
            ],
            _ => vec![self.version.clone()],
        }
    }

    fn is_backward_compatible(&self, other: &ApiVersion) -> bool {
        self.version.is_compatible_with(other)
    }
}

/// Input structure for Validation contract
#[derive(Debug, Clone)]
pub struct ValidationInput {
    pub classes: Vec<String>,
    pub strict_mode: bool,
    pub validate_responsive: bool,
    pub check_conflicts: bool,
    pub check_duplicates: bool,
}

impl ValidationInput {
    /// Create a new validation input
    pub fn new(classes: Vec<String>) -> Self {
        Self {
            classes,
            strict_mode: false,
            validate_responsive: true,
            check_conflicts: true,
            check_duplicates: true,
        }
    }

    /// Enable strict mode
    pub fn strict(mut self) -> Self {
        self.strict_mode = true;
        self
    }

    /// Configure responsive validation
    pub fn with_responsive_validation(mut self, enabled: bool) -> Self {
        self.validate_responsive = enabled;
        self
    }

    /// Configure conflict checking
    pub fn with_conflict_checking(mut self, enabled: bool) -> Self {
        self.check_conflicts = enabled;
        self
    }

    /// Configure duplicate checking
    pub fn with_duplicate_checking(mut self, enabled: bool) -> Self {
        self.check_duplicates = enabled;
        self
    }
}

impl Default for ValidationInput {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

/// Output structure for Validation contract
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl ValidationResult {
    /// Create a valid result
    pub fn valid() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// Create an invalid result with errors
    pub fn invalid(errors: Vec<String>) -> Self {
        Self {
            is_valid: false,
            errors,
            warnings: Vec::new(),
        }
    }

    /// Add a warning to the result
    pub fn with_warning(mut self, warning: String) -> Self {
        self.warnings.push(warning);
        self
    }

    /// Add an error to the result
    pub fn with_error(mut self, error: String) -> Self {
        self.errors.push(error);
        self.is_valid = false;
        self
    }

    /// Get total number of issues
    pub fn total_issues(&self) -> usize {
        self.errors.len() + self.warnings.len()
    }

    /// Check if result has any issues
    pub fn has_issues(&self) -> bool {
        !self.errors.is_empty() || !self.warnings.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contract_creation() {
        let contract = ValidationContract::new(ApiVersion::new(1, 0, 0));
        assert_eq!(contract.version(), ApiVersion::new(1, 0, 0));
        assert!(contract.supports_rule("class_name_format"));
        assert!(contract.capabilities().supports_class_name_validation);
    }

    #[test]
    fn input_validation() {
        let contract = ValidationContract::new(ApiVersion::new(1, 0, 0));

        // Valid input
        let input = ValidationInput::new(vec!["bg-red-500".to_string(), "text-white".to_string()]);
        assert!(contract.validate_input(&input).is_ok());

        // Invalid input - empty class
        let input = ValidationInput::new(vec!["".to_string()]);
        assert!(contract.validate_input(&input).is_err());
    }

    #[test]
    fn class_name_validation() {
        let contract = ValidationContract::new(ApiVersion::new(1, 0, 0));

        // Valid class names
        assert!(contract.validate_class_name("bg-red-500").is_ok());
        assert!(contract.validate_class_name("hover:bg-blue-500").is_ok());
        assert!(contract.validate_class_name("md:text-lg").is_ok());
        assert!(contract.validate_class_name("bg-[#ff0000]").is_ok());

        // Invalid class names
        assert!(contract.validate_class_name("").is_err());
        assert!(contract.validate_class_name("bg red").is_err()); // spaces
        assert!(contract.validate_class_name("bg[red").is_err()); // unbalanced brackets
    }

    #[test]
    fn validation_result_operations() {
        let valid_result = ValidationResult::valid();
        assert!(valid_result.is_valid);
        assert!(!valid_result.has_issues());

        let invalid_result = ValidationResult::invalid(vec!["error1".to_string()])
            .with_warning("warning1".to_string())
            .with_error("error2".to_string());

        assert!(!invalid_result.is_valid);
        assert!(invalid_result.has_issues());
        assert_eq!(invalid_result.total_issues(), 3);
        assert_eq!(invalid_result.errors.len(), 2);
        assert_eq!(invalid_result.warnings.len(), 1);
    }

    #[test]
    fn validation_input_configuration() {
        let input = ValidationInput::new(vec!["test".to_string()])
            .strict()
            .with_responsive_validation(false)
            .with_conflict_checking(false)
            .with_duplicate_checking(false);

        assert!(input.strict_mode);
        assert!(!input.validate_responsive);
        assert!(!input.check_conflicts);
        assert!(!input.check_duplicates);
    }

    #[test]
    fn capabilities_validation() {
        let capabilities = ValidationCapabilities {
            supports_class_name_validation: true,
            supports_property_conflicts: false,
            supports_responsive_validation: true,
            supports_custom_variant_validation: false,
            supports_theme_validation: false,
            strict_mode_available: true,
            max_classes_per_validation: 100,
        };

        let contract = ValidationContract::with_capabilities(ApiVersion::new(1, 0, 0), capabilities);

        assert!(contract.supports_rule("class_name_format"));
        assert!(!contract.supports_rule("property_conflicts"));
    }

    #[test]
    fn version_compatibility() {
        let contract = ValidationContract::new(ApiVersion::new(1, 1, 0));
        assert!(contract.is_backward_compatible(&ApiVersion::new(1, 0, 0)));
        assert!(!contract.is_backward_compatible(&ApiVersion::new(2, 0, 0)));
    }
}
