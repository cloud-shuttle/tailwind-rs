//! Custom variant validator
//!
//! This module provides custom variant validation functionality.

use super::{ValidationError, ValidationRules};
use crate::custom_variant::CustomVariantManager;

/// Custom variant validator
pub struct CustomVariantValidator {
    rules: ValidationRules,
    variant_manager: CustomVariantManager,
}

impl CustomVariantValidator {
    /// Create new custom variant validator
    pub fn new(rules: ValidationRules, variant_manager: CustomVariantManager) -> Self {
        Self {
            rules,
            variant_manager,
        }
    }

    /// Validate custom variant
    pub fn validate_custom_variant(&self, variant: &str) -> Result<(), ValidationError> {
        self.variant_manager
            .validate_variant(variant)
            .map_err(|_| ValidationError::InvalidCustomVariant(variant.to_string()))?;

        Ok(())
    }

    /// Validate custom variants in classes
    pub fn validate_custom_variants(&self, classes: &[String]) -> Result<(), ValidationError> {
        for class in classes {
            if let Some(variant) = self.extract_variant(class) {
                self.validate_custom_variant(&variant)?;
            }
        }
        Ok(())
    }

    /// Extract variant from class name
    fn extract_variant(&self, class: &str) -> Option<String> {
        // Simple variant extraction logic
        if class.contains(':') {
            Some(class.split(':').next()?.to_string())
        } else {
            None
        }
    }
}
