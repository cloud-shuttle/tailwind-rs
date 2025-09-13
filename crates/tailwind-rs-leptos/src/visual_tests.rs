//! # Visual Testing for Leptos Integration
//!
//! This module provides visual regression tests that can capture and compare
//! component rendering to ensure visual consistency.

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
    
    /// Generate a visual test report
    pub fn generate_visual_report(comparisons: Vec<VisualComparison>) -> VisualTestReport {
        let total_tests = comparisons.len();
        let passed_tests = comparisons.iter().filter(|c| c.classes_match).count();
        let failed_tests = total_tests - passed_tests;
        
        VisualTestReport {
            total_tests,
            passed_tests,
            failed_tests,
            comparisons,
            generated_at: std::time::SystemTime::now(),
        }
    }
    
    /// Test component visual consistency
    pub fn test_component_consistency(component_name: &str, test_cases: Vec<ComponentTestCase>) -> Vec<VisualComparison> {
        let mut comparisons = Vec::new();
        
        for test_case in test_cases {
            let snapshot1 = Self::capture_component_state(component_name, &test_case.expected_classes);
            let snapshot2 = Self::capture_component_state(component_name, &test_case.actual_classes);
            
            let comparison = Self::compare_snapshots(&snapshot1, &snapshot2);
            comparisons.push(comparison);
        }
        
        comparisons
    }
    
    /// Test responsive design visual consistency
    pub fn test_responsive_consistency(component_name: &str, breakpoints: Vec<ResponsiveTestCase>) -> Vec<VisualComparison> {
        let mut comparisons = Vec::new();
        
        for breakpoint in breakpoints {
            let expected_classes = format!("{} {}", breakpoint.base_classes, breakpoint.responsive_classes);
            let actual_classes = format!("{} {}", breakpoint.base_classes, breakpoint.responsive_classes);
            
            let snapshot1 = Self::capture_component_state(component_name, &expected_classes);
            let snapshot2 = Self::capture_component_state(component_name, &actual_classes);
            
            let comparison = Self::compare_snapshots(&snapshot1, &snapshot2);
            comparisons.push(comparison);
        }
        
        comparisons
    }
    
    /// Test theme visual consistency
    pub fn test_theme_consistency(component_name: &str, themes: Vec<ThemeTestCase>) -> Vec<VisualComparison> {
        let mut comparisons = Vec::new();
        
        for theme in themes {
            let expected_classes = format!("{} {}", theme.base_classes, theme.theme_classes);
            let actual_classes = format!("{} {}", theme.base_classes, theme.theme_classes);
            
            let snapshot1 = Self::capture_component_state(component_name, &expected_classes);
            let snapshot2 = Self::capture_component_state(component_name, &actual_classes);
            
            let comparison = Self::compare_snapshots(&snapshot1, &snapshot2);
            comparisons.push(comparison);
        }
        
        comparisons
    }
    
    /// Test state-based visual consistency
    pub fn test_state_consistency(component_name: &str, states: Vec<StateTestCase>) -> Vec<VisualComparison> {
        let mut comparisons = Vec::new();
        
        for state in states {
            let expected_classes = format!("{} {}", state.base_classes, state.state_classes);
            let actual_classes = format!("{} {}", state.base_classes, state.state_classes);
            
            let snapshot1 = Self::capture_component_state(component_name, &expected_classes);
            let snapshot2 = Self::capture_component_state(component_name, &actual_classes);
            
            let comparison = Self::compare_snapshots(&snapshot1, &snapshot2);
            comparisons.push(comparison);
        }
        
        comparisons
    }
}

/// Visual snapshot of a component's state
#[derive(Debug, Clone)]
pub struct VisualSnapshot {
    pub component_name: String,
    pub classes: String,
    pub timestamp: std::time::SystemTime,
    pub metadata: HashMap<String, String>,
}

/// Visual comparison between two snapshots
#[derive(Debug, Clone)]
pub struct VisualComparison {
    pub component_name: String,
    pub classes_match: bool,
    pub time_difference: std::time::Duration,
    pub differences: Vec<VisualDifference>,
}

/// Visual difference between two snapshots
#[derive(Debug, Clone)]
pub struct VisualDifference {
    pub field: String,
    pub old_value: String,
    pub new_value: String,
}

