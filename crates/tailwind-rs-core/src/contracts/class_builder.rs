//! Class Builder API Contract
//!
//! Defines the contract for the ClassBuilder API, ensuring stable and reliable
//! class construction and CSS generation.

use crate::classes::{ClassBuilder, ClassSet};
use crate::error::TailwindError;
use crate::responsive::Breakpoint;
use super::core::{traits::*, errors::*};

/// ClassBuilder API contract implementation
#[derive(Debug, Clone)]
pub struct ClassBuilderContract {
    version: ApiVersion,
    supported_methods: Vec<String>,
    capabilities: ClassBuilderCapabilities,
}

#[derive(Debug, Clone)]
pub struct ClassBuilderCapabilities {
    pub supports_responsive: bool,
    pub supports_conditional: bool,
    pub supports_custom: bool,
    pub supports_arbitrary_values: bool,
    pub max_classes_per_build: usize,
}

impl Default for ClassBuilderCapabilities {
    fn default() -> Self {
        Self {
            supports_responsive: true,
            supports_conditional: true,
            supports_custom: true,
            supports_arbitrary_values: true,
            max_classes_per_build: 1000,
        }
    }
}

impl ClassBuilderContract {
    /// Create a new ClassBuilder contract with specified version
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
                "validate".to_string(),
            ],
            capabilities: ClassBuilderCapabilities::default(),
        }
    }

    /// Create a contract with custom capabilities
    pub fn with_capabilities(version: ApiVersion, capabilities: ClassBuilderCapabilities) -> Self {
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
                "validate".to_string(),
            ],
            capabilities,
        }
    }

    /// Get supported methods
    pub fn supported_methods(&self) -> &[String] {
        &self.supported_methods
    }

    /// Get capabilities
    pub fn capabilities(&self) -> &ClassBuilderCapabilities {
        &self.capabilities
    }

    /// Check if method is supported
    pub fn supports_method(&self, method: &str) -> bool {
        self.supported_methods.contains(&method.to_string())
    }

    /// Validate class name format
    fn validate_class_name(&self, class: &str) -> Result<(), ContractError> {
        if class.is_empty() {
            return Err(ContractError::InvalidInput("Empty class name".to_string()));
        }

        if class.contains(" ") {
            return Err(ContractError::InvalidInput(
                "Class name contains spaces".to_string(),
            ));
        }

        // Check for valid Tailwind class format
        if !class.chars().all(|c| c.is_alphanumeric() || c == '-' || c == ':' || c == '/' || c == '[' || c == ']' || c == '(' || c == ')') {
            return Err(ContractError::InvalidInput(
                format!("Invalid class name format: {}", class)
            ));
        }

        Ok(())
    }

    /// Validate breakpoint
    fn validate_breakpoint(&self, breakpoint: &Breakpoint) -> Result<(), ContractError> {
        if !self.capabilities.supports_responsive {
            return Err(ContractError::BusinessRuleViolation {
                rule: "responsive_support".to_string(),
                details: "Responsive classes not supported by this contract".to_string(),
            });
        }

        match breakpoint {
            Breakpoint::Base | Breakpoint::Sm | Breakpoint::Md | Breakpoint::Lg | Breakpoint::Xl | Breakpoint::Xl2 => Ok(()),
            _ => Err(ContractError::InvalidInput(format!("Unsupported breakpoint: {:?}", breakpoint))),
        }
    }
}

impl ApiContract for ClassBuilderContract {
    type Input = ClassBuilderInput;
    type Output = ClassSet;
    type Error = TailwindError;

    fn validate_input(&self, input: &Self::Input) -> Result<(), ContractError> {
        // Validate base classes
        for class in &input.classes {
            self.validate_class_name(class)?;
        }

        // Validate responsive classes
        for (breakpoint, class) in &input.responsive {
            self.validate_breakpoint(breakpoint)?;
            self.validate_class_name(class)?;
        }

        // Validate conditional classes
        if !input.conditional.is_empty() && !self.capabilities.supports_conditional {
            return Err(ContractError::BusinessRuleViolation {
                rule: "conditional_support".to_string(),
                details: "Conditional classes not supported by this contract".to_string(),
            });
        }

        for (condition, class) in &input.conditional {
            if condition.is_empty() {
                return Err(ContractError::InvalidInput("Empty condition".to_string()));
            }
            self.validate_class_name(class)?;
        }

        // Validate custom properties
        if !input.custom.is_empty() && !self.capabilities.supports_custom {
            return Err(ContractError::BusinessRuleViolation {
                rule: "custom_support".to_string(),
                details: "Custom properties not supported by this contract".to_string(),
            });
        }

        for (property, _) in &input.custom {
            if property.is_empty() {
                return Err(ContractError::InvalidInput("Empty custom property".to_string()));
            }
        }

        // Check size limits
        let total_classes = input.classes.len() + input.responsive.len() + input.conditional.len();
        if total_classes > self.capabilities.max_classes_per_build {
            return Err(ContractError::ResourceExhausted {
                resource: "classes".to_string(),
                limit: self.capabilities.max_classes_per_build as u64,
                used: total_classes as u64,
            });
        }

        Ok(())
    }

    fn process(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        let mut builder = ClassBuilder::new();

        // Add base classes
        for class in input.classes {
            builder = builder.class(&class);
        }

        // Add responsive classes
        for (breakpoint, class) in input.responsive {
            builder = builder.responsive(breakpoint, &class);
        }

        // Add conditional classes
        for (condition, class) in input.conditional {
            builder = builder.conditional(&condition, &class);
        }

        // Add custom properties
        for (property, value) in input.custom {
            builder = builder.custom(&property, &value);
        }

        Ok(builder.build())
    }

