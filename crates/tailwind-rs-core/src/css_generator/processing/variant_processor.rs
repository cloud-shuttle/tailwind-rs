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
