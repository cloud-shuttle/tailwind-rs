//! Deprecated checker
//!
//! This module provides deprecated class checking functionality.

use super::{ValidationError, ValidationRules};
use std::collections::HashSet;

/// Deprecated checker
pub struct DeprecatedChecker {
    rules: ValidationRules,
}

impl DeprecatedChecker {
    /// Create new deprecated checker
    pub fn new(rules: ValidationRules) -> Self {
        Self { rules }
    }

    /// Check if a class is deprecated
    pub fn is_deprecated(&self, class: &str) -> bool {
        self.rules.deprecated_classes.contains(class)
    }

    /// Get all deprecated classes from a list
    pub fn get_deprecated_classes(&self, classes: &[String]) -> Vec<String> {
        classes
            .iter()
            .filter(|class| self.is_deprecated(class))
            .cloned()
            .collect()
    }

    /// Check for deprecated classes and return errors
    pub fn check_deprecated(&self, classes: &[String]) -> Result<(), ValidationError> {
        for class in classes {
            if self.is_deprecated(class) {
                return Err(ValidationError::DeprecatedClass(class.clone()));
            }
        }
        Ok(())
    }
}
