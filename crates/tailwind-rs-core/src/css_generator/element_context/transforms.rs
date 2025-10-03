//! Transform Context Module
//!
//! Handles transform classes like translate-x-*, scale-*, rotate-*, skew-*

use crate::css_generator::types::CssProperty;

/// Context for managing transform state
#[derive(Debug, Clone, Default)]
pub struct TransformContext {
    pub translate_x: Option<String>,
    pub translate_y: Option<String>,
    pub scale_x: Option<String>,
    pub scale_y: Option<String>,
    pub rotate: Option<String>,
    pub skew_x: Option<String>,
    pub skew_y: Option<String>,
}

impl TransformContext {
    /// Update transform context from a transform class
    pub fn update_from_class(&mut self, class: &str) {
        if let Some(transform_value) = Self::parse_transform_class(class) {
            // Parse transform value and update appropriate field
            if class.starts_with("translate-x-") {
                self.translate_x = Some(transform_value);
            } else if class.starts_with("translate-y-") {
                self.translate_y = Some(transform_value);
            } else if class.starts_with("scale-x-") {
                self.scale_x = Some(transform_value);
            } else if class.starts_with("scale-y-") {
                self.scale_y = Some(transform_value);
            } else if class.starts_with("rotate-") || class.starts_with("-rotate-") {
                self.rotate = Some(transform_value);
            } else if class.starts_with("skew-x-") {
                self.skew_x = Some(transform_value);
            } else if class.starts_with("skew-y-") {
                self.skew_y = Some(transform_value);
            }
        }
    }

    /// Parse transform classes (translate-*, scale-*, rotate-*, skew-*)
    fn parse_transform_class(class: &str) -> Option<String> {
        // Handle translate classes
        if let Some(value) = class.strip_prefix("translate-x-") {
            return Self::parse_spacing_value(value).map(|v| format!("translateX({})", v));
        }
        if let Some(value) = class.strip_prefix("translate-y-") {
            return Self::parse_spacing_value(value).map(|v| format!("translateY({})", v));
        }

        // Handle scale classes
        if let Some(value) = class.strip_prefix("scale-x-") {
            return value.parse::<f32>().ok().map(|v| format!("scaleX({})", v));
        }
        if let Some(value) = class.strip_prefix("scale-y-") {
            return value.parse::<f32>().ok().map(|v| format!("scaleY({})", v));
        }
        if let Some(value) = class.strip_prefix("scale-") {
            return value.parse::<f32>().ok().map(|v| format!("scale({})", v));
        }

        // Handle rotate classes
        if let Some(value) = class.strip_prefix("rotate-") {
            if let Some(deg) = Self::parse_angle_value(value) {
                return Some(format!("rotate({}deg)", deg));
            }
        }
        if let Some(value) = class.strip_prefix("-rotate-") {
            if let Some(deg) = Self::parse_angle_value(value) {
                return Some(format!("rotate({}deg)", -deg));
            }
        }

        // Handle skew classes
        if let Some(value) = class.strip_prefix("skew-x-") {
            if let Some(deg) = Self::parse_angle_value(value) {
                return Some(format!("skewX({}deg)", deg));
            }
        }
        if let Some(value) = class.strip_prefix("skew-y-") {
            if let Some(deg) = Self::parse_angle_value(value) {
                return Some(format!("skewY({}deg)", deg));
            }
        }

        None
    }

    /// Parse spacing values for transforms
    fn parse_spacing_value(value: &str) -> Option<String> {
        match value {
            "0" | "0px" => Some("0px".to_string()),
            "1" => Some("0.25rem".to_string()),
            "2" => Some("0.5rem".to_string()),
            "3" => Some("0.75rem".to_string()),
            "4" => Some("1rem".to_string()),
            "5" => Some("1.25rem".to_string()),
            "6" => Some("1.5rem".to_string()),
            "8" => Some("2rem".to_string()),
            "10" => Some("2.5rem".to_string()),
            "12" => Some("3rem".to_string()),
            "16" => Some("4rem".to_string()),
            "20" => Some("5rem".to_string()),
            "24" => Some("6rem".to_string()),
            "32" => Some("8rem".to_string()),
            "40" => Some("10rem".to_string()),
            "48" => Some("12rem".to_string()),
            "56" => Some("14rem".to_string()),
            "64" => Some("16rem".to_string()),
            "72" => Some("18rem".to_string()),
            "80" => Some("20rem".to_string()),
            "96" => Some("24rem".to_string()),
            "px" => Some("1px".to_string()),
            "full" => Some("100%".to_string()),
            _ => None,
        }
    }

