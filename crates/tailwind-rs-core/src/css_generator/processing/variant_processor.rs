//! Variant combination and processing
//! Handles responsive, state, and custom variants

use crate::error::Result;

/// Variant processor for handling CSS variants
#[derive(Debug)]
pub struct VariantProcessor {
    // This would contain variant processing logic
}

impl VariantProcessor {
    /// Create a new variant processor
    pub fn new() -> Self {
        Self {}
    }

    /// Combine variants into CSS selector
    pub fn build_css_selector(&self, base_selector: &str, variants: &[String]) -> Result<String> {
        if variants.is_empty() {
            return Ok(format!(".{}", base_selector));
        }

        // Build selector from variants
        let mut selector_parts = Vec::new();

        for variant in variants {
            match variant.as_str() {
                // Basic state variants
                "hover" => selector_parts.push(":hover".to_string()),
                "focus" => selector_parts.push(":focus".to_string()),
                "active" => selector_parts.push(":active".to_string()),
                "visited" => selector_parts.push(":visited".to_string()),
                "disabled" => selector_parts.push(":disabled".to_string()),

                // Dark mode
                "dark" => selector_parts.push(".dark".to_string()),

                // Group variants
                "group-hover" => selector_parts.push(".group:hover".to_string()),
                "group-focus" => selector_parts.push(".group:focus".to_string()),
                "group-active" => selector_parts.push(".group:active".to_string()),
                "peer-hover" => selector_parts.push(".peer:hover".to_string()),
                "peer-focus" => selector_parts.push(".peer:focus".to_string()),
                "peer-active" => selector_parts.push(".peer:active".to_string()),

                // Device and media variants - handled separately
                variant if self.is_device_variant(variant) => {
                    return Ok(format!(".{}", base_selector));
                },

                // Container queries - handled separately
                variant if variant.starts_with("@container-") => {
                    return Ok(format!(".{}", base_selector));
                },

                // Arbitrary variants
                variant if variant.starts_with('[') && variant.ends_with("]:") => {
                    let arbitrary_selector = &variant[1..variant.len() - 2]; // Remove [ and ]:
                    selector_parts.push(format!("[{}]", arbitrary_selector));
                },

                // Responsive variants - handled separately
                "sm" | "md" | "lg" | "xl" | "2xl" => {
                    return Ok(format!(".{}", base_selector));
                },

                _ => {
                    // Custom variants or unknown - try to handle as custom
                    // For now, skip unknown variants
                }
            }
        }

        let variant_prefix = selector_parts.join(" ");
        Ok(format!(".{}{}{}", variant_prefix, if variant_prefix.is_empty() { "" } else { " " }, base_selector))
    }

    /// Check if variant is a device/media variant
    fn is_device_variant(&self, variant: &str) -> bool {
        matches!(variant, "pointer-coarse:" | "pointer-fine:" | "pointer-none:" |
                       "hover-none:" | "hover-hover:" | "motion-reduce:" |
                       "motion-safe:" | "contrast-more:" | "contrast-less:" |
                       "contrast-custom:" | "color-gamut-srgb:" | "color-gamut-p3:" |
                       "color-gamut-rec2020:" | "orientation-portrait:" |
                       "orientation-landscape:" | "print:" | "screen:")
    }

    /// Apply responsive variants
    pub fn apply_responsive_variant(&self, variant: &str, css: &str) -> Result<String> {
        let media_query = match variant {
            "sm" => "@media (min-width: 640px)",
            "md" => "@media (min-width: 768px)",
            "lg" => "@media (min-width: 1024px)",
            "xl" => "@media (min-width: 1280px)",
            "2xl" => "@media (min-width: 1536px)",
            _ => return Ok(css.to_string()),
        };

        Ok(format!("{} {{\n{}\n}}\n", media_query, css))
    }

    /// Apply state variants (hover, focus, etc.)
    pub fn apply_state_variant(&self, variant: &str, css: &str) -> Result<String> {
        let pseudo_class = match variant {
            "hover" => ":hover",
            "focus" => ":focus",
            "active" => ":active",
            "visited" => ":visited",
            "disabled" => ":disabled",
            _ => return Ok(css.to_string()),
        };

        // Simple implementation - in reality this would need more sophisticated parsing
        Ok(css.replace("}", &format!("}}\n\n{}{{", pseudo_class)))
    }

    /// Handle container query variants
    pub fn apply_container_variant(&self, variant: &str, css: &str) -> Result<String> {
        if let Some(container_spec) = variant.strip_prefix("@container-") {
            // Parse container specification (e.g., "sm", "md:min-w-300", etc.)
            let (container_name, condition) = if let Some((name, cond)) = container_spec.split_once(':') {
                (name, Some(cond))
            } else {
                (container_spec, None)
            };

            let container_selector = if container_name.is_empty() {
                "@container".to_string()
            } else {
                format!("@container {}", container_name)
            };

            let condition_clause = if let Some(cond) = condition {
                format!(" ({})", cond)
            } else {
                "".to_string()
            };

            Ok(format!("{}{} {{\n{}\n}}\n", container_selector, condition_clause, css))
        } else {
            Ok(css.to_string())
        }
    }

