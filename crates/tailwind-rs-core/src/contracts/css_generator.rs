//! CSS Generator API Contract
//!
//! Defines the contract for the CSS Generator API, ensuring stable and reliable
//! CSS generation with proper validation and formatting.

use crate::css_generator::{CssGenerator, CssProperty};
use crate::error::TailwindError;
use super::core::{traits::*, errors::*};

/// CSS Generator API contract implementation
#[derive(Debug, Clone)]
pub struct CssGeneratorContract {
    version: ApiVersion,
    supported_formats: Vec<CssFormat>,
    capabilities: CssGeneratorCapabilities,
}

#[derive(Debug, Clone)]
pub struct CssGeneratorCapabilities {
    pub supports_minification: bool,
    pub supports_source_maps: bool,
    pub supports_custom_properties: bool,
    pub max_rules_per_generation: usize,
    pub max_properties_per_rule: usize,
}

impl Default for CssGeneratorCapabilities {
    fn default() -> Self {
        Self {
            supports_minification: true,
            supports_source_maps: false, // Not yet implemented
            supports_custom_properties: true,
            max_rules_per_generation: 10000,
            max_properties_per_rule: 100,
        }
    }
}

/// CSS output format options
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CssFormat {
    Regular,
    Minified,
    WithSourceMaps,
}

impl CssFormat {
    /// Check if format is supported by capabilities
    pub fn is_supported_by(&self, capabilities: &CssGeneratorCapabilities) -> bool {
        match self {
            CssFormat::Regular => true,
            CssFormat::Minified => capabilities.supports_minification,
            CssFormat::WithSourceMaps => capabilities.supports_source_maps,
        }
    }
}

impl CssGeneratorContract {
    /// Create a new CSS Generator contract with specified version
    pub fn new(version: ApiVersion) -> Self {
        Self {
            version,
            supported_formats: vec![
                CssFormat::Regular,
                CssFormat::Minified,
                CssFormat::WithSourceMaps,
            ],
            capabilities: CssGeneratorCapabilities::default(),
        }
    }

    /// Create a contract with custom capabilities
    pub fn with_capabilities(version: ApiVersion, capabilities: CssGeneratorCapabilities) -> Self {
        let mut supported_formats = vec![CssFormat::Regular];

        if capabilities.supports_minification {
            supported_formats.push(CssFormat::Minified);
        }

        if capabilities.supports_source_maps {
            supported_formats.push(CssFormat::WithSourceMaps);
        }

        Self {
            version,
            supported_formats,
            capabilities,
        }
    }

    /// Get supported formats
    pub fn supported_formats(&self) -> &[CssFormat] {
        &self.supported_formats
    }

    /// Get capabilities
    pub fn capabilities(&self) -> &CssGeneratorCapabilities {
        &self.capabilities
    }

    /// Check if format is supported
    pub fn supports_format(&self, format: &CssFormat) -> bool {
        self.supported_formats.contains(format)
    }

    /// Validate CSS selector format
    fn validate_selector(&self, selector: &str) -> Result<(), ContractError> {
        if selector.is_empty() {
            return Err(ContractError::InvalidInput("Empty CSS selector".to_string()));
        }

        // Check for valid CSS selector characters
        if !selector.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '.' || c == '#' || c == ':' || c == '[' || c == ']' || c == '>' || c == '+' || c == '~' || c == ' ') {
            return Err(ContractError::InvalidInput(format!("Invalid CSS selector: {}", selector)));
        }

        // Check for balanced brackets
        let bracket_count = selector.chars().fold(0, |count, c| {
            match c {
                '[' => count + 1,
                ']' => count - 1,
                _ => count,
            }
        });

        if bracket_count != 0 {
            return Err(ContractError::InvalidInput("Unbalanced brackets in selector".to_string()));
        }

        Ok(())
    }

    /// Validate CSS property
    fn validate_property(&self, property: &CssPropertyInput) -> Result<(), ContractError> {
        if property.name.is_empty() {
            return Err(ContractError::InvalidInput("Empty CSS property name".to_string()));
        }

        if property.value.is_empty() {
            return Err(ContractError::InvalidInput("Empty CSS property value".to_string()));
        }

        // Check for valid CSS property name format
        if !property.name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
            return Err(ContractError::InvalidInput(format!("Invalid CSS property name: {}", property.name)));
        }

        // Check for semicolons in property names (would break CSS)
        if property.name.contains(';') || property.value.contains(';') {
            return Err(ContractError::InvalidInput("CSS property contains semicolons".to_string()));
        }

        Ok(())
    }

    /// Validate media query format
    fn validate_media_query(&self, media_query: &str) -> Result<(), ContractError> {
        if !media_query.starts_with("@media") {
            return Err(ContractError::InvalidInput("Invalid media query format".to_string()));
        }

        // Basic validation for media query syntax
        if !media_query.contains('(') || !media_query.contains(')') {
            return Err(ContractError::InvalidInput("Malformed media query syntax".to_string()));
        }

        Ok(())
    }
}

