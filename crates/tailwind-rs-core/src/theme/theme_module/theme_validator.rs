//! Theme validator
//!
//! This module provides theme validation functionality.

use super::ThemeConfig;

/// Theme validator
pub struct ThemeValidator;

impl ThemeValidator {
    /// Create a new theme validator
    pub fn new() -> Self {
        Self
    }

    /// Validate a theme configuration
    pub fn validate_theme(&self, config: &ThemeConfig) -> Result<(), String> {
        // Validate theme name
        if config.name.is_empty() {
            return Err("Theme name cannot be empty".to_string());
        }

        // Validate color palettes
        for palette in &config.color_palettes {
            if palette.name.is_empty() {
                return Err("Color palette name cannot be empty".to_string());
            }
            if palette.colors.is_empty() {
                return Err(format!("Color palette '{}' is empty", palette.name));
            }
        }

        // Validate spacing scale
        if config.spacing.values().is_empty() {
            return Err("Spacing scale cannot be empty".to_string());
        }

        // Validate typography scale
        if config.typography.font_sizes.is_empty() {
            return Err("Typography scale cannot be empty".to_string());
        }

        // Validate breakpoints
        if config.breakpoints.breakpoints.is_empty() {
            return Err("Breakpoint system cannot be empty".to_string());
        }

        Ok(())
    }

    /// Validate theme configuration and return detailed errors
    pub fn validate_theme_detailed(&self, config: &ThemeConfig) -> Vec<String> {
        let mut errors = Vec::new();

        // Validate theme name
        if config.name.is_empty() {
            errors.push("Theme name cannot be empty".to_string());
        }

        // Validate color palettes
        for palette in &config.color_palettes {
            if palette.name.is_empty() {
                errors.push("Color palette name cannot be empty".to_string());
            }
            if palette.colors.is_empty() {
                errors.push(format!("Color palette '{}' is empty", palette.name));
            }
        }

        // Validate spacing scale
        if config.spacing.values().is_empty() {
            errors.push("Spacing scale cannot be empty".to_string());
        }

        // Validate typography scale
        if config.typography.font_sizes.is_empty() {
            errors.push("Typography scale cannot be empty".to_string());
        }

        // Validate breakpoints
        if config.breakpoints.breakpoints.is_empty() {
            errors.push("Breakpoint system cannot be empty".to_string());
        }

        errors
    }
}
