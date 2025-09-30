//! Theme Modules
//!
//! Comprehensive theme system implementation according to API documentation:
//! - Theme variants for component styles
//! - Spacing scales for consistent spacing
//! - Font families, sizes, and weights
//! - Color associations and utilities

use std::collections::HashMap;

// Re-export all theme utility types and traits
pub mod fonts;
pub mod spacing;
pub mod variants;

// Re-export all types for easy access
pub use fonts::{FontFamily, FontSizeScale, FontWeightScale};
pub use spacing::{SpacingScale, SpacingSize};
pub use variants::ThemeVariant;

/// Main theme system that coordinates all theme utilities
#[derive(Debug, Clone)]
pub struct ThemeSystem {
    variants: Vec<ThemeVariant>,
    spacing_scale: SpacingScale,
    font_family: FontFamily,
    font_size_scale: FontSizeScale,
    font_weight_scale: FontWeightScale,
}

impl ThemeSystem {
    /// Create a new theme system with defaults
    pub fn new() -> Self {
        Self {
            variants: ThemeVariant::variants().to_vec(),
            spacing_scale: SpacingScale::new(),
            font_family: FontFamily::Sans,
            font_size_scale: FontSizeScale::new(),
            font_weight_scale: FontWeightScale::new(),
        }
    }

    /// Create a custom theme system
    pub fn custom(
        variants: Vec<ThemeVariant>,
        spacing_scale: SpacingScale,
        font_family: FontFamily,
        font_size_scale: FontSizeScale,
        font_weight_scale: FontWeightScale,
    ) -> Self {
        Self {
            variants,
            spacing_scale,
            font_family,
            font_size_scale,
            font_weight_scale,
        }
    }

    /// Get all theme variants
    pub fn variants(&self) -> &[ThemeVariant] {
        &self.variants
    }

    /// Get the spacing scale
    pub fn spacing_scale(&self) -> &SpacingScale {
        &self.spacing_scale
    }

    /// Get the font family
    pub fn font_family(&self) -> &FontFamily {
        &self.font_family
    }

    /// Get the font size scale
    pub fn font_size_scale(&self) -> &FontSizeScale {
        &self.font_size_scale
    }

    /// Get the font weight scale
    pub fn font_weight_scale(&self) -> &FontWeightScale {
        &self.font_weight_scale
    }

    /// Set custom spacing scale
    pub fn set_spacing_scale(&mut self, scale: SpacingScale) {
        self.spacing_scale = scale;
    }

    /// Set custom font family
    pub fn set_font_family(&mut self, family: FontFamily) {
        self.font_family = family;
    }

    /// Set custom font size scale
    pub fn set_font_size_scale(&mut self, scale: FontSizeScale) {
        self.font_size_scale = scale;
    }

    /// Set custom font weight scale
    pub fn set_font_weight_scale(&mut self, scale: FontWeightScale) {
        self.font_weight_scale = scale;
    }

    /// Generate CSS custom properties for the theme
    pub fn generate_css_custom_properties(&self) -> String {
        let mut css = String::new();
        css.push_str(":root {\n");

        // Spacing custom properties
        for (size, value) in self.spacing_scale.all_values() {
            css.push_str(&format!("  --spacing-{}: {};\n", size.class_suffix(), value));
        }

        // Font size custom properties
        for (key, value) in self.font_size_scale.all_sizes() {
            css.push_str(&format!("  --font-size-{}: {};\n", key, value));
        }

        // Font weight custom properties
        for (key, value) in self.font_weight_scale.all_weights() {
            css.push_str(&format!("  --font-weight-{}: {};\n", key, value));
        }

        css.push_str("}\n");
        css
    }

    /// Generate Tailwind-compatible theme configuration
    pub fn to_tailwind_config(&self) -> HashMap<String, serde_json::Value> {
        let mut config = HashMap::new();

        // Font family
        config.insert(
            "fontFamily".to_string(),
            serde_json::json!({
                "sans": self.font_family.css_value(),
                "serif": FontFamily::Serif.css_value(),
                "mono": FontFamily::Mono.css_value()
            })
        );

        // Font size
        let mut font_sizes = HashMap::new();
        for (key, value) in self.font_size_scale.all_sizes() {
            font_sizes.insert(key.to_string(), serde_json::json!([value, { "lineHeight": "1.5" }]));
        }
        config.insert("fontSize".to_string(), serde_json::json!(font_sizes));

        // Font weight
        let mut font_weights = HashMap::new();
        for (key, value) in self.font_weight_scale.all_weights() {
            font_weights.insert(key.to_string(), value.to_string());
        }
        config.insert("fontWeight".to_string(), serde_json::json!(font_weights));

        // Spacing
        let spacing = self.spacing_scale.to_tailwind_scale();
        config.insert("spacing".to_string(), serde_json::json!(spacing));

        config
    }
}

