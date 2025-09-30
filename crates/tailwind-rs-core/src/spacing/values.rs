//! Spacing Values Module
//!
//! This module provides spacing value mappings and utilities for converting
//! Tailwind spacing tokens to CSS values.

use std::collections::HashMap;

/// Spacing value mapping from Tailwind tokens to CSS values
#[derive(Debug, Clone)]
pub struct SpacingValues {
    values: HashMap<&'static str, &'static str>,
}

impl SpacingValues {
    /// Create a new spacing values mapping
    pub fn new() -> Self {
        let mut values = HashMap::new();

        // Standard spacing scale (0-96)
        Self::populate_standard_spacing(&mut values);

        Self { values }
    }

    /// Get spacing value for a token
    pub fn get_value(&self, token: &str) -> Option<String> {
        self.values.get(token).map(|s| s.to_string())
    }

    /// Check if token is a valid spacing token
    pub fn is_valid_token(&self, token: &str) -> bool {
        self.values.contains_key(token)
    }

    /// Get all supported tokens
    pub fn supported_tokens(&self) -> Vec<&str> {
        self.values.keys().copied().collect()
    }

    /// Populate standard Tailwind spacing scale
    fn populate_standard_spacing(values: &mut HashMap<&'static str, &'static str>) {
        // Zero and pixel values
        values.insert("0", "0");
        values.insert("px", "1px");

        // Fractional values (0.5, 1, 1.5, 2, 2.5, 3, 3.5, 4, 5, 6, 7, 8, 9, 10, 11, 12)
        values.insert("0.5", "0.125rem");
        values.insert("1", "0.25rem");
        values.insert("1.5", "0.375rem");
        values.insert("2", "0.5rem");
        values.insert("2.5", "0.625rem");
        values.insert("3", "0.75rem");
        values.insert("3.5", "0.875rem");
        values.insert("4", "1rem");
        values.insert("5", "1.25rem");
        values.insert("6", "1.5rem");
        values.insert("7", "1.75rem");
        values.insert("8", "2rem");
        values.insert("9", "2.25rem");
        values.insert("10", "2.5rem");
        values.insert("11", "2.75rem");
        values.insert("12", "3rem");

        // Larger values (14, 16, 20, 24, 28, 32, 36, 40, 44, 48, 52, 56, 60, 64, 72, 80, 96)
        values.insert("14", "3.5rem");
        values.insert("16", "4rem");
        values.insert("20", "5rem");
        values.insert("24", "6rem");
        values.insert("28", "7rem");
        values.insert("32", "8rem");
        values.insert("36", "9rem");
        values.insert("40", "10rem");
        values.insert("44", "11rem");
        values.insert("48", "12rem");
        values.insert("52", "13rem");
        values.insert("56", "14rem");
        values.insert("60", "15rem");
        values.insert("64", "16rem");
        values.insert("72", "18rem");
        values.insert("80", "20rem");
        values.insert("96", "24rem");
    }
}

impl Default for SpacingValues {
    fn default() -> Self {
        Self::new()
    }
}

/// Spacing scale configuration
#[derive(Debug, Clone)]
pub struct SpacingScale {
    pub base_unit: f32,
    pub unit: SpacingUnit,
}

#[derive(Debug, Clone, Copy)]
pub enum SpacingUnit {
    Rem,
    Em,
    Px,
}

impl Default for SpacingScale {
    fn default() -> Self {
        Self {
            base_unit: 0.25, // 0.25rem base unit (4px at 16px root)
            unit: SpacingUnit::Rem,
        }
    }
}

impl SpacingScale {
    /// Convert a scale value to CSS value
    pub fn to_css_value(&self, scale_value: f32) -> String {
        let rem_value = scale_value * self.base_unit;

        match self.unit {
            SpacingUnit::Rem => format!("{}rem", rem_value),
            SpacingUnit::Em => format!("{}em", rem_value),
            SpacingUnit::Px => format!("{}px", rem_value * 16.0), // Assuming 16px root font size
        }
    }

    /// Get standard Tailwind spacing values
    pub fn get_standard_values(&self) -> Vec<(String, String)> {
        let standard_scales = vec![
            (0, "0"), (1, "0.5"), (2, "1"), (3, "1.5"), (4, "2"), (5, "2.5"),
            (6, "3"), (7, "3.5"), (8, "4"), (9, "5"), (10, "6"), (11, "7"),
            (12, "8"), (13, "9"), (14, "10"), (15, "11"), (16, "12"),
            (18, "14"), (20, "16"), (24, "20"), (28, "24"), (32, "28"),
            (36, "32"), (40, "36"), (44, "40"), (48, "44"), (52, "48"),
            (56, "52"), (60, "56"), (64, "60"), (72, "64"), (80, "72"),
            (96, "80"),
        ];

        standard_scales
            .into_iter()
            .map(|(scale, token)| (token.to_string(), self.to_css_value(scale as f32)))
            .collect()
    }
}

/// Spacing value utilities
pub struct SpacingValueUtils;

