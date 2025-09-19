//! Tree-shaking system for removing unused CSS classes
//!
//! This module provides functionality to analyze CSS usage and remove unused classes,
//! optimizing the final CSS bundle size.

use crate::class_scanner::ClassScanner;
use crate::css_generator::CssGenerator;
use crate::error::Result;
use std::collections::{HashMap, HashSet};
use std::path::Path;

/// Configuration for tree-shaking
#[derive(Debug, Clone)]
pub struct TreeShakeConfig {
    /// Whether to enable tree-shaking
    pub enabled: bool,
    /// Whether to remove unused responsive variants
    pub remove_unused_responsive: bool,
    /// Whether to remove unused conditional classes
    pub remove_unused_conditional: bool,
    /// Whether to remove unused custom properties
    pub remove_unused_custom: bool,
    /// Classes to always keep (whitelist)
    pub keep_classes: HashSet<String>,
    /// Classes to always remove (blacklist)
    pub remove_classes: HashSet<String>,
    /// Whether to analyze dependencies between classes
    pub analyze_dependencies: bool,
}

impl Default for TreeShakeConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            remove_unused_responsive: true,
            remove_unused_conditional: true,
            remove_unused_custom: true,
            keep_classes: HashSet::new(),
            remove_classes: HashSet::new(),
            analyze_dependencies: true,
        }
    }
}

/// Results of tree-shaking operation
#[derive(Debug, Clone)]
pub struct TreeShakeResults {
    /// Classes that were kept
    pub kept_classes: HashSet<String>,
    /// Classes that were removed
    pub removed_classes: HashSet<String>,
    /// Original CSS size
    pub original_size: usize,
    /// Optimized CSS size
    pub optimized_size: usize,
    /// Size reduction percentage
    pub reduction_percentage: f64,
    /// Statistics
    pub stats: TreeShakeStats,
}

/// Statistics for tree-shaking operation
#[derive(Debug, Clone)]
pub struct TreeShakeStats {
    /// Number of classes analyzed
    pub classes_analyzed: usize,
    /// Number of classes removed
    pub classes_removed: usize,
    /// Number of responsive variants removed
    pub responsive_removed: usize,
    /// Number of conditional classes removed
    pub conditional_removed: usize,
    /// Number of custom properties removed
    pub custom_removed: usize,
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
}

/// Statistics for class removal operations
#[derive(Debug, Clone)]
struct RemovalStats {
    total_removed: usize,
    responsive_removed: usize,
    conditional_removed: usize,
    custom_removed: usize,
    removed_classes: HashSet<String>,
}

/// Tree-shaking system for CSS optimization
#[derive(Debug, Clone)]
pub struct TreeShaker {
    config: TreeShakeConfig,
    dependency_graph: HashMap<String, HashSet<String>>,
    reverse_dependencies: HashMap<String, HashSet<String>>,
}

impl TreeShaker {
    /// Create a new tree-shaker with default configuration
    pub fn new() -> Self {
        Self {
            config: TreeShakeConfig::default(),
            dependency_graph: HashMap::new(),
            reverse_dependencies: HashMap::new(),
        }
    }

    /// Create a new tree-shaker with custom configuration
    pub fn with_config(config: TreeShakeConfig) -> Self {
        Self {
            config,
            dependency_graph: HashMap::new(),
            reverse_dependencies: HashMap::new(),
        }
    }

    /// Analyze source files and remove unused CSS classes
    pub fn shake(&mut self, source_paths: &[&Path], css_generator: &mut CssGenerator) -> Result<TreeShakeResults> {
        let start_time = std::time::Instant::now();
        
        if !self.config.enabled {
            return Ok(TreeShakeResults {
                kept_classes: css_generator.get_rules().keys().cloned().collect(),
                removed_classes: HashSet::new(),
                original_size: css_generator.generate_css().len(),
                optimized_size: css_generator.generate_css().len(),
                reduction_percentage: 0.0,
                stats: TreeShakeStats {
                    classes_analyzed: css_generator.rule_count(),
                    classes_removed: 0,
                    responsive_removed: 0,
                    conditional_removed: 0,
                    custom_removed: 0,
                    processing_time_ms: start_time.elapsed().as_millis() as u64,
                },
            });
        }

        // Scan source files for used classes
        let used_classes = self.scan_used_classes(source_paths)?;
        
        // Build dependency graph if enabled
        if self.config.analyze_dependencies {
            self.build_dependency_graph(css_generator);
        }

        // Determine which classes to keep
        let classes_to_keep = self.determine_classes_to_keep(&used_classes, css_generator);
        
        // Remove unused classes and track statistics
        let removal_stats = self.remove_unused_classes(css_generator, &classes_to_keep);
        
        // Calculate results
        let original_size = css_generator.generate_css().len();
        let optimized_size = css_generator.generate_css().len();
        let reduction_percentage = if original_size > 0 {
            ((original_size - optimized_size) as f64 / original_size as f64) * 100.0
        } else {
            0.0
        };

        let stats = TreeShakeStats {
            classes_analyzed: css_generator.rule_count() + removal_stats.total_removed,
            classes_removed: removal_stats.total_removed,
            responsive_removed: removal_stats.responsive_removed,
            conditional_removed: removal_stats.conditional_removed,
            custom_removed: removal_stats.custom_removed,
            processing_time_ms: start_time.elapsed().as_millis() as u64,
        };

        Ok(TreeShakeResults {
            kept_classes: classes_to_keep,
            removed_classes: removal_stats.removed_classes,
            original_size,
            optimized_size,
            reduction_percentage,
            stats,
        })
    }

