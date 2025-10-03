//! Filter Context Module
//!
//! Handles CSS filter classes like blur-sm, brightness-50, contrast-75, grayscale, hue-rotate-90, etc.

use crate::css_generator::types::CssProperty;

/// Context for managing filter state within an element
#[derive(Debug, Clone, Default)]
pub struct FilterContext {
    /// Blur filter value
    pub blur: Option<String>,
    /// Brightness filter value
    pub brightness: Option<String>,
    /// Contrast filter value
    pub contrast: Option<String>,
    /// Grayscale filter value
    pub grayscale: Option<String>,
    /// Hue-rotate filter value
    pub hue_rotate: Option<String>,
    /// Invert filter value
    pub invert: Option<String>,
    /// Saturate filter value
    pub saturate: Option<String>,
    /// Sepia filter value
    pub sepia: Option<String>,
    /// Drop-shadow filter value
    pub drop_shadow: Option<String>,
}

impl FilterContext {
    /// Update context from a filter class
    pub fn update_from_class(&mut self, class: &str) {
        if let Some(blur) = Self::parse_blur_class(class) {
            self.blur = Some(blur);
        } else if let Some(brightness) = Self::parse_brightness_class(class) {
            self.brightness = Some(brightness);
        } else if let Some(contrast) = Self::parse_contrast_class(class) {
            self.contrast = Some(contrast);
        } else if let Some(grayscale) = Self::parse_grayscale_class(class) {
            self.grayscale = Some(grayscale);
        } else if let Some(hue_rotate) = Self::parse_hue_rotate_class(class) {
            self.hue_rotate = Some(hue_rotate);
        } else if let Some(invert) = Self::parse_invert_class(class) {
            self.invert = Some(invert);
        } else if let Some(saturate) = Self::parse_saturate_class(class) {
            self.saturate = Some(saturate);
        } else if let Some(sepia) = Self::parse_sepia_class(class) {
            self.sepia = Some(sepia);
        } else if let Some(drop_shadow) = Self::parse_drop_shadow_class(class) {
            self.drop_shadow = Some(drop_shadow);
        }
    }

    /// Parse blur filter classes
    fn parse_blur_class(class: &str) -> Option<String> {
        match class {
            "blur-none" => Some("blur(0)".to_string()),
            "blur-sm" => Some("blur(4px)".to_string()),
            "blur" => Some("blur(8px)".to_string()),
            "blur-md" => Some("blur(12px)".to_string()),
            "blur-lg" => Some("blur(16px)".to_string()),
            "blur-xl" => Some("blur(24px)".to_string()),
            "blur-2xl" => Some("blur(40px)".to_string()),
            "blur-3xl" => Some("blur(64px)".to_string()),
            _ => None,
        }
    }

    /// Parse brightness filter classes
    fn parse_brightness_class(class: &str) -> Option<String> {
        match class {
            "brightness-0" => Some("brightness(0)".to_string()),
            "brightness-50" => Some("brightness(0.5)".to_string()),
            "brightness-75" => Some("brightness(0.75)".to_string()),
            "brightness-90" => Some("brightness(0.9)".to_string()),
            "brightness-95" => Some("brightness(0.95)".to_string()),
            "brightness-100" => Some("brightness(1)".to_string()),
            "brightness-105" => Some("brightness(1.05)".to_string()),
            "brightness-110" => Some("brightness(1.1)".to_string()),
            "brightness-125" => Some("brightness(1.25)".to_string()),
            "brightness-150" => Some("brightness(1.5)".to_string()),
            "brightness-200" => Some("brightness(2)".to_string()),
            _ => None,
        }
    }

    /// Parse contrast filter classes
    fn parse_contrast_class(class: &str) -> Option<String> {
        match class {
            "contrast-0" => Some("contrast(0)".to_string()),
            "contrast-50" => Some("contrast(0.5)".to_string()),
            "contrast-75" => Some("contrast(0.75)".to_string()),
            "contrast-100" => Some("contrast(1)".to_string()),
            "contrast-125" => Some("contrast(1.25)".to_string()),
            "contrast-150" => Some("contrast(1.5)".to_string()),
            "contrast-200" => Some("contrast(2)".to_string()),
            _ => None,
        }
    }

    /// Parse grayscale filter classes
    fn parse_grayscale_class(class: &str) -> Option<String> {
        match class {
            "grayscale-0" => Some("grayscale(0)".to_string()),
            "grayscale" => Some("grayscale(1)".to_string()),
            _ => None,
        }
    }

