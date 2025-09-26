//! # Visual Test Cases
//!
//! This module defines various test case structures for different types of visual testing.

/// Test case for component consistency testing
#[derive(Debug, Clone)]
pub struct ComponentTestCase {
    pub component_name: String,
    pub expected_classes: String,
}

impl ComponentTestCase {
    /// Create a new component test case
    pub fn new(component_name: &str, expected_classes: &str) -> Self {
        Self {
            component_name: component_name.to_string(),
            expected_classes: expected_classes.to_string(),
        }
    }

    /// Validate that the component classes match expectations
    pub fn validate(&self, actual_classes: &str) -> bool {
        self.expected_classes == actual_classes
    }
}

/// Test case for responsive design testing
#[derive(Debug, Clone)]
pub struct ResponsiveTestCase {
    pub responsive_classes: String,
}

impl ResponsiveTestCase {
    /// Create a new responsive test case
    pub fn new(responsive_classes: &str) -> Self {
        Self {
            responsive_classes: responsive_classes.to_string(),
        }
    }

    /// Validate responsive classes
    pub fn validate(&self, actual_classes: &str) -> bool {
        self.responsive_classes == actual_classes
    }
}

/// Test case for theme consistency testing
#[derive(Debug, Clone)]
pub struct ThemeTestCase {
    pub theme_classes: String,
}

impl ThemeTestCase {
    /// Create a new theme test case
    pub fn new(theme_classes: &str) -> Self {
        Self {
            theme_classes: theme_classes.to_string(),
        }
    }

    /// Validate theme classes
    pub fn validate(&self, actual_classes: &str) -> bool {
        self.theme_classes == actual_classes
    }
}

/// Test case for state-based styling testing
#[derive(Debug, Clone)]
pub struct StateTestCase {
    pub state_classes: String,
}

impl StateTestCase {
    /// Create a new state test case
    pub fn new(state_classes: &str) -> Self {
        Self {
            state_classes: state_classes.to_string(),
        }
    }

    /// Validate state classes
    pub fn validate(&self, actual_classes: &str) -> bool {
        self.state_classes == actual_classes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_test_case_creation() {
        let test_case = ComponentTestCase::new("Button", "px-4 py-2 bg-blue-600");

        assert_eq!(test_case.component_name, "Button");
        assert_eq!(test_case.expected_classes, "px-4 py-2 bg-blue-600");
    }

    #[test]
    fn test_component_test_case_validation() {
        let test_case = ComponentTestCase::new("Button", "px-4 py-2 bg-blue-600");

        assert!(test_case.validate("px-4 py-2 bg-blue-600"));
        assert!(!test_case.validate("px-4 py-2 bg-red-600"));
    }

    #[test]
    fn test_responsive_test_case_creation() {
        let test_case = ResponsiveTestCase::new("sm:px-2 md:px-4 lg:px-6");

        assert_eq!(test_case.responsive_classes, "sm:px-2 md:px-4 lg:px-6");
    }

    #[test]
    fn test_responsive_test_case_validation() {
        let test_case = ResponsiveTestCase::new("sm:px-2 md:px-4 lg:px-6");

        assert!(test_case.validate("sm:px-2 md:px-4 lg:px-6"));
        assert!(!test_case.validate("sm:px-2 md:px-4 lg:px-8"));
    }

    #[test]
    fn test_theme_test_case_creation() {
        let test_case = ThemeTestCase::new("dark:bg-gray-800 dark:text-white");

        assert_eq!(test_case.theme_classes, "dark:bg-gray-800 dark:text-white");
    }

    #[test]
    fn test_theme_test_case_validation() {
        let test_case = ThemeTestCase::new("dark:bg-gray-800 dark:text-white");

        assert!(test_case.validate("dark:bg-gray-800 dark:text-white"));
        assert!(!test_case.validate("dark:bg-gray-900 dark:text-white"));
    }

    #[test]
    fn test_state_test_case_creation() {
        let test_case = StateTestCase::new("hover:bg-blue-700 focus:ring-2");

        assert_eq!(test_case.state_classes, "hover:bg-blue-700 focus:ring-2");
    }

    #[test]
    fn test_state_test_case_validation() {
        let test_case = StateTestCase::new("hover:bg-blue-700 focus:ring-2");

        assert!(test_case.validate("hover:bg-blue-700 focus:ring-2"));
        assert!(!test_case.validate("hover:bg-blue-800 focus:ring-2"));
    }
}
