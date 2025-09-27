//! Utility functions for tailwind-rs

use crate::error::{Result, TailwindError};
use std::collections::HashSet;

/// Utility functions for string manipulation
pub mod string {

    /// Convert a string to kebab-case
    pub fn to_kebab_case(s: &str) -> String {
        s.chars()
            .map(|c| {
                if c.is_uppercase() {
                    format!("-{}", c.to_lowercase())
                } else {
                    c.to_string()
                }
            })
            .collect::<String>()
            .trim_start_matches('-')
            .to_string()
    }

    /// Convert a string to camelCase
    pub fn to_camel_case(s: &str) -> String {
        let mut result = String::new();
        let mut capitalize_next = false;
        let mut first_char = true;

        for c in s.chars() {
            if c == '-' || c == '_' {
                capitalize_next = true;
            } else if capitalize_next {
                result.push(c.to_uppercase().next().unwrap_or(c));
                capitalize_next = false;
            } else if first_char {
                result.push(c.to_lowercase().next().unwrap_or(c));
                first_char = false;
            } else {
                result.push(c);
            }
        }

        result
    }

    /// Convert a string to PascalCase
    pub fn to_pascal_case(s: &str) -> String {
        let camel_case = to_camel_case(s);
        if let Some(first_char) = camel_case.chars().next() {
            format!("{}{}", first_char.to_uppercase(), &camel_case[1..])
        } else {
            camel_case
        }
    }

    /// Check if a string is a valid CSS class name
    pub fn is_valid_css_class(s: &str) -> bool {
        if s.is_empty() {
            return false;
        }

        // CSS class names can contain letters, digits, hyphens, and underscores
        // but cannot start with a digit or hyphen
        let first_char = s.chars().next().unwrap();
        if first_char.is_ascii_digit() || first_char == '-' {
            return false;
        }

        s.chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    }

    /// Sanitize a string for use as a CSS class name
    pub fn sanitize_css_class(s: &str) -> String {
        s.chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() {
                    c
                } else if c == ' ' || c == '_' {
                    '-'
                } else {
                    // Remove invalid characters
                    '\0'
                }
            })
            .filter(|&c| c != '\0')
            .collect::<String>()
            .trim_matches('-')
            .to_lowercase()
    }
}

/// Utility functions for file operations
pub mod file {
    use super::*;
    use std::fs;
    use std::path::{Path, PathBuf};

    /// Check if a path exists and is a file
    pub fn exists(path: &Path) -> bool {
        path.exists() && path.is_file()
    }

    /// Check if a path exists and is a directory
    pub fn is_dir(path: &Path) -> bool {
        path.exists() && path.is_dir()
    }

    /// Get the file extension from a path
    pub fn get_extension(path: &Path) -> Option<String> {
        path.extension()?.to_str().map(|s| s.to_string())
    }

    /// Get the file name without extension from a path
    pub fn get_stem(path: &Path) -> Option<String> {
        path.file_stem()?.to_str().map(|s| s.to_string())
    }

    /// Get the parent directory of a path
    pub fn get_parent(path: &Path) -> Option<PathBuf> {
        path.parent().map(|p| p.to_path_buf())
    }

    /// Create a directory if it doesn't exist
    pub fn create_dir_if_not_exists(path: &Path) -> Result<()> {
        if !path.exists() {
            fs::create_dir_all(path).map_err(|e| {
                TailwindError::config(format!("Failed to create directory {:?}: {}", path, e))
            })?;
        }
        Ok(())
    }

    /// Read a file to string
    pub fn read_to_string(path: &Path) -> Result<String> {
        fs::read_to_string(path)
            .map_err(|e| TailwindError::config(format!("Failed to read file {:?}: {}", path, e)))
    }

    /// Write a string to a file
    pub fn write_string(path: &Path, content: &str) -> Result<()> {
        if let Some(parent) = path.parent() {
            create_dir_if_not_exists(parent)?;
        }

        fs::write(path, content)
            .map_err(|e| TailwindError::config(format!("Failed to write file {:?}: {}", path, e)))
    }

