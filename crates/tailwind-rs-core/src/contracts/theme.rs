//! Theme API Contract
//!
//! Defines the contract for the Theme API, ensuring stable and reliable
//! theme configuration and customization.

use crate::theme::ThemeConfig;
use crate::error::TailwindError;
use std::collections::HashMap;
use super::core::{traits::*, errors::*};

/// Theme API contract implementation
#[derive(Debug, Clone)]
pub struct ThemeContract {
    version: ApiVersion,
    supported_features: Vec<String>,
    capabilities: ThemeCapabilities,
}

#[derive(Debug, Clone)]
pub struct ThemeCapabilities {
    pub supports_colors: bool,
    pub supports_spacing: bool,
    pub supports_typography: bool,
    pub supports_breakpoints: bool,
    pub supports_custom_properties: bool,
    pub supports_extends: bool,
    pub max_theme_size: usize,
}

impl Default for ThemeCapabilities {
    fn default() -> Self {
        Self {
            supports_colors: true,
            supports_spacing: true,
            supports_typography: true,
            supports_breakpoints: true,
            supports_custom_properties: true,
            supports_extends: false, // Not yet implemented
            max_theme_size: 10000,
        }
    }
}

impl ThemeContract {
    /// Create a new Theme contract with specified version
    pub fn new(version: ApiVersion) -> Self {
        Self {
            version,
            supported_features: vec![
                "colors".to_string(),
                "spacing".to_string(),
                "typography".to_string(),
                "breakpoints".to_string(),
                "custom_properties".to_string(),
                "extends".to_string(),
            ],
            capabilities: ThemeCapabilities::default(),
        }
    }

    /// Create a contract with custom capabilities
    pub fn with_capabilities(version: ApiVersion, capabilities: ThemeCapabilities) -> Self {
        Self {
            version,
            supported_features: vec![
                if capabilities.supports_colors { Some("colors".to_string()) } else { None },
                if capabilities.supports_spacing { Some("spacing".to_string()) } else { None },
                if capabilities.supports_typography { Some("typography".to_string()) } else { None },
                if capabilities.supports_breakpoints { Some("breakpoints".to_string()) } else { None },
                if capabilities.supports_custom_properties { Some("custom_properties".to_string()) } else { None },
                if capabilities.supports_extends { Some("extends".to_string()) } else { None },
            ].into_iter().flatten().collect(),
            capabilities,
        }
    }

    /// Get supported features
    pub fn supported_features(&self) -> &[String] {
        &self.supported_features
    }

    /// Get capabilities
    pub fn capabilities(&self) -> &ThemeCapabilities {
        &self.capabilities
    }

    /// Check if feature is supported
    pub fn supports_feature(&self, feature: &str) -> bool {
        self.supported_features.contains(&feature.to_string())
    }

    /// Validate color value format
    fn is_valid_color_value(&self, value: &str) -> bool {
        // Basic color validation - accept hex colors, rgb(), hsl(), hwb(), and common named colors
        if value.starts_with('#') {
            // Hex color: #RGB, #RRGGBB, #RRGGBBAA
            return value.len() == 4 || value.len() == 7 || value.len() == 9;
        }

        if value.starts_with("rgb(") || value.starts_with("hsl(") || value.starts_with("hwb(") {
            // Function-based colors must end with )
            return value.ends_with(')');
        }

        // Common named colors (basic set)
        let named_colors = [
            "black", "white", "gray", "grey", "red", "green", "blue", "yellow",
            "purple", "pink", "orange", "brown", "cyan", "magenta", "lime", "teal",
            "indigo", "violet", "maroon", "navy", "olive", "silver", "gold", "aqua"
        ];

        named_colors.contains(&value)
    }

    /// Validate spacing value format
    fn is_valid_spacing_value(&self, value: &str) -> bool {
        // Basic spacing validation - accept numbers with units, percentages, or 'auto'
        if value == "auto" {
            return true;
        }

        if value.ends_with('%') {
            let num_part = value.trim_end_matches('%');
            return num_part.parse::<f32>().is_ok();
        }

        // Check for valid CSS units (ordered by length descending to avoid substring matches)
        let units = ["vmax", "vmin", "rem", "em", "vh", "vw", "px", "pt", "pc", "in", "cm", "mm"];
        for unit in &units {
            if value.ends_with(unit) {
                let num_part = value.trim_end_matches(unit);
                return num_part.parse::<f32>().is_ok();
            }
        }

        // Allow plain numbers (interpreted as rem or px depending on context)
        value.parse::<f32>().is_ok()
    }

    /// Validate typography value format
    fn is_valid_typography_value(&self, value: &str) -> bool {
        // Basic typography validation - accept font family names, sizes with units, or weights
        if value.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == ' ') {
            return true; // Font family name
        }

