//! Rotation Parser Module
//!
//! Handles parsing of rotation transform utilities:
//! - rotate-0, rotate-1, rotate-2, ..., rotate-180, -rotate-1, -rotate-2, etc.
//! - rotate-x-*, rotate-y-*, rotate-z-* for 3D rotations

use crate::css_generator::types::CssProperty;

#[derive(Debug, Clone)]
pub struct RotationParser;

impl RotationParser {
    pub fn new() -> Self { Self }

    /// Parse rotation classes
    pub fn parse_rotate_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Handle negative rotations (-rotate-*)
        if let Some(negative_value) = class.strip_prefix("-rotate-") {
            if let Ok(degrees) = negative_value.parse::<i32>() {
                let negative_degrees = -degrees;
                return Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: format!("rotate({}deg)", negative_degrees),
                    important: false,
                }]);
            }
        }

        // Handle positive rotations (rotate-*)
        if let Some(value) = class.strip_prefix("rotate-") {
            if let Ok(degrees) = value.parse::<i32>() {
                return Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: format!("rotate({}deg)", degrees),
                    important: false,
                }]);
            }
        }

        None
    }

    /// Parse 3D rotation classes
    pub fn parse_rotate_3d_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Handle X-axis rotations
        if let Some(value) = class.strip_prefix("rotate-x-") {
            if let Ok(degrees) = value.parse::<i32>() {
                return Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: format!("rotateX({}deg)", degrees),
                    important: false,
                }]);
            }
        }

        // Handle Y-axis rotations
        if let Some(value) = class.strip_prefix("rotate-y-") {
            if let Ok(degrees) = value.parse::<i32>() {
                return Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: format!("rotateY({}deg)", degrees),
                    important: false,
                }]);
            }
        }

        // Handle Z-axis rotations
        if let Some(value) = class.strip_prefix("rotate-z-") {
            if let Ok(degrees) = value.parse::<i32>() {
                return Some(vec![CssProperty {
                    name: "transform".to_string(),
                    value: format!("rotateZ({}deg)", degrees),
                    important: false,
                }]);
            }
        }

        None
    }

    /// Get supported rotation patterns
    pub fn supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "rotate-0", "rotate-1", "rotate-2", "rotate-3", "rotate-6", "rotate-12", "rotate-45", "rotate-90", "rotate-180",
            "-rotate-1", "-rotate-2", "-rotate-3", "-rotate-6", "-rotate-12", "-rotate-45", "-rotate-90", "-rotate-180",
            "rotate-x-0", "rotate-x-45", "rotate-x-90", "rotate-x-180",
            "rotate-y-0", "rotate-y-45", "rotate-y-90", "rotate-y-180",
            "rotate-z-0", "rotate-z-45", "rotate-z-90", "rotate-z-180",
        ]
    }

    /// Check if a class is a valid rotation class
    pub fn is_valid_rotation_class(&self, class: &str) -> bool {
        // Check negative rotations
        if let Some(value) = class.strip_prefix("-rotate-") {
            return value.parse::<i32>().is_ok_and(|deg| deg >= 1 && deg <= 180);
        }

        // Check positive rotations
        if let Some(value) = class.strip_prefix("rotate-") {
            return value.parse::<i32>().is_ok_and(|deg| deg >= 0 && deg <= 180);
        }

        false
    }
}

impl Default for RotationParser {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_rotation_parsing() {
        let parser = RotationParser::new();

        // Test positive rotations
        let test_cases = vec![
            ("rotate-0", "rotate(0deg)"),
            ("rotate-45", "rotate(45deg)"),
            ("rotate-90", "rotate(90deg)"),
            ("rotate-180", "rotate(180deg)"),
        ];

        for (class, expected_value) in test_cases {
            let result = parser.parse_rotate_class(class);
            assert!(result.is_some(), "Failed to parse: {}", class);
            let properties = result.unwrap();
            assert_eq!(properties.len(), 1);
            assert_eq!(properties[0].name, "transform");
            assert_eq!(properties[0].value, expected_value);
            assert!(!properties[0].important);
        }
    }

    #[test]
    fn negative_rotation_parsing() {
        let parser = RotationParser::new();

        // Test negative rotations
        let test_cases = vec![
            ("-rotate-1", "rotate(-1deg)"),
            ("-rotate-3", "rotate(-3deg)"),
            ("-rotate-45", "rotate(-45deg)"),
            ("-rotate-90", "rotate(-90deg)"),
            ("-rotate-180", "rotate(-180deg)"),
        ];

        for (class, expected_value) in test_cases {
            let result = parser.parse_rotate_class(class);
            assert!(result.is_some(), "Failed to parse: {}", class);
            let properties = result.unwrap();
            assert_eq!(properties.len(), 1);
            assert_eq!(properties[0].name, "transform");
            assert_eq!(properties[0].value, expected_value);
            assert!(!properties[0].important);
        }
    }

