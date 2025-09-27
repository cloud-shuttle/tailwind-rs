//! Validation rules
//!
//! This module provides validation rules for class validation.

use std::collections::{HashMap, HashSet};

/// Validation rules for class validation
#[derive(Debug, Clone)]
pub struct ValidationRules {
    /// Allowed class patterns
    pub allowed_patterns: Vec<regex::Regex>,
    /// Forbidden class patterns
    pub forbidden_patterns: Vec<regex::Regex>,
    /// Deprecated classes
    pub deprecated_classes: HashSet<String>,
    /// Class conflicts (classes that can't be used together)
    pub class_conflicts: HashMap<String, HashSet<String>>,
    /// Required classes (classes that must be present when certain classes are used)
    pub required_classes: HashMap<String, HashSet<String>>,
}

impl Default for ValidationRules {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidationRules {
    /// Create new validation rules
    pub fn new() -> Self {
        Self {
            allowed_patterns: vec![],
            forbidden_patterns: vec![],
            deprecated_classes: HashSet::new(),
            class_conflicts: HashMap::new(),
            required_classes: HashMap::new(),
        }
    }

    /// Add allowed pattern
    pub fn add_allowed_pattern(&mut self, pattern: regex::Regex) {
        self.allowed_patterns.push(pattern);
    }

    /// Add forbidden pattern
    pub fn add_forbidden_pattern(&mut self, pattern: regex::Regex) {
        self.forbidden_patterns.push(pattern);
    }

    /// Add deprecated class
    pub fn add_deprecated_class(&mut self, class: String) {
        self.deprecated_classes.insert(class);
    }

    /// Add class conflict
    pub fn add_class_conflict(&mut self, class1: String, class2: String) {
        self.class_conflicts
            .entry(class1)
            .or_insert_with(HashSet::new)
            .insert(class2);
    }

    /// Add required class
    pub fn add_required_class(&mut self, class: String, required: String) {
        self.required_classes
            .entry(class)
            .or_insert_with(HashSet::new)
            .insert(required);
    }
}