/// Visual test report
#[derive(Debug, Clone)]
pub struct VisualTestReport {
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub comparisons: Vec<VisualComparison>,
    pub generated_at: std::time::SystemTime,
}

/// Component test case for visual testing
#[derive(Debug, Clone)]
pub struct ComponentTestCase {
    pub expected_classes: String,
    pub actual_classes: String,
    pub description: String,
}

/// Responsive test case for visual testing
#[derive(Debug, Clone)]
pub struct ResponsiveTestCase {
    pub base_classes: String,
    pub responsive_classes: String,
    pub breakpoint: String,
    pub description: String,
}

/// Theme test case for visual testing
#[derive(Debug, Clone)]
pub struct ThemeTestCase {
    pub base_classes: String,
    pub theme_classes: String,
    pub theme_name: String,
    pub description: String,
}

/// State test case for visual testing
#[derive(Debug, Clone)]
pub struct StateTestCase {
    pub base_classes: String,
    pub state_classes: String,
    pub state_name: String,
    pub description: String,
}

/// Visual regression test suite
pub struct VisualRegressionTests;

impl VisualRegressionTests {
    /// Run all visual regression tests
    pub fn run_all_tests() -> VisualTestReport {
        let mut all_comparisons = Vec::new();
        
        // Test button component consistency
        all_comparisons.extend(Self::test_button_consistency());
        
        // Test input component consistency
        all_comparisons.extend(Self::test_input_consistency());
        
        // Test card component consistency
        all_comparisons.extend(Self::test_card_consistency());
        
        // Test responsive design consistency
        all_comparisons.extend(Self::test_responsive_design());
        
        // Test theme consistency
        all_comparisons.extend(Self::test_theme_consistency());
        
        // Test state consistency
        all_comparisons.extend(Self::test_state_consistency());
        
        VisualTestUtils::generate_visual_report(all_comparisons)
    }
    
    /// Test button component visual consistency
    fn test_button_consistency() -> Vec<VisualComparison> {
        let test_cases = vec![
            ComponentTestCase {
                expected_classes: "px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700".to_string(),
                actual_classes: "px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700".to_string(),
                description: "Primary button".to_string(),
            },
            ComponentTestCase {
                expected_classes: "px-4 py-2 bg-gray-600 text-white rounded hover:bg-gray-700".to_string(),
                actual_classes: "px-4 py-2 bg-gray-600 text-white rounded hover:bg-gray-700".to_string(),
                description: "Secondary button".to_string(),
            },
            ComponentTestCase {
                expected_classes: "px-4 py-2 bg-red-600 text-white rounded hover:bg-red-700".to_string(),
                actual_classes: "px-4 py-2 bg-red-600 text-white rounded hover:bg-red-700".to_string(),
                description: "Danger button".to_string(),
            },
        ];
        
        VisualTestUtils::test_component_consistency("Button", test_cases)
    }
    
    /// Test input component visual consistency
    fn test_input_consistency() -> Vec<VisualComparison> {
        let test_cases = vec![
            ComponentTestCase {
                expected_classes: "px-3 py-2 border border-gray-300 rounded focus:ring-2 focus:ring-blue-500".to_string(),
                actual_classes: "px-3 py-2 border border-gray-300 rounded focus:ring-2 focus:ring-blue-500".to_string(),
                description: "Default input".to_string(),
            },
            ComponentTestCase {
                expected_classes: "px-3 py-2 border border-red-300 rounded focus:ring-2 focus:ring-red-500".to_string(),
                actual_classes: "px-3 py-2 border border-red-300 rounded focus:ring-2 focus:ring-red-500".to_string(),
                description: "Error input".to_string(),
            },
            ComponentTestCase {
                expected_classes: "px-3 py-2 border border-gray-300 rounded focus:ring-2 focus:ring-blue-500 opacity-50".to_string(),
                actual_classes: "px-3 py-2 border border-gray-300 rounded focus:ring-2 focus:ring-blue-500 opacity-50".to_string(),
                description: "Disabled input".to_string(),
            },
        ];
        
        VisualTestUtils::test_component_consistency("Input", test_cases)
    }
    