    #[test]
    fn rotation_3d_parsing() {
        let parser = RotationParser::new();

        // Test 3D rotations
        let test_cases = vec![
            ("rotate-x-45", "rotateX(45deg)"),
            ("rotate-y-90", "rotateY(90deg)"),
            ("rotate-z-180", "rotateZ(180deg)"),
        ];

        for (class, expected_value) in test_cases {
            let result = parser.parse_rotate_3d_class(class);
            assert!(result.is_some(), "Failed to parse: {}", class);
            let properties = result.unwrap();
            assert_eq!(properties.len(), 1);
            assert_eq!(properties[0].name, "transform");
            assert_eq!(properties[0].value, expected_value);
            assert!(!properties[0].important);
        }
    }

    #[test]
    fn invalid_rotation_parsing() {
        let parser = RotationParser::new();

        // Test invalid classes
        assert!(parser.parse_rotate_class("rotate-181").is_none());
        assert!(parser.parse_rotate_class("-rotate-0").is_none());
        assert!(parser.parse_rotate_class("rotate-invalid").is_none());
        assert!(parser.parse_rotate_class("rotate").is_none());
        assert!(parser.parse_rotate_3d_class("rotate-w-45").is_none());
    }

    #[test]
    fn supported_patterns() {
        let parser = RotationParser::new();
        let patterns = parser.supported_patterns();

        // Should contain basic rotations
        assert!(patterns.contains(&"rotate-0"));
        assert!(patterns.contains(&"rotate-90"));
        assert!(patterns.contains(&"rotate-180"));

        // Should contain negative rotations
        assert!(patterns.contains(&"-rotate-1"));
        assert!(patterns.contains(&"-rotate-45"));
        assert!(patterns.contains(&"-rotate-180"));

        // Should contain 3D rotations
        assert!(patterns.contains(&"rotate-x-45"));
        assert!(patterns.contains(&"rotate-y-90"));
        assert!(patterns.contains(&"rotate-z-180"));

        // Should have reasonable number of patterns
        assert!(patterns.len() > 500); // 181 positive + 180 negative + 3*181 3D
    }

    #[test]
    fn validation() {
        let parser = RotationParser::new();

        // Valid classes
        assert!(parser.is_valid_rotation_class("rotate-45"));
        assert!(parser.is_valid_rotation_class("-rotate-3"));
        assert!(parser.is_valid_rotation_class("rotate-180"));
        assert!(parser.is_valid_rotation_class("-rotate-90"));

        // Invalid classes
        assert!(!parser.is_valid_rotation_class("rotate-181"));
        assert!(!parser.is_valid_rotation_class("-rotate-0"));
        assert!(!parser.is_valid_rotation_class("rotate-invalid"));
        assert!(!parser.is_valid_rotation_class("rotate"));
        assert!(!parser.is_valid_rotation_class("rotate-x-invalid"));
    }

    #[test]
    fn comprehensive_rotation_test() {
        let parser = RotationParser::new();
        let test_cases = vec![
            ("rotate-0", Some("rotate(0deg)")),
            ("rotate-45", Some("rotate(45deg)")),
            ("rotate-90", Some("rotate(90deg)")),
            ("-rotate-3", Some("rotate(-3deg)")),
            ("-rotate-45", Some("rotate(-45deg)")),
            ("-rotate-180", Some("rotate(-180deg)")),
            ("rotate-x-45", Some("rotateX(45deg)")),
            ("rotate-y-90", Some("rotateY(90deg)")),
            ("rotate-z-180", Some("rotateZ(180deg)")),
            ("rotate-181", None),
            ("-rotate-0", None),
            ("rotate-invalid", None),
            ("invalid", None),
        ];

        for (class, expected_value) in test_cases {
            let result = if class.contains("-x-") || class.contains("-y-") || class.contains("-z-") {
                parser.parse_rotate_3d_class(class)
            } else {
                parser.parse_rotate_class(class)
            };

            match expected_value {
                Some(value) => {
                    assert!(result.is_some(), "Expected parsing for: {}", class);
                    assert_eq!(result.unwrap()[0].value, value, "Wrong value for: {}", class);
                }
                None => {
                    assert!(result.is_none(), "Expected no parsing for: {}", class);
                }
            }
        }
    }
}