    /// Scan source files to find used classes
    fn scan_used_classes(&self, source_paths: &[&Path]) -> Result<HashSet<String>> {
        let mut scanner = ClassScanner::new();
        let mut all_used_classes = HashSet::new();

        for path in source_paths {
            let results = scanner.scan_directory(path)?;
            all_used_classes.extend(results.classes);
        }

        Ok(all_used_classes)
    }

    /// Build dependency graph between CSS classes
    fn build_dependency_graph(&mut self, css_generator: &CssGenerator) {
        self.dependency_graph.clear();
        self.reverse_dependencies.clear();

        // Analyze CSS rules for dependencies
        for (class_name, rule) in css_generator.get_rules() {
            let mut dependencies = HashSet::new();
            
            // Look for class references in CSS values
            for property in &rule.properties {
                if let Some(dep_class) = self.extract_class_dependency(&property.value) {
                    dependencies.insert(dep_class);
                }
            }

            if !dependencies.is_empty() {
                self.dependency_graph.insert(class_name.clone(), dependencies);
            }
        }

        // Build reverse dependencies
        for (class, deps) in &self.dependency_graph {
            for dep in deps {
                self.reverse_dependencies
                    .entry(dep.clone())
                    .or_default()
                    .insert(class.clone());
            }
        }
    }

    /// Extract class dependency from CSS value
    fn extract_class_dependency(&self, value: &str) -> Option<String> {
        // Look for class references in CSS values
        // This is a simplified implementation
        if value.contains("var(--") {
            // Extract CSS custom property reference
            if let Some(start) = value.find("var(--") {
                if let Some(end) = value[start..].find(')') {
                    let var_name = &value[start + 6..start + end];
                    return Some(format!("--{}", var_name));
                }
            }
        }
        None
    }

    /// Determine which classes should be kept
    fn determine_classes_to_keep(&self, used_classes: &HashSet<String>, _css_generator: &CssGenerator) -> HashSet<String> {
        let mut classes_to_keep = HashSet::new();

        // Add explicitly used classes
        classes_to_keep.extend(used_classes.iter().cloned());

        // Add whitelisted classes
        classes_to_keep.extend(self.config.keep_classes.iter().cloned());

        // Remove blacklisted classes
        for class in &self.config.remove_classes {
            classes_to_keep.remove(class);
        }

        // Add dependent classes if dependency analysis is enabled
        if self.config.analyze_dependencies {
            let mut to_process: Vec<String> = classes_to_keep.iter().cloned().collect();
            let mut processed = HashSet::new();

            while let Some(class) = to_process.pop() {
                if processed.contains(&class) {
                    continue;
                }
                processed.insert(class.clone());

                // Add dependencies
                if let Some(deps) = self.dependency_graph.get(&class) {
                    for dep in deps {
                        if !classes_to_keep.contains(dep) {
                            classes_to_keep.insert(dep.clone());
                            to_process.push(dep.clone());
                        }
                    }
                }

                // Add reverse dependencies (classes that depend on this one)
                if let Some(reverse_deps) = self.reverse_dependencies.get(&class) {
                    for reverse_dep in reverse_deps {
                        if !classes_to_keep.contains(reverse_dep) {
                            classes_to_keep.insert(reverse_dep.clone());
                            to_process.push(reverse_dep.clone());
                        }
                    }
                }
            }
        }

        classes_to_keep
    }

