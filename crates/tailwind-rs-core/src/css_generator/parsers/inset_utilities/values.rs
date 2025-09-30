//! Inset Values Module
//!
//! Contains spacing and fraction value mappings for inset utilities.

use std::collections::HashMap;

/// Spacing value mappings for inset utilities
#[derive(Debug, Clone)]
pub struct InsetSpacingValues {
    spacing_map: HashMap<String, String>,
    fraction_map: HashMap<String, String>,
}

impl InsetSpacingValues {
    /// Create a new InsetSpacingValues instance with all standard mappings
    pub fn new() -> Self {
        let mut spacing_map = HashMap::new();

        // Standard spacing values
        spacing_map.insert("0".to_string(), "0".to_string());
        spacing_map.insert("px".to_string(), "1px".to_string());
        spacing_map.insert("0.5".to_string(), "0.125rem".to_string());
        spacing_map.insert("1".to_string(), "0.25rem".to_string());
        spacing_map.insert("1.5".to_string(), "0.375rem".to_string());
        spacing_map.insert("2".to_string(), "0.5rem".to_string());
        spacing_map.insert("2.5".to_string(), "0.625rem".to_string());
        spacing_map.insert("3".to_string(), "0.75rem".to_string());
        spacing_map.insert("3.5".to_string(), "0.875rem".to_string());
        spacing_map.insert("4".to_string(), "1rem".to_string());
        spacing_map.insert("5".to_string(), "1.25rem".to_string());
        spacing_map.insert("6".to_string(), "1.5rem".to_string());
        spacing_map.insert("7".to_string(), "1.75rem".to_string());
        spacing_map.insert("8".to_string(), "2rem".to_string());
        spacing_map.insert("9".to_string(), "2.25rem".to_string());
        spacing_map.insert("10".to_string(), "2.5rem".to_string());
        spacing_map.insert("11".to_string(), "2.75rem".to_string());
        spacing_map.insert("12".to_string(), "3rem".to_string());
        spacing_map.insert("14".to_string(), "3.5rem".to_string());
        spacing_map.insert("16".to_string(), "4rem".to_string());
        spacing_map.insert("20".to_string(), "5rem".to_string());
        spacing_map.insert("24".to_string(), "6rem".to_string());
        spacing_map.insert("28".to_string(), "7rem".to_string());
        spacing_map.insert("32".to_string(), "8rem".to_string());
        spacing_map.insert("36".to_string(), "9rem".to_string());
        spacing_map.insert("40".to_string(), "10rem".to_string());
        spacing_map.insert("44".to_string(), "11rem".to_string());
        spacing_map.insert("48".to_string(), "12rem".to_string());
        spacing_map.insert("52".to_string(), "13rem".to_string());
        spacing_map.insert("56".to_string(), "14rem".to_string());
        spacing_map.insert("60".to_string(), "15rem".to_string());
        spacing_map.insert("64".to_string(), "16rem".to_string());
        spacing_map.insert("72".to_string(), "18rem".to_string());
        spacing_map.insert("80".to_string(), "20rem".to_string());
        spacing_map.insert("96".to_string(), "24rem".to_string());
        spacing_map.insert("auto".to_string(), "auto".to_string());
        spacing_map.insert("full".to_string(), "100%".to_string());

        let mut fraction_map = HashMap::new();
        fraction_map.insert("1/2".to_string(), "50%".to_string());
        fraction_map.insert("1/3".to_string(), "33.333333%".to_string());
        fraction_map.insert("2/3".to_string(), "66.666667%".to_string());
        fraction_map.insert("1/4".to_string(), "25%".to_string());
        fraction_map.insert("2/4".to_string(), "50%".to_string());
        fraction_map.insert("3/4".to_string(), "75%".to_string());
        fraction_map.insert("1/5".to_string(), "20%".to_string());
        fraction_map.insert("2/5".to_string(), "40%".to_string());
        fraction_map.insert("3/5".to_string(), "60%".to_string());
        fraction_map.insert("4/5".to_string(), "80%".to_string());
        fraction_map.insert("1/6".to_string(), "16.666667%".to_string());
        fraction_map.insert("2/6".to_string(), "33.333333%".to_string());
        fraction_map.insert("3/6".to_string(), "50%".to_string());
        fraction_map.insert("4/6".to_string(), "66.666667%".to_string());
        fraction_map.insert("5/6".to_string(), "83.333333%".to_string());
        fraction_map.insert("1/12".to_string(), "8.333333%".to_string());
        fraction_map.insert("2/12".to_string(), "16.666667%".to_string());
        fraction_map.insert("3/12".to_string(), "25%".to_string());
        fraction_map.insert("4/12".to_string(), "33.333333%".to_string());
        fraction_map.insert("5/12".to_string(), "41.666667%".to_string());
        fraction_map.insert("6/12".to_string(), "50%".to_string());
        fraction_map.insert("7/12".to_string(), "58.333333%".to_string());
        fraction_map.insert("8/12".to_string(), "66.666667%".to_string());
        fraction_map.insert("9/12".to_string(), "75%".to_string());
        fraction_map.insert("10/12".to_string(), "83.333333%".to_string());
        fraction_map.insert("11/12".to_string(), "91.666667%".to_string());

        Self {
            spacing_map,
            fraction_map,
        }
    }

