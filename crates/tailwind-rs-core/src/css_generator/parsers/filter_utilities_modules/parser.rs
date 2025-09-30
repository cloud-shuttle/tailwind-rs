//! Filter Utilities Parser Module
//!
//! Main parsing logic for filter utilities:
//! - FilterUtilitiesParser: Core parser implementation
//! - Individual filter type parsers

use super::types::*;
use super::utilities::*;
use crate::css_generator::types::CssProperty;

/// Core filter utilities parser
#[derive(Debug, Clone)]
pub struct FilterUtilitiesParser;

impl FilterUtilitiesParser {
    pub fn new() -> Self {
        Self
    }

    /// Parse filter classes
    pub fn parse_filter_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "filter-none" => Some(vec![FilterCssGenerator::generate_filter_property("none")]),
            _ => {
                // Custom properties for filter
                if let Some(value) = FilterValueParser::extract_custom_property(class, "filter") {
                    return Some(vec![FilterCssGenerator::generate_filter_property(&format!("var({})", value))]);
                }

                // Arbitrary values for filter
                if let Some(value) = FilterValueParser::extract_arbitrary_value(class, "filter") {
                    return Some(vec![FilterCssGenerator::generate_filter_property(&value)]);
                }

                None
            }
        }
    }

    /// Parse blur classes
    pub fn parse_blur_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(blur_size) = FilterClassParser::parse_blur_size(class) {
            let value = format!("blur({})", blur_size.css_variable());
            return Some(vec![FilterCssGenerator::generate_filter_property(&value)]);
        }

        // Custom properties for blur
        if let Some(value) = FilterValueParser::extract_custom_property(class, "blur") {
            let css_value = format!("blur(var({}))", value);
            return Some(vec![FilterCssGenerator::generate_filter_property(&css_value)]);
        }

        // Arbitrary values for blur
        if let Some(value) = FilterValueParser::extract_arbitrary_value(class, "blur") {
            let css_value = format!("blur({})", value);
            return Some(vec![FilterCssGenerator::generate_filter_property(&css_value)]);
        }

        None
    }

    /// Parse brightness classes
    pub fn parse_brightness_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_percentage_based_filter(class, "brightness")
    }

    /// Parse contrast classes
    pub fn parse_contrast_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_percentage_based_filter(class, "contrast")
    }

    /// Parse grayscale classes
    pub fn parse_grayscale_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_percentage_based_filter(class, "grayscale")
    }

    /// Parse invert classes
    pub fn parse_invert_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_percentage_based_filter(class, "invert")
    }

    /// Parse opacity classes
    pub fn parse_opacity_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_percentage_based_filter(class, "opacity")
    }

    /// Parse saturate classes
    pub fn parse_saturate_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_percentage_based_filter(class, "saturate")
    }

    /// Parse sepia classes
    pub fn parse_sepia_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        self.parse_percentage_based_filter(class, "sepia")
    }

    /// Parse hue-rotate classes
    pub fn parse_hue_rotate_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value_str) = FilterClassParser::extract_filter_value(class, &FilterType::HueRotate) {
            // Custom properties for hue-rotate
            if let Some(value) = FilterValueParser::extract_custom_property(&format!("hue-rotate-{}", value_str), "hue-rotate") {
                let css_value = format!("hue-rotate(var({}))", value);
                return Some(vec![FilterCssGenerator::generate_filter_property(&css_value)]);
            }

            // Arbitrary values for hue-rotate
            if let Some(value) = FilterValueParser::extract_arbitrary_value(&format!("hue-rotate-{}", value_str), "hue-rotate") {
                if let Some(angle) = FilterValueParser::parse_angle(&value) {
                    let css_value = format!("hue-rotate({}deg)", angle);
                    return Some(vec![FilterCssGenerator::generate_filter_property(&css_value)]);
                }
                let css_value = format!("hue-rotate({})", value);
                return Some(vec![FilterCssGenerator::generate_filter_property(&css_value)]);
            }

            // Predefined angle values
            if let Some(angle) = self.parse_predefined_angle(&value_str) {
                let css_value = format!("hue-rotate({}deg)", angle);
                return Some(vec![FilterCssGenerator::generate_filter_property(&css_value)]);
            }
        }

        None
    }

    /// Parse drop-shadow classes
    pub fn parse_drop_shadow_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(value_str) = FilterClassParser::extract_filter_value(class, &FilterType::DropShadow) {
            // Custom properties for drop-shadow
            if let Some(value) = FilterValueParser::extract_custom_property(&format!("drop-shadow-{}", value_str), "drop-shadow") {
                let css_value = format!("drop-shadow(var({}))", value);
                return Some(vec![FilterCssGenerator::generate_filter_property(&css_value)]);
            }

            // Arbitrary values for drop-shadow
            if let Some(value) = FilterValueParser::extract_arbitrary_value(&format!("drop-shadow-{}", value_str), "drop-shadow") {
                let css_value = format!("drop-shadow({})", value);
                return Some(vec![FilterCssGenerator::generate_filter_property(&css_value)]);
            }

            // Predefined drop-shadow values
            if let Some(shadow_value) = self.parse_predefined_drop_shadow(&value_str) {
                let css_value = format!("drop-shadow({})", shadow_value);
                return Some(vec![FilterCssGenerator::generate_filter_property(&css_value)]);
            }
        }

        None
    }

    /// Parse backdrop filter classes
    pub fn parse_backdrop_filter_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        match class {
            "backdrop-filter-none" => Some(vec![FilterCssGenerator::generate_backdrop_filter_property("none")]),
            _ => {
                // Convert backdrop filter to regular filter parsing
                if let Some(regular_class) = BackdropFilterUtils::convert_to_regular_filter(class) {
                    if let Some(mut properties) = self.parse_class(&regular_class) {
                        // Convert filter properties to backdrop-filter properties
                        for prop in &mut properties {
                            if prop.name == "filter" {
                                prop.name = "backdrop-filter".to_string();
                            }
                        }
                        return Some(properties);
                    }
                }

                None
            }
        }
    }

    /// Helper method for percentage-based filters
    fn parse_percentage_based_filter(&self, class: &str, filter_name: &str) -> Option<Vec<CssProperty>> {
        if let Some(value_str) = FilterClassParser::extract_filter_value(class, &FilterType::Brightness) {
            // Custom properties
            if let Some(value) = FilterValueParser::extract_custom_property(&format!("{}-{}", filter_name, value_str), filter_name) {
                let css_value = format!("{}(var({}))", filter_name, value);
                return Some(vec![FilterCssGenerator::generate_filter_property(&css_value)]);
            }

            // Arbitrary values
            if let Some(value) = FilterValueParser::extract_arbitrary_value(&format!("{}-{}", filter_name, value_str), filter_name) {
                if let Some(percentage) = FilterValueParser::parse_percentage(&value) {
                    let css_value = format!("{}({}%)", filter_name, percentage);
                    return Some(vec![FilterCssGenerator::generate_filter_property(&css_value)]);
                }
                let css_value = format!("{}({})", filter_name, value);
                return Some(vec![FilterCssGenerator::generate_filter_property(&css_value)]);
            }

            // Predefined percentage values
            if let Some(percentage) = self.parse_predefined_percentage(&value_str) {
                let css_value = format!("{}({}%)", filter_name, percentage);
                return Some(vec![FilterCssGenerator::generate_filter_property(&css_value)]);
            }
        }

        None
    }

    /// Parse predefined percentage values
    fn parse_predefined_percentage(&self, value: &str) -> Option<f32> {
        match value {
            "0" => Some(0.0),
            "25" => Some(25.0),
            "50" => Some(50.0),
            "75" => Some(75.0),
            "100" => Some(100.0),
            "125" => Some(125.0),
            "150" => Some(150.0),
            "200" => Some(200.0),
            _ => None,
        }
    }

    /// Parse predefined angle values for hue-rotate
    fn parse_predefined_angle(&self, value: &str) -> Option<f32> {
        match value {
            "0" => Some(0.0),
            "15" => Some(15.0),
            "30" => Some(30.0),
            "60" => Some(60.0),
            "90" => Some(90.0),
            "120" => Some(120.0),
            "150" => Some(150.0),
            "180" => Some(180.0),
            "210" => Some(210.0),
            "240" => Some(240.0),
            "270" => Some(270.0),
            "300" => Some(300.0),
            "330" => Some(330.0),
            _ => None,
        }
    }

    /// Parse predefined drop-shadow values
    fn parse_predefined_drop_shadow(&self, value: &str) -> Option<&'static str> {
        match value {
            "sm" => Some("0 1px 1px rgb(0 0 0 / 0.05)"),
            "md" => Some("0 4px 3px rgb(0 0 0 / 0.07), 0 2px 2px rgb(0 0 0 / 0.06)"),
            "lg" => Some("0 10px 8px rgb(0 0 0 / 0.04), 0 4px 3px rgb(0 0 0 / 0.1)"),
            "xl" => Some("0 20px 13px rgb(0 0 0 / 0.03), 0 8px 5px rgb(0 0 0 / 0.08)"),
            "2xl" => Some("0 25px 25px rgb(0 0 0 / 0.15)"),
            "none" => Some("0 0 #0000"),
            _ => None,
        }
    }

    /// Main parsing method that dispatches to specific filter parsers
    pub fn parse_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        // Check for backdrop filters first
        if BackdropFilterUtils::is_backdrop_filter(class) {
            return self.parse_backdrop_filter_class(class);
        }

        // Check for general filter classes
        if class.starts_with("filter-") {
            return self.parse_filter_class(class);
        }

        // Dispatch to specific filter parsers
        if class.starts_with("blur-") {
            self.parse_blur_class(class)
        } else if class.starts_with("brightness-") {
            self.parse_brightness_class(class)
        } else if class.starts_with("contrast-") {
            self.parse_contrast_class(class)
        } else if class.starts_with("drop-shadow-") {
            self.parse_drop_shadow_class(class)
        } else if class.starts_with("grayscale-") {
            self.parse_grayscale_class(class)
        } else if class.starts_with("hue-rotate-") {
            self.parse_hue_rotate_class(class)
        } else if class.starts_with("invert-") {
            self.parse_invert_class(class)
        } else if class.starts_with("opacity-") {
            self.parse_opacity_class(class)
        } else if class.starts_with("saturate-") {
            self.parse_saturate_class(class)
        } else if class.starts_with("sepia-") {
            self.parse_sepia_class(class)
        } else {
            None
        }
    }
}

