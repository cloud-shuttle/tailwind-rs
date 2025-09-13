//! Responsive testing utilities for tailwind-rs

use tailwind_rs_core::responsive::{Breakpoint, ResponsiveConfig, ResponsiveValue};

/// Result of a responsive test
#[derive(Debug, Clone)]
pub struct ResponsiveTestResult {
    pub success: bool,
    pub message: String,
    pub expected_value: Option<String>,
    pub actual_value: Option<String>,
}

impl ResponsiveTestResult {
    pub fn success(message: impl Into<String>) -> Self {
        Self {
            success: true,
            message: message.into(),
            expected_value: None,
            actual_value: None,
        }
    }

    pub fn failure(message: impl Into<String>) -> Self {
        Self {
            success: false,
            message: message.into(),
            expected_value: None,
            actual_value: None,
        }
    }

    pub fn with_values(mut self, expected: impl Into<String>, actual: impl Into<String>) -> Self {
        self.expected_value = Some(expected.into());
        self.actual_value = Some(actual.into());
        self
    }
}

/// Test responsive breakpoint values
pub fn test_responsive_breakpoint(
    breakpoint: Breakpoint,
    expected_min_width: u32,
) -> ResponsiveTestResult {
    let actual_min_width = breakpoint.min_width();

    if actual_min_width == expected_min_width {
        ResponsiveTestResult::success(format!("Breakpoint {:?} has correct min width", breakpoint))
    } else {
        ResponsiveTestResult::failure(format!(
            "Breakpoint {:?} has incorrect min width",
            breakpoint
        ))
        .with_values(expected_min_width.to_string(), actual_min_width.to_string())
    }
}

/// Test responsive value at specific breakpoint
pub fn test_responsive_value<T: PartialEq + std::fmt::Debug>(
    responsive_value: &ResponsiveValue<T>,
    breakpoint: Breakpoint,
    expected_value: &T,
) -> ResponsiveTestResult {
    let actual_value = responsive_value.get_breakpoint(breakpoint);

    if actual_value == expected_value {
        ResponsiveTestResult::success(format!(
            "Responsive value at {:?} matches expected",
            breakpoint
        ))
    } else {
        ResponsiveTestResult::failure(format!(
            "Responsive value at {:?} does not match expected",
            breakpoint
        ))
        .with_values(
            format!("{:?}", expected_value),
            format!("{:?}", actual_value),
        )
    }
}

/// Test responsive configuration
pub fn test_responsive_config(
    config: &ResponsiveConfig,
    expected_breakpoints: &[(&str, u32)],
) -> ResponsiveTestResult {
    let mut missing_breakpoints = Vec::new();
    let mut incorrect_breakpoints = Vec::new();

    for (name, expected_value) in expected_breakpoints {
        match config.get_breakpoint(name) {
            Ok(actual_value) => {
                if actual_value != *expected_value {
                    incorrect_breakpoints.push(format!(
                        "{}: expected {}, got {}",
                        name, expected_value, actual_value
                    ));
                }
            }
            Err(_) => {
                missing_breakpoints.push(name.to_string());
            }
        }
    }

    if missing_breakpoints.is_empty() && incorrect_breakpoints.is_empty() {
        ResponsiveTestResult::success("All breakpoints match expected values")
    } else {
        let mut message = String::new();
        if !missing_breakpoints.is_empty() {
            message.push_str(&format!("Missing breakpoints: {:?}", missing_breakpoints));
        }
        if !incorrect_breakpoints.is_empty() {
            if !message.is_empty() {
                message.push_str("; ");
            }
            message.push_str(&format!(
                "Incorrect breakpoints: {:?}",
                incorrect_breakpoints
            ));
        }
        ResponsiveTestResult::failure(message)
    }
}

/// Test responsive classes
pub fn test_responsive_classes(
    classes: &std::collections::HashSet<String>,
    expected_responsive: &[(&str, &str)],
) -> ResponsiveTestResult {
    let mut missing_classes = Vec::new();
    let mut found_classes = Vec::new();

    for (breakpoint, class) in expected_responsive {
        let responsive_class = format!("{}:{}", breakpoint, class);
        if classes.contains(&responsive_class) {
            found_classes.push(responsive_class);
        } else {
            missing_classes.push(responsive_class);
        }
    }

    if missing_classes.is_empty() {
        ResponsiveTestResult::success(format!("All responsive classes found: {:?}", found_classes))
    } else {
        ResponsiveTestResult::failure(format!("Missing responsive classes: {:?}", missing_classes))
    }
}

/// Assert responsive value matches expected
pub fn assert_responsive_value<T: PartialEq + std::fmt::Debug>(
    responsive_value: &ResponsiveValue<T>,
    breakpoint: Breakpoint,
    expected_value: &T,
) {
    let result = test_responsive_value(responsive_value, breakpoint, expected_value);
    if !result.success {
        panic!("Responsive assertion failed: {}", result.message);
    }
}

/// Assert responsive breakpoint has correct min width
pub fn assert_responsive_breakpoint(breakpoint: Breakpoint, expected_min_width: u32) {
    let result = test_responsive_breakpoint(breakpoint, expected_min_width);
    if !result.success {
        panic!("Responsive assertion failed: {}", result.message);
    }
}

/// Assert responsive configuration matches expected
pub fn assert_responsive_config(config: &ResponsiveConfig, expected_breakpoints: &[(&str, u32)]) {
    let result = test_responsive_config(config, expected_breakpoints);
    if !result.success {
        panic!("Responsive assertion failed: {}", result.message);
    }
}

/// Assert responsive classes are present
pub fn assert_responsive_classes(
    classes: &std::collections::HashSet<String>,
    expected_responsive: &[(&str, &str)],
) {
    let result = test_responsive_classes(classes, expected_responsive);
    if !result.success {
        panic!("Responsive assertion failed: {}", result.message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_responsive_breakpoint_testing() {
        let result = test_responsive_breakpoint(Breakpoint::Sm, 640);
        assert!(result.success);

        let result = test_responsive_breakpoint(Breakpoint::Sm, 768);
        assert!(!result.success);
    }

    #[test]
    fn test_responsive_value_testing() {
        let responsive_value = ResponsiveValue::new(10);
        let result = test_responsive_value(&responsive_value, Breakpoint::Base, &10);
        assert!(result.success);

        let result = test_responsive_value(&responsive_value, Breakpoint::Base, &20);
        assert!(!result.success);
    }

    #[test]
    fn test_responsive_config_testing() {
        let config = ResponsiveConfig::new();
        let expected_breakpoints = [
            ("sm", 640),
            ("md", 768),
            ("lg", 1024),
            ("xl", 1280),
            ("2xl", 1536),
        ];

        let result = test_responsive_config(&config, &expected_breakpoints);
        assert!(result.success);
    }

    #[test]
    fn test_responsive_classes_testing() {
        let classes: std::collections::HashSet<String> =
            ["sm:text-sm".to_string(), "md:text-md".to_string()]
                .iter()
                .cloned()
                .collect();

        let expected_responsive = [("sm", "text-sm"), ("md", "text-md")];

        let result = test_responsive_classes(&classes, &expected_responsive);
        assert!(result.success);
    }
}
