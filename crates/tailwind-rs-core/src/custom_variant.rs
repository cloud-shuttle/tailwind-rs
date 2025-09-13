//! Custom variant system for tailwind-rs
//! 
//! Implements Tailwind CSS v4.1.13 @custom-variant features
//! This replaces the old aria, data, and supports theme keys with a unified system

use crate::error::{Result, TailwindError};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt;

/// Custom variant types supported by Tailwind v4.1.13
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CustomVariantType {
    /// ARIA attributes (aria-*)
    Aria,
    /// Data attributes (data-*)
    Data,
    /// CSS feature queries (@supports)
    Supports,
    /// Container queries (@container)
    Container,
    /// Media queries (@media)
    Media,
    /// User-defined custom variants
    Custom(String),
}

impl CustomVariantType {
    /// Get the prefix for this variant type
    pub fn prefix(&self) -> &'static str {
        match self {
            CustomVariantType::Aria => "aria-",
            CustomVariantType::Data => "data-",
            CustomVariantType::Supports => "supports-",
            CustomVariantType::Container => "container-",
            CustomVariantType::Media => "media-",
            CustomVariantType::Custom(_name) => {
                // Custom variants should not start or end with - or _
                // For now, return empty string for all custom variants
                ""
            }
        }
    }

    /// Validate a custom variant name
    pub fn validate_name(name: &str) -> Result<()> {
        if name.is_empty() {
            return Err(TailwindError::validation("Custom variant name cannot be empty"));
        }

        // Custom variants cannot start or end with - or _
        if name.starts_with('-') || name.starts_with('_') || 
           name.ends_with('-') || name.ends_with('_') {
            return Err(TailwindError::validation(
                format!("Custom variant '{}' cannot start or end with '-' or '_'", name)
            ));
        }

        // Custom variants cannot start with @-
        if name.starts_with("@-") {
            return Err(TailwindError::validation(
                format!("Custom variant '{}' cannot start with '@-'", name)
            ));
        }

        Ok(())
    }
}

/// A custom variant definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomVariant {
    /// The variant type
    pub variant_type: CustomVariantType,
    /// The variant name (without prefix)
    pub name: String,
    /// The variant value (for aria, data, etc.)
    pub value: Option<String>,
    /// Whether this variant is enabled
    pub enabled: bool,
    /// Custom configuration for this variant
    pub config: HashMap<String, serde_json::Value>,
}

impl CustomVariant {
    /// Create a new custom variant
    pub fn new(variant_type: CustomVariantType, name: String, value: Option<String>) -> Result<Self> {
        // Validate the name
        CustomVariantType::validate_name(&name)?;

        Ok(Self {
            variant_type,
            name,
            value,
            enabled: true,
            config: HashMap::new(),
        })
    }

    /// Create an ARIA variant
    pub fn aria(name: String, value: Option<String>) -> Result<Self> {
        Self::new(CustomVariantType::Aria, name, value)
    }

    /// Create a data variant
    pub fn data(name: String, value: Option<String>) -> Result<Self> {
        Self::new(CustomVariantType::Data, name, value)
    }

    /// Create a supports variant
    pub fn supports(name: String, value: Option<String>) -> Result<Self> {
        Self::new(CustomVariantType::Supports, name, value)
    }

    /// Create a container variant
    pub fn container(name: String, value: Option<String>) -> Result<Self> {
        Self::new(CustomVariantType::Container, name, value)
    }

    /// Create a media variant
    pub fn media(name: String, value: Option<String>) -> Result<Self> {
        Self::new(CustomVariantType::Media, name, value)
    }

    /// Create a custom variant
    pub fn custom(name: String, value: Option<String>) -> Result<Self> {
        Self::new(CustomVariantType::Custom(name.clone()), name, value)
    }

    /// Get the full variant string (e.g., "aria-checked", "data-theme=dark")
    pub fn to_variant_string(&self) -> String {
        let prefix = self.variant_type.prefix();
        let base = format!("{}{}", prefix, self.name);
        
        if let Some(value) = &self.value {
            format!("{}={}", base, value)
        } else {
            base
        }
    }

