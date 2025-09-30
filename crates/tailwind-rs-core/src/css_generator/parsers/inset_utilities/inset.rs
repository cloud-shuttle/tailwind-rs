//! General Inset Utilities Module
//!
//! Handles parsing of general inset utilities:
//! - inset-* (all sides)
//! - inset-x-* (horizontal inset)
//! - inset-y-* (vertical inset)

use crate::css_generator::types::CssProperty;
use super::values::InsetSpacingValues;

/// General inset utilities parser
#[derive(Debug, Clone)]
pub struct GeneralInsetParser {
    values: InsetSpacingValues,
}

impl GeneralInsetParser {
    /// Create a new general inset parser
    pub fn new(values: InsetSpacingValues) -> Self {
        Self { values }
    }

    /// Parse inset class (all sides)
    pub fn parse_inset_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Positive inset values
        if let Some(value) = class.strip_prefix("inset-") {
            if let Some(spacing_value) = self.values.get_spacing_value(value) {
                return Some(vec![CssProperty {
                    name: "inset".to_string(),
                    value: spacing_value.clone(),
                    important: false,
                }]);
            }
            if let Some(fraction_value) = self.values.get_fraction_value(value) {
                return Some(vec![CssProperty {
                    name: "inset".to_string(),
                    value: fraction_value.clone(),
                    important: false,
                }]);
            }
        }

        // Negative inset values
        if let Some(value) = class.strip_prefix("-inset-") {
            if let Some(spacing_value) = self.values.get_spacing_value(value) {
                return Some(vec![CssProperty {
                    name: "inset".to_string(),
                    value: format!("-{}", spacing_value),
                    important: false,
                }]);
            }
            if let Some(fraction_value) = self.values.get_fraction_value(value) {
                return Some(vec![CssProperty {
                    name: "inset".to_string(),
                    value: format!("-{}", fraction_value),
                    important: false,
                }]);
            }
        }

        // Special case for inset-auto
        if class == "inset-auto" {
            return Some(vec![CssProperty {
                name: "inset".to_string(),
                value: "auto".to_string(),
                important: false,
            }]);
        }

        None
    }

    /// Parse inset-x class (horizontal inset)
    pub fn parse_inset_x_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Positive inset-x values
        if let Some(value) = class.strip_prefix("inset-x-") {
            if let Some(spacing_value) = self.values.get_spacing_value(value) {
                return Some(vec![
                    CssProperty {
                        name: "left".to_string(),
                        value: spacing_value.clone(),
                        important: false,
                    },
                    CssProperty {
                        name: "right".to_string(),
                        value: spacing_value.clone(),
                        important: false,
                    },
                ]);
            }
            if let Some(fraction_value) = self.values.get_fraction_value(value) {
                return Some(vec![
                    CssProperty {
                        name: "left".to_string(),
                        value: fraction_value.clone(),
                        important: false,
                    },
                    CssProperty {
                        name: "right".to_string(),
                        value: fraction_value.clone(),
                        important: false,
                    },
                ]);
            }
        }

        // Negative inset-x values
        if let Some(value) = class.strip_prefix("-inset-x-") {
            if let Some(spacing_value) = self.values.get_spacing_value(value) {
                return Some(vec![
                    CssProperty {
                        name: "left".to_string(),
                        value: format!("-{}", spacing_value),
                        important: false,
                    },
                    CssProperty {
                        name: "right".to_string(),
                        value: format!("-{}", spacing_value),
                        important: false,
                    },
                ]);
            }
            if let Some(fraction_value) = self.values.get_fraction_value(value) {
                return Some(vec![
                    CssProperty {
                        name: "left".to_string(),
                        value: format!("-{}", fraction_value),
                        important: false,
                    },
                    CssProperty {
                        name: "right".to_string(),
                        value: format!("-{}", fraction_value),
                        important: false,
                    },
                ]);
            }
        }

        // Special case for inset-x-auto
        if class == "inset-x-auto" {
            return Some(vec![
                CssProperty {
                    name: "left".to_string(),
                    value: "auto".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "right".to_string(),
                    value: "auto".to_string(),
                    important: false,
                },
            ]);
        }

        None
    }

    /// Parse inset-y class (vertical inset)
    pub fn parse_inset_y_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Positive inset-y values
        if let Some(value) = class.strip_prefix("inset-y-") {
            if let Some(spacing_value) = self.values.get_spacing_value(value) {
                return Some(vec![
                    CssProperty {
                        name: "top".to_string(),
                        value: spacing_value.clone(),
                        important: false,
                    },
                    CssProperty {
                        name: "bottom".to_string(),
                        value: spacing_value.clone(),
                        important: false,
                    },
                ]);
            }
            if let Some(fraction_value) = self.values.get_fraction_value(value) {
                return Some(vec![
                    CssProperty {
                        name: "top".to_string(),
                        value: fraction_value.clone(),
                        important: false,
                    },
                    CssProperty {
                        name: "bottom".to_string(),
                        value: fraction_value.clone(),
                        important: false,
                    },
                ]);
            }
        }

        // Negative inset-y values
        if let Some(value) = class.strip_prefix("-inset-y-") {
            if let Some(spacing_value) = self.values.get_spacing_value(value) {
                return Some(vec![
                    CssProperty {
                        name: "top".to_string(),
                        value: format!("-{}", spacing_value),
                        important: false,
                    },
                    CssProperty {
                        name: "bottom".to_string(),
                        value: format!("-{}", spacing_value),
                        important: false,
                    },
                ]);
            }
            if let Some(fraction_value) = self.values.get_fraction_value(value) {
                return Some(vec![
                    CssProperty {
                        name: "top".to_string(),
                        value: format!("-{}", fraction_value),
                        important: false,
                    },
                    CssProperty {
                        name: "bottom".to_string(),
                        value: format!("-{}", fraction_value),
                        important: false,
                    },
                ]);
            }
        }

        // Special case for inset-y-auto
        if class == "inset-y-auto" {
            return Some(vec![
                CssProperty {
                    name: "top".to_string(),
                    value: "auto".to_string(),
                    important: false,
                },
                CssProperty {
                    name: "bottom".to_string(),
                    value: "auto".to_string(),
                    important: false,
                },
            ]);
        }

        None
    }

    /// Get supported general inset patterns
    pub fn supported_patterns(&self) -> Vec<String> {
        let mut patterns = Vec::new();

        // inset patterns
        for key in self.values.spacing_keys() {
            patterns.push(format!("inset-{}", key));
            patterns.push(format!("-inset-{}", key));
        }
        for key in self.values.fraction_keys() {
            patterns.push(format!("inset-{}", key));
            patterns.push(format!("-inset-{}", key));
        }
        patterns.push("inset-auto".to_string());

        // inset-x patterns
        for key in self.values.spacing_keys() {
            patterns.push(format!("inset-x-{}", key));
            patterns.push(format!("-inset-x-{}", key));
        }
        for key in self.values.fraction_keys() {
            patterns.push(format!("inset-x-{}", key));
            patterns.push(format!("-inset-x-{}", key));
        }
        patterns.push("inset-x-auto".to_string());

        // inset-y patterns
        for key in self.values.spacing_keys() {
            patterns.push(format!("inset-y-{}", key));
            patterns.push(format!("-inset-y-{}", key));
        }
        for key in self.values.fraction_keys() {
            patterns.push(format!("inset-y-{}", key));
            patterns.push(format!("-inset-y-{}", key));
        }
        patterns.push("inset-y-auto".to_string());

        patterns
    }
}

