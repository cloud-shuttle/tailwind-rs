//! Class testing utilities for tailwind-rs

use std::collections::HashSet;

/// Result of a class test
#[derive(Debug, Clone)]
pub struct ClassTestResult {
    pub success: bool,
    pub message: String,
    pub expected_classes: HashSet<String>,
    pub actual_classes: HashSet<String>,
    pub missing_classes: HashSet<String>,
    pub extra_classes: HashSet<String>,
}

impl ClassTestResult {
    /// Create a successful test result
    pub fn success(
        message: impl Into<String>,
        expected: HashSet<String>,
        actual: HashSet<String>,
    ) -> Self {
        Self {
            success: true,
            message: message.into(),
            expected_classes: expected,
            actual_classes: actual,
            missing_classes: HashSet::new(),
            extra_classes: HashSet::new(),
        }
    }

    /// Create a failed test result
    pub fn failure(
        message: impl Into<String>,
        expected: HashSet<String>,
        actual: HashSet<String>,
        missing: HashSet<String>,
        extra: HashSet<String>,
    ) -> Self {
        Self {
            success: false,
            message: message.into(),
            expected_classes: expected,
            actual_classes: actual,
            missing_classes: missing,
            extra_classes: extra,
        }
    }
}

/// Test CSS classes for correctness
pub fn test_classes(
    actual_classes: &HashSet<String>,
    expected_classes: &HashSet<String>,
) -> ClassTestResult {
    let missing_classes: HashSet<String> = expected_classes
        .difference(actual_classes)
        .cloned()
        .collect();
    let extra_classes: HashSet<String> = actual_classes
        .difference(expected_classes)
        .cloned()
        .collect();

    if missing_classes.is_empty() && extra_classes.is_empty() {
        ClassTestResult::success(
            "All expected classes found",
            expected_classes.clone(),
            actual_classes.clone(),
        )
    } else {
        let mut message = String::new();
        if !missing_classes.is_empty() {
            message.push_str(&format!("Missing classes: {:?}", missing_classes));
        }
        if !extra_classes.is_empty() {
            if !message.is_empty() {
                message.push_str("; ");
            }
            message.push_str(&format!("Extra classes: {:?}", extra_classes));
        }

        ClassTestResult::failure(
            message,
            expected_classes.clone(),
            actual_classes.clone(),
            missing_classes,
            extra_classes,
        )
    }
}

/// Test that classes contain all expected classes
pub fn test_classes_contain(
    actual_classes: &HashSet<String>,
    expected_classes: &HashSet<String>,
) -> ClassTestResult {
    let missing_classes: HashSet<String> = expected_classes
        .difference(actual_classes)
        .cloned()
        .collect();

    if missing_classes.is_empty() {
        ClassTestResult::success(
            "All expected classes found",
            expected_classes.clone(),
            actual_classes.clone(),
        )
    } else {
        ClassTestResult::failure(
            format!("Missing classes: {:?}", missing_classes),
            expected_classes.clone(),
            actual_classes.clone(),
            missing_classes,
            HashSet::new(),
        )
    }
}

/// Test that classes don't contain any unexpected classes
pub fn test_classes_not_contain(
    actual_classes: &HashSet<String>,
    unexpected_classes: &HashSet<String>,
) -> ClassTestResult {
    let extra_classes: HashSet<String> = actual_classes
        .intersection(unexpected_classes)
        .cloned()
        .collect();

    if extra_classes.is_empty() {
        ClassTestResult::success(
            "No unexpected classes found",
            HashSet::new(),
            actual_classes.clone(),
        )
    } else {
        ClassTestResult::failure(
            format!("Unexpected classes found: {:?}", extra_classes),
            HashSet::new(),
            actual_classes.clone(),
            HashSet::new(),
            extra_classes,
        )
    }
}

/// Test that classes match exactly
pub fn test_classes_exact(
    actual_classes: &HashSet<String>,
    expected_classes: &HashSet<String>,
) -> ClassTestResult {
    test_classes(actual_classes, expected_classes)
}