impl Default for ThemeSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Theme utilities trait for extending ClassBuilder
pub trait ThemeUtilities {
    /// Add theme variant class
    fn variant(self, variant: ThemeVariant) -> Self;

    /// Add spacing class
    fn spacing(self, size: SpacingSize) -> Self;

    /// Add font family class
    fn font_family(self, family: FontFamily) -> Self;

    /// Add font size class
    fn font_size(self, key: &str) -> Self;

    /// Add font weight class
    fn font_weight(self, key: &str) -> Self;
}

impl ThemeUtilities for crate::classes::ClassBuilder {
    fn variant(self, variant: ThemeVariant) -> Self {
        self.class(&format!("variant-{}", variant.class_name()))
    }

    fn spacing(self, size: SpacingSize) -> Self {
        self.class(&format!("spacing-{}", size.class_suffix()))
    }

    fn font_family(self, family: FontFamily) -> Self {
        self.class(family.class())
    }

    fn font_size(self, key: &str) -> Self {
        self.class(&format!("text-{}", key))
    }

    fn font_weight(self, key: &str) -> Self {
        self.class(&format!("font-{}", key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_system_creation() {
        let theme = ThemeSystem::new();

        assert_eq!(theme.variants().len(), 8);
        assert_eq!(theme.spacing_scale().get(SpacingSize::Md), "1rem");
        assert_eq!(theme.font_family().name(), "sans");
    }

    #[test]
    fn theme_system_custom() {
        let spacing = SpacingScale::custom("1px", "2px", "3px", "4px", "5px", "6px", "7px");
        let theme = ThemeSystem::custom(
            vec![ThemeVariant::Primary],
            spacing,
            FontFamily::Mono,
            FontSizeScale::new(),
            FontWeightScale::new(),
        );

        assert_eq!(theme.variants().len(), 1);
        assert_eq!(theme.spacing_scale().get(SpacingSize::Md), "3px");
        assert_eq!(theme.font_family().name(), "mono");
    }

    #[test]
    fn theme_system_modification() {
        let mut theme = ThemeSystem::new();

        let custom_spacing = SpacingScale::custom("10px", "20px", "30px", "40px", "50px", "60px", "70px");
        theme.set_spacing_scale(custom_spacing);
        theme.set_font_family(FontFamily::Serif);

        assert_eq!(theme.spacing_scale().get(SpacingSize::Xs), "10px");
        assert_eq!(theme.font_family().name(), "serif");
    }

    #[test]
    fn theme_system_css_generation() {
        let theme = ThemeSystem::new();
        let css = theme.generate_css_custom_properties();

        assert!(css.contains(":root {"));
        assert!(css.contains("--spacing-md: 1rem;"));
        assert!(css.contains("--font-size-base: 1rem;"));
        assert!(css.contains("--font-weight-normal: 400;"));
        assert!(css.contains("}"));
    }

    #[test]
    fn theme_system_tailwind_config() {
        let theme = ThemeSystem::new();
        let config = theme.to_tailwind_config();

        assert!(config.contains_key("fontFamily"));
        assert!(config.contains_key("fontSize"));
        assert!(config.contains_key("fontWeight"));
        assert!(config.contains_key("spacing"));
    }

    #[test]
    fn theme_utilities_trait() {
        use crate::classes::ClassBuilder;

        let builder = ClassBuilder::new();

        // Test trait methods (simplified - would need actual ClassBuilder implementation)
        let _result = builder
            .variant(ThemeVariant::Primary)
            .spacing(SpacingSize::Md)
            .font_family(FontFamily::Mono)
            .font_size("lg")
            .font_weight("bold");
        // In a real implementation, this would add classes to the builder
    }
}
