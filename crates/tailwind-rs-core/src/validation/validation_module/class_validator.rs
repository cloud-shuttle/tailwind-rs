//! Class validator
//!
//! This module provides class validation functionality.

use super::{ValidationError, ValidationRules};
use std::collections::HashSet;

/// Class validator
pub struct ClassValidator {
    rules: ValidationRules,
}

impl ClassValidator {
    /// Create new class validator
    pub fn new(rules: ValidationRules) -> Self {
        Self { rules }
    }

    /// Validate a single class
    pub fn validate_class(&self, class: &str) -> Result<(), ValidationError> {
        // Check if class is deprecated
        if self.rules.deprecated_classes.contains(class) {
            return Err(ValidationError::DeprecatedClass(class.to_string()));
        }

        // Check if class matches forbidden patterns
        for pattern in &self.rules.forbidden_patterns {
            if pattern.is_match(class) {
                return Err(ValidationError::UnsupportedClass(class.to_string()));
            }
        }

        // Check if class matches allowed patterns
        if !self.rules.allowed_patterns.is_empty() {
            let mut matches = false;
            for pattern in &self.rules.allowed_patterns {
                if pattern.is_match(class) {
                    matches = true;
                    break;
                }
            }
            if !matches {
                return Err(ValidationError::InvalidClass(class.to_string()));
            }
        }

        Ok(())
    }

    /// Validate multiple classes
    pub fn validate_classes(&self, classes: &[String]) -> Result<(), ValidationError> {
        for class in classes {
            self.validate_class(class)?;
        }
        Ok(())
    }
}