    /// Get spacing value for a given key
    pub fn get_spacing_value(&self, key: &str) -> Option<&String> {
        self.spacing_map.get(key)
    }

    /// Get fraction value for a given key
    pub fn get_fraction_value(&self, key: &str) -> Option<&String> {
        self.fraction_map.get(key)
    }

    /// Get all spacing keys
    pub fn spacing_keys(&self) -> Vec<&String> {
        self.spacing_map.keys().collect()
    }

    /// Get all fraction keys
    pub fn fraction_keys(&self) -> Vec<&String> {
        self.fraction_map.keys().collect()
    }

    /// Check if a key is a valid spacing value
    pub fn is_valid_spacing(&self, key: &str) -> bool {
        self.spacing_map.contains_key(key)
    }

    /// Check if a key is a valid fraction value
    pub fn is_valid_fraction(&self, key: &str) -> bool {
        self.fraction_map.contains_key(key)
    }
}

impl Default for InsetSpacingValues {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spacing_values_creation() {
        let values = InsetSpacingValues::new();

        // Test some standard spacing values
        assert_eq!(values.get_spacing_value("0"), Some(&"0".to_string()));
        assert_eq!(values.get_spacing_value("4"), Some(&"1rem".to_string()));
        assert_eq!(values.get_spacing_value("auto"), Some(&"auto".to_string()));
        assert_eq!(values.get_spacing_value("full"), Some(&"100%".to_string()));

        // Test invalid key
        assert_eq!(values.get_spacing_value("invalid"), None);
    }

    #[test]
    fn fraction_values() {
        let values = InsetSpacingValues::new();

        assert_eq!(values.get_fraction_value("1/2"), Some(&"50%".to_string()));
        assert_eq!(values.get_fraction_value("1/4"), Some(&"25%".to_string()));
        assert_eq!(values.get_fraction_value("3/4"), Some(&"75%".to_string()));

        // Test invalid fraction
        assert_eq!(values.get_fraction_value("invalid"), None);
    }

    #[test]
    fn validation_methods() {
        let values = InsetSpacingValues::new();

        assert!(values.is_valid_spacing("0"));
        assert!(values.is_valid_spacing("auto"));
        assert!(!values.is_valid_spacing("invalid"));

        assert!(values.is_valid_fraction("1/2"));
        assert!(values.is_valid_fraction("1/4"));
        assert!(!values.is_valid_fraction("invalid"));
    }

    #[test]
    fn keys_collection() {
        let values = InsetSpacingValues::new();

        let spacing_keys = values.spacing_keys();
        assert!(!spacing_keys.is_empty());
        assert!(spacing_keys.contains(&&"0".to_string()));
        assert!(spacing_keys.contains(&&"auto".to_string()));

        let fraction_keys = values.fraction_keys();
        assert!(!fraction_keys.is_empty());
        assert!(fraction_keys.contains(&&"1/2".to_string()));
        assert!(fraction_keys.contains(&&"1/4".to_string()));
    }
}