    /// Parse hue-rotate filter classes
    fn parse_hue_rotate_class(class: &str) -> Option<String> {
        match class {
            "hue-rotate-0" => Some("hue-rotate(0deg)".to_string()),
            "hue-rotate-15" => Some("hue-rotate(15deg)".to_string()),
            "hue-rotate-30" => Some("hue-rotate(30deg)".to_string()),
            "hue-rotate-60" => Some("hue-rotate(60deg)".to_string()),
            "hue-rotate-90" => Some("hue-rotate(90deg)".to_string()),
            "hue-rotate-180" => Some("hue-rotate(180deg)".to_string()),
            _ => None,
        }
    }

    /// Parse invert filter classes
    fn parse_invert_class(class: &str) -> Option<String> {
        match class {
            "invert-0" => Some("invert(0)".to_string()),
            "invert" => Some("invert(1)".to_string()),
            _ => None,
        }
    }

    /// Parse saturate filter classes
    fn parse_saturate_class(class: &str) -> Option<String> {
        match class {
            "saturate-0" => Some("saturate(0)".to_string()),
            "saturate-50" => Some("saturate(0.5)".to_string()),
            "saturate-100" => Some("saturate(1)".to_string()),
            "saturate-150" => Some("saturate(1.5)".to_string()),
            "saturate-200" => Some("saturate(2)".to_string()),
            _ => None,
        }
    }

    /// Parse sepia filter classes
    fn parse_sepia_class(class: &str) -> Option<String> {
        match class {
            "sepia-0" => Some("sepia(0)".to_string()),
            "sepia" => Some("sepia(1)".to_string()),
            _ => None,
        }
    }

    /// Parse drop-shadow filter classes
    fn parse_drop_shadow_class(class: &str) -> Option<String> {
        match class {
            "drop-shadow-sm" => Some("drop-shadow(0 1px 1px rgb(0 0 0 / 0.05))".to_string()),
            "drop-shadow" => Some("drop-shadow(0 1px 2px rgb(0 0 0 / 0.1)) drop-shadow(0 1px 1px rgb(0 0 0 / 0.06))".to_string()),
            "drop-shadow-md" => Some("drop-shadow(0 4px 3px rgb(0 0 0 / 0.07)) drop-shadow(0 2px 2px rgb(0 0 0 / 0.06))".to_string()),
            "drop-shadow-lg" => Some("drop-shadow(0 10px 8px rgb(0 0 0 / 0.04)) drop-shadow(0 4px 3px rgb(0 0 0 / 0.1))".to_string()),
            "drop-shadow-xl" => Some("drop-shadow(0 20px 13px rgb(0 0 0 / 0.03)) drop-shadow(0 8px 5px rgb(0 0 0 / 0.08))".to_string()),
            "drop-shadow-2xl" => Some("drop-shadow(0 25px 25px rgb(0 0 0 / 0.15))".to_string()),
            "drop-shadow-none" => Some("drop-shadow(0 0 #0000)".to_string()),
            _ => None,
        }
    }

    /// Generate CSS properties from current filter state
    pub fn to_css_properties(&self) -> Vec<CssProperty> {
        let mut filters = Vec::new();

        if let Some(ref blur) = self.blur {
            filters.push(blur.clone());
        }
        if let Some(ref brightness) = self.brightness {
            filters.push(brightness.clone());
        }
        if let Some(ref contrast) = self.contrast {
            filters.push(contrast.clone());
        }
        if let Some(ref grayscale) = self.grayscale {
            filters.push(grayscale.clone());
        }
        if let Some(ref hue_rotate) = self.hue_rotate {
            filters.push(hue_rotate.clone());
        }
        if let Some(ref invert) = self.invert {
            filters.push(invert.clone());
        }
        if let Some(ref saturate) = self.saturate {
            filters.push(saturate.clone());
        }
        if let Some(ref sepia) = self.sepia {
            filters.push(sepia.clone());
        }
        if let Some(ref drop_shadow) = self.drop_shadow {
            filters.push(drop_shadow.clone());
        }

        if filters.is_empty() {
            Vec::new()
        } else {
            vec![CssProperty {
                name: "filter".to_string(),
                value: filters.join(" "),
                important: false,
            }]
        }
    }

