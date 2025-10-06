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
                "hover" => selector_parts.push(":hover".to_string()),
                "focus" => selector_parts.push(":focus".to_string()),
                "active" => selector_parts.push(":active".to_string()),
                "dark" => selector_parts.push(".dark".to_string()),
                "sm" => {
                    // Media query handled separately
                    return Ok(format!(".{}", base_selector));
                },
                "group-hover" => selector_parts.push(".group:hover".to_string()),
                variant if variant.starts_with("@container-") => {
                    // Container queries handled separately
                    return Ok(format!(".{}", base_selector));
                },
                _ => {
                    // Unknown variant, skip
                }
            }
        }

        let variant_prefix = selector_parts.join(" ");
        Ok(format!(".{}{}{}", variant_prefix, if variant_prefix.is_empty() { "" } else { " " }, base_selector))
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
        if let Some(container_name) = variant.strip_prefix("@container-") {
            let container_query = if container_name.is_empty() {
                "@container".to_string()
            } else {
                format!("@container {}", container_name)
            };

            Ok(format!("{} {{\n{}\n}}\n", container_query, css))
        } else {
            Ok(css.to_string())
        }
    }
}
