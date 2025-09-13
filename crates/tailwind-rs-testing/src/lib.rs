//! # tailwind-rs-testing
//!
//! Testing utilities for the tailwind-rs library.
//! This crate provides tools for testing Tailwind CSS integration in Rust applications.

pub mod class_testing;
pub mod component_testing;
pub mod mock_components;
pub mod responsive_testing;
pub mod theme_testing;

#[cfg(test)]
mod property_tests;

#[cfg(test)]
mod framework_integration_tests;

#[cfg(test)]
mod api_stability_tests;

// Re-export commonly used testing utilities
pub use class_testing::{
    ClassTestResult, assert_classes_contain, assert_classes_not_contain, test_classes,
};
pub use component_testing::{TestApp, create_test_app, extract_classes, render_to_string};
pub use mock_components::{MockComponent, create_mock_component};
pub use responsive_testing::{ResponsiveTestResult, assert_responsive_value};
pub use theme_testing::{ThemeTestResult, assert_theme_value};

/// Initialize the testing environment
pub fn init_test_env() {
    #[cfg(target_arch = "wasm32")]
    {
        console_error_panic_hook::set_once();
        console_log::init_with_level(log::Level::Info).expect("Failed to initialize console_log");
    }
}

/// Test configuration for tailwind-rs testing
#[derive(Debug, Clone, Default)]
pub struct TestConfig {
    pub theme: Option<String>,
    pub breakpoint: Option<String>,
    pub dark_mode: bool,
    pub custom_css: Vec<String>,
}

/// Test result for tailwind-rs operations
#[derive(Debug, Clone)]
pub struct TestResult {
    pub success: bool,
    pub message: String,
    pub details: Option<String>,
}

impl TestResult {
    pub fn success(message: impl Into<String>) -> Self {
        Self {
            success: true,
            message: message.into(),
            details: None,
        }
    }

    pub fn failure(message: impl Into<String>) -> Self {
        Self {
            success: false,
            message: message.into(),
            details: None,
        }
    }

    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_test_env() {
        // This test ensures the function can be called without panicking
        init_test_env();
    }

    #[test]
    fn test_test_config_default() {
        let config = TestConfig::default();
        assert!(config.theme.is_none());
        assert!(config.breakpoint.is_none());
        assert!(!config.dark_mode);
        assert!(config.custom_css.is_empty());
    }

    #[test]
    fn test_test_result() {
        let success_result = TestResult::success("Test passed");
        assert!(success_result.success);
        assert_eq!(success_result.message, "Test passed");
        assert!(success_result.details.is_none());

        let failure_result = TestResult::failure("Test failed");
        assert!(!failure_result.success);
        assert_eq!(failure_result.message, "Test failed");
        assert!(failure_result.details.is_none());

        let detailed_result = TestResult::success("Test passed").with_details("Additional info");
        assert!(detailed_result.success);
        assert_eq!(detailed_result.message, "Test passed");
        assert_eq!(detailed_result.details, Some("Additional info".to_string()));
    }
}