    /// Parse angle values for rotate/skew
    fn parse_angle_value(value: &str) -> Option<f32> {
        match value {
            "0" => Some(0.0),
            "1" => Some(1.0),
            "2" => Some(2.0),
            "3" => Some(3.0),
            "6" => Some(6.0),
            "12" => Some(12.0),
            "45" => Some(45.0),
            "90" => Some(90.0),
            "180" => Some(180.0),
            _ => value.parse::<f32>().ok(),
        }
    }

    /// Check if transform context has any transforms
    pub fn has_transform(&self) -> bool {
        self.translate_x.is_some() || self.translate_y.is_some() ||
        self.scale_x.is_some() || self.scale_y.is_some() ||
        self.rotate.is_some() || self.skew_x.is_some() || self.skew_y.is_some()
    }

    /// Generate CSS properties for transforms
    pub fn to_css_properties(&self) -> Vec<CssProperty> {
        if !self.has_transform() {
            return Vec::new();
        }

        let mut transforms = Vec::new();

        if let Some(tx) = &self.translate_x {
            transforms.push(tx.clone());
        }
        if let Some(ty) = &self.translate_y {
            transforms.push(ty.clone());
        }
        if let Some(sx) = &self.scale_x {
            transforms.push(sx.clone());
        }
        if let Some(sy) = &self.scale_y {
            transforms.push(sy.clone());
        }
        if let Some(r) = &self.rotate {
            transforms.push(r.clone());
        }
        if let Some(sx) = &self.skew_x {
            transforms.push(sx.clone());
        }
        if let Some(sy) = &self.skew_y {
            transforms.push(sy.clone());
        }

        if transforms.is_empty() {
            return Vec::new();
        }

        vec![CssProperty {
            name: "transform".to_string(),
            value: transforms.join(" "),
            important: false,
        }]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translate_class_parsing() {
        let mut context = TransformContext::default();

        // Test translate-x
        context.update_from_class("translate-x-4");
        assert_eq!(context.translate_x, Some("translateX(1rem)".to_string()));

        // Test translate-y
        context = TransformContext::default();
        context.update_from_class("translate-y-2");
        assert_eq!(context.translate_y, Some("translateY(0.5rem)".to_string()));
    }

    #[test]
    fn scale_class_parsing() {
        let mut context = TransformContext::default();

        // Test scale-x
        context.update_from_class("scale-x-50");
        assert_eq!(context.scale_x, Some("scaleX(50)".to_string()));

        // Test scale uniform
        context = TransformContext::default();
        context.update_from_class("scale-75");
        assert_eq!(context.scale_x, Some("scale(75)".to_string()));
    }

    #[test]
    fn rotate_class_parsing() {
        let mut context = TransformContext::default();

        // Test positive rotation
        context.update_from_class("rotate-45");
        assert_eq!(context.rotate, Some("rotate(45deg)".to_string()));

        // Test negative rotation
        context = TransformContext::default();
        context.update_from_class("-rotate-90");
        assert_eq!(context.rotate, Some("rotate(-90deg)".to_string()));
    }

    #[test]
    fn skew_class_parsing() {
        let mut context = TransformContext::default();

        // Test skew-x
        context.update_from_class("skew-x-12");
        assert_eq!(context.skew_x, Some("skewX(12deg)".to_string()));

        // Test skew-y
        context = TransformContext::default();
        context.update_from_class("skew-y-6");
        assert_eq!(context.skew_y, Some("skewY(6deg)".to_string()));
    }

    #[test]
    fn transform_css_generation() {
        let mut context = TransformContext::default();
        context.update_from_class("translate-x-4");
        context.update_from_class("rotate-45");
        context.update_from_class("scale-75");

        let properties = context.to_css_properties();
        assert_eq!(properties.len(), 1);

        let transform_property = &properties[0];
        assert_eq!(transform_property.name, "transform");
        assert!(transform_property.value.contains("translateX(1rem)"));
        assert!(transform_property.value.contains("rotate(45deg)"));
        assert!(transform_property.value.contains("scale(75)"));
    }

    #[test]
    fn empty_transform_generation() {
        let context = TransformContext::default();
        let properties = context.to_css_properties();
        assert_eq!(properties.len(), 0);
    }
}
