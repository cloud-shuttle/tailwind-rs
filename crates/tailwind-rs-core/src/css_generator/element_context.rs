//! Element-Based Context System
//!
//! This module implements the new stateful, element-based parsing architecture
//! that matches how real Tailwind CSS works with CSS variables and complex fallbacks.
//!
//! The architecture is organized into modular contexts:
//! - CustomPropertyContext: CSS custom properties ([--my-var:red])
//! - ArbitraryValueContext: Arbitrary values (w-[100px], bg-[#ff0000])
//! - GradientContext: Gradient classes (bg-gradient-to-r, from-pink-400)
//! - ShadowContext: Box shadow classes (shadow-lg, shadow-inner)
//! - TransformContext: Transform classes (translate-x-4, rotate-45, scale-75)
//! - VariantContext: Variant classes (hover:, md:, dark:, group-hover:)
//! - FilterContext: Filter classes (blur-sm, brightness-50, grayscale)
//! - AnimationContext: Animation classes (animate-spin, duration-300, ease-in-out)

use super::types::{CssProperty, CssRule};

// Re-export all contexts for external use
pub mod custom_properties;
pub mod arbitrary_values;
pub mod gradients;
pub mod shadows;
pub mod transforms;
pub mod variants;
pub mod filters;
pub mod animations;

pub use custom_properties::CustomPropertyContext;
pub use arbitrary_values::ArbitraryValueContext;
pub use gradients::{GradientContext, GradientStopType};
pub use shadows::ShadowContext;
pub use transforms::TransformContext;
pub use variants::VariantContext;
pub use filters::FilterContext;
pub use animations::AnimationContext;

/// Complete context for an element's stateful utilities
#[derive(Debug, Clone, Default)]
pub struct ElementContext {
    pub gradients: GradientContext,
    pub shadows: ShadowContext,
    pub transforms: TransformContext,
    pub variants: VariantContext,
    pub filters: FilterContext,
    pub animations: AnimationContext,
    pub arbitrary_values: ArbitraryValueContext,
    pub custom_properties: CustomPropertyContext,
    // Add more contexts as needed (opacity, etc.)
}

impl ElementContext {
    /// Update context from a class (handles variant-prefixed classes)
    pub fn update_from_class(&mut self, class: &str) {
        let (variants, base_class) = Self::parse_variants_from_class(class);

        // Update variant context
        for variant in variants {
            self.variants.update_from_class(&variant);
        }

        // Update utility contexts based on base class
        self.update_utility_context(&base_class);
    }

    /// Parse variants from a class name (e.g., "hover:md:bg-blue-500" -> ["hover", "md"], "bg-blue-500")
    pub fn parse_variants_from_class(class: &str) -> (Vec<String>, String) {
        let parts: Vec<&str> = class.split(':').collect();
        if parts.len() <= 1 {
            // No variants
            return (vec![], class.to_string());
        }

        let mut variants = Vec::new();
        let mut base_class = String::new();

        // Check each prefix to see if it's a variant
        for i in 0..parts.len() {
            let prefix = parts[i];

            // Check if this prefix is a variant
            if Self::is_variant_prefix(prefix) {
                variants.push(prefix.to_string());

                // If we have more parts after this variant, continue
                if i + 1 < parts.len() {
                    // Reconstruct base class from remaining parts
                    base_class = parts[i + 1..].join(":");
                } else {
                    // This was the last part, so base class is empty (invalid)
                    base_class = String::new();
                }
            } else {
                // Not a variant, so this and remaining parts are the base class
                base_class = parts[i..].join(":");
                break;
            }
        }

        (variants, base_class)
    }

    /// Check if a string is a variant prefix
    fn is_variant_prefix(prefix: &str) -> bool {
        matches!(prefix,
            "hover" | "focus" | "active" | "visited" | "disabled" |
            "first" | "last" | "odd" | "even" | "dark" |
            "group-hover" | "group-focus" | "peer-hover" | "peer-focus" |
            "sm" | "md" | "lg" | "xl" | "2xl"
        )
    }

    /// Update utility contexts based on base class (without variants)
    fn update_utility_context(&mut self, base_class: &str) {
        // Gradient classes
        if base_class.starts_with("bg-gradient-") || base_class.starts_with("bg-linear-") ||
           base_class.starts_with("bg-conic") || base_class.starts_with("bg-radial") ||
           base_class.starts_with("from-") || base_class.starts_with("via-") || base_class.starts_with("to-") {
            self.gradients.update_from_class(base_class);
        }
        // Shadow classes
        else if base_class.starts_with("shadow-") {
            self.shadows.update_from_class(base_class);
        }
        // Transform classes
        else if base_class.starts_with("translate-") || base_class.starts_with("scale-") ||
                base_class.starts_with("rotate-") || base_class.starts_with("-rotate-") ||
                base_class.starts_with("skew-") {
            self.transforms.update_from_class(base_class);
        }
        // Filter classes
        else if base_class.starts_with("blur-") || base_class.starts_with("brightness-") ||
                base_class.starts_with("contrast-") || base_class.starts_with("grayscale") ||
                base_class.starts_with("hue-rotate-") || base_class.starts_with("invert") ||
                base_class.starts_with("saturate-") || base_class.starts_with("sepia") ||
                base_class.starts_with("drop-shadow") {
            self.filters.update_from_class(base_class);
        }
        // Animation classes
        else if base_class.starts_with("animate-") || base_class.starts_with("duration-") ||
                base_class.starts_with("delay-") || base_class.starts_with("ease-") {
            self.animations.update_from_class(base_class);
        }
        // Arbitrary value classes
        else if base_class.contains("[") && base_class.contains("]") {
            self.arbitrary_values.update_from_class(base_class);
        }
        // Custom property classes (like [--my-var:red])
        else if base_class.starts_with("[--") && base_class.contains(":") && base_class.ends_with("]") {
            self.custom_properties.update_from_class(base_class);
        }
        // Add other stateful updates here as we implement them
    }