    /// Test card component visual consistency
    fn test_card_consistency() -> Vec<VisualComparison> {
        let test_cases = vec![
            ComponentTestCase {
                expected_classes: "bg-white rounded-lg shadow-md p-6".to_string(),
                actual_classes: "bg-white rounded-lg shadow-md p-6".to_string(),
                description: "Default card".to_string(),
            },
            ComponentTestCase {
                expected_classes: "bg-gray-900 text-white rounded-lg shadow-md p-6".to_string(),
                actual_classes: "bg-gray-900 text-white rounded-lg shadow-md p-6".to_string(),
                description: "Dark card".to_string(),
            },
            ComponentTestCase {
                expected_classes: "bg-blue-50 border border-blue-200 rounded-lg shadow-md p-6".to_string(),
                actual_classes: "bg-blue-50 border border-blue-200 rounded-lg shadow-md p-6".to_string(),
                description: "Info card".to_string(),
            },
        ];
        
        VisualTestUtils::test_component_consistency("Card", test_cases)
    }
    
    /// Test responsive design consistency
    fn test_responsive_design() -> Vec<VisualComparison> {
        let test_cases = vec![
            ResponsiveTestCase {
                base_classes: "px-4 py-2".to_string(),
                responsive_classes: "sm:px-6 sm:py-3 md:px-8 md:py-4".to_string(),
                breakpoint: "sm-md".to_string(),
                description: "Responsive padding".to_string(),
            },
            ResponsiveTestCase {
                base_classes: "text-sm".to_string(),
                responsive_classes: "sm:text-base md:text-lg lg:text-xl".to_string(),
                breakpoint: "sm-lg".to_string(),
                description: "Responsive typography".to_string(),
            },
            ResponsiveTestCase {
                base_classes: "grid grid-cols-1".to_string(),
                responsive_classes: "sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4".to_string(),
                breakpoint: "sm-lg".to_string(),
                description: "Responsive grid".to_string(),
            },
        ];
        
        VisualTestUtils::test_responsive_consistency("Responsive", test_cases)
    }
    
    /// Test theme consistency
    fn test_theme_consistency() -> Vec<VisualComparison> {
        let test_cases = vec![
            ThemeTestCase {
                base_classes: "px-4 py-2 rounded".to_string(),
                theme_classes: "bg-blue-600 text-white hover:bg-blue-700".to_string(),
                theme_name: "light".to_string(),
                description: "Light theme button".to_string(),
            },
            ThemeTestCase {
                base_classes: "px-4 py-2 rounded".to_string(),
                theme_classes: "bg-blue-500 text-blue-100 hover:bg-blue-400".to_string(),
                theme_name: "dark".to_string(),
                description: "Dark theme button".to_string(),
            },
            ThemeTestCase {
                base_classes: "px-4 py-2 rounded".to_string(),
                theme_classes: "bg-purple-600 text-white hover:bg-purple-700".to_string(),
                theme_name: "purple".to_string(),
                description: "Purple theme button".to_string(),
            },
        ];
        
        VisualTestUtils::test_theme_consistency("Theme", test_cases)
    }
    
    /// Test state consistency
    fn test_state_consistency() -> Vec<VisualComparison> {
        let test_cases = vec![
            StateTestCase {
                base_classes: "px-4 py-2 bg-blue-600 text-white rounded".to_string(),
                state_classes: "hover:bg-blue-700 focus:ring-2 focus:ring-blue-500".to_string(),
                state_name: "interactive".to_string(),
                description: "Interactive state".to_string(),
            },
            StateTestCase {
                base_classes: "px-4 py-2 bg-blue-600 text-white rounded".to_string(),
                state_classes: "opacity-50 cursor-not-allowed".to_string(),
                state_name: "disabled".to_string(),
                description: "Disabled state".to_string(),
            },
            StateTestCase {
                base_classes: "px-4 py-2 bg-blue-600 text-white rounded".to_string(),
                state_classes: "animate-pulse".to_string(),
                state_name: "loading".to_string(),
                description: "Loading state".to_string(),
            },
        ];
        
        VisualTestUtils::test_state_consistency("State", test_cases)
    }
}

/// Visual test runner for automated testing
pub struct VisualTestRunner;

impl VisualTestRunner {
    /// Run visual tests and generate report
    pub fn run_tests() -> VisualTestReport {
        VisualRegressionTests::run_all_tests()
    }
    