/// Test that classes contain at least one of the expected classes
pub fn test_classes_any(
    actual_classes: &HashSet<String>,
    expected_classes: &HashSet<String>,
) -> ClassTestResult {
    let found_classes: HashSet<String> = actual_classes
        .intersection(expected_classes)
        .cloned()
        .collect();

    if !found_classes.is_empty() {
        ClassTestResult::success(
            format!("Found expected classes: {:?}", found_classes),
            expected_classes.clone(),
            actual_classes.clone(),
        )
    } else {
        ClassTestResult::failure(
            format!(
                "No expected classes found. Expected any of: {:?}",
                expected_classes
            ),
            expected_classes.clone(),
            actual_classes.clone(),
            expected_classes.clone(),
            HashSet::new(),
        )
    }
}

/// Test that classes contain all of the expected classes
pub fn test_classes_all(
    actual_classes: &HashSet<String>,
    expected_classes: &HashSet<String>,
) -> ClassTestResult {
    test_classes_contain(actual_classes, expected_classes)
}

/// Test that classes contain none of the unexpected classes
pub fn test_classes_none(
    actual_classes: &HashSet<String>,
    unexpected_classes: &HashSet<String>,
) -> ClassTestResult {
    test_classes_not_contain(actual_classes, unexpected_classes)
}

/// Assert that classes contain all expected classes
pub fn assert_classes_contain(
    actual_classes: &HashSet<String>,
    expected_classes: &HashSet<String>,
) {
    let result = test_classes_contain(actual_classes, expected_classes);
    if !result.success {
        panic!("Class assertion failed: {}", result.message);
    }
}

/// Assert that classes don't contain any unexpected classes
pub fn assert_classes_not_contain(
    actual_classes: &HashSet<String>,
    unexpected_classes: &HashSet<String>,
) {
    let result = test_classes_not_contain(actual_classes, unexpected_classes);
    if !result.success {
        panic!("Class assertion failed: {}", result.message);
    }
}

/// Assert that classes match exactly
pub fn assert_classes_exact(actual_classes: &HashSet<String>, expected_classes: &HashSet<String>) {
    let result = test_classes_exact(actual_classes, expected_classes);
    if !result.success {
        panic!("Class assertion failed: {}", result.message);
    }
}

/// Assert that classes contain at least one of the expected classes
pub fn assert_classes_any(actual_classes: &HashSet<String>, expected_classes: &HashSet<String>) {
    let result = test_classes_any(actual_classes, expected_classes);
    if !result.success {
        panic!("Class assertion failed: {}", result.message);
    }
}

/// Assert that classes contain all of the expected classes
pub fn assert_classes_all(actual_classes: &HashSet<String>, expected_classes: &HashSet<String>) {
    let result = test_classes_all(actual_classes, expected_classes);
    if !result.success {
        panic!("Class assertion failed: {}", result.message);
    }
}

/// Assert that classes contain none of the unexpected classes
pub fn assert_classes_none(actual_classes: &HashSet<String>, unexpected_classes: &HashSet<String>) {
    let result = test_classes_none(actual_classes, unexpected_classes);
    if !result.success {
        panic!("Class assertion failed: {}", result.message);
    }
}

/// Test responsive classes
pub fn test_responsive_classes(
    actual_classes: &HashSet<String>,
    expected_responsive: &[(&str, &str)],
) -> ClassTestResult {
    let mut missing_classes = HashSet::new();
    let mut found_classes = HashSet::new();

    for (breakpoint, class) in expected_responsive {
        let responsive_class = format!("{}:{}", breakpoint, class);
        if actual_classes.contains(&responsive_class) {
            found_classes.insert(responsive_class);
        } else {
            missing_classes.insert(responsive_class);
        }
    }

    if missing_classes.is_empty() {
        ClassTestResult::success(
            "All expected responsive classes found",
            expected_responsive
                .iter()
                .map(|(bp, cls)| format!("{}:{}", bp, cls))
                .collect(),
            actual_classes.clone(),
        )
    } else {
        ClassTestResult::failure(
            format!("Missing responsive classes: {:?}", missing_classes),
            expected_responsive
                .iter()
                .map(|(bp, cls)| format!("{}:{}", bp, cls))
                .collect(),
            actual_classes.clone(),
            missing_classes,
            HashSet::new(),
        )
    }
}