    fn validate_output(&self, output: &Self::Output) -> Result<(), ContractError> {
        // Validate CSS class string format
        let css_classes = output.to_css_classes();

        // Check for empty output (should have at least some classes)
        if css_classes.trim().is_empty() {
            return Err(ContractError::InvalidOutput(
                "Empty CSS classes output".to_string(),
            ));
        }

        // Check for double spaces
        if css_classes.contains("  ") {
            return Err(ContractError::InvalidOutput(
                "CSS classes contain double spaces".to_string(),
            ));
        }

        // Check for valid CSS class format
        if !css_classes.chars().all(|c| c.is_alphanumeric() || c == '-' || c == ' ' || c == ':' || c == '[' || c == ']' || c == '(' || c == ')' || c == '/' || c == '.') {
            return Err(ContractError::InvalidOutput(
                "CSS classes contain invalid characters".to_string(),
            ));
        }

        // Check for balanced brackets in arbitrary values
        let bracket_count = css_classes.chars().fold(0, |count, c| {
            match c {
                '[' => count + 1,
                ']' => count - 1,
                _ => count,
            }
        });

        if bracket_count != 0 {
            return Err(ContractError::InvalidOutput(
                "Unbalanced brackets in CSS classes".to_string(),
            ));
        }

        Ok(())
    }
}

impl VersionedContract for ClassBuilderContract {
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

/// Input structure for ClassBuilder contract
#[derive(Debug, Clone, Default)]
pub struct ClassBuilderInput {
    pub classes: Vec<String>,
    pub responsive: Vec<(Breakpoint, String)>,
    pub conditional: Vec<(String, String)>,
    pub custom: Vec<(String, String)>,
}

impl ClassBuilderInput {
    /// Create a new input with base classes
    pub fn new(classes: Vec<String>) -> Self {
        Self {
            classes,
            responsive: Vec::new(),
            conditional: Vec::new(),
            custom: Vec::new(),
        }
    }

    /// Add responsive classes
    pub fn with_responsive(mut self, responsive: Vec<(Breakpoint, String)>) -> Self {
        self.responsive = responsive;
        self
    }

    /// Add conditional classes
    pub fn with_conditional(mut self, conditional: Vec<(String, String)>) -> Self {
        self.conditional = conditional;
        self
    }

    /// Add custom properties
    pub fn with_custom(mut self, custom: Vec<(String, String)>) -> Self {
        self.custom = custom;
        self
    }

    /// Get total number of classes
    pub fn total_classes(&self) -> usize {
        self.classes.len() + self.responsive.len() + self.conditional.len()
    }

    /// Check if input is empty
    pub fn is_empty(&self) -> bool {
        self.total_classes() == 0 && self.custom.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contract_creation() {
        let contract = ClassBuilderContract::new(ApiVersion::new(1, 0, 0));
        assert_eq!(contract.version(), ApiVersion::new(1, 0, 0));
        assert!(contract.supports_method("class"));
        assert!(contract.capabilities().supports_responsive);
    }

    #[test]
    fn input_validation() {
        let contract = ClassBuilderContract::new(ApiVersion::new(1, 0, 0));

        // Valid input
        let valid_input = ClassBuilderInput::new(vec!["bg-red-500".to_string(), "text-white".to_string()]);
        assert!(contract.validate_input(&valid_input).is_ok());

        // Invalid input - empty class
        let invalid_input = ClassBuilderInput::new(vec!["".to_string()]);
        assert!(contract.validate_input(&invalid_input).is_err());

        // Invalid input - spaces in class name
        let invalid_input = ClassBuilderInput::new(vec!["bg red".to_string()]);
        assert!(contract.validate_input(&invalid_input).is_err());
    }

    #[test]
    fn version_compatibility() {
        let contract = ClassBuilderContract::new(ApiVersion::new(1, 1, 0));
        assert!(contract.is_backward_compatible(&ApiVersion::new(1, 0, 0)));
        assert!(!contract.is_backward_compatible(&ApiVersion::new(2, 0, 0)));
    }

    #[test]
    fn class_builder_input_operations() {
        let input = ClassBuilderInput::new(vec!["bg-blue-500".to_string()])
            .with_responsive(vec![(Breakpoint::Md, "text-lg".to_string())])
            .with_conditional(vec![("hover".to_string(), "scale-105".to_string())])
            .with_custom(vec![("animation-duration".to_string(), "300ms".to_string())]);

        assert_eq!(input.total_classes(), 3);
        assert!(!input.is_empty());
    }

    #[test]
    fn capabilities_validation() {
        let capabilities = ClassBuilderCapabilities {
            supports_responsive: false,
            supports_conditional: true,
            supports_custom: true,
            supports_arbitrary_values: true,
            max_classes_per_build: 10,
        };

        let contract = ClassBuilderContract::with_capabilities(ApiVersion::new(1, 0, 0), capabilities);

        // Should reject responsive classes
        let input_with_responsive = ClassBuilderInput::new(vec!["bg-red-500".to_string()])
            .with_responsive(vec![(Breakpoint::Md, "text-lg".to_string())]);

        assert!(contract.validate_input(&input_with_responsive).is_err());

        // Should accept conditional classes
        let input_with_conditional = ClassBuilderInput::new(vec!["bg-red-500".to_string()])
            .with_conditional(vec![("hover".to_string(), "scale-105".to_string())]);

        assert!(contract.validate_input(&input_with_conditional).is_ok());
    }
}