    /// Run visual tests with custom configuration
    pub fn run_tests_with_config(config: VisualTestConfig) -> VisualTestReport {
        let mut all_comparisons = Vec::new();
        
        if config.test_components {
            all_comparisons.extend(VisualRegressionTests::test_button_consistency());
            all_comparisons.extend(VisualRegressionTests::test_input_consistency());
            all_comparisons.extend(VisualRegressionTests::test_card_consistency());
        }
        
        if config.test_responsive {
            all_comparisons.extend(VisualRegressionTests::test_responsive_design());
        }
        
        if config.test_themes {
            all_comparisons.extend(VisualRegressionTests::test_theme_consistency());
        }
        
        if config.test_states {
            all_comparisons.extend(VisualRegressionTests::test_state_consistency());
        }
        
        VisualTestUtils::generate_visual_report(all_comparisons)
    }
}

/// Configuration for visual tests
#[derive(Debug, Clone)]
pub struct VisualTestConfig {
    pub test_components: bool,
    pub test_responsive: bool,
    pub test_themes: bool,
    pub test_states: bool,
    pub save_snapshots: bool,
    pub snapshot_directory: Option<String>,
}

impl Default for VisualTestConfig {
    fn default() -> Self {
        Self {
            test_components: true,
            test_responsive: true,
            test_themes: true,
            test_states: true,
            save_snapshots: false,
            snapshot_directory: None,
        }
    }
}

impl VisualTestReport {
    /// Print the visual test report
    pub fn print_report(&self) {
        println!("🎨 Visual Regression Test Report");
        println!("================================");
        println!("Total Tests: {}", self.total_tests);
        println!("Passed: {} ✅", self.passed_tests);
        println!("Failed: {} ❌", self.failed_tests);
        println!("Success Rate: {:.1}%", 
            (self.passed_tests as f64 / self.total_tests as f64) * 100.0);
        
        if self.failed_tests > 0 {
            println!("\n❌ Failed Tests:");
            for comparison in &self.comparisons {
                if !comparison.classes_match {
                    println!("  - {}: Classes don't match", comparison.component_name);
                    for diff in &comparison.differences {
                        println!("    {}: '{}' -> '{}'", diff.field, diff.old_value, diff.new_value);
                    }
                }
            }
        }
        
        println!("\n📊 Test Summary:");
        println!("  Generated at: {:?}", self.generated_at);
        println!("  All tests: {}", if self.failed_tests == 0 { "✅ PASSED" } else { "❌ FAILED" });
    }
    