    /// Handle arbitrary value variants
    pub fn apply_arbitrary_variant(&self, variant: &str, css: &str) -> Result<String> {
        // Handle arbitrary variants like [data-value="test"]:, [aria-expanded="true"]:, etc.
        if variant.starts_with('[') && variant.ends_with(']') && variant.ends_with("]:") {
            let arbitrary_selector = &variant[1..variant.len() - 2]; // Remove [ and ]:
            Ok(format!("[{}] {{\n{}\n}}\n", arbitrary_selector, css))
        } else {
            Ok(css.to_string())
        }
    }

    /// Handle custom variants
    pub fn apply_custom_variant(&self, variant: &str, css: &str) -> Result<String> {
        // Handle custom variants defined in config
        // For now, pass through as-is - would be extended with config support
        Ok(css.to_string())
    }

    /// Handle advanced device variants
    pub fn apply_device_variant(&self, variant: &str, css: &str) -> Result<String> {
        match variant {
            "pointer-coarse:" => Ok(format!("@media (pointer: coarse) {{\n{}\n}}\n", css)),
            "pointer-fine:" => Ok(format!("@media (pointer: fine) {{\n{}\n}}\n", css)),
            "pointer-none:" => Ok(format!("@media (pointer: none) {{\n{}\n}}\n", css)),
            "hover-none:" => Ok(format!("@media (hover: none) {{\n{}\n}}\n", css)),
            "hover-hover:" => Ok(format!("@media (hover: hover) {{\n{}\n}}\n", css)),
            "motion-reduce:" => Ok(format!("@media (prefers-reduced-motion: reduce) {{\n{}\n}}\n", css)),
            "motion-safe:" => Ok(format!("@media (prefers-reduced-motion: no-preference) {{\n{}\n}}\n", css)),
            "contrast-more:" => Ok(format!("@media (prefers-contrast: more) {{\n{}\n}}\n", css)),
            "contrast-less:" => Ok(format!("@media (prefers-contrast: less) {{\n{}\n}}\n", css)),
            "contrast-custom:" => Ok(format!("@media (prefers-contrast: custom) {{\n{}\n}}\n", css)),
            "color-gamut-srgb:" => Ok(format!("@media (color-gamut: srgb) {{\n{}\n}}\n", css)),
            "color-gamut-p3:" => Ok(format!("@media (color-gamut: p3) {{\n{}\n}}\n", css)),
            "color-gamut-rec2020:" => Ok(format!("@media (color-gamut: rec2020) {{\n{}\n}}\n", css)),
            "orientation-portrait:" => Ok(format!("@media (orientation: portrait) {{\n{}\n}}\n", css)),
            "orientation-landscape:" => Ok(format!("@media (orientation: landscape) {{\n{}\n}}\n", css)),
            "print:" => Ok(format!("@media print {{\n{}\n}}\n", css)),
            "screen:" => Ok(format!("@media screen {{\n{}\n}}\n", css)),
            _ => Ok(css.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Result;

    #[test]
    fn test_basic_selector_building() -> Result<()> {
        let processor = VariantProcessor::new();

        let selector = processor.build_css_selector("bg-blue-500", &[])?;
        assert_eq!(selector, ".bg-blue-500");

        Ok(())
    }

    #[test]
    fn test_state_variant_selectors() -> Result<()> {
        let processor = VariantProcessor::new();

        let selector = processor.build_css_selector("bg-blue-600", &["hover".to_string()])?;
        assert_eq!(selector, ".hover\\:bg-blue-600:hover");

        let selector = processor.build_css_selector("text-red-500", &["focus".to_string()])?;
        assert_eq!(selector, ".focus\\:text-red-500:focus");

        Ok(())
    }

    #[test]
    fn test_group_variant_selectors() -> Result<()> {
        let processor = VariantProcessor::new();

        let selector = processor.build_css_selector("bg-blue-600", &["group-hover".to_string()])?;
        assert_eq!(selector, ".group-hover\\:bg-blue-600 .group:hover");

        let selector = processor.build_css_selector("text-red-500", &["peer-focus".to_string()])?;
        assert_eq!(selector, ".peer-focus\\:text-red-500 .peer:focus");

        Ok(())
    }

    #[test]
    fn test_arbitrary_value_variants() -> Result<()> {
        let processor = VariantProcessor::new();

        let selector = processor.build_css_selector("bg-green-500", &["[data-state=\"open\"]".to_string()])?;
        assert_eq!(selector, ".[data-state\\=\\\"open\\\"]\\:bg-green-500 [data-state=\"open\"]");

        Ok(())
    }

    #[test]
    fn test_combined_variants() -> Result<()> {
        let processor = VariantProcessor::new();

        let selector = processor.build_css_selector("bg-blue-600", &["hover".to_string(), "sm".to_string()])?;
        // Responsive variants are handled separately, so should just be hover
        assert_eq!(selector, ".hover\\:bg-blue-600:hover");

        Ok(())
    }

    #[test]
    fn test_responsive_variant_media_queries() -> Result<()> {
        let processor = VariantProcessor::new();

        let media_query = processor.apply_responsive_variant("sm", ".bg-red-500 { background-color: red; }")?;
        assert!(media_query.contains("@media (min-width: 640px)"));
        assert!(media_query.contains(".bg-red-500"));

        let media_query = processor.apply_responsive_variant("md", ".text-blue-500 { color: blue; }")?;
        assert!(media_query.contains("@media (min-width: 768px)"));

        Ok(())
    }

    #[test]
    fn test_container_query_variants() -> Result<()> {
        let processor = VariantProcessor::new();

        let css = processor.apply_container_variant("@container-sm", ".bg-green-500 { background-color: green; }")?;
        assert!(css.contains("@container"));
        assert!(!css.contains("(min-width"));
        assert!(css.contains(".bg-green-500"));

        let css = processor.apply_container_variant("@container-md:min-w-300", ".text-blue-600 { color: blue; }")?;
        assert!(css.contains("@container md"));
        assert!(css.contains("(min-width: 300px)"));

        Ok(())
    }

    #[test]
    fn test_device_variant_media_queries() -> Result<()> {
        let processor = VariantProcessor::new();

        let css = processor.apply_device_variant("motion-reduce:", ".opacity-50 { opacity: 0.5; }")?;
        assert!(css.contains("@media (prefers-reduced-motion: reduce)"));
        assert!(css.contains(".opacity-50"));

        let css = processor.apply_device_variant("pointer-coarse:", ".p-6 { padding: 1.5rem; }")?;
        assert!(css.contains("@media (pointer: coarse)"));

        let css = processor.apply_device_variant("contrast-more:", ".text-lg { font-size: 1.125rem; }")?;
        assert!(css.contains("@media (prefers-contrast: more)"));

        Ok(())
    }

    #[test]
    fn test_pseudo_class_variant_application() -> Result<()> {
        let processor = VariantProcessor::new();

        let css = processor.apply_pseudo_class_variant("hover", ".bg-blue-500 { background-color: blue; }")?;
        assert!(css.contains(".bg-blue-500:hover"));
        assert!(css.contains("background-color: blue"));

        Ok(())
    }

    #[test]
    fn test_is_device_variant_detection() {
        let processor = VariantProcessor::new();

        assert!(processor.is_device_variant("motion-reduce:"));
        assert!(processor.is_device_variant("pointer-coarse:"));
        assert!(processor.is_device_variant("contrast-more:"));
        assert!(processor.is_device_variant("orientation-landscape:"));
        assert!(processor.is_device_variant("print:"));
        assert!(processor.is_device_variant("screen:"));

        assert!(!processor.is_device_variant("hover"));
        assert!(!processor.is_device_variant("sm"));
        assert!(!processor.is_device_variant("@container-sm"));
    }

    #[test]
    fn test_unknown_variants() -> Result<()> {
        let processor = VariantProcessor::new();

        // Unknown variants should be ignored
        let selector = processor.build_css_selector("bg-blue-500", &["unknown-variant".to_string()])?;
        assert_eq!(selector, ".bg-blue-500");

        Ok(())
    }

    #[test]
    fn test_responsive_variants_are_handled_separately() -> Result<()> {
        let processor = VariantProcessor::new();

        // Responsive variants return base selector since they're handled by media queries
        let selector = processor.build_css_selector("bg-red-500", &["sm".to_string()])?;
        assert_eq!(selector, ".bg-red-500");

        Ok(())
    }

    #[test]
    fn test_container_variants_are_handled_separately() -> Result<()> {
        let processor = VariantProcessor::new();

        // Container variants return base selector since they're handled separately
        let selector = processor.build_css_selector("bg-green-500", &["@container-sm".to_string()])?;
        assert_eq!(selector, ".bg-green-500");

        Ok(())
    }

    #[test]
    fn test_complex_variant_combinations() -> Result<()> {
        let processor = VariantProcessor::new();

        let selector = processor.build_css_selector("bg-blue-600", &[
            "hover".to_string(),
            "group-focus".to_string(),
            "[data-active=\"true\"]".to_string()
        ])?;
        assert!(selector.contains(":hover"));
        assert!(selector.contains(".group:focus"));
        assert!(selector.contains("[data-active=\"true\"]"));

        Ok(())
    }
}