        // Check for font sizes with units
        let units = ["rem", "em", "px", "pt", "pc"];
        for unit in &units {
            if value.ends_with(unit) {
                let num_part = value.trim_end_matches(unit);
                return num_part.parse::<f32>().is_ok();
            }
        }

        // Check for font weights
        value.parse::<u32>().is_ok() && value.len() <= 3
    }
}

impl ApiContract for ThemeContract {
    type Input = ThemeInput;
    type Output = ThemeConfig;
    type Error = TailwindError;

    fn validate_input(&self, input: &Self::Input) -> Result<(), ContractError> {
        // Validate theme name
        if input.name.trim().is_empty() {
            return Err(ContractError::InvalidInput("Empty theme name".to_string()));
        }

        if input.name.len() > 100 {
            return Err(ContractError::InvalidInput("Theme name too long".to_string()));
        }

        // Validate color palette
        if self.capabilities.supports_colors && !input.colors.is_empty() {
            for (color_name, color_value) in &input.colors {
                if color_name.is_empty() {
                    return Err(ContractError::InvalidInput("Empty color name".to_string()));
                }
                if color_name.len() > 50 {
                    return Err(ContractError::InvalidInput(format!("Color name too long: {}", color_name)));
                }
                if !self.is_valid_color_value(color_value) {
                    return Err(ContractError::InvalidInput(format!("Invalid color value: {}", color_value)));
                }
            }
        }

        // Validate spacing scale
        if self.capabilities.supports_spacing && !input.spacing.is_empty() {
            for (spacing_name, spacing_value) in &input.spacing {
                if spacing_name.is_empty() {
                    return Err(ContractError::InvalidInput("Empty spacing name".to_string()));
                }
                if spacing_name.len() > 50 {
                    return Err(ContractError::InvalidInput(format!("Spacing name too long: {}", spacing_name)));
                }
                if !self.is_valid_spacing_value(spacing_value) {
                    return Err(ContractError::InvalidInput(format!("Invalid spacing value: {}", spacing_value)));
                }
            }
        }

        // Validate typography
        if self.capabilities.supports_typography && !input.typography.is_empty() {
            for (typography_name, typography_value) in &input.typography {
                if typography_name.is_empty() {
                    return Err(ContractError::InvalidInput("Empty typography name".to_string()));
                }
                if typography_name.len() > 50 {
                    return Err(ContractError::InvalidInput(format!("Typography name too long: {}", typography_name)));
                }
                if !self.is_valid_typography_value(typography_value) {
                    return Err(ContractError::InvalidInput(format!("Invalid typography value: {}", typography_value)));
                }
            }
        }

        // Check size limits
        let total_items = input.colors.len() + input.spacing.len() + input.typography.len() + input.breakpoints.len();
        if total_items > self.capabilities.max_theme_size {
            return Err(ContractError::ResourceExhausted {
                resource: "theme_items".to_string(),
                limit: self.capabilities.max_theme_size as u64,
                used: total_items as u64,
            });
        }

        Ok(())
    }

    fn process(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        let mut theme = ThemeConfig::default();
        theme.name = input.name;

        // In a real implementation, this would properly integrate with ThemeConfig
        // For now, we create a basic theme configuration

        // Add colors (would use theme.add_color() in real implementation)
        for (name, value) in input.colors {
            // theme.add_color(name, value);
        }

        // Add spacing (would use theme.add_spacing() in real implementation)
        for (name, value) in input.spacing {
            // theme.add_spacing(name, value);
        }

        // Add typography (would use theme.add_typography() in real implementation)
        for (name, value) in input.typography {
            // theme.add_typography(name, value);
        }

        // Add breakpoints (would use theme.add_breakpoint() in real implementation)
        for (name, value) in input.breakpoints {
            // theme.add_breakpoint(name, value);
        }

        Ok(theme)
    }

    fn validate_output(&self, output: &Self::Output) -> Result<(), ContractError> {
        // Validate theme has required fields
        if output.name.trim().is_empty() {
            return Err(ContractError::InvalidOutput("Theme missing name".to_string()));
        }

        // In a real implementation, this would validate the actual theme contents
        // For now, we do basic validation

        Ok(())
    }
}

impl VersionedContract for ThemeContract {
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

/// Input structure for Theme contract
#[derive(Debug, Clone, Default)]
pub struct ThemeInput {
    pub name: String,
    pub colors: HashMap<String, String>,
    pub spacing: HashMap<String, String>,
    pub typography: HashMap<String, String>,
    pub breakpoints: HashMap<String, String>,
}

impl ThemeInput {
    /// Create a new theme input
    pub fn new(name: String) -> Self {
        Self {
            name,
            colors: HashMap::new(),
            spacing: HashMap::new(),
            typography: HashMap::new(),
            breakpoints: HashMap::new(),
        }
    }

