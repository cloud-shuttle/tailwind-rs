//! # Visual Test Utilities
//!
//! This module provides core utilities for capturing and comparing component rendering
//! to ensure visual consistency.

use std::collections::HashMap;

/// Visual test utilities for capturing and comparing component rendering
pub struct VisualTestUtils;

impl VisualTestUtils {
    /// Capture the visual state of a component
    pub fn capture_component_state(component_name: &str, classes: &str) -> VisualSnapshot {
        VisualSnapshot {
            component_name: component_name.to_string(),
            classes: classes.to_string(),
            timestamp: std::time::SystemTime::now(),
            metadata: HashMap::new(),
        }
    }
    
    /// Compare two visual snapshots
    pub fn compare_snapshots(snapshot1: &VisualSnapshot, snapshot2: &VisualSnapshot) -> VisualComparison {
        let classes_match = snapshot1.classes == snapshot2.classes;
        let time_diff = snapshot2.timestamp.duration_since(snapshot1.timestamp)
            .unwrap_or_default();
        
        VisualComparison {
            component_name: snapshot1.component_name.clone(),
            classes_match,
            time_difference: time_diff,
            differences: if classes_match {
                Vec::new()
            } else {
                vec![VisualDifference {
                    field: "classes".to_string(),
                    old_value: snapshot1.classes.clone(),
                    new_value: snapshot2.classes.clone(),
                }]
            },
        }
    }
    
    /// Validate that a component's classes are consistent
    pub fn validate_component_consistency(component_name: &str, expected_classes: &str, actual_classes: &str) -> bool {
        expected_classes == actual_classes
    }
    
    /// Extract class differences between two class strings
    pub fn extract_class_differences(expected: &str, actual: &str) -> Vec<VisualDifference> {
        let expected_classes: std::collections::HashSet<&str> = expected.split_whitespace().collect();
        let actual_classes: std::collections::HashSet<&str> = actual.split_whitespace().collect();
        
        let mut differences = Vec::new();
        
        // Find missing classes
        for missing_class in expected_classes.difference(&actual_classes) {
            differences.push(VisualDifference {
                field: "missing_class".to_string(),
                old_value: missing_class.to_string(),
                new_value: String::new(),
            });
        }
        
        // Find extra classes
        for extra_class in actual_classes.difference(&expected_classes) {
            differences.push(VisualDifference {
                field: "extra_class".to_string(),
                old_value: String::new(),
                new_value: extra_class.to_string(),
            });
        }
        
        differences
    }
}

/// A snapshot of a component's visual state
#[derive(Debug, Clone)]
pub struct VisualSnapshot {
    pub component_name: String,
    pub classes: String,
    pub timestamp: std::time::SystemTime,
    pub metadata: HashMap<String, String>,
}

/// A comparison between two visual snapshots
#[derive(Debug, Clone)]
pub struct VisualComparison {
    pub component_name: String,
    pub classes_match: bool,
    pub time_difference: std::time::Duration,
    pub differences: Vec<VisualDifference>,
}

/// A specific difference found between two visual states
#[derive(Debug, Clone)]
pub struct VisualDifference {
    pub field: String,
    pub old_value: String,
    pub new_value: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capture_component_state() {
        let snapshot = VisualTestUtils::capture_component_state("Button", "px-4 py-2 bg-blue-600");
        
        assert_eq!(snapshot.component_name, "Button");
        assert_eq!(snapshot.classes, "px-4 py-2 bg-blue-600");
        assert!(snapshot.metadata.is_empty());
    }

    #[test]
    fn test_compare_snapshots_identical() {
        let snapshot1 = VisualTestUtils::capture_component_state("Button", "px-4 py-2 bg-blue-600");
        let snapshot2 = VisualTestUtils::capture_component_state("Button", "px-4 py-2 bg-blue-600");
        
        let comparison = VisualTestUtils::compare_snapshots(&snapshot1, &snapshot2);
        
        assert!(comparison.classes_match);
        assert!(comparison.differences.is_empty());
    }

    #[test]
    fn test_compare_snapshots_different() {
        let snapshot1 = VisualTestUtils::capture_component_state("Button", "px-4 py-2 bg-blue-600");
        let snapshot2 = VisualTestUtils::capture_component_state("Button", "px-4 py-2 bg-red-600");
        
        let comparison = VisualTestUtils::compare_snapshots(&snapshot1, &snapshot2);
        
        assert!(!comparison.classes_match);
        assert_eq!(comparison.differences.len(), 1);
        assert_eq!(comparison.differences[0].field, "classes");
    }

    #[test]
    fn test_validate_component_consistency() {
        assert!(VisualTestUtils::validate_component_consistency("Button", "px-4 py-2", "px-4 py-2"));
        assert!(!VisualTestUtils::validate_component_consistency("Button", "px-4 py-2", "px-4 py-3"));
    }

    #[test]
    fn test_extract_class_differences() {
        let differences = VisualTestUtils::extract_class_differences("px-4 py-2", "px-4 py-3");
        
        assert_eq!(differences.len(), 2);
        assert!(differences.iter().any(|d| d.field == "missing_class" && d.old_value == "py-2"));
        assert!(differences.iter().any(|d| d.field == "extra_class" && d.new_value == "py-3"));
    }
}
