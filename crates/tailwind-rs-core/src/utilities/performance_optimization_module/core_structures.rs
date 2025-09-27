//! Core performance optimization structures
//! 
//! This module contains the core data structures for performance optimization.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt;

/// Class usage analyzer for tree-shaking
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClassAnalyzer {
    /// Set of used classes
    pub used_classes: HashSet<String>,
    /// Set of unused classes
    pub unused_classes: HashSet<String>,
    /// Class dependencies mapping
    pub dependencies: HashMap<String, HashSet<String>>,
    /// Critical classes that should never be removed
    pub critical_classes: HashSet<String>,
}

/// CSS purger for dead code elimination
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CssPurger {
    /// Classes to keep
    pub keep_classes: HashSet<String>,
    /// Classes to remove
    pub remove_classes: HashSet<String>,
    /// CSS rules to keep
    pub keep_rules: HashSet<String>,
    /// CSS rules to remove
    pub remove_rules: HashSet<String>,
}

/// Performance optimization result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OptimizationResult {
    /// Original CSS size in bytes
    pub original_size: usize,
    /// Optimized CSS size in bytes
    pub optimized_size: usize,
    /// Size reduction percentage
    pub reduction_percentage: f32,
    /// Number of classes removed
    pub classes_removed: usize,
    /// Number of rules removed
    pub rules_removed: usize,
    /// Optimization warnings
    pub warnings: Vec<String>,
}

impl Default for ClassAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl ClassAnalyzer {
    /// Create a new class analyzer
    pub fn new() -> Self {
        Self {
            used_classes: HashSet::new(),
            unused_classes: HashSet::new(),
            dependencies: HashMap::new(),
            critical_classes: HashSet::new(),
        }
    }

    /// Add a used class
    pub fn add_used_class(&mut self, class: String) {
        self.used_classes.insert(class);
    }

    /// Add multiple used classes
    pub fn add_used_classes(&mut self, classes: Vec<String>) {
        for class in classes {
            self.used_classes.insert(class);
        }
    }

    /// Add a class dependency
    pub fn add_dependency(&mut self, class: String, dependency: String) {
        self.dependencies
            .entry(class)
            .or_default()
            .insert(dependency);
    }

    /// Mark a class as critical
    pub fn mark_critical(&mut self, class: String) {
        self.critical_classes.insert(class);
    }

    /// Analyze class usage and identify unused classes
    pub fn analyze_usage(&mut self, all_classes: HashSet<String>) {
        // Find unused classes
        self.unused_classes = all_classes
            .difference(&self.used_classes)
            .cloned()
            .collect();

        // Remove critical classes from unused list
        self.unused_classes = self
            .unused_classes
            .difference(&self.critical_classes)
            .cloned()
            .collect();

        // Add dependent classes to used classes
        let mut to_check = self.used_classes.clone();
        while !to_check.is_empty() {
            let mut new_dependencies = HashSet::new();
            for class in &to_check {
                if let Some(deps) = self.dependencies.get(class) {
                    for dep in deps {
                        if !self.used_classes.contains(dep) {
                            new_dependencies.insert(dep.clone());
                            self.used_classes.insert(dep.clone());
                        }
                    }
                }
            }
            to_check = new_dependencies;
        }
    }

    /// Get optimization suggestions
    pub fn get_optimization_suggestions(&self) -> Vec<String> {
        let mut suggestions = Vec::new();

        if !self.unused_classes.is_empty() {
            suggestions.push(format!(
                "Remove {} unused classes to reduce bundle size",
                self.unused_classes.len()
            ));
        }

        let critical_count = self.critical_classes.len();
        if critical_count > 0 {
            suggestions.push(format!(
                "{} critical classes are protected from removal",
                critical_count
            ));
        }

        let dependency_count: usize = self.dependencies.values().map(|deps| deps.len()).sum();
        if dependency_count > 0 {
            suggestions.push(format!(
                "Found {} class dependencies that may affect optimization",
                dependency_count
            ));
        }

        suggestions
    }
}