    /// Get the CSS selector for this variant
    pub fn to_css_selector(&self) -> String {
        match &self.variant_type {
            CustomVariantType::Aria => {
                if let Some(value) = &self.value {
                    format!("[aria-{}={}]", self.name, value)
                } else {
                    format!("[aria-{}]", self.name)
                }
            }
            CustomVariantType::Data => {
                if let Some(value) = &self.value {
                    format!("[data-{}={}]", self.name, value)
                } else {
                    format!("[data-{}]", self.name)
                }
            }
            CustomVariantType::Supports => {
                format!("@supports ({})", self.name)
            }
            CustomVariantType::Container => {
                format!("@container {}", self.name)
            }
            CustomVariantType::Media => {
                format!("@media {}", self.name)
            }
            CustomVariantType::Custom(name) => {
                // Custom variants are handled by the user
                name.clone()
            }
        }
    }

    /// Enable this variant
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable this variant
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Set a configuration value
    pub fn set_config(&mut self, key: String, value: serde_json::Value) {
        self.config.insert(key, value);
    }

    /// Get a configuration value
    pub fn get_config(&self, key: &str) -> Option<&serde_json::Value> {
        self.config.get(key)
    }
}

/// Manager for custom variants
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomVariantManager {
    /// Registered custom variants
    variants: HashMap<String, CustomVariant>,
    /// Known variant values for suggestions
    known_values: HashMap<String, HashSet<String>>,
}

impl CustomVariantManager {
    /// Create a new custom variant manager
    pub fn new() -> Self {
        Self {
            variants: HashMap::new(),
            known_values: HashMap::new(),
        }
    }

    /// Register a custom variant
    pub fn register(&mut self, variant: CustomVariant) -> Result<()> {
        let key = variant.to_variant_string();
        
        // Check for conflicts
        if self.variants.contains_key(&key) {
            return Err(TailwindError::validation(
                format!("Custom variant '{}' is already registered", key)
            ));
        }

        self.variants.insert(key, variant);
        Ok(())
    }

    /// Get a custom variant by key
    pub fn get(&self, key: &str) -> Option<&CustomVariant> {
        self.variants.get(key)
    }

    /// Get all registered variants
    pub fn get_all(&self) -> &HashMap<String, CustomVariant> {
        &self.variants
    }

    /// Get variants by type
    pub fn get_by_type(&self, variant_type: &CustomVariantType) -> Vec<&CustomVariant> {
        self.variants
            .values()
            .filter(|v| &v.variant_type == variant_type)
            .collect()
    }

    /// Remove a custom variant
    pub fn remove(&mut self, key: &str) -> Option<CustomVariant> {
        self.variants.remove(key)
    }

    /// Check if a variant is registered
    pub fn contains(&self, key: &str) -> bool {
        self.variants.contains_key(key)
    }

    /// Add known values for a variant (for suggestions)
    pub fn add_known_values(&mut self, variant_key: String, values: HashSet<String>) {
        self.known_values.insert(variant_key, values);
    }

    /// Get known values for a variant
    pub fn get_known_values(&self, variant_key: &str) -> Option<&HashSet<String>> {
        self.known_values.get(variant_key)
    }

    /// Get suggestions for a variant
    pub fn get_suggestions(&self, partial: &str) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        // Add exact matches
        for key in self.variants.keys() {
            if key.starts_with(partial) {
                suggestions.push(key.clone());
            }
        }

        // Add known values
        for (variant_key, values) in &self.known_values {
            if variant_key.starts_with(partial) {
                for value in values {
                    suggestions.push(format!("{}={}", variant_key, value));
                }
            }
        }