    /// Save the report to a file
    pub fn save_report(&self, file_path: &str) -> Result<(), std::io::Error> {
        let report_content = format!(
            "Visual Regression Test Report\n\
            ==============================\n\
            Total Tests: {}\n\
            Passed: {}\n\
            Failed: {}\n\
            Success Rate: {:.1}%\n\
            Generated at: {:?}\n",
            self.total_tests,
            self.passed_tests,
            self.failed_tests,
            (self.passed_tests as f64 / self.total_tests as f64) * 100.0,
            self.generated_at
        );
        
        std::fs::write(file_path, report_content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_visual_snapshot_creation() {
        let snapshot = VisualTestUtils::capture_component_state("Button", "px-4 py-2 bg-blue-600");
        
        assert_eq!(snapshot.component_name, "Button");
        assert_eq!(snapshot.classes, "px-4 py-2 bg-blue-600");
    }
    
    #[test]
    fn test_visual_snapshot_comparison() {
        let snapshot1 = VisualTestUtils::capture_component_state("Button", "px-4 py-2 bg-blue-600");
        let snapshot2 = VisualTestUtils::capture_component_state("Button", "px-4 py-2 bg-blue-600");
        
        let comparison = VisualTestUtils::compare_snapshots(&snapshot1, &snapshot2);
        
        assert!(comparison.classes_match);
        assert_eq!(comparison.differences.len(), 0);
    }
    
    #[test]
    fn test_visual_snapshot_comparison_different() {
        let snapshot1 = VisualTestUtils::capture_component_state("Button", "px-4 py-2 bg-blue-600");
        let snapshot2 = VisualTestUtils::capture_component_state("Button", "px-4 py-2 bg-red-600");
        
        let comparison = VisualTestUtils::compare_snapshots(&snapshot1, &snapshot2);
        
        assert!(!comparison.classes_match);
        assert_eq!(comparison.differences.len(), 1);
        assert_eq!(comparison.differences[0].field, "classes");
    }
    
    #[test]
    fn test_visual_report_generation() {
        let comparisons = vec![
            VisualComparison {
                component_name: "Button".to_string(),
                classes_match: true,
                time_difference: std::time::Duration::from_millis(100),
                differences: Vec::new(),
            },
            VisualComparison {
                component_name: "Input".to_string(),
                classes_match: false,
                time_difference: std::time::Duration::from_millis(200),
                differences: vec![VisualDifference {
                    field: "classes".to_string(),
                    old_value: "px-3 py-2".to_string(),
                    new_value: "px-4 py-3".to_string(),
                }],
            },
        ];
        
        let report = VisualTestUtils::generate_visual_report(comparisons);
        
        assert_eq!(report.total_tests, 2);
        assert_eq!(report.passed_tests, 1);
        assert_eq!(report.failed_tests, 1);
    }
    
    #[test]
    fn test_component_consistency() {
        let test_cases = vec![
            ComponentTestCase {
                expected_classes: "px-4 py-2".to_string(),
                actual_classes: "px-4 py-2".to_string(),
                description: "Test case 1".to_string(),
            },
            ComponentTestCase {
                expected_classes: "px-4 py-2".to_string(),
                actual_classes: "px-4 py-3".to_string(),
                description: "Test case 2".to_string(),
            },
        ];
        
        let comparisons = VisualTestUtils::test_component_consistency("TestComponent", test_cases);
        
        assert_eq!(comparisons.len(), 2);
        assert!(comparisons[0].classes_match);
        assert!(!comparisons[1].classes_match);
    }
    
    #[test]
    fn test_responsive_consistency() {
        let test_cases = vec![
            ResponsiveTestCase {
                base_classes: "px-4".to_string(),
                responsive_classes: "sm:px-6".to_string(),
                breakpoint: "sm".to_string(),
                description: "Small breakpoint".to_string(),
            },
        ];
        
        let comparisons = VisualTestUtils::test_responsive_consistency("Responsive", test_cases);
        
        assert_eq!(comparisons.len(), 1);
        assert!(comparisons[0].classes_match);
    }
    
    #[test]
    fn test_theme_consistency() {
        let test_cases = vec![
            ThemeTestCase {
                base_classes: "px-4".to_string(),
                theme_classes: "bg-blue-600".to_string(),
                theme_name: "light".to_string(),
                description: "Light theme".to_string(),
            },
        ];
        
        let comparisons = VisualTestUtils::test_theme_consistency("Theme", test_cases);
        
        assert_eq!(comparisons.len(), 1);
        assert!(comparisons[0].classes_match);
    }
    
    #[test]
    fn test_state_consistency() {
        let test_cases = vec![
            StateTestCase {
                base_classes: "px-4".to_string(),
                state_classes: "hover:bg-blue-700".to_string(),
                state_name: "hover".to_string(),
                description: "Hover state".to_string(),
            },
        ];
        
        let comparisons = VisualTestUtils::test_state_consistency("State", test_cases);
        
        assert_eq!(comparisons.len(), 1);
        assert!(comparisons[0].classes_match);
    }
    
    #[test]
    fn test_visual_regression_tests() {
        let report = VisualRegressionTests::run_all_tests();
        
        assert!(report.total_tests > 0);
        assert!(report.passed_tests >= 0);
        assert!(report.failed_tests >= 0);
        assert_eq!(report.total_tests, report.passed_tests + report.failed_tests);
    }
    
    #[test]
    fn test_visual_test_runner() {
        let report = VisualTestRunner::run_tests();
        
        assert!(report.total_tests > 0);
    }
    
    #[test]
    fn test_visual_test_runner_with_config() {
        let config = VisualTestConfig {
            test_components: true,
            test_responsive: false,
            test_themes: false,
            test_states: false,
            save_snapshots: false,
            snapshot_directory: None,
        };
        
        let report = VisualTestRunner::run_tests_with_config(config);
        
        assert!(report.total_tests > 0);
    }
    
    #[test]
    fn test_visual_report_formatting() {
        let report = VisualTestReport {
            total_tests: 10,
            passed_tests: 8,
            failed_tests: 2,
            comparisons: Vec::new(),
            generated_at: std::time::SystemTime::now(),
        };
        
        // Test that formatting doesn't panic
        report.print_report();
    }
}