    /// Generate CSS for all contexts (without variants)
    pub fn generate_css(&self) -> Vec<CssProperty> {
        let mut properties = Vec::new();

        // Generate gradient CSS
        if self.gradients.has_gradient() {
            properties.extend(self.gradients.to_css_properties());
        }

        // Generate shadow CSS
        if self.shadows.has_shadow() {
            properties.extend(self.shadows.to_css_properties());
        }

        // Generate transform CSS
        if self.transforms.has_transform() {
            properties.extend(self.transforms.to_css_properties());
        }

        // Generate filter CSS
        let filter_properties = self.filters.to_css_properties();
        if !filter_properties.is_empty() {
            properties.extend(filter_properties);
        }

        // Generate animation CSS
        let animation_properties = self.animations.to_css_properties();
        if !animation_properties.is_empty() {
            properties.extend(animation_properties);
        }

        // Generate arbitrary value CSS
        let arbitrary_properties = self.arbitrary_values.to_css_properties();
        if !arbitrary_properties.is_empty() {
            properties.extend(arbitrary_properties);
        }

        // Generate custom property CSS
        let custom_properties = self.custom_properties.to_css_properties();
        if !custom_properties.is_empty() {
            properties.extend(custom_properties);
        }

        // Add other context CSS generation here

        properties
    }

    /// Generate variant-aware CSS with proper selectors and media queries
    pub fn generate_variant_css(&self, base_class: &str) -> Vec<CssRule> {
        let mut rules = Vec::new();
        let properties = self.generate_css();

        if properties.is_empty() {
            return rules;
        }

        // Create the appropriate CSS rule based on variants
        if self.variants.has_variants() {
            if let Some(media_query) = self.variants.to_media_query() {
                // Responsive variant - wrap in media query
                let selector = self.variants.to_css_selector(base_class);
                rules.push(CssRule {
                    selector,
                    properties: properties.clone(),
                    media_query: Some(media_query),
                    specificity: 100, // Responsive variants have higher specificity
                });
            } else {
                // State variant - just use the selector
                let selector = self.variants.to_css_selector(base_class);
                rules.push(CssRule {
                    selector,
                    properties,
                    media_query: None,
                    specificity: 10, // State variants have medium specificity
                });
            }
        } else {
            // No variants - basic selector
            let selector = format!(".{}", base_class.replace(":", "\\:"));
            rules.push(CssRule {
                selector,
                properties,
                media_query: None,
                specificity: 1, // Base classes have low specificity
            });
        }

        rules
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gradient_context_simple() {
        let mut context = GradientContext::default();
        context.update_from_class("bg-gradient-to-r");
        context.update_from_class("from-red-500");

        let css = context.to_css_properties();
        assert!(css.iter().any(|p| p.name == "--tw-gradient-position" && p.value == "to right"));
        assert!(css.iter().any(|p| p.name == "--tw-gradient-from" && p.value == "red-500"));
        assert!(css.iter().any(|p| p.name == "background-image" && p.value == "linear-gradient(var(--tw-gradient-stops))"));
    }

    #[test]
    fn test_gradient_context_three_stops() {
        let mut context = GradientContext::default();
        context.update_from_class("bg-gradient-to-r");
        context.update_from_class("from-red-500");
        context.update_from_class("via-blue-500");
        context.update_from_class("to-green-500");

        let css = context.to_css_properties();
        assert!(css.iter().any(|p| p.name == "--tw-gradient-via-stops"));
        assert!(css.iter().any(|p| p.name == "--tw-gradient-stops" && p.value == "var(--tw-gradient-via-stops)"));
    }

    #[test]
    fn test_element_context() {
        let mut context = ElementContext::default();
        context.update_from_class("bg-gradient-to-r");
        context.update_from_class("from-pink-400");

        let css = context.generate_css();
        assert!(!css.is_empty());
        assert!(css.iter().any(|p| p.name == "--tw-gradient-from" && p.value == "pink-400"));
    }

    #[test]
    fn test_element_context_with_variants() {
        let mut context = ElementContext::default();
        context.update_from_class("hover:shadow-lg");
        context.update_from_class("md:bg-gradient-to-r");
        context.update_from_class("from-purple-500");

        let css = context.generate_css();
        assert!(!css.is_empty());

        let rules = context.generate_variant_css("test-class");
        assert!(!rules.is_empty());
        // Check that we have both hover and responsive variants
        assert!(rules.iter().any(|r| r.selector.contains(":hover")));
    }

    #[test]
    fn test_complex_element_processing() {
        let mut context = ElementContext::default();

        // Add various classes
        context.update_from_class("shadow-lg");
        context.update_from_class("translate-x-4");
        context.update_from_class("rotate-45");
        context.update_from_class("blur-sm");
        context.update_from_class("animate-spin");
        context.update_from_class("w-[100px]");
        context.update_from_class("[--my-color:red]");

        let css = context.generate_css();
        assert!(!css.is_empty());

        // Should have properties from multiple contexts
        let property_names: Vec<&str> = css.iter().map(|p| p.name.as_str()).collect();
        assert!(property_names.contains(&"box-shadow"));
        assert!(property_names.contains(&"transform"));
        assert!(property_names.contains(&"filter"));
        assert!(property_names.contains(&"animation-name"));
        assert!(property_names.contains(&"width"));
        assert!(property_names.contains(&"--my-color"));
    }
}
