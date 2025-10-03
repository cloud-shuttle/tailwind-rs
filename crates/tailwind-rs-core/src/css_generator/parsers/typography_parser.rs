//! Advanced Typography Parser for Tailwind-RS
//!
//! This parser handles advanced typography features including:
//! - Font feature settings (OpenType features)
//! - Font variants
//! - Text shadows
//! - Typography presets

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;

/// Advanced Typography Parser
#[derive(Debug, Clone)]
pub struct TypographyParser;

impl TypographyParser {
    pub fn new() -> Self {
        Self
    }

    /// Parse text shadow classes (text-shadow-*)
    fn parse_text_shadow_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(shadow) = class.strip_prefix("text-shadow") {
            let shadow_value = match shadow {
                "" => "0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)", // default
                "-none" => "none",
                "-sm" => "0 1px 2px 0 rgb(0 0 0 / 0.05)",
                "-md" => "0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)",
                "-lg" => "0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)",
                "-xl" => "0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)",
                "-2xl" => "0 25px 50px -12px rgb(0 0 0 / 0.25)",
                "-inner" => "inset 0 2px 4px 0 rgb(0 0 0 / 0.05)",
                _ => return None,
            };

            return Some(vec![CssProperty {
                name: "text-shadow".to_string(),
                value: shadow_value.to_string(),
                important: false,
            }]);
        }
        None
    }

    /// Parse font feature classes (font-feature-*)
    fn parse_font_feature_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(feature) = class.strip_prefix("font-feature-") {
            let feature_value = self.parse_font_feature_value(feature)?;
            return Some(vec![CssProperty {
                name: "font-feature-settings".to_string(),
                value: feature_value,
                important: false,
            }]);
        }
        None
    }

    /// Parse font variant classes (font-variant-*)
    fn parse_font_variant_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(variant) = class.strip_prefix("font-variant-") {
            let variant_value = self.parse_font_variant_value(variant)?;
            return Some(vec![CssProperty {
                name: "font-variant".to_string(),
                value: variant_value,
                important: false,
            }]);
        }
        None
    }

    // Value parsers for different typography features

    fn parse_font_feature_value(&self, feature: &str) -> Option<String> {
        match feature {
            // Ligatures
            "ligatures" => Some("\"liga\"".to_string()),
            "clig" => Some("\"clig\"".to_string()),
            "dlig" => Some("\"dlig\"".to_string()),
            "hlig" => Some("\"hlig\"".to_string()),
            "calt" => Some("\"calt\"".to_string()),

            // Stylistic Sets
            "ss01" => Some("\"ss01\"".to_string()),
            "ss02" => Some("\"ss02\"".to_string()),
            "ss03" => Some("\"ss03\"".to_string()),
            "ss04" => Some("\"ss04\"".to_string()),
            "ss05" => Some("\"ss05\"".to_string()),
            "ss06" => Some("\"ss06\"".to_string()),
            "ss07" => Some("\"ss07\"".to_string()),
            "ss08" => Some("\"ss08\"".to_string()),
            "ss09" => Some("\"ss09\"".to_string()),
            "ss10" => Some("\"ss10\"".to_string()),

            // Contextual Alternates
            "calt" => Some("\"calt\"".to_string()),

            // Swashes
            "swsh" => Some("\"swsh\"".to_string()),
            "cswh" => Some("\"cswh\"".to_string()),

            // Stylistic Alternates
            "salt" => Some("\"salt\"".to_string()),

            // Positioning
            "subs" => Some("\"subs\"".to_string()),
            "sups" => Some("\"sups\"".to_string()),

            // Fractions
            "frac" => Some("\"frac\"".to_string()),
            "afrc" => Some("\"afrc\"".to_string()),

            // Numerals
            "lnum" => Some("\"lnum\"".to_string()),
            "onum" => Some("\"onum\"".to_string()),
            "pnum" => Some("\"pnum\"".to_string()),
            "tnum" => Some("\"tnum\"".to_string()),

            // Case
            "smcp" => Some("\"smcp\"".to_string()),
            "pcap" => Some("\"pcap\"".to_string()),
            "cpsp" => Some("\"cpsp\"".to_string()),

            // Kerning
            "kern" => Some("\"kern\"".to_string()),

            // Handle custom features with values (e.g., "rlig-1")
            custom if custom.contains('-') => {
                let parts: Vec<&str> = custom.split('-').collect();
                if parts.len() >= 2 {
                    let tag = parts[0];
                    let value = parts[1];
                    Some(format!("\"{}\" {}", tag, value))
                } else {
                    None
                }
            }

            _ => None,
        }
    }

    fn parse_font_variant_value(&self, variant: &str) -> Option<String> {
        match variant {
            "normal" => Some("normal".to_string()),
            "small-caps" => Some("small-caps".to_string()),
            "all-small-caps" => Some("all-small-caps".to_string()),
            "petite-caps" => Some("petite-caps".to_string()),
            "all-petite-caps" => Some("all-petite-caps".to_string()),
            "unicase" => Some("unicase".to_string()),
            "titling-caps" => Some("titling-caps".to_string()),
            "lining-nums" => Some("lining-nums".to_string()),
            "oldstyle-nums" => Some("oldstyle-nums".to_string()),
            "proportional-nums" => Some("proportional-nums".to_string()),
            "tabular-nums" => Some("tabular-nums".to_string()),
            "diagonal-fractions" => Some("diagonal-fractions".to_string()),
            "stacked-fractions" => Some("stacked-fractions".to_string()),
            "ordinal" => Some("ordinal".to_string()),
            "slashed-zero" => Some("slashed-zero".to_string()),
            "superscript" => Some("superscript".to_string()),
            "subscript" => Some("subscript".to_string()),
            _ => None,
        }
    }
}