    /// Get all files matching a glob pattern
    pub fn glob_files(pattern: &str) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        for entry in glob::glob(pattern).map_err(|e| {
            TailwindError::config(format!("Invalid glob pattern '{}': {}", pattern, e))
        })? {
            let entry = entry
                .map_err(|e| TailwindError::config(format!("Error reading glob entry: {}", e)))?;

            if entry.is_file() {
                files.push(entry);
            }
        }

        Ok(files)
    }
}

/// Utility functions for CSS operations
pub mod css {
    use super::*;

    /// Parse CSS class names from a string
    pub fn parse_classes(s: &str) -> HashSet<String> {
        s.split_whitespace()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }

    /// Join CSS class names into a string
    pub fn join_classes(classes: &HashSet<String>) -> String {
        let mut sorted_classes: Vec<String> = classes.iter().cloned().collect();
        sorted_classes.sort();
        sorted_classes.join(" ")
    }

    /// Validate CSS class names
    pub fn validate_classes(classes: &HashSet<String>) -> Result<()> {
        for class in classes {
            if !string::is_valid_css_class(class) {
                return Err(TailwindError::class_generation(format!(
                    "Invalid CSS class name: '{}'",
                    class
                )));
            }
        }
        Ok(())
    }

    /// Sanitize CSS class names
    pub fn sanitize_classes(classes: &HashSet<String>) -> HashSet<String> {
        classes
            .iter()
            .map(|class| string::sanitize_css_class(class))
            .filter(|class| !class.is_empty())
            .collect()
    }

    /// Generate CSS custom properties from a map
    pub fn generate_custom_properties(
        properties: &std::collections::HashMap<String, String>,
    ) -> String {
        if properties.is_empty() {
            return String::new();
        }

        let mut css_properties = Vec::new();
        for (property, value) in properties {
            css_properties.push(format!("--{}: {}", property, value));
        }

        format!("style=\"{}\"", css_properties.join("; "))
    }
}

/// Utility functions for validation
pub mod validation {
    use super::*;

    /// Validate a file path
    pub fn validate_file_path(path: &str) -> Result<()> {
        if path.is_empty() {
            return Err(TailwindError::config("File path cannot be empty"));
        }

        if path.contains("..") {
            return Err(TailwindError::config("File path cannot contain '..'"));
        }

        Ok(())
    }

    /// Validate a glob pattern
    pub fn validate_glob_pattern(pattern: &str) -> Result<()> {
        if pattern.is_empty() {
            return Err(TailwindError::config("Glob pattern cannot be empty"));
        }

        // Basic validation - check for common issues
        if pattern.starts_with('/') {
            return Err(TailwindError::config(
                "Glob pattern should not start with '/'",
            ));
        }

        Ok(())
    }

    /// Validate a CSS class name
    pub fn validate_css_class(class: &str) -> Result<()> {
        if class.is_empty() {
            return Err(TailwindError::class_generation(
                "CSS class name cannot be empty",
            ));
        }

        if !string::is_valid_css_class(class) {
            return Err(TailwindError::class_generation(format!(
                "Invalid CSS class name: '{}'",
                class
            )));
        }

        Ok(())
    }
}

/// Utility functions for timing and performance
pub mod timing {
    use std::time::{Duration, Instant};

    /// A simple timer for measuring execution time
    pub struct Timer {
        start: Instant,
    }

    impl Timer {
        /// Create a new timer
        pub fn new() -> Self {
            Self {
                start: Instant::now(),
            }
        }

        /// Get the elapsed time
        pub fn elapsed(&self) -> Duration {
            self.start.elapsed()
        }

        /// Get the elapsed time in milliseconds
        pub fn elapsed_ms(&self) -> u128 {
            self.elapsed().as_millis()
        }

        /// Get the elapsed time in microseconds
        pub fn elapsed_us(&self) -> u128 {
            self.elapsed().as_micros()
        }