impl Default for GeneralInsetParser {
    fn default() -> Self {
        Self::new(InsetSpacingValues::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inset_class_parsing() {
        let parser = GeneralInsetParser::new(InsetSpacingValues::new());

        // Test positive inset
        let result = parser.parse_inset_class("inset-4");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties.len(), 1);
        assert_eq!(properties[0].name, "inset");
        assert_eq!(properties[0].value, "1rem");

        // Test negative inset
        let result = parser.parse_inset_class("-inset-2");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].value, "-0.5rem");

        // Test fraction
        let result = parser.parse_inset_class("inset-1/2");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].value, "50%");

        // Test inset-auto
        let result = parser.parse_inset_class("inset-auto");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].value, "auto");
    }

    #[test]
    fn inset_x_class_parsing() {
        let parser = GeneralInsetParser::new(InsetSpacingValues::new());

        let result = parser.parse_inset_x_class("inset-x-4");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties.len(), 2);
        assert_eq!(properties[0].name, "left");
        assert_eq!(properties[0].value, "1rem");
        assert_eq!(properties[1].name, "right");
        assert_eq!(properties[1].value, "1rem");

        // Test negative
        let result = parser.parse_inset_x_class("-inset-x-2");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].value, "-0.5rem");
        assert_eq!(properties[1].value, "-0.5rem");
    }

    #[test]
    fn inset_y_class_parsing() {
        let parser = GeneralInsetParser::new(InsetSpacingValues::new());

        let result = parser.parse_inset_y_class("inset-y-4");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties.len(), 2);
        assert_eq!(properties[0].name, "top");
        assert_eq!(properties[0].value, "1rem");
        assert_eq!(properties[1].name, "bottom");
        assert_eq!(properties[1].value, "1rem");
    }

    #[test]
    fn invalid_classes() {
        let parser = GeneralInsetParser::new(InsetSpacingValues::new());

        assert!(parser.parse_inset_class("inset-invalid").is_none());
        assert!(parser.parse_inset_x_class("inset-x-invalid").is_none());
        assert!(parser.parse_inset_y_class("inset-y-invalid").is_none());
    }
}
