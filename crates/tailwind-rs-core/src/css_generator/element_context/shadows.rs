//! Shadow Context Module
//!
//! Handles box shadow classes like shadow-sm, shadow-lg, shadow-xl, shadow-inner

use crate::css_generator::types::CssProperty;

/// Context for managing box shadow state
#[derive(Debug, Clone, Default)]
pub struct ShadowContext {
    pub color: Option<String>,
    pub offset_x: Option<String>,
    pub offset_y: Option<String>,
    pub blur: Option<String>,
    pub spread: Option<String>,
    pub inset: bool,
}

impl ShadowContext {
    /// Update shadow context from a shadow class
    pub fn update_from_class(&mut self, class: &str) {
        let (shadow_value, is_inset) = Self::parse_shadow_class(class);
        if let Some(shadow_value) = shadow_value {
            self.inset = is_inset;

            // Parse shadow value like "0 1px 3px 0 rgb(0 0 0 / 0.1)"
            let parts: Vec<&str> = shadow_value.split_whitespace().collect();
            if parts.len() >= 4 {
                // Standard shadow format: offset-x offset-y blur spread color
                self.offset_x = Some(parts[0].to_string());
                self.offset_y = Some(parts[1].to_string());
                self.blur = Some(parts[2].to_string());
                if parts.len() > 4 {
                    self.spread = Some(parts[3].to_string());
                    // Color would be the rest, but for now we'll use default
                }
            }
        }
    }

    /// Parse shadow classes (shadow-*, drop-shadow-*)
    fn parse_shadow_class(class: &str) -> (Option<String>, bool) {
        match class {
            "shadow-sm" => (Some("0 1px 2px 0 rgb(0 0 0 / 0.05)".to_string()), false),
            "shadow" => (Some("0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)".to_string()), false),
            "shadow-md" => (Some("0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)".to_string()), false),
            "shadow-lg" => (Some("0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)".to_string()), false),
            "shadow-xl" => (Some("0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)".to_string()), false),
            "shadow-2xl" => (Some("0 25px 50px -12px rgb(0 0 0 / 0.25)".to_string()), false),
            "shadow-inner" => (Some("0 2px 4px 0 rgb(0 0 0 / 0.05)".to_string()), true),
            "shadow-none" => (Some("0 0 #0000".to_string()), false),
            _ => (None, false),
        }
    }

    /// Check if shadow context has any shadow
    pub fn has_shadow(&self) -> bool {
        self.offset_x.is_some() || self.offset_y.is_some() || self.blur.is_some() || self.color.is_some()
    }

    /// Generate CSS properties for shadows
    pub fn to_css_properties(&self) -> Vec<CssProperty> {
        if !self.has_shadow() {
            return Vec::new();
        }

        let mut shadow_value = String::new();

        if self.inset {
            shadow_value.push_str("inset ");
        }

        if let Some(x) = &self.offset_x {
            shadow_value.push_str(x);
            shadow_value.push(' ');
        } else {
            shadow_value.push_str("0 ");
        }

        if let Some(y) = &self.offset_y {
            shadow_value.push_str(y);
            shadow_value.push(' ');
        } else {
            shadow_value.push_str("0 ");
        }

        if let Some(blur) = &self.blur {
            shadow_value.push_str(blur);
            shadow_value.push(' ');
        }

        if let Some(spread) = &self.spread {
            shadow_value.push_str(spread);
            shadow_value.push(' ');
        }

        // Default color if none specified
        if self.color.is_none() {
            shadow_value.push_str("rgb(0 0 0 / 0.1)");
        } else {
            shadow_value.push_str(&self.color.as_ref().unwrap());
        }

        vec![CssProperty {
            name: "box-shadow".to_string(),
            value: shadow_value,
            important: false,
        }]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shadow_class_parsing() {
        let mut context = ShadowContext::default();

        // Test shadow-sm
        context.update_from_class("shadow-sm");
        assert_eq!(context.offset_x, Some("0".to_string()));
        assert_eq!(context.offset_y, Some("1px".to_string()));
        assert_eq!(context.blur, Some("2px".to_string()));
        assert_eq!(context.inset, false);

        // Test shadow-inner
        context = ShadowContext::default();
        context.update_from_class("shadow-inner");
        assert_eq!(context.inset, true);

        // Test shadow-lg
        context = ShadowContext::default();
        context.update_from_class("shadow-lg");
        assert_eq!(context.offset_x, Some("0".to_string()));
        assert_eq!(context.offset_y, Some("10px".to_string()));
        assert_eq!(context.blur, Some("15px".to_string()));
        assert_eq!(context.inset, false);
    }

    #[test]
    fn shadow_css_generation() {
        let mut context = ShadowContext::default();
        context.update_from_class("shadow-lg");

        let properties = context.to_css_properties();
        assert_eq!(properties.len(), 1);

        let shadow_property = &properties[0];
        assert_eq!(shadow_property.name, "box-shadow");
        assert!(shadow_property.value.contains("0 10px 15px"));
        assert!(shadow_property.value.contains("rgb(0 0 0 / 0.1)"));
    }

    #[test]
    fn shadow_inner_css_generation() {
        let mut context = ShadowContext::default();
        context.update_from_class("shadow-inner");

        let properties = context.to_css_properties();
        assert_eq!(properties.len(), 1);

        let shadow_property = &properties[0];
        assert_eq!(shadow_property.name, "box-shadow");
        assert!(shadow_property.value.starts_with("inset"));
    }

    #[test]
    fn shadow_none() {
        let mut context = ShadowContext::default();
        context.update_from_class("shadow-none");

        let properties = context.to_css_properties();
        assert_eq!(properties.len(), 1);

        let shadow_property = &properties[0];
        assert_eq!(shadow_property.name, "box-shadow");
        assert!(shadow_property.value.contains("#0000"));
    }
}