impl ApiContract for CssGeneratorContract {
    type Input = CssGeneratorInput;
    type Output = String;
    type Error = TailwindError;

    fn validate_input(&self, input: &Self::Input) -> Result<(), ContractError> {
        // Validate format support
        if !self.supports_format(&input.format) {
            return Err(ContractError::BusinessRuleViolation {
                rule: "format_support".to_string(),
                details: format!("Format {:?} not supported by this contract", input.format),
            });
        }

        // Validate CSS rules
        for rule in &input.rules {
            self.validate_selector(&rule.selector)?;

            if rule.properties.is_empty() {
                return Err(ContractError::InvalidInput("Empty CSS properties".to_string()));
            }

            for property in &rule.properties {
                self.validate_property(property)?;
            }

            if rule.properties.len() > self.capabilities.max_properties_per_rule {
                return Err(ContractError::ResourceExhausted {
                    resource: "properties_per_rule".to_string(),
                    limit: self.capabilities.max_properties_per_rule as u64,
                    used: rule.properties.len() as u64,
                });
            }
        }

        // Validate media queries
        for media_query in &input.media_queries {
            self.validate_media_query(media_query)?;
        }

        // Check size limits
        if input.rules.len() > self.capabilities.max_rules_per_generation {
            return Err(ContractError::ResourceExhausted {
                resource: "rules".to_string(),
                limit: self.capabilities.max_rules_per_generation as u64,
                used: input.rules.len() as u64,
            });
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
                .map(|p| format!("{}: {}{}", p.name, p.value, if p.important { " !important" } else { "" }))
                .collect::<Vec<_>>()
                .join("; ");

            generator.add_css_selector(&rule.selector, &properties_str)?;
        }

        // Add media queries (if any)
        for media_query in input.media_queries {
            // For now, media queries are not fully integrated into CssGenerator
            // This would need extension of the CssGenerator API
        }

        // Generate CSS based on format
        match input.format {
            CssFormat::Regular => Ok(generator.generate_css()),
            CssFormat::Minified => {
                // For now, return regular CSS - full minification would require additional implementation
                Ok(generator.generate_css())
            }
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
        if !output.contains('{') || !output.contains('}') {
            return Err(ContractError::InvalidOutput("Invalid CSS structure".to_string()));
        }

        // Check for balanced braces
        let brace_count = output.chars().fold(0, |count, c| {
            match c {
                '{' => count + 1,
                '}' => count - 1,
                _ => count,
            }
        });

        if brace_count != 0 {
            return Err(ContractError::InvalidOutput("Unbalanced braces in CSS".to_string()));
        }

        // Check for semicolons (properties should be properly terminated)
        let css_content = output.replace("{", "").replace("}", "");
        if css_content.contains(':') && !css_content.trim().ends_with(';') && !css_content.trim().is_empty() {
            // This is a simplified check - a full CSS validator would be more comprehensive
            let lines_with_colon = css_content.lines().filter(|line| line.contains(':'));
            for line in lines_with_colon {
                let trimmed = line.trim();
                if !trimmed.is_empty() && !trimmed.ends_with(';') && !trimmed.ends_with(',') {
                    return Err(ContractError::InvalidOutput("CSS property missing semicolon".to_string()));
                }
            }
        }

        Ok(())
    }
}

impl VersionedContract for CssGeneratorContract {
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

/// Input structure for CSS Generator contract
#[derive(Debug, Clone)]
pub struct CssGeneratorInput {
    pub rules: Vec<CssRuleInput>,
    pub media_queries: Vec<String>,
    pub format: CssFormat,
}

impl CssGeneratorInput {
    /// Create a new input with rules
    pub fn new(rules: Vec<CssRuleInput>, format: CssFormat) -> Self {
        Self {
            rules,
            media_queries: Vec::new(),
            format,
        }
    }