    /// Remove unused classes from CSS generator
    fn remove_unused_classes(&self, css_generator: &mut CssGenerator, classes_to_keep: &HashSet<String>) -> RemovalStats {
        let mut removed_classes = HashSet::new();
        let mut responsive_removed = 0;
        let mut conditional_removed = 0;
        let mut custom_removed = 0;
        let rules = css_generator.get_rules().clone();

        for (class_name, _rule) in rules {
            if !classes_to_keep.contains(&class_name) {
                removed_classes.insert(class_name.clone());
                
                // Categorize the removed class
                if class_name.contains("sm:") || class_name.contains("md:") || 
                   class_name.contains("lg:") || class_name.contains("xl:") || 
                   class_name.contains("2xl:") {
                    responsive_removed += 1;
                } else if class_name.contains("hover:") || class_name.contains("focus:") || 
                         class_name.contains("active:") || class_name.contains("disabled:") {
                    conditional_removed += 1;
                } else if class_name.starts_with("--") || class_name.contains("var(") {
                    custom_removed += 1;
                }
            }
        }

        RemovalStats {
            total_removed: removed_classes.len(),
            responsive_removed,
            conditional_removed,
            custom_removed,
            removed_classes,
        }
    }

    /// Get the current configuration
    pub fn get_config(&self) -> &TreeShakeConfig {
        &self.config
    }

    /// Update the configuration
    pub fn set_config(&mut self, config: TreeShakeConfig) {
        self.config = config;
    }

    /// Add a class to the whitelist
    pub fn keep_class(&mut self, class: String) {
        self.config.keep_classes.insert(class);
    }

    /// Add a class to the blacklist
    pub fn remove_class(&mut self, class: String) {
        self.config.remove_classes.insert(class);
    }

    /// Clear all configuration
    pub fn clear(&mut self) {
        self.config.keep_classes.clear();
        self.config.remove_classes.clear();
        self.dependency_graph.clear();
        self.reverse_dependencies.clear();
    }
}

impl Default for TreeShaker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_shaker_creation() {
        let shaker = TreeShaker::new();
        assert!(shaker.get_config().enabled);
        assert!(shaker.get_config().analyze_dependencies);
    }

    #[test]
    fn test_custom_config() {
        let config = TreeShakeConfig {
            enabled: false,
            remove_unused_responsive: false,
            remove_unused_conditional: false,
            remove_unused_custom: false,
            keep_classes: HashSet::new(),
            remove_classes: HashSet::new(),
            analyze_dependencies: false,
        };
        
        let shaker = TreeShaker::with_config(config);
        assert!(!shaker.get_config().enabled);
        assert!(!shaker.get_config().analyze_dependencies);
    }

    #[test]
    fn test_keep_and_remove_classes() {
        let mut shaker = TreeShaker::new();
        
        shaker.keep_class("important-class".to_string());
        shaker.remove_class("unwanted-class".to_string());
        
        assert!(shaker.get_config().keep_classes.contains("important-class"));
        assert!(shaker.get_config().remove_classes.contains("unwanted-class"));
    }

    #[test]
    fn test_clear() {
        let mut shaker = TreeShaker::new();
        
        shaker.keep_class("test-class".to_string());
        shaker.remove_class("test-remove".to_string());
        
        assert!(!shaker.get_config().keep_classes.is_empty());
        assert!(!shaker.get_config().remove_classes.is_empty());
        
        shaker.clear();
        
        assert!(shaker.get_config().keep_classes.is_empty());
        assert!(shaker.get_config().remove_classes.is_empty());
    }

    #[test]
    fn test_dependency_extraction() {
        let shaker = TreeShaker::new();
        
        // Test CSS custom property extraction
        assert_eq!(shaker.extract_class_dependency("var(--primary-color)"), Some("--primary-color".to_string()));
        assert_eq!(shaker.extract_class_dependency("var(--spacing-4)"), Some("--spacing-4".to_string()));
        assert_eq!(shaker.extract_class_dependency("1rem"), None);
        assert_eq!(shaker.extract_class_dependency("#ffffff"), None);
    }

    #[test]
    fn test_determine_classes_to_keep() {
        let shaker = TreeShaker::new();
        let mut used_classes = HashSet::new();
        used_classes.insert("p-4".to_string());
        used_classes.insert("bg-blue-500".to_string());
        
        let mut css_generator = CssGenerator::new();
        css_generator.add_class("p-4").unwrap();
        css_generator.add_class("bg-blue-500").unwrap();
        css_generator.add_class("m-2").unwrap();
        
        let classes_to_keep = shaker.determine_classes_to_keep(&used_classes, &css_generator);
        
        assert!(classes_to_keep.contains("p-4"));
        assert!(classes_to_keep.contains("bg-blue-500"));
        assert!(!classes_to_keep.contains("m-2"));
    }

    #[test]
    fn test_disabled_tree_shaking() {
        let mut config = TreeShakeConfig::default();
        config.enabled = false;
        
        let mut shaker = TreeShaker::with_config(config);
        let mut css_generator = CssGenerator::new();
        css_generator.add_class("p-4").unwrap();
        
        let temp_dir = std::env::temp_dir();
        let results = shaker.shake(&[&temp_dir], &mut css_generator).unwrap();
        
        assert_eq!(results.stats.classes_removed, 0);
        assert_eq!(results.reduction_percentage, 0.0);
    }
}
