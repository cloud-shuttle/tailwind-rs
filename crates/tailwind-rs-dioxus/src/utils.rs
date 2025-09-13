//! Utility functions for Dioxus integration

/// Join multiple class strings into a single string
pub fn join_classes(classes: &[&str]) -> String {
    classes
        .iter()
        .filter(|&&class| !class.is_empty())
        .map(|&class| class)
        .collect::<Vec<_>>()
        .join(" ")
}

/// Create a conditional class string
pub fn conditional_class(condition: bool, class: &str) -> String {
    if condition {
        class.to_string()
    } else {
        String::new()
    }
}

/// Create a conditional class string with an alternative
pub fn conditional_class_with_alt(condition: bool, true_class: &str, false_class: &str) -> String {
    if condition {
        true_class.to_string()
    } else {
        false_class.to_string()
    }
}