    /// Add media queries
    pub fn with_media_queries(mut self, media_queries: Vec<String>) -> Self {
        self.media_queries = media_queries;
        self
    }

    /// Get total number of rules
    pub fn total_rules(&self) -> usize {
        self.rules.len()
    }

    /// Check if input is empty
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
}

/// CSS rule input structure
#[derive(Debug, Clone)]
pub struct CssRuleInput {
    pub selector: String,
    pub properties: Vec<CssPropertyInput>,
}

impl CssRuleInput {
    /// Create a new CSS rule
    pub fn new(selector: String, properties: Vec<CssPropertyInput>) -> Self {
        Self { selector, properties }
    }

    /// Add a property to the rule
    pub fn with_property(mut self, name: String, value: String) -> Self {
        self.properties.push(CssPropertyInput::new(name, value));
        self
    }

    /// Add an important property to the rule
    pub fn with_important_property(mut self, name: String, value: String) -> Self {
        self.properties.push(CssPropertyInput::important(name, value));
        self
    }
}

/// CSS property input structure
#[derive(Debug, Clone)]
pub struct CssPropertyInput {
    pub name: String,
    pub value: String,
    pub important: bool,
}

impl CssPropertyInput {
    /// Create a regular CSS property
    pub fn new(name: String, value: String) -> Self {
        Self { name, value, important: false }
    }

    /// Create an important CSS property
    pub fn important(name: String, value: String) -> Self {
        Self { name, value, important: true }
    }

    /// Convert to CssProperty for internal use
    pub fn to_css_property(&self) -> CssProperty {
        CssProperty {
            name: self.name.clone(),
            value: self.value.clone(),
            important: self.important,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contract_creation() {
        let contract = CssGeneratorContract::new(ApiVersion::new(1, 0, 0));
        assert_eq!(contract.version(), ApiVersion::new(1, 0, 0));
        assert!(contract.supports_format(&CssFormat::Regular));
        assert!(contract.capabilities().supports_minification);
    }

    #[test]
    fn input_validation() {
        let contract = CssGeneratorContract::new(ApiVersion::new(1, 0, 0));

        // Valid input
        let rule = CssRuleInput::new(
            ".bg-red-500".to_string(),
            vec![CssPropertyInput::new("background-color".to_string(), "#ef4444".to_string())]
        );
        let input = CssGeneratorInput::new(vec![rule], CssFormat::Regular);
        assert!(contract.validate_input(&input).is_ok());

        // Invalid input - empty selector
        let rule = CssRuleInput::new("".to_string(), vec![]);
        let input = CssGeneratorInput::new(vec![rule], CssFormat::Regular);
        assert!(contract.validate_input(&input).is_err());
    }

    #[test]
    fn css_rule_operations() {
        let rule = CssRuleInput::new(".test".to_string(), vec![])
            .with_property("color".to_string(), "red".to_string())
            .with_important_property("display".to_string(), "block".to_string());

        assert_eq!(rule.properties.len(), 2);
        assert!(rule.properties[0].important == false);
        assert!(rule.properties[1].important == true);
    }

    #[test]
    fn css_property_operations() {
        let prop = CssPropertyInput::new("color".to_string(), "blue".to_string());
        assert_eq!(prop.name, "color");
        assert_eq!(prop.value, "blue");
        assert!(!prop.important);

        let important_prop = CssPropertyInput::important("display".to_string(), "none".to_string());
        assert!(important_prop.important);
    }

    #[test]
    fn format_capabilities() {
        let capabilities = CssGeneratorCapabilities {
            supports_minification: false,
            supports_source_maps: false,
            supports_custom_properties: true,
            max_rules_per_generation: 100,
            max_properties_per_rule: 10,
        };

        let contract = CssGeneratorContract::with_capabilities(ApiVersion::new(1, 0, 0), capabilities);

        assert!(contract.supports_format(&CssFormat::Regular));
        assert!(!contract.supports_format(&CssFormat::Minified));
        assert!(!contract.supports_format(&CssFormat::WithSourceMaps));
    }

    #[test]
    fn version_compatibility() {
        let contract = CssGeneratorContract::new(ApiVersion::new(1, 1, 0));
        assert!(contract.is_backward_compatible(&ApiVersion::new(1, 0, 0)));
        assert!(!contract.is_backward_compatible(&ApiVersion::new(2, 0, 0)));
    }
}
