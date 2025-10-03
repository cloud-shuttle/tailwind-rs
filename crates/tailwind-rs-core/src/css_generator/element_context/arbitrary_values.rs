//! Arbitrary Values Context Module
//!
//! Handles arbitrary values using bracket notation like w-[100px], bg-[#ff0000]

use crate::css_generator::types::CssProperty;

/// Context for managing arbitrary values within an element
#[derive(Debug, Clone, Default)]
pub struct ArbitraryValueContext {
    /// Arbitrary width values
    pub width: Option<String>,
    /// Arbitrary height values
    pub height: Option<String>,
    /// Arbitrary padding values
    pub padding: Option<String>,
    /// Arbitrary margin values
    pub margin: Option<String>,
    /// Arbitrary background color values
    pub background_color: Option<String>,
    /// Arbitrary text color values
    pub color: Option<String>,
    /// Arbitrary border color values
    pub border_color: Option<String>,
    /// Arbitrary border width values
    pub border_width: Option<String>,
    /// Arbitrary border radius values
    pub border_radius: Option<String>,
    /// Arbitrary font size values
    pub font_size: Option<String>,
    /// Arbitrary line height values
    pub line_height: Option<String>,
    /// Arbitrary letter spacing values
    pub letter_spacing: Option<String>,
    /// Arbitrary font weight values
    pub font_weight: Option<String>,
    /// Arbitrary opacity values
    pub opacity: Option<String>,
    /// Arbitrary z-index values
    pub z_index: Option<String>,
    /// Arbitrary top/left/bottom/right values
    pub inset: Option<String>,
    /// Arbitrary gap values
    pub gap: Option<String>,
    /// Arbitrary grid template columns
    pub grid_template_columns: Option<String>,
    /// Arbitrary grid template rows
    pub grid_template_rows: Option<String>,
    /// Arbitrary flex values
    pub flex: Option<String>,
    /// Arbitrary position values
    pub position: Option<String>,
    /// Arbitrary display values
    pub display: Option<String>,
    /// Arbitrary overflow values
    pub overflow: Option<String>,
    /// Arbitrary cursor values
    pub cursor: Option<String>,
    /// Arbitrary transition values
    pub transition: Option<String>,
    /// Arbitrary transform values
    pub transform: Option<String>,
    /// Arbitrary animation values
    pub animation: Option<String>,
    /// Arbitrary filter values
    pub filter: Option<String>,
    /// Arbitrary backdrop filter values
    pub backdrop_filter: Option<String>,
}

impl ArbitraryValueContext {
    /// Update context from an arbitrary value class
    pub fn update_from_class(&mut self, class: &str) {
        if let Some(arbitrary) = Self::parse_arbitrary_value(class) {
            // Route arbitrary values to appropriate properties based on prefix
            if class.starts_with("w-[") {
                self.width = Some(arbitrary);
            } else if class.starts_with("h-[") {
                self.height = Some(arbitrary);
            } else if class.starts_with("p-[") {
                self.padding = Some(arbitrary);
            } else if class.starts_with("m-[") {
                self.margin = Some(arbitrary);
            } else if class.starts_with("bg-[") {
                if arbitrary.starts_with("#") || arbitrary.starts_with("rgb") || arbitrary.starts_with("hsl") {
                    self.background_color = Some(arbitrary);
                } else {
                    // Handle other background arbitrary values
                    self.background_color = Some(arbitrary);
                }
            } else if class.starts_with("text-[") {
                if arbitrary.starts_with("#") || arbitrary.starts_with("rgb") || arbitrary.starts_with("hsl") {
                    self.color = Some(arbitrary);
                } else {
                    // Handle font-size arbitrary values
                    self.font_size = Some(arbitrary);
                }
            } else if class.starts_with("border-[") {
                if arbitrary.starts_with("#") || arbitrary.starts_with("rgb") || arbitrary.starts_with("hsl") {
                    self.border_color = Some(arbitrary);
                } else {
                    self.border_width = Some(arbitrary);
                }
            } else if class.starts_with("rounded-[") {
                self.border_radius = Some(arbitrary);
            } else if class.starts_with("text-[") && !arbitrary.starts_with("#") && !arbitrary.starts_with("rgb") && !arbitrary.starts_with("hsl") {
                self.font_size = Some(arbitrary);
            } else if class.starts_with("leading-[") {
                self.line_height = Some(arbitrary);
            } else if class.starts_with("tracking-[") {
                self.letter_spacing = Some(arbitrary);
            } else if class.starts_with("font-[") {
                self.font_weight = Some(arbitrary);
            } else if class.starts_with("opacity-[") {
                self.opacity = Some(arbitrary);
            } else if class.starts_with("z-[") {
                self.z_index = Some(arbitrary);
            } else if class.starts_with("inset-[") || class.starts_with("top-[") || class.starts_with("left-[") || class.starts_with("bottom-[") || class.starts_with("right-[") {
                self.inset = Some(arbitrary);
            } else if class.starts_with("gap-[") {
                self.gap = Some(arbitrary);
            } else if class.starts_with("grid-cols-[") {
                self.grid_template_columns = Some(arbitrary);
            } else if class.starts_with("grid-rows-[") {
                self.grid_template_rows = Some(arbitrary);
            } else if class.starts_with("flex-[") {
                self.flex = Some(arbitrary);
            } else if class.starts_with("cursor-[") {
                self.cursor = Some(arbitrary);
            } else if class.starts_with("transition-[") {
                self.transition = Some(arbitrary);
            } else if class.starts_with("transform-[") {
                self.transform = Some(arbitrary);
            } else if class.starts_with("animate-[") {
                self.animation = Some(arbitrary);
            } else if class.starts_with("filter-[") {
                self.filter = Some(arbitrary);
            } else if class.starts_with("backdrop-[") {
                self.backdrop_filter = Some(arbitrary);
            }
        }
    }