    /// Check if this context has any filters
    pub fn has_filters(&self) -> bool {
        self.blur.is_some() || self.brightness.is_some() || self.contrast.is_some() ||
        self.grayscale.is_some() || self.hue_rotate.is_some() || self.invert.is_some() ||
        self.saturate.is_some() || self.sepia.is_some() || self.drop_shadow.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blur_filter_parsing() {
        let mut context = FilterContext::default();

        context.update_from_class("blur-sm");
        assert_eq!(context.blur, Some("blur(4px)".to_string()));

        context = FilterContext::default();
        context.update_from_class("blur-lg");
        assert_eq!(context.blur, Some("blur(16px)".to_string()));

        context = FilterContext::default();
        context.update_from_class("blur-none");
        assert_eq!(context.blur, Some("blur(0)".to_string()));
    }

    #[test]
    fn brightness_filter_parsing() {
        let mut context = FilterContext::default();

        context.update_from_class("brightness-50");
        assert_eq!(context.brightness, Some("brightness(0.5)".to_string()));

        context = FilterContext::default();
        context.update_from_class("brightness-200");
        assert_eq!(context.brightness, Some("brightness(2)".to_string()));
    }

    #[test]
    fn contrast_filter_parsing() {
        let mut context = FilterContext::default();

        context.update_from_class("contrast-75");
        assert_eq!(context.contrast, Some("contrast(0.75)".to_string()));

        context = FilterContext::default();
        context.update_from_class("contrast-200");
        assert_eq!(context.contrast, Some("contrast(2)".to_string()));
    }

    #[test]
    fn grayscale_filter_parsing() {
        let mut context = FilterContext::default();

        context.update_from_class("grayscale");
        assert_eq!(context.grayscale, Some("grayscale(1)".to_string()));

        context = FilterContext::default();
        context.update_from_class("grayscale-0");
        assert_eq!(context.grayscale, Some("grayscale(0)".to_string()));
    }

    #[test]
    fn hue_rotate_filter_parsing() {
        let mut context = FilterContext::default();

        context.update_from_class("hue-rotate-90");
        assert_eq!(context.hue_rotate, Some("hue-rotate(90deg)".to_string()));

        context = FilterContext::default();
        context.update_from_class("hue-rotate-180");
        assert_eq!(context.hue_rotate, Some("hue-rotate(180deg)".to_string()));
    }

    #[test]
    fn invert_filter_parsing() {
        let mut context = FilterContext::default();

        context.update_from_class("invert");
        assert_eq!(context.invert, Some("invert(1)".to_string()));

        context = FilterContext::default();
        context.update_from_class("invert-0");
        assert_eq!(context.invert, Some("invert(0)".to_string()));
    }

    #[test]
    fn saturate_filter_parsing() {
        let mut context = FilterContext::default();

        context.update_from_class("saturate-150");
        assert_eq!(context.saturate, Some("saturate(1.5)".to_string()));

        context = FilterContext::default();
        context.update_from_class("saturate-200");
        assert_eq!(context.saturate, Some("saturate(2)".to_string()));
    }

    #[test]
    fn sepia_filter_parsing() {
        let mut context = FilterContext::default();

        context.update_from_class("sepia");
        assert_eq!(context.sepia, Some("sepia(1)".to_string()));

        context = FilterContext::default();
        context.update_from_class("sepia-0");
        assert_eq!(context.sepia, Some("sepia(0)".to_string()));
    }

    #[test]
    fn drop_shadow_filter_parsing() {
        let mut context = FilterContext::default();

        context.update_from_class("drop-shadow-sm");
        assert_eq!(context.drop_shadow, Some("drop-shadow(0 1px 1px rgb(0 0 0 / 0.05))".to_string()));

        context = FilterContext::default();
        context.update_from_class("drop-shadow-lg");
        assert!(context.drop_shadow.as_ref().unwrap().contains("drop-shadow(0 10px 8px"));
    }

    #[test]
    fn filter_css_generation() {
        let mut context = FilterContext::default();
        context.update_from_class("blur-sm");
        context.update_from_class("brightness-50");
        context.update_from_class("contrast-125");

        let properties = context.to_css_properties();
        assert_eq!(properties.len(), 1);

        let filter_property = &properties[0];
        assert_eq!(filter_property.name, "filter");
        assert!(filter_property.value.contains("blur(4px)"));
        assert!(filter_property.value.contains("brightness(0.5)"));
        assert!(filter_property.value.contains("contrast(1.25)"));
    }

    #[test]
    fn empty_filter_generation() {
        let context = FilterContext::default();
        let properties = context.to_css_properties();
        assert_eq!(properties.len(), 0);
        assert!(!context.has_filters());
    }
}