/// Test conditional classes
pub fn test_conditional_classes(
    actual_classes: &HashSet<String>,
    expected_conditional: &[(&str, &str)],
) -> ClassTestResult {
    let mut missing_classes = HashSet::new();
    let mut found_classes = HashSet::new();

    for (condition, class) in expected_conditional {
        let conditional_class = format!("{}:{}", condition, class);
        if actual_classes.contains(&conditional_class) {
            found_classes.insert(conditional_class);
        } else {
            missing_classes.insert(conditional_class);
        }
    }

    if missing_classes.is_empty() {
        ClassTestResult::success(
            "All expected conditional classes found",
            expected_conditional
                .iter()
                .map(|(cond, cls)| format!("{}:{}", cond, cls))
                .collect(),
            actual_classes.clone(),
        )
    } else {
        ClassTestResult::failure(
            format!("Missing conditional classes: {:?}", missing_classes),
            expected_conditional
                .iter()
                .map(|(cond, cls)| format!("{}:{}", cond, cls))
                .collect(),
            actual_classes.clone(),
            missing_classes,
            HashSet::new(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_classes_success() {
        let actual: HashSet<String> = ["bg-blue-500", "text-white"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected: HashSet<String> = ["bg-blue-500", "text-white"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        let result = test_classes(&actual, &expected);
        assert!(result.success);
        assert!(result.missing_classes.is_empty());
        assert!(result.extra_classes.is_empty());
    }

    #[test]
    fn test_test_classes_missing() {
        let actual: HashSet<String> = ["bg-blue-500"].iter().map(|s| s.to_string()).collect();
        let expected: HashSet<String> = ["bg-blue-500", "text-white"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        let result = test_classes(&actual, &expected);
        assert!(!result.success);
        assert!(result.missing_classes.contains("text-white"));
        assert!(result.extra_classes.is_empty());
    }

    #[test]
    fn test_test_classes_extra() {
        let actual: HashSet<String> = ["bg-blue-500", "text-white", "extra-class"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected: HashSet<String> = ["bg-blue-500", "text-white"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        let result = test_classes(&actual, &expected);
        assert!(!result.success);
        assert!(result.missing_classes.is_empty());
        assert!(result.extra_classes.contains("extra-class"));
    }

    #[test]
    fn test_test_classes_contain() {
        let actual: HashSet<String> = ["bg-blue-500", "text-white", "extra-class"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected: HashSet<String> = ["bg-blue-500", "text-white"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        let result = test_classes_contain(&actual, &expected);
        assert!(result.success);
        assert!(result.missing_classes.is_empty());
    }

    #[test]
    fn test_test_classes_not_contain() {
        let actual: HashSet<String> = ["bg-blue-500", "text-white"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let unexpected: HashSet<String> = ["bg-red-500", "text-black"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        let result = test_classes_not_contain(&actual, &unexpected);
        assert!(result.success);
        assert!(result.extra_classes.is_empty());
    }

    #[test]
    fn test_test_classes_any() {
        let actual: HashSet<String> = ["bg-blue-500", "text-white"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected: HashSet<String> = ["bg-red-500", "bg-blue-500"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        let result = test_classes_any(&actual, &expected);
        assert!(result.success);
    }

    #[test]
    fn test_test_responsive_classes() {
        let actual: HashSet<String> = ["sm:text-sm", "md:text-md"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = [("sm", "text-sm"), ("md", "text-md")];

        let result = test_responsive_classes(&actual, &expected);
        assert!(result.success);
    }

    #[test]
    fn test_test_conditional_classes() {
        let actual: HashSet<String> = ["hover:bg-blue-600", "focus:ring-2"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let expected = [("hover", "bg-blue-600"), ("focus", "ring-2")];

        let result = test_conditional_classes(&actual, &expected);
        assert!(result.success);
    }
}
