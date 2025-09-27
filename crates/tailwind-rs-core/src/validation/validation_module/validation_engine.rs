//! Validation engine
//!
//! This module provides the main validation engine functionality.

use super::{
    ClassValidator, ConflictDetector, CustomVariantValidator, DeprecatedChecker, ValidationError,
    ValidationRules,
};
use crate::custom_variant::CustomVariantManager;
use std::collections::HashSet;

/// Validation engine
pub struct ValidationEngine {
    class_validator: ClassValidator,
    conflict_detector: ConflictDetector,
    deprecated_checker: DeprecatedChecker,
    custom_variant_validator: CustomVariantValidator,
}

impl ValidationEngine {
    /// Create new validation engine
    pub fn new(rules: ValidationRules, variant_manager: CustomVariantManager) -> Self {
        Self {
            class_validator: ClassValidator::new(rules.clone()),
            conflict_detector: ConflictDetector::new(rules.clone()),
            deprecated_checker: DeprecatedChecker::new(rules.clone()),
            custom_variant_validator: CustomVariantValidator::new(rules, variant_manager),
        }
    }

    /// Validate classes comprehensively
    pub fn validate_classes(&self, classes: &[String]) -> Result<(), ValidationError> {
        // Validate individual classes
        self.class_validator.validate_classes(classes)?;

        // Check for conflicts
        self.conflict_detector.detect_conflicts(classes)?;

        // Check for deprecated classes
        self.deprecated_checker.check_deprecated(classes)?;

        // Validate custom variants
        self.custom_variant_validator.validate_custom_variants(classes)?;

        Ok(())
    }

    /// Get validation summary
    pub fn get_validation_summary(&self, classes: &[String]) -> ValidationSummary {
        let deprecated = self.deprecated_checker.get_deprecated_classes(classes);
        let conflicts = self.detect_conflicts(classes);
        let warnings = deprecated.len() + conflicts.len();

        ValidationSummary {
            total_classes: classes.len(),
            warnings,
            errors: 0,
            deprecated_classes: deprecated,
            conflicts,
        }
    }

    /// Detect conflicts (helper method)
    fn detect_conflicts(&self, classes: &[String]) -> Vec<String> {
        let mut conflicts = Vec::new();
        let class_set: HashSet<String> = classes.iter().cloned().collect();

        for class in classes {
            if let Some(conflict_classes) = self.conflict_detector.rules().class_conflicts.get(class) {
                for conflict in conflict_classes {
                    if class_set.contains(conflict) {
                        conflicts.push(format!("{} conflicts with {}", class, conflict));
                    }
                }
            }
        }

        conflicts
    }
}

/// Validation summary
#[derive(Debug, Clone)]
pub struct ValidationSummary {
    pub total_classes: usize,
    pub warnings: usize,
    pub errors: usize,
    pub deprecated_classes: Vec<String>,
    pub conflicts: Vec<String>,
}