impl SpacingValueUtils {
    /// Parse arbitrary spacing values (e.g., [123px], [2rem])
    pub fn parse_arbitrary_value(value: &str) -> Option<String> {
        if value.starts_with('[') && value.ends_with(']') {
            let inner = &value[1..value.len() - 1];

            // Check if it's a valid CSS length
            if Self::is_valid_css_length(inner) {
                Some(inner.to_string())
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Check if a string is a valid CSS length value
    pub fn is_valid_css_length(value: &str) -> bool {
        // Basic validation for CSS length units
        let units = ["px", "rem", "em", "vh", "vw", "vmin", "vmax", "%"];

        for unit in &units {
            if value.ends_with(unit) {
                let num_part = value.trim_end_matches(unit);
                return num_part.parse::<f32>().is_ok();
            }
        }

        // Allow plain numbers (interpreted as rem)
        value.parse::<f32>().is_ok()
    }

    /// Convert spacing token to numeric value
    pub fn token_to_numeric(token: &str) -> Option<f32> {
        match token {
            "0" => Some(0.0),
            "px" => Some(0.0625), // 1px in rem (assuming 16px base)
            "0.5" => Some(0.5),
            "1" => Some(1.0),
            "1.5" => Some(1.5),
            "2" => Some(2.0),
            "2.5" => Some(2.5),
            "3" => Some(3.0),
            "3.5" => Some(3.5),
            "4" => Some(4.0),
            "5" => Some(5.0),
            "6" => Some(6.0),
            "7" => Some(7.0),
            "8" => Some(8.0),
            "9" => Some(9.0),
            "10" => Some(10.0),
            "11" => Some(11.0),
            "12" => Some(12.0),
            "14" => Some(14.0),
            "16" => Some(16.0),
            "20" => Some(20.0),
            "24" => Some(24.0),
            "28" => Some(28.0),
            "32" => Some(32.0),
            "36" => Some(36.0),
            "40" => Some(40.0),
            "44" => Some(44.0),
            "48" => Some(48.0),
            "52" => Some(52.0),
            "56" => Some(56.0),
            "60" => Some(60.0),
            "64" => Some(64.0),
            "72" => Some(72.0),
            "80" => Some(80.0),
            "96" => Some(96.0),
            _ => None,
        }
    }

    /// Validate spacing token format
    pub fn is_valid_spacing_token(token: &str) -> bool {
        // Allow numbers, decimal points, and common spacing tokens
        token.chars().all(|c| c.is_alphanumeric() || c == '.' || c == '-')
            && !token.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spacing_values_basic() {
        let values = SpacingValues::new();

        assert_eq!(values.get_value("0"), Some("0".to_string()));
        assert_eq!(values.get_value("4"), Some("1rem".to_string()));
        assert_eq!(values.get_value("px"), Some("1px".to_string()));
        assert_eq!(values.get_value("invalid"), None);
    }

    #[test]
    fn spacing_values_validity() {
        let values = SpacingValues::new();

        assert!(values.is_valid_token("0"));
        assert!(values.is_valid_token("16"));
        assert!(!values.is_valid_token("999"));
    }

    #[test]
    fn spacing_scale_conversion() {
        let scale = SpacingScale::default();

        assert_eq!(scale.to_css_value(4.0), "1rem");
        assert_eq!(scale.to_css_value(0.0), "0rem");

        let px_scale = SpacingScale {
            base_unit: 0.25,
            unit: SpacingUnit::Px,
        };

        assert_eq!(px_scale.to_css_value(4.0), "16px");
    }

    #[test]
    fn arbitrary_value_parsing() {
        assert_eq!(SpacingValueUtils::parse_arbitrary_value("[10px]"), Some("10px".to_string()));
        assert_eq!(SpacingValueUtils::parse_arbitrary_value("[2rem]"), Some("2rem".to_string()));
        assert_eq!(SpacingValueUtils::parse_arbitrary_value("10px"), None);
        assert_eq!(SpacingValueUtils::parse_arbitrary_value("[invalid]"), None);
    }

    #[test]
    fn css_length_validation() {
        assert!(SpacingValueUtils::is_valid_css_length("10px"));
        assert!(SpacingValueUtils::is_valid_css_length("2.5rem"));
        assert!(SpacingValueUtils::is_valid_css_length("100%"));
        assert!(SpacingValueUtils::is_valid_css_length("16"));
        assert!(!SpacingValueUtils::is_valid_css_length("invalid"));
    }

    #[test]
    fn token_to_numeric_conversion() {
        assert_eq!(SpacingValueUtils::token_to_numeric("0"), Some(0.0));
        assert_eq!(SpacingValueUtils::token_to_numeric("4"), Some(4.0));
        assert_eq!(SpacingValueUtils::token_to_numeric("invalid"), None);
    }

    #[test]
    fn spacing_token_validation() {
        assert!(SpacingValueUtils::is_valid_spacing_token("0"));
        assert!(SpacingValueUtils::is_valid_spacing_token("16"));
        assert!(SpacingValueUtils::is_valid_spacing_token("2.5"));
        assert!(!SpacingValueUtils::is_valid_spacing_token(""));
        assert!(!SpacingValueUtils::is_valid_spacing_token("invalid token"));
    }
}
