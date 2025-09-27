//! Bundle analysis and optimization utilities
//! 
//! This module contains bundle analysis functionality for performance optimization.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt;

/// Bundle analyzer for performance insights
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BundleAnalyzer {
    /// Class usage statistics
    pub class_stats: HashMap<String, ClassUsageStats>,
    /// CSS rule statistics
    pub rule_stats: HashMap<String, RuleUsageStats>,
    /// Performance metrics
    pub metrics: PerformanceMetrics,
}

/// Class usage statistics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClassUsageStats {
    /// Number of times the class is used
    pub usage_count: u32,
    /// Files where the class is used
    pub used_in_files: HashSet<String>,
    /// Whether the class is critical
    pub is_critical: bool,
    /// Dependencies of this class
    pub dependencies: HashSet<String>,
}

/// CSS rule usage statistics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuleUsageStats {
    /// Number of times the rule is used
    pub usage_count: u32,
    /// Selectors in the rule
    pub selectors: HashSet<String>,
    /// Properties in the rule
    pub properties: HashSet<String>,
    /// Rule size in bytes
    pub size_bytes: usize,
}

/// Performance metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Total bundle size in bytes
    pub total_size: usize,
    /// Number of classes
    pub class_count: usize,
    /// Number of CSS rules
    pub rule_count: usize,
    /// Average class size in bytes
    pub avg_class_size: f32,
    /// Average rule size in bytes
    pub avg_rule_size: f32,
    /// Compression ratio
    pub compression_ratio: f32,
}

impl Default for BundleAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl BundleAnalyzer {
    /// Create a new bundle analyzer
    pub fn new() -> Self {
        Self {
            class_stats: HashMap::new(),
            rule_stats: HashMap::new(),
            metrics: PerformanceMetrics {
                total_size: 0,
                class_count: 0,
                rule_count: 0,
                avg_class_size: 0.0,
                avg_rule_size: 0.0,
                compression_ratio: 0.0,
            },
        }
    }

    /// Analyze a CSS bundle
    pub fn analyze_bundle(&mut self, css: &str) {
        self.analyze_classes(css);
        self.analyze_rules(css);
        self.calculate_metrics(css);
    }

    /// Analyze class usage in CSS
    fn analyze_classes(&mut self, css: &str) {
        let lines: Vec<&str> = css.lines().collect();

        for line in lines {
            if line.contains('.') && line.contains('{') {
                let selectors = self.extract_selectors(line);
                for selector in selectors {
                    if let Some(class_name) = self.extract_class_name(&selector) {
                        let stats =
                            self.class_stats
                                .entry(class_name.clone())
                                .or_insert_with(|| ClassUsageStats {
                                    usage_count: 0,
                                    used_in_files: HashSet::new(),
                                    is_critical: false,
                                    dependencies: HashSet::new(),
                                });
                        stats.usage_count += 1;
                    }
                }
            }
        }
    }

    /// Analyze CSS rules
    fn analyze_rules(&mut self, css: &str) {
        let lines: Vec<&str> = css.lines().collect();
        let mut current_rule = String::new();
        let mut in_rule = false;

        for line in lines {
            let trimmed = line.trim();

            if trimmed.ends_with('{') {
                in_rule = true;
                current_rule = line.to_string();
            } else if trimmed == "}" && in_rule {
                current_rule.push_str(&format!("{}\n", line));

                let rule_id = format!("rule_{}", self.rule_stats.len());
                let selectors = self.extract_selectors(&current_rule);
                let properties = self.extract_properties(&current_rule);

                self.rule_stats.insert(
                    rule_id,
                    RuleUsageStats {
                        usage_count: 1,
                        selectors: selectors.into_iter().collect(),
                        properties: properties.into_iter().collect(),
                        size_bytes: current_rule.len(),
                    },
                );

                in_rule = false;
                current_rule.clear();
            } else if in_rule {
                current_rule.push_str(&format!("{}\n", line));
            }
        }
    }

    /// Calculate performance metrics
    fn calculate_metrics(&mut self, css: &str) {
        self.metrics.total_size = css.len();
        self.metrics.class_count = self.class_stats.len();
        self.metrics.rule_count = self.rule_stats.len();

        if self.metrics.class_count > 0 {
            let total_class_size: usize = self
                .class_stats
                .values()
                .map(|stats| stats.usage_count as usize * 10) // Estimate class size
                .sum();
            self.metrics.avg_class_size = total_class_size as f32 / self.metrics.class_count as f32;
        }

        if self.metrics.rule_count > 0 {
            let total_rule_size: usize =
                self.rule_stats.values().map(|stats| stats.size_bytes).sum();
            self.metrics.avg_rule_size = total_rule_size as f32 / self.metrics.rule_count as f32;
        }

        // Estimate compression ratio (simplified)
        self.metrics.compression_ratio = if self.metrics.total_size > 0 {
            (self.metrics.total_size as f32 - self.metrics.avg_rule_size)
                / self.metrics.total_size as f32
        } else {
            0.0
        };
    }

    /// Extract selectors from a CSS line
    fn extract_selectors(&self, line: &str) -> Vec<String> {
        let selector_part = line.trim_end_matches(" {");
        selector_part
            .split(',')
            .map(|s| s.trim().to_string())
            .collect()
    }

    /// Extract class name from a selector
    fn extract_class_name(&self, selector: &str) -> Option<String> {
        if let Some(start) = selector.find('.') {
            let class_part = &selector[start + 1..];
            if let Some(end) =
                class_part.find(|c: char| !c.is_alphanumeric() && c != '-' && c != '_')
            {
                Some(class_part[..end].to_string())
            } else {
                Some(class_part.to_string())
            }
        } else {
            None
        }
    }

    /// Extract properties from a CSS rule
    fn extract_properties(&self, rule: &str) -> Vec<String> {
        let mut properties = Vec::new();
        let lines: Vec<&str> = rule.lines().collect();

        for line in lines {
            let trimmed = line.trim();
            if trimmed.contains(':') && !trimmed.ends_with('{') && !trimmed.ends_with('}') {
                if let Some(colon_pos) = trimmed.find(':') {
                    let property = trimmed[..colon_pos].trim().to_string();
                    properties.push(property);
                }
            }
        }

        properties
    }

    /// Get performance recommendations
    pub fn get_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();

        if self.metrics.total_size > 100_000 {
            recommendations.push("Bundle size is large. Consider code splitting.".to_string());
        }

        if self.metrics.class_count > 1000 {
            recommendations
                .push("Many classes detected. Consider purging unused classes.".to_string());
        }

        if self.metrics.avg_rule_size > 200.0 {
            recommendations
                .push("Large CSS rules detected. Consider breaking them down.".to_string());
        }

        if self.metrics.compression_ratio < 0.3 {
            recommendations.push("Low compression ratio. Consider optimization.".to_string());
        }

        let unused_classes: Vec<_> = self
            .class_stats
            .iter()
            .filter(|(_, stats)| stats.usage_count == 1)
            .map(|(name, _)| name)
            .collect();

        if unused_classes.len() > 50 {
            recommendations.push(format!(
                "{} classes used only once. Consider consolidation.",
                unused_classes.len()
            ));
        }

        recommendations
    }
}

impl fmt::Display for PerformanceMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Bundle: {} bytes, {} classes, {} rules, {:.1}% compression",
            self.total_size,
            self.class_count,
            self.rule_count,
            self.compression_ratio * 100.0
        )
    }
}
