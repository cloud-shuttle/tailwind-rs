//! Component testing utilities for tailwind-rs

use crate::{TestConfig, TestResult};
use std::collections::HashSet;

/// A test application for component testing
#[derive(Debug, Clone)]
pub struct TestApp {
    pub config: TestConfig,
    pub components: Vec<TestComponent>,
}

/// A test component with metadata
#[derive(Debug, Clone)]
pub struct TestComponent {
    pub name: String,
    pub html: String,
    pub classes: HashSet<String>,
    pub custom_properties: std::collections::HashMap<String, String>,
}

impl TestApp {
    /// Create a new test app
    pub fn new(config: TestConfig) -> Self {
        Self {
            config,
            components: Vec::new(),
        }
    }

    /// Add a component to the test app
    pub fn add_component(&mut self, component: TestComponent) {
        self.components.push(component);
    }

    /// Get a component by name
    pub fn get_component(&self, name: &str) -> Option<&TestComponent> {
        self.components.iter().find(|c| c.name == name)
    }

    /// Get all components
    pub fn get_components(&self) -> &[TestComponent] {
        &self.components
    }

    /// Set the theme for the test app
    pub fn set_theme(&mut self, theme: String) {
        self.config.theme = Some(theme);
    }

    /// Set the breakpoint for the test app
    pub fn set_breakpoint(&mut self, breakpoint: String) {
        self.config.breakpoint = Some(breakpoint);
    }

    /// Enable dark mode for the test app
    pub fn enable_dark_mode(&mut self) {
        self.config.dark_mode = true;
    }

    /// Disable dark mode for the test app
    pub fn disable_dark_mode(&mut self) {
        self.config.dark_mode = false;
    }

    /// Add custom CSS to the test app
    pub fn add_custom_css(&mut self, css: String) {
        self.config.custom_css.push(css);
    }
}

impl TestComponent {
    /// Create a new test component
    pub fn new(name: impl Into<String>, html: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            html: html.into(),
            classes: HashSet::new(),
            custom_properties: std::collections::HashMap::new(),
        }
    }

    /// Add a class to the component
    pub fn add_class(&mut self, class: impl Into<String>) {
        self.classes.insert(class.into());
    }

    /// Add multiple classes to the component
    pub fn add_classes(&mut self, classes: impl IntoIterator<Item = String>) {
        for class in classes {
            self.classes.insert(class);
        }
    }

    /// Add a custom property to the component
    pub fn add_custom_property(&mut self, property: impl Into<String>, value: impl Into<String>) {
        self.custom_properties.insert(property.into(), value.into());
    }

    /// Check if the component has a specific class
    pub fn has_class(&self, class: &str) -> bool {
        self.classes.contains(class)
    }

    /// Get all classes as a vector
    pub fn get_classes(&self) -> Vec<String> {
        self.classes.iter().cloned().collect()
    }

    /// Get all classes as a string
    pub fn get_classes_string(&self) -> String {
        let mut sorted_classes: Vec<String> = self.classes.iter().cloned().collect();
        sorted_classes.sort();
        sorted_classes.join(" ")
    }

    /// Get a custom property value
    pub fn get_custom_property(&self, property: &str) -> Option<&String> {
        self.custom_properties.get(property)
    }

    /// Get all custom properties
    pub fn get_custom_properties(&self) -> &std::collections::HashMap<String, String> {
        &self.custom_properties
    }
}

/// Create a test app with default configuration
pub fn create_test_app<F>(component_fn: F) -> TestApp
where
    F: Fn() -> String,
{
    let config = TestConfig::default();
    let mut app = TestApp::new(config);

    let html = component_fn();
    let component = TestComponent::new("test_component", html);
    app.add_component(component);

    app
}

/// Create a test app with custom configuration
pub fn create_test_app_with_config<F>(component_fn: F, config: TestConfig) -> TestApp
where
    F: Fn() -> String,
{
    let mut app = TestApp::new(config);

    let html = component_fn();
    let component = TestComponent::new("test_component", html);
    app.add_component(component);

    app
}

/// Render a component to HTML string
pub fn render_to_string<F>(component_fn: F) -> String
where
    F: Fn() -> String,
{
    component_fn()
}

/// Extract CSS classes from a component
pub fn extract_classes<F>(component_fn: F) -> HashSet<String>
where
    F: Fn() -> String,
{
    let html = component_fn();
    extract_classes_from_html(&html)
}

/// Extract CSS classes from HTML string
pub fn extract_classes_from_html(html: &str) -> HashSet<String> {
    let mut classes = HashSet::new();

    // Simple parsing for class attributes
    if let Some(class_start) = html.find("class=\"") {
        if let Some(class_end) = html[class_start + 7..].find("\"") {
            let class_content = &html[class_start + 7..class_start + 7 + class_end];

            for class in class_content.split_whitespace() {
                if !class.is_empty() {
                    classes.insert(class.to_string());
                }
            }
        }
    }

    classes
}