    /// Add a color to the theme
    pub fn with_color(mut self, name: String, value: String) -> Self {
        self.colors.insert(name, value);
        self
    }

    /// Add spacing to the theme
    pub fn with_spacing(mut self, name: String, value: String) -> Self {
        self.spacing.insert(name, value);
        self
    }

    /// Add typography to the theme
    pub fn with_typography(mut self, name: String, value: String) -> Self {
        self.typography.insert(name, value);
        self
    }

    /// Add a breakpoint to the theme
    pub fn with_breakpoint(mut self, name: String, value: String) -> Self {
        self.breakpoints.insert(name, value);
        self
    }

    /// Get total number of theme items
    pub fn total_items(&self) -> usize {
        self.colors.len() + self.spacing.len() + self.typography.len() + self.breakpoints.len()
    }

    /// Check if theme input is empty
    pub fn is_empty(&self) -> bool {
        self.name.trim().is_empty() && self.total_items() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contract_creation() {
        let contract = ThemeContract::new(ApiVersion::new(1, 0, 0));
        assert_eq!(contract.version(), ApiVersion::new(1, 0, 0));
        assert!(contract.supports_feature("colors"));
        assert!(contract.capabilities().supports_colors);
    }

    #[test]
    fn input_validation() {
        let contract = ThemeContract::new(ApiVersion::new(1, 0, 0));

        // Valid input
        let input = ThemeInput::new("test-theme".to_string())
            .with_color("primary".to_string(), "#ff0000".to_string())
            .with_spacing("small".to_string(), "0.5rem".to_string());

        assert!(contract.validate_input(&input).is_ok());

        // Invalid input - empty name
        let invalid_input = ThemeInput::new("".to_string());
        assert!(contract.validate_input(&invalid_input).is_err());

        // Invalid input - bad color
        let invalid_input = ThemeInput::new("test".to_string())
            .with_color("primary".to_string(), "not-a-color".to_string());
        assert!(contract.validate_input(&invalid_input).is_err());
    }

    #[test]
    fn color_value_validation() {
        let contract = ThemeContract::new(ApiVersion::new(1, 0, 0));

        // Valid colors
        assert!(contract.is_valid_color_value("#fff"));
        assert!(contract.is_valid_color_value("#ff0000"));
        assert!(contract.is_valid_color_value("#ff000080"));
        assert!(contract.is_valid_color_value("rgb(255, 0, 0)"));
        assert!(contract.is_valid_color_value("red"));
        assert!(contract.is_valid_color_value("hsl(0, 100%, 50%)"));

        // Invalid colors
        assert!(!contract.is_valid_color_value(""));
        assert!(!contract.is_valid_color_value("#ff"));
        assert!(!contract.is_valid_color_value("#gggggg"));
        assert!(!contract.is_valid_color_value("rgb("));
    }

    #[test]
    fn spacing_value_validation() {
        let contract = ThemeContract::new(ApiVersion::new(1, 0, 0));

        // Valid spacing
        assert!(contract.is_valid_spacing_value("auto"));
        assert!(contract.is_valid_spacing_value("10px"));
        assert!(contract.is_valid_spacing_value("1.5rem"));
        assert!(contract.is_valid_spacing_value("100%"));
        assert!(contract.is_valid_spacing_value("2.5"));

        // Invalid spacing
        assert!(!contract.is_valid_spacing_value(""));
        assert!(!contract.is_valid_spacing_value("px"));
        assert!(!contract.is_valid_spacing_value("10unknown"));
    }

    #[test]
    fn theme_input_operations() {
        let input = ThemeInput::new("my-theme".to_string())
            .with_color("primary".to_string(), "#007acc".to_string())
            .with_spacing("base".to_string(), "1rem".to_string())
            .with_typography("heading".to_string(), "bold".to_string())
            .with_breakpoint("tablet".to_string(), "768px".to_string());

        assert_eq!(input.name, "my-theme");
        assert_eq!(input.total_items(), 4);
        assert!(!input.is_empty());
    }

    #[test]
    fn capabilities_validation() {
        let capabilities = ThemeCapabilities {
            supports_colors: false,
            supports_spacing: true,
            supports_typography: true,
            supports_breakpoints: true,
            supports_custom_properties: true,
            supports_extends: false,
            max_theme_size: 10,
        };

        let contract = ThemeContract::with_capabilities(ApiVersion::new(1, 0, 0), capabilities);

        assert!(!contract.supports_feature("colors"));
        assert!(contract.supports_feature("spacing"));
    }

    #[test]
    fn version_compatibility() {
        let contract = ThemeContract::new(ApiVersion::new(1, 1, 0));
        assert!(contract.is_backward_compatible(&ApiVersion::new(1, 0, 0)));
        assert!(!contract.is_backward_compatible(&ApiVersion::new(2, 0, 0)));
    }
}