impl Default for FilterUtilitiesParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_utilities_parser_creation() {
        let parser = FilterUtilitiesParser::new();
        assert!(parser.parse_class("filter-none").is_some());
    }

    #[test]
    fn parse_blur_classes() {
        let parser = FilterUtilitiesParser::new();

        let result = parser.parse_blur_class("blur-md");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].value, "blur(var(--blur-md))");

        let result = parser.parse_blur_class("blur-[5px]");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].value, "blur(5px)");
    }

    #[test]
    fn parse_percentage_based_filters() {
        let parser = FilterUtilitiesParser::new();

        let result = parser.parse_brightness_class("brightness-50");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].value, "brightness(50%)");

        let result = parser.parse_contrast_class("contrast-[1.2]");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].value, "contrast(1.2)");
    }

    #[test]
    fn parse_hue_rotate() {
        let parser = FilterUtilitiesParser::new();

        let result = parser.parse_hue_rotate_class("hue-rotate-90");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].value, "hue-rotate(90deg)");
    }

    #[test]
    fn parse_drop_shadow() {
        let parser = FilterUtilitiesParser::new();

        let result = parser.parse_drop_shadow_class("drop-shadow-lg");
        assert!(result.is_some());
        // The exact value depends on the predefined shadow, but it should exist
        assert!(result.unwrap()[0].value.starts_with("drop-shadow("));
    }

    #[test]
    fn parse_backdrop_filters() {
        let parser = FilterUtilitiesParser::new();

        let result = parser.parse_backdrop_filter_class("backdrop-blur-md");
        assert!(result.is_some());
        let prop = &result.unwrap()[0];
        assert_eq!(prop.name, "backdrop-filter");
        assert_eq!(prop.value, "blur(var(--blur-md))");
    }

    #[test]
    fn parse_none_filters() {
        let parser = FilterUtilitiesParser::new();

        let result = parser.parse_filter_class("filter-none");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].value, "none");

        let result = parser.parse_backdrop_filter_class("backdrop-filter-none");
        assert!(result.is_some());
        assert_eq!(result.unwrap()[0].value, "none");
    }
}