impl Default for CssPurger {
    fn default() -> Self {
        Self::new()
    }
}

impl CssPurger {
    /// Create a new CSS purger
    pub fn new() -> Self {
        Self {
            keep_classes: HashSet::new(),
            remove_classes: HashSet::new(),
            keep_rules: HashSet::new(),
            remove_rules: HashSet::new(),
        }
    }

    /// Add classes to keep
    pub fn keep_classes(&mut self, classes: HashSet<String>) {
        self.keep_classes.extend(classes);
    }

    /// Add classes to remove
    pub fn remove_classes(&mut self, classes: HashSet<String>) {
        self.remove_classes.extend(classes);
    }

    /// Purge CSS by removing unused classes and rules
    pub fn purge_css(&self, css: &str) -> String {
        let mut result = String::new();
        let lines: Vec<&str> = css.lines().collect();

        let mut in_rule = false;
        let mut current_rule = String::new();
        let mut rule_selectors = Vec::new();

        for line in lines {
            let trimmed = line.trim();

            if trimmed.ends_with('{') {
                // Start of a CSS rule
                in_rule = true;
                current_rule = line.to_string();
                rule_selectors = self.extract_selectors(trimmed);
            } else if trimmed == "}" && in_rule {
                // End of a CSS rule
                current_rule.push_str(&format!("{}\n", line));

                if self.should_keep_rule(&rule_selectors) {
                    result.push_str(&current_rule);
                }

                in_rule = false;
                current_rule.clear();
                rule_selectors.clear();
            } else if in_rule {
                // Inside a CSS rule
                current_rule.push_str(&format!("{}\n", line));
            } else {
                // Outside of rules (comments, imports, etc.)
                result.push_str(&format!("{}\n", line));
            }
        }

        result
    }

    /// Extract selectors from a CSS rule line
    fn extract_selectors(&self, line: &str) -> Vec<String> {
        let selector_part = line.trim_end_matches(" {");
        selector_part
            .split(',')
            .map(|s| s.trim().to_string())
            .collect()
    }

    /// Check if a rule should be kept
    fn should_keep_rule(&self, selectors: &[String]) -> bool {
        for selector in selectors {
            if self.should_keep_selector(selector) {
                return true;
            }
        }
        false
    }

    /// Check if a selector should be kept
    fn should_keep_selector(&self, selector: &str) -> bool {
        // Keep critical selectors
        if selector.starts_with('*') || selector.starts_with("html") || selector.starts_with("body")
        {
            return true;
        }

        // Check if selector contains any classes we want to keep
        for class in &self.keep_classes {
            if selector.contains(&format!(".{}", class)) {
                return true;
            }
        }

        // Check if selector contains any classes we want to remove
        for class in &self.remove_classes {
            if selector.contains(&format!(".{}", class)) {
                return false;
            }
        }

        // Keep selectors that don't contain classes (element selectors, etc.)
        !selector.contains('.')
    }

    /// Calculate optimization result
    pub fn calculate_optimization(
        &self,
        original_css: &str,
        optimized_css: &str,
    ) -> OptimizationResult {
        let original_size = original_css.len();
        let optimized_size = optimized_css.len();
        let reduction_percentage = if original_size > 0 {
            ((original_size - optimized_size) as f32 / original_size as f32) * 100.0
        } else {
            0.0
        };

        let classes_removed = self.remove_classes.len();
        let rules_removed = self.remove_rules.len();

        let mut warnings = Vec::new();
        if reduction_percentage > 50.0 {
            warnings.push(
                "Large size reduction detected. Verify all functionality still works.".to_string(),
            );
        }
        if classes_removed > 100 {
            warnings.push("Many classes removed. Check for missing styles.".to_string());
        }

        OptimizationResult {
            original_size,
            optimized_size,
            reduction_percentage,
            classes_removed,
            rules_removed,
            warnings,
        }
    }
}

impl fmt::Display for OptimizationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Optimization Result: {} bytes -> {} bytes ({}% reduction)",
            self.original_size,
            self.optimized_size,
            self.reduction_percentage
        )
    }
}