impl UtilityParser for TypographyParser {
    fn get_priority(&self) -> u32 {
        65 // Lower than layout parsers but higher than basic utilities
    }

    fn get_category(&self) -> ParserCategory {
        ParserCategory::Typography
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "text-shadow",
            "text-shadow-*",
            "font-feature-*",
            "font-variant-*",
        ]
    }

    fn parse_class(&self, class: &str) -> Option<Vec<crate::css_generator::types::CssProperty>> {
        // Try each parsing method in order
        if let Some(properties) = self.parse_text_shadow_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_font_feature_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_font_variant_class(class) {
            return Some(properties);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_text_shadow() {
        let parser = TypographyParser::new();

        // Test default text shadow
        let result = parser.parse_class("text-shadow");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].name, "text-shadow");
        assert!(properties[0].value.contains("rgb(0 0 0 / 0.1)"));

        // Test text-shadow-none
        let result = parser.parse_class("text-shadow-none");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].value, "none");

        // Test text-shadow-sm
        let result = parser.parse_class("text-shadow-sm");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert!(properties[0].value.contains("rgb(0 0 0 / 0.05)"));

        // Test text-shadow-inner
        let result = parser.parse_class("text-shadow-inner");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert!(properties[0].value.contains("inset"));
    }

    #[test]
    fn test_parse_font_features() {
        let parser = TypographyParser::new();

        // Test ligatures
        let result = parser.parse_class("font-feature-ligatures");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].name, "font-feature-settings");
        assert_eq!(properties[0].value, "\"liga\"");

        // Test stylistic sets
        let result = parser.parse_class("font-feature-ss01");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].value, "\"ss01\"");

        // Test small capitals
        let result = parser.parse_class("font-feature-smcp");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].value, "\"smcp\"");

        // Test tabular figures
        let result = parser.parse_class("font-feature-tnum");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].value, "\"tnum\"");

        // Test kerning
        let result = parser.parse_class("font-feature-kern");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].value, "\"kern\"");
    }

    #[test]
    fn test_parse_font_variants() {
        let parser = TypographyParser::new();

        // Test small caps
        let result = parser.parse_class("font-variant-small-caps");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].name, "font-variant");
        assert_eq!(properties[0].value, "small-caps");

        // Test lining nums
        let result = parser.parse_class("font-variant-lining-nums");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].value, "lining-nums");

        // Test tabular nums
        let result = parser.parse_class("font-variant-tabular-nums");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].value, "tabular-nums");

        // Test ordinal
        let result = parser.parse_class("font-variant-ordinal");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].value, "ordinal");
    }

    #[test]
    fn test_parse_custom_font_features() {
        let parser = TypographyParser::new();

        // Test custom feature with value
        let result = parser.parse_class("font-feature-rlig-1");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].name, "font-feature-settings");
        assert_eq!(properties[0].value, "\"rlig\" 1");
    }

    #[test]
    fn test_parser_metadata() {
        let parser = TypographyParser::new();

        assert_eq!(parser.get_priority(), 65);
        assert_eq!(parser.get_category(), ParserCategory::Typography);

        let patterns = parser.get_supported_patterns();
        assert!(patterns.contains(&"text-shadow"));
        assert!(patterns.contains(&"text-shadow-*"));
        assert!(patterns.contains(&"font-feature-*"));
        assert!(patterns.contains(&"font-variant-*"));
    }

    #[test]
    fn test_invalid_classes() {
        let parser = TypographyParser::new();

        assert!(parser.parse_class("invalid-class").is_none());
        assert!(parser.parse_class("text-shadow-invalid").is_none());
        assert!(parser.parse_class("font-feature-invalid").is_none());
        assert!(parser.parse_class("font-variant-invalid").is_none());
    }

    #[test]
    fn test_complex_text_shadow() {
        let parser = TypographyParser::new();

        // Test 2xl shadow
        let result = parser.parse_class("text-shadow-2xl");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert!(properties[0].value.contains("0.25"));
        assert!(properties[0].value.contains("50px"));
    }

    #[test]
    fn test_font_feature_contextual_alternates() {
        let parser = TypographyParser::new();

        let result = parser.parse_class("font-feature-calt");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].value, "\"calt\"");
    }

    #[test]
    fn test_font_variant_fractions() {
        let parser = TypographyParser::new();

        let result = parser.parse_class("font-variant-diagonal-fractions");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].value, "diagonal-fractions");

        let result = parser.parse_class("font-variant-stacked-fractions");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].value, "stacked-fractions");
    }
}