    /// Parse arbitrary values from classes like "w-[100px]" -> "100px"
    pub fn parse_arbitrary_value(class: &str) -> Option<String> {
        if let Some(start) = class.find('[') {
            if let Some(end) = class.rfind(']') {
                if end > start {
                    let arbitrary = &class[start + 1..end];
                    if !arbitrary.is_empty() {
                        return Some(arbitrary.to_string());
                    }
                }
            }
        }
        None
    }

    /// Generate CSS properties from current arbitrary values
    pub fn to_css_properties(&self) -> Vec<CssProperty> {
        let mut properties = Vec::new();

        if let Some(ref width) = self.width {
            properties.push(CssProperty {
                name: "width".to_string(),
                value: width.clone(),
                important: false,
            });
        }

        if let Some(ref height) = self.height {
            properties.push(CssProperty {
                name: "height".to_string(),
                value: height.clone(),
                important: false,
            });
        }

        if let Some(ref padding) = self.padding {
            properties.push(CssProperty {
                name: "padding".to_string(),
                value: padding.clone(),
                important: false,
            });
        }

        if let Some(ref margin) = self.margin {
            properties.push(CssProperty {
                name: "margin".to_string(),
                value: margin.clone(),
                important: false,
            });
        }

        if let Some(ref background_color) = self.background_color {
            properties.push(CssProperty {
                name: "background-color".to_string(),
                value: background_color.clone(),
                important: false,
            });
        }

        if let Some(ref color) = self.color {
            properties.push(CssProperty {
                name: "color".to_string(),
                value: color.clone(),
                important: false,
            });
        }

        if let Some(ref border_color) = self.border_color {
            properties.push(CssProperty {
                name: "border-color".to_string(),
                value: border_color.clone(),
                important: false,
            });
        }

        if let Some(ref border_width) = self.border_width {
            properties.push(CssProperty {
                name: "border-width".to_string(),
                value: border_width.clone(),
                important: false,
            });
        }

        if let Some(ref border_radius) = self.border_radius {
            properties.push(CssProperty {
                name: "border-radius".to_string(),
                value: border_radius.clone(),
                important: false,
            });
        }

        if let Some(ref font_size) = self.font_size {
            properties.push(CssProperty {
                name: "font-size".to_string(),
                value: font_size.clone(),
                important: false,
            });
        }

        if let Some(ref line_height) = self.line_height {
            properties.push(CssProperty {
                name: "line-height".to_string(),
                value: line_height.clone(),
                important: false,
            });
        }

        if let Some(ref letter_spacing) = self.letter_spacing {
            properties.push(CssProperty {
                name: "letter-spacing".to_string(),
                value: letter_spacing.clone(),
                important: false,
            });
        }

        if let Some(ref font_weight) = self.font_weight {
            properties.push(CssProperty {
                name: "font-weight".to_string(),
                value: font_weight.clone(),
                important: false,
            });
        }

        if let Some(ref opacity) = self.opacity {
            properties.push(CssProperty {
                name: "opacity".to_string(),
                value: opacity.clone(),
                important: false,
            });
        }

        if let Some(ref z_index) = self.z_index {
            properties.push(CssProperty {
                name: "z-index".to_string(),
                value: z_index.clone(),
                important: false,
            });
        }

        if let Some(ref inset) = self.inset {
            properties.push(CssProperty {
                name: "inset".to_string(),
                value: inset.clone(),
                important: false,
            });
        }

        if let Some(ref gap) = self.gap {
            properties.push(CssProperty {
                name: "gap".to_string(),
                value: gap.clone(),
                important: false,
            });
        }

        if let Some(ref grid_template_columns) = self.grid_template_columns {
            properties.push(CssProperty {
                name: "grid-template-columns".to_string(),
                value: grid_template_columns.clone(),
                important: false,
            });
        }

        if let Some(ref grid_template_rows) = self.grid_template_rows {
            properties.push(CssProperty {
                name: "grid-template-rows".to_string(),
                value: grid_template_rows.clone(),
                important: false,
            });
        }

        if let Some(ref flex) = self.flex {
            properties.push(CssProperty {
                name: "flex".to_string(),
                value: flex.clone(),
                important: false,
            });
        }

        if let Some(ref cursor) = self.cursor {
            properties.push(CssProperty {
                name: "cursor".to_string(),
                value: cursor.clone(),
                important: false,
            });
        }

        if let Some(ref transition) = self.transition {
            properties.push(CssProperty {
                name: "transition".to_string(),
                value: transition.clone(),
                important: false,
            });
        }

        if let Some(ref transform) = self.transform {
            properties.push(CssProperty {
                name: "transform".to_string(),
                value: transform.clone(),
                important: false,
            });
        }

        if let Some(ref animation) = self.animation {
            properties.push(CssProperty {
                name: "animation".to_string(),
                value: animation.clone(),
                important: false,
            });
        }

        if let Some(ref filter) = self.filter {
            properties.push(CssProperty {
                name: "filter".to_string(),
                value: filter.clone(),
                important: false,
            });
        }

        if let Some(ref backdrop_filter) = self.backdrop_filter {
            properties.push(CssProperty {
                name: "backdrop-filter".to_string(),
                value: backdrop_filter.clone(),
                important: false,
            });
        }

        properties
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arbitrary_value_parsing() {
        let mut context = ArbitraryValueContext::default();

        // Test various arbitrary value classes
        context.update_from_class("w-[100px]");
        context.update_from_class("h-[50vh]");
        context.update_from_class("bg-[#ff0000]");
        context.update_from_class("text-[#00ff00]");
        context.update_from_class("text-[24px]");
        context.update_from_class("rounded-[12px]");
        context.update_from_class("p-[20px]");
        context.update_from_class("m-[16px]");
        context.update_from_class("border-[#000]");
        context.update_from_class("grid-cols-[repeat(3,1fr)]");
        context.update_from_class("transform-[rotate(45deg)_scale(1.2)]");

        // Generate CSS
        let properties = context.to_css_properties();

        // Should have multiple arbitrary value properties
        assert!(properties.len() > 5);

        // Test parsing individual arbitrary values
        assert_eq!(ArbitraryValueContext::parse_arbitrary_value("w-[100px]"), Some("100px".to_string()));
        assert_eq!(ArbitraryValueContext::parse_arbitrary_value("bg-[#ff0000]"), Some("#ff0000".to_string()));
        assert_eq!(ArbitraryValueContext::parse_arbitrary_value("text-[24px]"), Some("24px".to_string()));
        assert_eq!(ArbitraryValueContext::parse_arbitrary_value("grid-cols-[repeat(3,1fr)]"), Some("repeat(3,1fr)".to_string()));
        assert_eq!(ArbitraryValueContext::parse_arbitrary_value("transform-[rotate(45deg)_scale(1.2)]"), Some("rotate(45deg)_scale(1.2)".to_string()));

        // Test invalid arbitrary values
        assert_eq!(ArbitraryValueContext::parse_arbitrary_value("w-100px"), None);
        assert_eq!(ArbitraryValueContext::parse_arbitrary_value("bg-[#ff0000"), None); // missing closing bracket
        assert_eq!(ArbitraryValueContext::parse_arbitrary_value("w-[]"), None); // empty brackets
    }
}
