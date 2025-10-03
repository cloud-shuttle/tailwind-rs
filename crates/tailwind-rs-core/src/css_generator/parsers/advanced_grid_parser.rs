//! Advanced CSS Grid Parser for Tailwind-RS
//!
//! This parser handles advanced CSS Grid features including:
//! - Grid template areas
//! - Masonry layout
//! - Subgrid support
//! - Complex grid patterns
//! - Named grid lines

use super::{ParserCategory, UtilityParser};
use crate::css_generator::types::CssProperty;

/// Advanced CSS Grid Parser
#[derive(Debug, Clone)]
pub struct AdvancedGridParser;

impl AdvancedGridParser {
    pub fn new() -> Self {
        Self
    }

    /// Parse grid areas classes (grid-areas-*)
    fn parse_grid_areas_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(areas) = class.strip_prefix("grid-areas-") {
            let areas_value = self.parse_grid_areas_value(areas)?;
            return Some(vec![CssProperty {
                name: "grid-template-areas".to_string(),
                value: areas_value,
                important: false,
            }]);
        }
        None
    }

    /// Parse grid masonry classes (grid-masonry-*)
    fn parse_grid_masonry_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(masonry) = class.strip_prefix("grid-masonry-") {
            let masonry_value = self.parse_masonry_value(masonry)?;
            return Some(vec![CssProperty {
                name: "grid-template-columns".to_string(),
                value: masonry_value,
                important: false,
            }]);
        }
        None
    }

    /// Parse grid subgrid classes (grid-subgrid-*)
    fn parse_grid_subgrid_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(subgrid) = class.strip_prefix("grid-subgrid-") {
            let subgrid_value = self.parse_subgrid_value(subgrid)?;
            return Some(vec![
                CssProperty {
                    name: "grid-template-columns".to_string(),
                    value: subgrid_value.clone(),
                    important: false,
                },
                CssProperty {
                    name: "grid-template-rows".to_string(),
                    value: subgrid_value,
                    important: false,
                },
            ]);
        }
        None
    }

    /// Parse grid auto fit classes (grid-cols-auto-fit-*)
    fn parse_grid_auto_fit_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(params) = class.strip_prefix("grid-cols-auto-fit-") {
            let template_value = self.parse_auto_fit_value(params)?;
            return Some(vec![CssProperty {
                name: "grid-template-columns".to_string(),
                value: template_value,
                important: false,
            }]);
        }
        None
    }

    /// Parse grid auto fill classes (grid-cols-auto-fill-*)
    fn parse_grid_auto_fill_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(params) = class.strip_prefix("grid-cols-auto-fill-") {
            let template_value = self.parse_auto_fill_value(params)?;
            return Some(vec![CssProperty {
                name: "grid-template-columns".to_string(),
                value: template_value,
                important: false,
            }]);
        }
        None
    }

    /// Parse grid named lines classes (grid-rows-named-*, grid-cols-named-*)
    fn parse_grid_named_lines_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(lines) = class.strip_prefix("grid-rows-named-") {
            let template_value = self.parse_named_lines_value(lines)?;
            return Some(vec![CssProperty {
                name: "grid-template-rows".to_string(),
                value: template_value,
                important: false,
            }]);
        }
        if let Some(lines) = class.strip_prefix("grid-cols-named-") {
            let template_value = self.parse_named_lines_value(lines)?;
            return Some(vec![CssProperty {
                name: "grid-template-columns".to_string(),
                value: template_value,
                important: false,
            }]);
        }
        None
    }

    /// Parse grid complex pattern classes (grid-cols-complex-*)
    fn parse_grid_complex_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(pattern) = class.strip_prefix("grid-cols-complex-") {
            let template_value = self.parse_complex_pattern_value(pattern)?;
            return Some(vec![CssProperty {
                name: "grid-template-columns".to_string(),
                value: template_value,
                important: false,
            }]);
        }
        None
    }

    /// Parse grid area placement classes (grid-area-*)
    fn parse_grid_area_placement_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(area) = class.strip_prefix("grid-area-") {
            return Some(vec![CssProperty {
                name: "grid-area".to_string(),
                value: area.to_string(),
                important: false,
            }]);
        }
        None
    }

    /// Parse grid flow classes (grid-flow-*)
    fn parse_grid_flow_class(&self, class: &str) -> Option<Vec<CssProperty>> {
        if let Some(flow) = class.strip_prefix("grid-flow-") {
            let flow_value = match flow {
                "dense" => "dense",
                "row-dense" => "row dense",
                "col-dense" => "column dense",
                _ => return None,
            };
            return Some(vec![CssProperty {
                name: "grid-auto-flow".to_string(),
                value: flow_value.to_string(),
                important: false,
            }]);
        }
        None
    }

    // Value parsers for different grid features

    fn parse_grid_areas_value(&self, areas: &str) -> Option<String> {
        match areas {
            "header-main-sidebar" => Some("\"header header\" \"main sidebar\"".to_string()),
            "header-sidebar-main" => Some("\"header sidebar\" \"header main\"".to_string()),
            "sidebar-main-footer" => Some("\"sidebar main\" \"sidebar footer\"".to_string()),
            "header-main-footer" => Some("\"header\" \"main\" \"footer\"".to_string()),
            _ if areas.contains('-') => {
                // Handle custom patterns like "header-main-sidebar"
                let parts: Vec<&str> = areas.split('-').collect();
                if parts.len() >= 2 {
                    Some(format!("\"{}\"", parts.join(" ")))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn parse_masonry_value(&self, masonry: &str) -> Option<String> {
        match masonry {
            "horizontal" | "h" => Some("masonry".to_string()),
            "vertical" | "v" => Some("masonry".to_string()),
            "dense" => Some("masonry dense".to_string()),
            _ => None,
        }
    }

    fn parse_subgrid_value(&self, subgrid: &str) -> Option<String> {
        match subgrid {
            "inherit" => Some("subgrid".to_string()),
            "auto" => Some("auto".to_string()),
            _ => None,
        }
    }

    fn parse_auto_fit_value(&self, params: &str) -> Option<String> {
        // Parse format like "100px_1fr" -> "repeat(auto-fit, minmax(100px, 1fr))"
        let parts: Vec<&str> = params.split('_').collect();
        if parts.len() == 2 {
            Some(format!("repeat(auto-fit, minmax({}, {}))", parts[0], parts[1]))
        } else {
            None
        }
    }

    fn parse_auto_fill_value(&self, params: &str) -> Option<String> {
        // Parse format like "200px_2fr" -> "repeat(auto-fill, minmax(200px, 2fr))"
        let parts: Vec<&str> = params.split('_').collect();
        if parts.len() == 2 {
            Some(format!("repeat(auto-fill, minmax({}, {}))", parts[0], parts[1]))
        } else {
            None
        }
    }

    fn parse_named_lines_value(&self, lines: &str) -> Option<String> {
        // Parse format like "header-main-sidebar" -> "[header] 1fr [main] 2fr [sidebar]"
        let parts: Vec<&str> = lines.split('-').collect();
        if parts.is_empty() {
            return None;
        }

        let mut result = String::new();
        for (i, part) in parts.iter().enumerate() {
            if i > 0 {
                result.push_str(" 1fr ");
            }
            result.push_str(&format!("[{}] 1fr", part));
        }
        Some(result)
    }

    fn parse_complex_pattern_value(&self, pattern: &str) -> Option<String> {
        // Parse complex patterns like "1fr_2fr_1fr" -> "1fr 2fr 1fr"
        let parts: Vec<&str> = pattern.split('_').collect();
        if parts.is_empty() {
            return None;
        }

        Some(parts.join(" "))
    }
}

impl UtilityParser for AdvancedGridParser {
    fn get_priority(&self) -> u32 {
        75 // Higher than basic grid parser
    }

    fn get_category(&self) -> ParserCategory {
        ParserCategory::Layout
    }

    fn get_supported_patterns(&self) -> Vec<&'static str> {
        vec![
            "grid-areas-*",
            "grid-masonry-*",
            "grid-subgrid-*",
            "grid-cols-auto-fit-*",
            "grid-cols-auto-fill-*",
            "grid-rows-named-*",
            "grid-cols-named-*",
            "grid-cols-complex-*",
            "grid-area-*",
            "grid-flow-*",
        ]
    }

    fn parse_class(&self, class: &str) -> Option<Vec<crate::css_generator::types::CssProperty>> {
        // Try each parsing method in order
        if let Some(properties) = self.parse_grid_areas_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_grid_masonry_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_grid_subgrid_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_grid_auto_fit_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_grid_auto_fill_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_grid_named_lines_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_grid_complex_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_grid_area_placement_class(class) {
            return Some(properties);
        }
        if let Some(properties) = self.parse_grid_flow_class(class) {
            return Some(properties);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_grid_areas() {
        let parser = AdvancedGridParser::new();

        let result = parser.parse_class("grid-areas-header-main-sidebar");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].name, "grid-template-areas");
        assert_eq!(properties[0].value, "\"header header\" \"main sidebar\"");

        let result = parser.parse_class("grid-areas-header-main-footer");
        assert!(result.is_some());
    }

    #[test]
    fn test_parse_grid_masonry() {
        let parser = AdvancedGridParser::new();

        let result = parser.parse_class("grid-masonry-dense");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].name, "grid-template-columns");
        assert_eq!(properties[0].value, "masonry dense");
    }

    #[test]
    fn test_parse_grid_subgrid() {
        let parser = AdvancedGridParser::new();

        let result = parser.parse_class("grid-subgrid-inherit");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties.len(), 2); // Both columns and rows
        assert_eq!(properties[0].name, "grid-template-columns");
        assert_eq!(properties[0].value, "subgrid");
    }

    #[test]
    fn test_parse_grid_auto_fit() {
        let parser = AdvancedGridParser::new();

        let result = parser.parse_class("grid-cols-auto-fit-200px_1fr");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].name, "grid-template-columns");
        assert_eq!(properties[0].value, "repeat(auto-fit, minmax(200px, 1fr))");
    }

    #[test]
    fn test_parse_grid_auto_fill() {
        let parser = AdvancedGridParser::new();

        let result = parser.parse_class("grid-cols-auto-fill-100px_2fr");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].name, "grid-template-columns");
        assert_eq!(properties[0].value, "repeat(auto-fill, minmax(100px, 2fr))");
    }

    #[test]
    fn test_parse_grid_named_lines() {
        let parser = AdvancedGridParser::new();

        let result = parser.parse_class("grid-cols-named-header-main-sidebar");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].name, "grid-template-columns");
        assert!(properties[0].value.contains("[header]"));
        assert!(properties[0].value.contains("[main]"));
        assert!(properties[0].value.contains("[sidebar]"));
    }

    #[test]
    fn test_parse_grid_complex() {
        let parser = AdvancedGridParser::new();

        let result = parser.parse_class("grid-cols-complex-1fr_2fr_1fr");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].name, "grid-template-columns");
        assert_eq!(properties[0].value, "1fr 2fr 1fr");
    }

    #[test]
    fn test_parse_grid_area_placement() {
        let parser = AdvancedGridParser::new();

        let result = parser.parse_class("grid-area-header");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].name, "grid-area");
        assert_eq!(properties[0].value, "header");
    }

    #[test]
    fn test_parse_grid_flow() {
        let parser = AdvancedGridParser::new();

        let result = parser.parse_class("grid-flow-row-dense");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].name, "grid-auto-flow");
        assert_eq!(properties[0].value, "row dense");

        let result = parser.parse_class("grid-flow-dense");
        assert!(result.is_some());
        let properties = result.unwrap();
        assert_eq!(properties[0].value, "dense");
    }

    #[test]
    fn test_parser_metadata() {
        let parser = AdvancedGridParser::new();

        assert_eq!(parser.get_priority(), 75);
        assert_eq!(parser.get_category(), ParserCategory::Layout);

        let patterns = parser.get_supported_patterns();
        assert!(patterns.contains(&"grid-areas-*"));
        assert!(patterns.contains(&"grid-masonry-*"));
        assert!(patterns.contains(&"grid-flow-*"));
    }

    #[test]
    fn test_invalid_classes() {
        let parser = AdvancedGridParser::new();

        assert!(parser.parse_class("invalid-class").is_none());
        assert!(parser.parse_class("grid-areas-invalid").is_none());
        assert!(parser.parse_class("grid-masonry-invalid").is_none());
    }
}
