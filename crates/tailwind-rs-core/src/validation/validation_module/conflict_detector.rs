//! Conflict detector
//!
//! This module provides class conflict detection functionality.

use super::{ValidationError, ValidationRules};
use std::collections::{HashMap, HashSet};

/// Conflict detector
pub struct ConflictDetector {
    rules: ValidationRules,
}

impl ConflictDetector {
    /// Create new conflict detector
    pub fn new(rules: ValidationRules) -> Self {
        Self { rules }
    }

    /// Get the validation rules
    pub fn rules(&self) -> &ValidationRules {
        &self.rules
    }

    /// Detect conflicts between classes
    pub fn detect_conflicts(&self, classes: &[String]) -> Result<(), ValidationError> {
        let class_set: HashSet<String> = classes.iter().cloned().collect();

        for class in classes {
            if let Some(conflicts) = self.rules.class_conflicts.get(class) {
                for conflict in conflicts {
                    if class_set.contains(conflict) {
                        return Err(ValidationError::ClassConflict(
                            class.clone(),
                            conflict.clone(),
                        ));
                    }
                }
            }
        }

        Ok(())
    }

    /// Check if two classes conflict
    pub fn classes_conflict(&self, class1: &str, class2: &str) -> bool {
        if let Some(conflicts) = self.rules.class_conflicts.get(class1) {
            conflicts.contains(class2)
        } else {
            false
        }
    }
}