        /// Reset the timer
        pub fn reset(&mut self) {
            self.start = Instant::now();
        }
    }

    impl Default for Timer {
        fn default() -> Self {
            Self::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_utilities() {
        assert_eq!(string::to_kebab_case("camelCase"), "camel-case");
        assert_eq!(string::to_kebab_case("PascalCase"), "pascal-case");
        assert_eq!(string::to_kebab_case("snake_case"), "snake_case");

        assert_eq!(string::to_camel_case("kebab-case"), "kebabCase");
        assert_eq!(string::to_camel_case("snake_case"), "snakeCase");
        assert_eq!(string::to_camel_case("PascalCase"), "pascalCase");

        assert_eq!(string::to_pascal_case("kebab-case"), "KebabCase");
        assert_eq!(string::to_pascal_case("snake_case"), "SnakeCase");
        assert_eq!(string::to_pascal_case("camelCase"), "CamelCase");

        assert!(string::is_valid_css_class("valid-class"));
        assert!(string::is_valid_css_class("valid_class"));
        assert!(string::is_valid_css_class("validClass"));
        assert!(!string::is_valid_css_class(""));
        assert!(!string::is_valid_css_class("123invalid"));
        assert!(!string::is_valid_css_class("-invalid"));

        assert_eq!(string::sanitize_css_class("valid class"), "valid-class");
        assert_eq!(string::sanitize_css_class("invalid@class"), "invalidclass");
        assert_eq!(string::sanitize_css_class("  spaced  "), "spaced");
    }

    #[test]
    fn test_css_utilities() {
        let classes_str = "bg-blue-500 text-white hover:bg-blue-600";
        let classes = css::parse_classes(classes_str);

        assert!(classes.contains("bg-blue-500"));
        assert!(classes.contains("text-white"));
        assert!(classes.contains("hover:bg-blue-600"));

        let joined = css::join_classes(&classes);
        assert!(joined.contains("bg-blue-500"));
        assert!(joined.contains("text-white"));
        assert!(joined.contains("hover:bg-blue-600"));

        let valid_classes: HashSet<String> = ["valid-class".to_string(), "valid_class".to_string()]
            .iter()
            .cloned()
            .collect();
        assert!(css::validate_classes(&valid_classes).is_ok());

        let invalid_classes: HashSet<String> =
            ["123invalid".to_string(), "valid-class".to_string()]
                .iter()
                .cloned()
                .collect();
        assert!(css::validate_classes(&invalid_classes).is_err());

        let sanitized = css::sanitize_classes(&invalid_classes);
        assert!(sanitized.contains("validclass"));
        // Note: 123invalid stays as 123invalid, valid-class becomes validclass after sanitization
    }

    #[test]
    fn test_validation_utilities() {
        assert!(validation::validate_file_path("valid/path.rs").is_ok());
        assert!(validation::validate_file_path("").is_err());
        assert!(validation::validate_file_path("../invalid").is_err());

        assert!(validation::validate_glob_pattern("src/**/*.rs").is_ok());
        assert!(validation::validate_glob_pattern("").is_err());
        assert!(validation::validate_glob_pattern("/invalid").is_err());

        assert!(validation::validate_css_class("valid-class").is_ok());
        assert!(validation::validate_css_class("").is_err());
        assert!(validation::validate_css_class("123invalid").is_err());
    }

    #[test]
    fn test_timing_utilities() {
        let mut timer = timing::Timer::new();

        // Sleep for a short time to test timing
        std::thread::sleep(std::time::Duration::from_millis(10));

        let elapsed = timer.elapsed();
        assert!(elapsed.as_millis() >= 10);

        let elapsed_millis = timer.elapsed_ms();
        assert!(elapsed_millis >= 10);

        let elapsed_micros = timer.elapsed_us();
        assert!(elapsed_micros >= 10000);

        timer.reset();
        let reset_elapsed = timer.elapsed();
        assert!(reset_elapsed.as_millis() < 10);
    }
}