        suggestions.sort();
        suggestions.dedup();
        suggestions
    }

    /// Validate a variant string
    pub fn validate_variant(&self, variant: &str) -> Result<()> {
        // Check if it's a registered variant
        if self.variants.contains_key(variant) {
            return Ok(());
        }

        // Check if it matches a known pattern
        if variant.starts_with("aria-") || 
           variant.starts_with("data-") || 
           variant.starts_with("supports-") ||
           variant.starts_with("container-") ||
           variant.starts_with("media-") {
            return Ok(());
        }

        // Check for invalid patterns
        if variant.starts_with("@-") {
            return Err(TailwindError::validation(
                format!("Variant '{}' cannot start with '@-'", variant)
            ));
        }

        if variant.starts_with('-') || variant.starts_with('_') || 
           variant.ends_with('-') || variant.ends_with('_') {
            return Err(TailwindError::validation(
                format!("Variant '{}' cannot start or end with '-' or '_'", variant)
            ));
        }

        Ok(())
    }

    /// Create default variants (migrated from old theme keys)
    pub fn with_defaults() -> Self {
        let mut manager = Self::new();
        
        // Add common ARIA variants
        let aria_variants = vec![
            ("checked", None),
            ("disabled", None),
            ("expanded", None),
            ("hidden", None),
            ("pressed", None),
            ("required", None),
            ("selected", None),
        ];

        for (name, value) in aria_variants {
            if let Ok(variant) = CustomVariant::aria(name.to_string(), value) {
                let _ = manager.register(variant);
            }
        }

        // Add common data variants
        let data_variants = vec![
            ("theme", Some("dark".to_string())),
            ("theme", Some("light".to_string())),
            ("state", Some("loading".to_string())),
            ("state", Some("error".to_string())),
        ];

        for (name, value) in data_variants {
            if let Ok(variant) = CustomVariant::data(name.to_string(), value) {
                let _ = manager.register(variant);
            }
        }

        // Add common supports variants
        let supports_variants = vec![
            ("backdrop-filter", None),
            ("grid", None),
            ("flexbox", None),
        ];

        for (name, value) in supports_variants {
            if let Ok(variant) = CustomVariant::supports(name.to_string(), value) {
                let _ = manager.register(variant);
            }
        }

        manager
    }
}

impl Default for CustomVariantManager {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl fmt::Display for CustomVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_variant_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_variant_creation() {
        let variant = CustomVariant::aria("checked".to_string(), None).unwrap();
        assert_eq!(variant.to_variant_string(), "aria-checked");
        assert_eq!(variant.to_css_selector(), "[aria-checked]");
    }

    #[test]
    fn test_custom_variant_with_value() {
        let variant = CustomVariant::data("theme".to_string(), Some("dark".to_string())).unwrap();
        assert_eq!(variant.to_variant_string(), "data-theme=dark");
        assert_eq!(variant.to_css_selector(), "[data-theme=dark]");
    }

    #[test]
    fn test_custom_variant_validation() {
        // Valid variants
        assert!(CustomVariantType::validate_name("valid-name").is_ok());
        assert!(CustomVariantType::validate_name("valid_name").is_ok());
        assert!(CustomVariantType::validate_name("validname").is_ok());

        // Invalid variants
        assert!(CustomVariantType::validate_name("-invalid").is_err());
        assert!(CustomVariantType::validate_name("invalid-").is_err());
        assert!(CustomVariantType::validate_name("_invalid").is_err());
        assert!(CustomVariantType::validate_name("invalid_").is_err());
    }

    #[test]
    fn test_custom_variant_manager() {
        let mut manager = CustomVariantManager::new();
        
        let variant = CustomVariant::aria("checked".to_string(), None).unwrap();
        manager.register(variant).unwrap();
        
        assert!(manager.contains("aria-checked"));
        assert!(manager.get("aria-checked").is_some());
    }

    #[test]
    fn test_custom_variant_suggestions() {
        let mut manager = CustomVariantManager::with_defaults();
        
        let suggestions = manager.get_suggestions("aria-");
        assert!(!suggestions.is_empty());
        assert!(suggestions.contains(&"aria-checked".to_string()));
    }

    #[test]
    fn test_custom_variant_validation_in_manager() {
        let manager = CustomVariantManager::with_defaults();
        
        // Valid variants
        assert!(manager.validate_variant("aria-checked").is_ok());
        assert!(manager.validate_variant("data-theme=dark").is_ok());
        
        // Invalid variants
        assert!(manager.validate_variant("@-invalid").is_err());
        assert!(manager.validate_variant("-invalid").is_err());
        assert!(manager.validate_variant("invalid-").is_err());
    }
}