/// Test a component's classes
pub fn test_component_classes<F>(component_fn: F, expected_classes: &[&str]) -> TestResult
where
    F: Fn() -> String,
{
    let actual_classes = extract_classes(component_fn);
    let expected_set: HashSet<String> = expected_classes.iter().map(|s| s.to_string()).collect();

    let missing_classes: Vec<String> = expected_set.difference(&actual_classes).cloned().collect();
    let extra_classes: Vec<String> = actual_classes.difference(&expected_set).cloned().collect();

    if missing_classes.is_empty() && extra_classes.is_empty() {
        TestResult::success("All expected classes found")
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
        TestResult::failure(message)
    }
}

/// Test a component's HTML structure
pub fn test_component_html<F>(component_fn: F, expected_html: &str) -> TestResult
where
    F: Fn() -> String,
{
    let actual_html = component_fn();

    if actual_html.contains(expected_html) {
        TestResult::success("Expected HTML found")
    } else {
        TestResult::failure(format!(
            "Expected HTML not found. Expected: '{}', Got: '{}'",
            expected_html, actual_html
        ))
    }
}

/// Test a component's custom properties
pub fn test_component_custom_properties<F>(
    component_fn: F,
    expected_properties: &[(&str, &str)],
) -> TestResult
where
    F: Fn() -> String,
{
    let html = component_fn();
    let mut found_properties = std::collections::HashMap::new();

    // Simple parsing for style attributes
    if let Some(style_start) = html.find("style=\"") {
        if let Some(style_end) = html[style_start + 7..].find("\"") {
            let style_content = &html[style_start + 7..style_start + 7 + style_end];

            for property in style_content.split(';') {
                if let Some(colon_pos) = property.find(':') {
                    let prop = property[..colon_pos].trim();
                    let value = property[colon_pos + 1..].trim();
                    found_properties.insert(prop.to_string(), value.to_string());
                }
            }
        }
    }

    let mut missing_properties = Vec::new();
    let mut incorrect_properties = Vec::new();

    for (expected_prop, expected_value) in expected_properties {
        if let Some(actual_value) = found_properties.get(*expected_prop) {
            if actual_value != expected_value {
                incorrect_properties.push(format!(
                    "{}: expected '{}', got '{}'",
                    expected_prop, expected_value, actual_value
                ));
            }
        } else {
            missing_properties.push(expected_prop.to_string());
        }
    }

    if missing_properties.is_empty() && incorrect_properties.is_empty() {
        TestResult::success("All expected custom properties found")
    } else {
        let mut message = String::new();
        if !missing_properties.is_empty() {
            message.push_str(&format!("Missing properties: {:?}", missing_properties));
        }
        if !incorrect_properties.is_empty() {
            if !message.is_empty() {
                message.push_str("; ");
            }
            message.push_str(&format!("Incorrect properties: {:?}", incorrect_properties));
        }
        TestResult::failure(message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_app_creation() {
        let config = TestConfig::default();
        let app = TestApp::new(config);
        assert!(app.components.is_empty());
    }

    #[test]
    fn test_test_component_creation() {
        let component = TestComponent::new("test", "<div>Test</div>");
        assert_eq!(component.name, "test");
        assert_eq!(component.html, "<div>Test</div>");
        assert!(component.classes.is_empty());
    }

    #[test]
    fn test_test_component_classes() {
        let mut component = TestComponent::new("test", "<div>Test</div>");
        component.add_class("bg-blue-500");
        component.add_class("text-white");

        assert!(component.has_class("bg-blue-500"));
        assert!(component.has_class("text-white"));
        assert!(!component.has_class("bg-red-500"));

        let classes = component.get_classes();
        assert_eq!(classes.len(), 2);
        assert!(classes.contains(&"bg-blue-500".to_string()));
        assert!(classes.contains(&"text-white".to_string()));
    }

    #[test]
    fn test_extract_classes_from_html() {
        let html = r#"<div class="bg-blue-500 text-white hover:bg-blue-600">Test</div>"#;
        let classes = extract_classes_from_html(html);

        assert!(classes.contains("bg-blue-500"));
        assert!(classes.contains("text-white"));
        assert!(classes.contains("hover:bg-blue-600"));
    }

    #[test]
    fn test_test_component_classes_function() {
        let component_fn = || r#"<div class="bg-blue-500 text-white">Test</div>"#.to_string();
        let result = test_component_classes(component_fn, &["bg-blue-500", "text-white"]);

        assert!(result.success);
    }

    #[test]
    fn test_test_component_html_function() {
        let component_fn = || r#"<div class="bg-blue-500">Test</div>"#.to_string();
        let result = test_component_html(component_fn, "bg-blue-500");

        assert!(result.success);
    }

    #[test]
    fn test_test_component_custom_properties_function() {
        let component_fn =
            || r#"<div style="--primary-color: #3b82f6; --spacing: 1rem">Test</div>"#.to_string();
        let result = test_component_custom_properties(
            component_fn,
            &[("--primary-color", "#3b82f6"), ("--spacing", "1rem")],
        );

        assert!(result.success);
    }
}
